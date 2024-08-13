// assemble molecule table or dynvec
// https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md#table
// The table is a dynamic-size type. It can be considered as a dynvec but the length is fixed.
// The serializing steps are same as dynvec:
// Serialize the full size in bytes as a 32 bit unsigned integer in little-endian.
// Serialize all offset of fields as 32 bit unsigned integer in little-endian.
// Serialize all fields in it in the order they are declared.
//
pub fn assemble_table(parts: Vec<Vec<u8>>) -> Vec<u8> {
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
    parts.into_iter().fold(&mut result, |acc, item| {
        acc.extend(item);
        acc
    });
    result
}

// assemble molecule fixvec
// https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md#fixvec---fixed-vector
// There are two steps of serializing a fixvec:
// Serialize the length as a 32 bit unsigned integer in little-endian.
// Serialize all items in it.
pub fn assemble_fixvec(parts: Vec<Vec<u8>>) -> Vec<u8> {
    if parts.len() > 1 {
        let len = parts[0].len();
        for item in &parts {
            assert_eq!(item.len(), len);
        }
    }

    let mut result = vec![];
    let len = parts.len() as u32;
    result.extend(len.to_le_bytes());
    parts.into_iter().fold(&mut result, |acc, item| {
        acc.extend(item);
        acc
    });
    result
}
// assemble molecule struct
pub fn assemble_struct(parts: Vec<Vec<u8>>) -> Vec<u8> {
    let mut result = vec![];
    parts.into_iter().fold(&mut result, |acc, item| {
        acc.extend(item);
        acc
    });
    result
}
