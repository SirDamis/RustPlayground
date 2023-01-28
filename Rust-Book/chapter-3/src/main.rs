fn main() {
    // Variables are immutable by default in rust but there exist option to make it mutable by using the `mut` keyword
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    /*
        - Constants are bound to a name and are not allowed to change.
        - Constants are always immutable, therefore `mut` are not allowed to be used with constant
        - Constants are declared using the `const` keyword
        - Can be declared in any scope
        - May be set to only a constant exprression, not the result of a value that could be computed at runtime
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}")


}
