use std::io::Read;
#[allow(unused)]
fn read_version(byte_slice: &mut &[u8]) -> u32 {
    let mut buffer: [u8; 4] = [0; 4];
    byte_slice.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
}
#[allow(unused)]
fn main() {
    let txn_hex = "02000000090101f900f89be2380933f6d8c7df5b9012730e742fdb068e454be12245537715ee660100000000ffffffff023e9a050000000000160014187c612bec9e7fd60a80cd5c3731099628b05212bd441100000000001600143617fa7957a69770db16822dc579cd3ed4b6e38002483045022100e98c522cb6ac06ae6bd6d8b67ea9d5a26da43d5ee113efbdc7db92f314d32faa02203131fc60ca4e3717f63459e87754a806722a147e5771af48019aa5914d2094110121034e149c95b1935a2d9e51ea1e211c560e591969fd00a2a4b9a7272f6a795bbccc00000000";
    let transaction_bytes = hex::decode(txn_hex).unwrap();
    let mut byte_slice = transaction_bytes.as_slice();
    let version = read_version(&mut byte_slice);
    println!("version is {}: ", version);
    println!("next element is {}: ", byte_slice[0]);
}
