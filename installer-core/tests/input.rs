use installer_core::input::printable_char;

#[test]
fn accepts_printable_letters() {
    assert_eq!(printable_char("a"), Some('a'));
    assert_eq!(printable_char("ś"), Some('ś'));
    assert_eq!(printable_char("你"), Some('你'));
    assert_eq!(printable_char("z"), Some('z'));
}

#[test]
fn accepts_symbols_and_space() {
    assert_eq!(printable_char("-"), Some('-'));
    assert_eq!(printable_char(" "), Some(' '));
    assert_eq!(printable_char("?"), Some('?'));
}

#[test]
fn rejects_non_printable_keys() {
    assert_eq!(printable_char("\t"), None);
    assert_eq!(printable_char("\u{10}"), None);
    assert_eq!(printable_char("\u{7f}"), None);
    assert_eq!(printable_char("\u{f703}"), None);
}

#[test]
fn rejects_multi_char_input() {
    assert_eq!(printable_char(""), None);
    assert_eq!(printable_char("ab"), None);
}