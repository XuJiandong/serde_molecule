use crate::error::Error;
use alloc::vec;
use alloc::vec::Vec;

const NUMBER_SIZE: usize = 4;

pub(crate) const MOLECULE_SER: &str = "MoleculeSerializer";
pub(crate) const MOLECULE_DE: &str = "MoleculeDeserializer";

///
/// Assemble molecule table or dynvec. See
/// <https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md#table>
/// The table is a dynamic-size type. It can be considered as a dynvec but the length is fixed.
/// The serializing steps are same as dynvec:
/// * Serialize the full size in bytes as a 32 bit unsigned integer in little-endian.
/// * Serialize all offset of fields as 32 bit unsigned integer in little-endian.
/// * Serialize all fields in it in the order they are declared.
///
pub fn assemble_table(parts: &[Vec<u8>]) -> Vec<u8> {
    let header_len = parts.len() + 1;
    let mut header = vec![0u32; header_len];
    let mut offset = (header_len * 4) as u32;
    for i in 1..header_len {
        header[i] = offset;
        offset += parts[i - 1].len() as u32;
    }
    header[0] = offset;
    let mut result = vec![];
    header
        .into_iter()
        .map(|u| u.to_le_bytes().to_vec())
        .fold(&mut result, |acc, item| {
            acc.extend(item);
            acc
        });
    parts.iter().fold(&mut result, |acc, item| {
        acc.extend(item);
        acc
    });
    result
}

/// Assemble molecule fixvec. See
/// <https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md#fixvec---fixed-vector>
/// There are two steps of serializing a fixvec:
/// * Serialize the length as a 32 bit unsigned integer in little-endian.
/// * Serialize all items in it.
pub fn assemble_fixvec(parts: &Vec<Vec<u8>>) -> Result<Vec<u8>, Error> {
    if parts.len() > 1 {
        let len = parts[0].len();
        for item in parts {
            if len != item.len() {
                return Err(Error::AssembleFixvec);
            }
        }
    }
    if !parts.is_empty() && parts.iter().all(|e| e.is_empty()) {
        return Err(Error::AssembleFixvec);
    }
    let mut result = vec![];
    let len = parts.len() as u32;
    result.extend(len.to_le_bytes());
    parts.iter().fold(&mut result, |acc, item| {
        acc.extend(item);
        acc
    });
    Ok(result)
}

/// assemble molecule struct
pub fn assemble_struct(parts: Vec<Vec<u8>>) -> Vec<u8> {
    let mut result = vec![];
    parts.into_iter().fold(&mut result, |acc, item| {
        acc.extend(item);
        acc
    });
    result
}

pub fn unpack_number(data: &[u8], offset: usize) -> Result<usize, Error> {
    if data.len() < (4 + offset) {
        Err(Error::LengthNotEnough)
    } else {
        let bytes: [u8; 4] = data[offset..offset + 4].try_into().unwrap();
        Ok(u32::from_le_bytes(bytes) as usize)
    }
}

/// Disassemble molecule table or dynvec
pub fn disassemble_fixvec(data: &[u8]) -> Result<Vec<&[u8]>, Error> {
    let item_count = unpack_number(data, 0)?;
    if item_count == 0 {
        return Ok(vec![]);
    }
    let remaining = data.len().checked_sub(4).ok_or(Error::Overflow)?;
    if remaining == 0 {
        return Err(Error::InvalidFixvec);
    }
    if remaining % item_count != 0 {
        return Err(Error::InvalidFixvec);
    }
    let item_size = remaining / item_count;
    let mut result = vec![];
    for i in 0..item_count {
        result.push(&data[4 + i * item_size..4 + (i + 1) * item_size]);
    }

    Ok(result)
}

/// Disassemble molecule table or dynvec
pub fn disassemble_table(data: &[u8]) -> Result<Vec<&[u8]>, Error> {
    let mut result = vec![];
    let total_size = unpack_number(data, 0)?;
    if data.len() != total_size {
        return Err(Error::InvalidTableLength);
    }
    if total_size == NUMBER_SIZE {
        return Ok(result);
    }
    if total_size < NUMBER_SIZE * 2 {
        return Err(Error::InvalidTableLength);
    }
    let mut cur = 0;
    cur += NUMBER_SIZE;
    let first_offset = unpack_number(data, cur)?;
    if first_offset % NUMBER_SIZE != 0 || first_offset < NUMBER_SIZE * 2 {
        return Err(Error::InvalidTableHeader);
    }
    if total_size < first_offset {
        return Err(Error::InvalidTableHeader);
    }
    let count = first_offset / 4 - 1;
    let mut last_offset = first_offset;
    let mut offset = first_offset;
    cur += NUMBER_SIZE;
    for _ in 1..count {
        offset = unpack_number(data, cur)?;
        if last_offset > offset {
            return Err(Error::InvalidTable);
        }
        if offset > data.len() {
            return Err(Error::InvalidTable);
        }
        result.push(&data[last_offset..offset]);
        last_offset = offset;
        cur += NUMBER_SIZE;
    }
    result.push(&data[offset..]);

    Ok(result)
}
