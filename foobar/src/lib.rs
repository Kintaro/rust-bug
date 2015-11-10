extern crate foobarcore;

use foobarcore::SomeTestStruct;

#[no_mangle]
pub fn do_stuff(some_struct: &mut SomeTestStruct) {
    println!("In library function");
    some_struct.value = String::from("new value");
}
