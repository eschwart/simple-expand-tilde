#[test]
fn expand_tilde_test() {
    let expected = std::env::home_dir().map(|p| p.join("foo"));
    let resulted = simple_expand_tilde::expand_tilde("~/foo");
    assert_eq!(expected, resulted)
}
