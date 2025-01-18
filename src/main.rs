use std::fmt;
use std::io::Read;

#[derive(Debug)]
struct Input {
    txid: [u8; 32],
    output_index: u32,
    sequence: u32,
}
#[allow(unused)]
fn read_version(byte_slice: &mut &[u8]) -> u32 {
    let mut buffer: [u8; 4] = [0; 4];
    byte_slice.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
}
#[allow(unused)]
fn read_compact(byte_slice: &mut &[u8]) -> u64 {
    let mut compact: [u8; 1] = [0; 1];
    byte_slice.read(&mut compact).unwrap();
    if (0..253).contains(&compact[0]) {
        return compact[0] as u64;
    } else if compact[0] == 253 {
        let mut buffer: [u8; 2] = [0; 2];
        byte_slice.read(&mut buffer).unwrap();
        return u16::from_le_bytes(buffer) as u64;
    }
    compact[0] as u64
}
#[allow(unused)]
// fn main() {
//     let txn_hex = "02000000090101f900f89be2380933f6d8c7df5b9012730e742fdb068e454be12245537715ee660100000000ffffffff023e9a050000000000160014187c612bec9e7fd60a80cd5c3731099628b05212bd441100000000001600143617fa7957a69770db16822dc579cd3ed4b6e38002483045022100e98c522cb6ac06ae6bd6d8b67ea9d5a26da43d5ee113efbdc7db92f314d32faa02203131fc60ca4e3717f63459e87754a806722a147e5771af48019aa5914d2094110121034e149c95b1935a2d9e51ea1e211c560e591969fd00a2a4b9a7272f6a795bbccc00000000";
//     let transaction_bytes = hex::decode(txn_hex).unwrap();
//     let mut byte_slice = transaction_bytes.as_slice();
//     let version = read_version(&mut byte_slice);
//     println!("version is {}: ", version);
//     let input_len = read_compact(&mut byte_slice);
//     println!("input len is: {}", input_len);
//     let input = Input {
//         txid: [0_8; 32],
//         output_index: 0,
//         sequence: 0,
//     };
//     println!("input is: {:?}", input);
// }
struct MyVec(Vec<u8>);
impl fmt::Display for MyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Values: \n")?;
        for v in &self.0 {
            write!(f, "{}\n", v)?;
        }
        Ok(())
    }
}

fn main() {
    let vec = MyVec(vec![1, 2, 3]);
    println!("Vec: {}", vec);
}

#[cfg(test)]
mod test {
    use super::read_compact;

    #[test]
    fn test_compact_size() {
        let mut bytes = [1_u8].as_slice();
        let cnt = read_compact(&mut bytes);
        assert_eq!(cnt, 1_u64);
    }
}
