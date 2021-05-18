#[macro_use]
extern crate uti_derive;

use uti::UniversalTypeId;

#[derive(UniversalType)]
struct Person {
    _name: String,
    _year: i16,
}

fn main() {
    let uid = UniversalTypeId::of::<Person>();
    println!("Numerical value of universal type ID: {}", uid.as_u128());
}
