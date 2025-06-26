use hidapi::HidError;
use tempers::read_temp;

fn main() -> Result<(), HidError> {
    let temp = read_temp()?;
    println!("{}", temp);
    Ok(())
}
