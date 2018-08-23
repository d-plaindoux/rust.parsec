use parsecute::parsers::basic::*;
use parsecute::parsers::execution::*;
use parsecute::parsers::response::*;

#[test]
fn it_parse_with_returns() {
    let r = returns(1);

    assert_eq!(1, r.execute(&"a", 0).fold(
        |a: u32, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_returns_no_consumed() {
    let r = returns(1);

    assert_eq!(false, r.execute(&"a", 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_fails_no_consumed() {
    let r = fail();

    assert_eq!(false, r.execute(&"a", 0).fold(
        |_: u32, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_any_success() {
    let r = any();

    assert_eq!('a', r.execute(&"a", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_any_reject() {
    let r = any();

    assert_eq!(false, r.execute(&"", 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_eos_success() {
    let r = eos();

    assert_eq!((), r.execute(&"", 0).fold(
        |a, _, _| a,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_eos_reject() {
    let r = eos();

    assert_eq!(false, r.execute(&"a", 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_try_any_reject() {
    let r = do_try(any());

    assert_eq!(false, r.execute(&"", 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_try_any_success() {
    let r = do_try(any());

    assert_eq!(true, r.execute(&"a", 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_satisfy_any_reject() {
    let r = satisfy(any(), Box::new(|c: &char| *c == 'a'));

    assert_eq!(true, r.execute(&"b", 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_satisfy_any_success_unwrap() {
    let r = satisfy(any(), Box::new(|c: &char| *c == 'a'));

    assert_eq!(true, r.execute(&"a", 0).fold(
        |_, s, _| s == 1,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_satisfy_any_success() {
    let r = satisfy(any(), Box::new(|c: &char| *c == 'a'));

    assert_eq!(true, r.execute(&"a", 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_lookahead_any_reject() {
    let r = lookahead(any());

    assert_eq!(false, r.execute(&"", 0).fold(
        |_, _, _| panic!("Parse error"),
        |_, b| b,
    ));
}

#[test]
fn it_parse_with_lookahead_any_success() {
    let r = lookahead(any());

    assert_eq!(true, r.execute(&"a", 0).fold(
        |_, _, b| b,
        |_, _| panic!("Parse error"),
    ));
}

#[test]
fn it_parse_with_lookahead_any_success_no_unwrap() {
    let r = lookahead(any());

    assert_eq!(true, r.execute(&"a", 0).fold(
        |_, s, _| s == 0,
        |_, _| panic!("Parse error"),
    ));
}