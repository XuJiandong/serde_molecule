use crate::molecule::{assemble_fixvec, assemble_table, disassemble_fixvec, disassemble_table};

#[test]
fn test_disassemble_fixvec() {
    let data: [u8; 0] = [];
    let result = disassemble_fixvec(&data);
    assert!(result.is_err());
    let data = [0, 0];
    let result = disassemble_fixvec(&data);
    assert!(result.is_err());
    let data = [0u8; 4];
    let result = disassemble_fixvec(&data);
    assert_eq!(result.unwrap().len(), 0);
    let data = [0x01, 0x00, 0x00, 0x00, 0x12];
    let result = disassemble_fixvec(&data);
    assert_eq!(result.unwrap().len(), 1);
    let data = [
        0x08, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
    ];
    let result = disassemble_fixvec(&data);
    assert_eq!(result.unwrap().len(), 8);
    let data = [0x01, 0x00, 0x00, 0x00, 0x23, 0x01, 0x00, 0x00];
    let result = disassemble_fixvec(&data);
    let result = result.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].len(), 4);
    let data = [
        0x06, 0x00, 0x00, 0x00, 0x23, 0x01, 0x00, 0x00, 0x56, 0x04, 0x00, 0x00, 0x90, 0x78, 0x00,
        0x00, 0x0a, 0x00, 0x00, 0x00, 0xbc, 0x00, 0x00, 0x00, 0xef, 0x0d, 0x00, 0x00,
    ];
    let result = disassemble_fixvec(&data);
    let result = result.unwrap();
    assert_eq!(result.len(), 6);
    assert_eq!(result[5].len(), 4);
}

fn assert_fixvec_result(d1: &[u8], d2: Vec<&[u8]>) {
    let data: Vec<u8> = d2.into_iter().fold(vec![], |mut acc, elem| {
        acc.extend(elem);
        acc
    });
    assert_eq!(&d1[4..], &data);
}

fn test_fixvec(data: Vec<Vec<u8>>) {
    let result = assemble_fixvec(&data);
    let result = result.unwrap();
    let result2 = disassemble_fixvec(&result);
    let result2 = result2.unwrap();
    assert_fixvec_result(&result, result2);
}

#[test]
fn test_assemble_fixvec() {
    let result = assemble_fixvec(&vec![vec![], vec![1]]);
    assert!(result.is_err());

    test_fixvec(vec![]);
    test_fixvec(vec![vec![]]);
    test_fixvec(vec![vec![1]]);
    test_fixvec(vec![vec![], vec![]]);
    test_fixvec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    test_fixvec(vec![vec![1], vec![2], vec![3]]);
    test_fixvec(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
}

#[test]
fn test_disassemble_table() {
    let data = [];
    let result = disassemble_table(&data);
    assert!(result.is_err());

    let data = [1];
    let result = disassemble_table(&data);
    assert!(result.is_err());

    let data = [1, 1, 1, 1, 1];
    let result = disassemble_table(&data);
    assert!(result.is_err());

    let data = [1, 1, 1, 1, 1, 1, 1];
    let result = disassemble_table(&data);
    assert!(result.is_err());

    let data = [0x04, 0x00, 0x00, 0x00];
    let result = disassemble_table(&data);
    let result = result.unwrap();
    assert_eq!(result.len(), 0);

    let data = [
        0x0e, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x12, 0x34,
    ];
    let result = disassemble_table(&data);
    let result = result.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].len(), 6);

    let data = [
        0x34, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00,
        0x00, 0x28, 0x00, 0x00, 0x00, 0x2d, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x12, 0x34,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x05, 0x67, 0x01, 0x00, 0x00, 0x00, 0x89,
        0x03, 0x00, 0x00, 0x00, 0xab, 0xcd, 0xef,
    ];
    let result = disassemble_table(&data);
    let result = result.unwrap();
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].len(), 6);
    assert_eq!(result[1].len(), 4);
    assert_eq!(result[2].len(), 6);
    assert_eq!(result[3].len(), 5);
    assert_eq!(result[4].len(), 7);
}

fn test_table(data: &Vec<Vec<u8>>) {
    let result1 = assemble_table(data);
    let result2 = disassemble_table(&result1);
    let result2 = result2.unwrap();
    let result2: Vec<_> = result2.into_iter().map(|e| e.to_vec()).collect();
    assert_eq!(data.len(), result2.len());
    for (index, item) in data.iter().enumerate() {
        assert_eq!(item, &result2[index]);
    }
}

#[test]
fn test_assemble_table() {
    test_table(&vec![]);
    test_table(&vec![vec![]]);
    test_table(&vec![vec![1]]);
    test_table(&vec![vec![1, 2, 3]]);
    test_table(&vec![vec![], vec![]]);
    test_table(&vec![vec![], vec![1]]);
    test_table(&vec![vec![1], vec![]]);
    test_table(&vec![vec![], vec![1], vec![2]]);
    test_table(&vec![vec![1], vec![], vec![2]]);
    test_table(&vec![vec![1], vec![2], vec![]]);
    test_table(&vec![vec![1], vec![2], vec![3]]);
    test_table(&vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
}
