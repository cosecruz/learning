/*
Procedural macros

These are Rust functions that run at compile time and transform code.

There are three main kinds:
 */

/*
(a) Function-like procedural macros

Look like foo!(...) but are implemented as Rust functions.

Example: serde_json::json!

You define them in a separate crate of type proc-macro.
 */
// in a proc-macro crate
// #[proc_macro]
// pub fn make_answer(_input: TokenStream) -> TokenStream {
//     "42".parse().unwrap()
// }

/*
(b) Derive macros

Look like #[derive(Debug)]

Automatically implement traits for structs/enums.

Example: #[derive(Clone)], #[derive(Error)] from thiserror.

You define them like this in a proc-macro crate:
 */

// #[proc_macro_derive(MyTrait)]
// pub fn my_trait(input: TokenStream) -> TokenStream {
//     // generate code to implement MyTrait
// }

/*
(c) Attribute macros

Look like #[my_attribute] on a function, struct, or module.

Can take arguments: #[my_attribute(arg1, arg2)]

Example: #[error(...)] from thiserror, or #[tokio::main].
 */

// #[proc_macro_attribute]
// pub fn my_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
//     // modify item based on attr
// }

//declarative macros
macro_rules! greetings {
    () => {
        println!("Hello, world!")
    };
}
