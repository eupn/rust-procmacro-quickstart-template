// You might need to use this in future since quote! macro can be highly-recursive
//#![recursion_limit="1024"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    println!("my_macro! sees input: \"{}\"", input);

    // Do nothing by emitting empty token stream
    let output = quote!();

    output.into()
}

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    println!("MyDerive is being derived from \"{}\"", input.to_string());

    // Do nothing by emitting empty token stream
    TokenStream::new()
}

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn my_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("my_attribute sees args: \"{}\"", args.to_string());
    println!("my_attribute sees input: \"{}\"", input.to_string());

    // Do nothing and short-circuit input to output.
    // Since attribute consumes input in contrary to derive-mode macros, which aren't
    input
}
