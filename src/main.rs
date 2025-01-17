#[allow(unused)]
fn read_version(txn_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(txn_hex).unwrap();
    let ans = <[u8; 4]>::try_from(&transaction_bytes[0..4]).unwrap();
    u32::from_le_bytes(ans)
}
#[allow(unused)]
fn main() {
    let version=read_version("02000000000101f900f89be2380933f6d8c7df5b9012730e742fdb068e454be12245537715ee660100000000ffffffff023e9a050000000000160014187c612bec9e7fd60a80cd5c3731099628b05212bd441100000000001600143617fa7957a69770db16822dc579cd3ed4b6e38002483045022100e98c522cb6ac06ae6bd6d8b67ea9d5a26da43d5ee113efbdc7db92f314d32faa02203131fc60ca4e3717f63459e87754a806722a147e5771af48019aa5914d2094110121034e149c95b1935a2d9e51ea1e211c560e591969fd00a2a4b9a7272f6a795bbccc00000000");
    println!("version: {:?}", version);
}
