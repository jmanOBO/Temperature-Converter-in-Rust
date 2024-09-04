extern crate temperature_converter;
use temperature_converter::init;
//I made the main file clean
fn main() {
    let i = init();
    if let Err(e) = i {
        println!("An error occurred: {} . Pls try again.", e);
    }
}
