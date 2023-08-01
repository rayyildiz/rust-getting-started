use banner::print_banner;
use std::num::ParseIntError;

fn parse_str(v: &str) -> Result<i32, ParseIntError> {
    let a = v.parse::<i32>()?;

    println!("{}  is valid", a);
    Ok(a)
}

fn divisor(a: f32, b: f32) -> Result<f32, String> {
    let result = match b {
        0.0 => Err(String::from("devide by zero")),
        _ => Ok(a / b),
    }?;

    println!("valid number, {}/{} = {:?}", a, b, result);

    Ok(result)
}

fn divisor_opt(a: f32, b: f32) -> Option<f32> {
    let result = match b {
        0.0 => None,
        _ => Some(a / b),
    }?;

    println!("valid number, {}/{} = {:?}", a, b, result);

    Some(result)
}

fn main() {
    print_banner();

    println!("valid :{:?}", parse_str("32"));
    println!("not valid :{:?}", parse_str("s32"));

    println!("div, valid: {:?}", divisor(5.0, 2.1));
    println!("div, not valid: {:?}", divisor(5.0, 0.0));

    println!("div_opt, valid: {:?}", divisor_opt(5.0, 2.1));
    println!("div_opt, not valid: {:?}", divisor_opt(5.0, 0.0));

    println!("call app service :{:?}", do_call_service());
}

#[derive(Debug)]
enum AppError {
    DbConnectionError,
    ValidationError,
    RequiredFieldError { name: String },
}

type AppResult = Result<(), AppError>;

fn connect_to_db() -> AppResult {
    // connect to db
    Ok(())
}

fn validate_fields() -> AppResult {
    // validate fields
    Ok(())
}

fn required_email_field() -> AppResult {
    // check email is valid ...

    Ok(())
}

fn do_call_service() -> AppResult {
    connect_to_db()?;
    validate_fields()?;
    required_email_field()?;

    Ok(())
}
