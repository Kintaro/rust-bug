extern crate foobarcore;
extern crate dylib;

use foobarcore::SomeTestStruct;
use std::path::Path;
use std::mem;
use dylib::DynamicLibrary;

fn main() {
    let mut some_struct = SomeTestStruct { value: String::from("value") };

    if let Ok(lib) = DynamicLibrary::open(Some(&Path::new(&"../foobar/target/debug/libfoobar.so"))) {
        unsafe {
            if let Ok(symbol) = lib.symbol("do_stuff") {
                let result = mem::transmute::<*mut u8, extern fn(&mut SomeTestStruct)>(symbol);

                println!("Calling do_stuff()");
                result(&mut some_struct);
            } else {
                panic!("Ooops");
            }
        }
    }
    println!("{}", some_struct.value);
}
