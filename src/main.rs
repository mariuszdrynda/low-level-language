#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar);
fn main() {
    ()
}
#[test]
fn lall_grammar_test() {
    assert!(grammar::TopParser::new().parse("fn main()->(){declare_i32(celsius);}").is_ok());
    assert!(grammar::TopParser::new().parse("fn main()->(){return(0);}").is_ok());
    assert!(grammar::TopParser::new().parse("include(printf) declare_set_global(a, 10)").is_ok());
    assert!(grammar::TopParser::new().parse("declare_set_global(a, 's')").is_ok());
    assert!(grammar::TopParser::new().parse(r#"declare_set_global(a, "")"#).is_ok());
    assert!(grammar::TopParser::new().parse(r#"declare_set_global(a, "\n\tset ")"#).is_ok());
    assert!(grammar::TopParser::new().parse("fn main()->(){declare_set_i32(lower, 0i32); declare_set_i32(upper, 300); declare_set_i32(step, 20);}").is_ok());
    assert!(grammar::TopParser::new().parse("fn main()->(){set_f32(celsius, mul_f32(div_f32(5.0, 9.0), sub_f32(fahr, 32.0)));}").is_ok());
    assert!(grammar::TopParser::new().parse("fn main()->(){declare_set_f32(fahr, cast_i32_f32(lower)); loop l1{nop(());}}").is_ok());
    // assert!(grammar::TopParser::new().parse("declare_set_global(a, '\n')").is_ok());
    // assert!(grammar::TopParser::new().parse("fn main()->(){case(leq_f32(fahr, upper)){false -> {break(l1);} true -> {nop(());}}}").is_ok());
    // assert!(grammar::TopParser::new().parse("fn main()->(){printf(\"%3.0f %6.1f\n\", fahr, celsius);}").is_ok());
    //TODO: all files with .lll extension from /examples folder
}