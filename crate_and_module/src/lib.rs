//! doc to the entire library crate
 
/// doc to subsequent elements
pub fn say_hello() {
    println!("Hello, lib.rs!");
}

/// doc of **hidden_if_private**
/// ### Examples of Use
/// '''
/// fn main() {
///     crate_and_module::hidden_if_private();
/// }
/// '''
pub fn hidden_if_private() {
    println!("So shy!!");
}

// $ cargo doc --no-deps --open