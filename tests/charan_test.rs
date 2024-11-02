use matra_lib::Charan;
#[test]
fn test_charan_from_str() {
    let charan = Charan::from_str("जय श्री राम");
    assert_eq!(charan.shabds.len(), 3);
    assert_eq!(charan.shabds[0].matra + charan.shabds[1].matra + charan.shabds[2].matra, charan.matra);
}
