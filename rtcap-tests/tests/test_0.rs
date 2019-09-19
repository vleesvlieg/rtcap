#[test]
fn test_0() {
    //panic!("No tests");
}

#[test]
fn test_bind() {
    let rs = rtcap::RawSocket::new_non_blocking().unwrap();
    let result = rs.bind("enp4s0");
    assert!(result.is_ok());
}
