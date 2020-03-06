extern crate compose_errors;
use compose_errors::AnswerFn;


//#[derive(AnswerFn)]
#[allow(dead_code)]
struct Struct;

fn main() {
    assert_eq!(42, answer());
    println!("*************************************\r\n {}", answer().to_string());

    println!("{}", compose().unwrap());
}

fn compose() -> Result<String, SomeError3> {

    let _x = doSomething1(1)?;
    let _y = doSomething2(2)?;



    return Ok("ghi".to_string());
}

#[allow(non_snake_case)]
fn doSomething1(_i:u32) -> Result<String, SomeError1> {
  return Ok("abc".to_string());
}

#[allow(non_snake_case)]
fn doSomething2(_i:u32) -> Result<String, SomeError2> {
  return Ok("def".to_string());
}

#[derive(std::fmt::Debug)]
#[allow(dead_code)]
enum SomeError1 {
    SomeError1_1,
    SomeError1_2,
    SomeError1_3(String),
}

#[derive(std::fmt::Debug)]
#[allow(dead_code)]
#[derive(AnswerFn)]
enum SomeError2 {
    SomeError2_1,
    SomeError2_2
}

#[compose_errors::compose_errors_fn (
    //doc = "example",
    //prefix = NoPrefix,
    //prefix(None),
    prefix = "CustPref",
    //prefix = "Some_prefix_here",
    //multiple,
    //crate::SomeError2,
    //Blah::Blah2::Blah3,
    SomeError1,
    SomeError2
 )]
#[derive(std::fmt::Debug)]
#[allow(dead_code)]
enum SomeError3 {
    SomeError3_1,
    SomeError3_2
}

/*
impl std::convert::From<SomeError1> for SomeError3 {
    fn from(e: SomeError1) -> Self {
        return SomeError3::SomeError3_1(e);
    }
}

impl std::convert::From<SomeError2> for SomeError3 {
    fn from(e: SomeError2) -> Self {
        return SomeError3::SomeError3_2(e);
    }
}
*/
