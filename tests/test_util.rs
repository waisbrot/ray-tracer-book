use book_renderer::util::feq;

#[test]
fn test_feq() {
    assert!(feq(&0.0, &0.0));
    assert!(feq(&0.0, &5.0e-10));
    assert!(feq(&1.0, &1.00000001));
    assert!(!feq(&1.0, &1.001));
}
