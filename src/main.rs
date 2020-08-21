#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar);
fn main() {
    ()
}
#[test]
fn lall_grammar_test() {
    assert!(grammar::TopParser::new().parse("22").is_ok());
    assert!(grammar::TopParser::new().parse("(22)").is_ok());
    assert!(grammar::TopParser::new().parse("((((22))))").is_ok());
    assert!(grammar::TopParser::new().parse("((22)").is_err());
}