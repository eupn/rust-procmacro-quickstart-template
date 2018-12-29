#![allow(dead_code)]

// Import our procedural macros crate
extern crate rust_procmacro_quickstart_template;

// Import procedural macro attribute
use rust_procmacro_quickstart_template::{my_attribute, my_macro, MyDerive};

#[my_attribute]
struct TestAttrStruct {}

#[my_attribute]
fn test_attr_function() {}

#[derive(MyDerive)]
struct TestDeriveStruct {}

#[derive(MyDerive)]
enum TestDeriveEnum {}

my_macro!(2 + 2);

pub fn main() {
    // my_macro!(2 + 2);

    // ^ as of rust 1.30, this gives an error[E0658]: procedural macros cannot be expanded to statements (see issue #54727)
    // This error might be fixed in future, although, there's workaround exists: https://github.com/dtolnay/proc-macro-hack
}
