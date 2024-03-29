use std::io;

fn main() {
    // VARIABLES AND CONSTANTS

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
    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing
    /*
        - If a new variable is declared with the same name as the previous one, the new variable has shadowed the old one.
        - The compiler sees the the second variable and use it

    */    
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    /* 
    Shadowing vs Mutable
    - When you shadow a number, you block it (not destroy it.
    - New variable is created
    e.g let spaces = "   ";
        let spaces = spaces.len();
    The example above compiles due to shadowing.
    However, the example below won't compile.
    e.g let mut spaces = "   ";
        spaces = spaces.len();
    */

    // DATA TYPES
    /*let mut spaces = "   ";
    spaces = spaces.len();
    Every value in Rust is of certain data type
    */

    // Scalar Type
    /*
    Four Primary Scalar Type
        - Integers (Signed or Unsigned)
            * 8bits : Signed (i8) and Unsigned (u8)
            * ...
            * arch : Signed (isize) and Unsigned (usize)
        
        - Floating-point
            * f32 and f64. Default is f64

        - Booleans
            *true and false (One byte in size)
            e.g let flag: bool = true;

        - Characters
        let c: char = 'Z';
    */

    // Compound Type: Tuple Type and Array Type
    /* 
    Tuple Type
        - Way of grouping together a number of values with a variety of types
        - Can not shrink or grow in size.
    */
    let tup: (i32, f64, u8) =(500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y and z is {x}, {y} and {z}");

    // Tuple can also be accessed using periodd (.)
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}, {six_point_four} and {one}");
    
    /* 
    Array Type
        - Way of grouping together a number of values with a the same of types
        - Fixed length
        - Useful when data is allocated on stack rather than heap
        - Not as flexible as vector type(which can shrink and grow)

    Accessing Array Elements:
    variable_names[index]
    */
    let a: [i32; 6]= [1,2,3,4,5,6];
    let first = a[0];
    println!("Index 0: {first}, ...");
    println!("Array: {:?}", a);
    

    // Function
    let mut current_age = String::new();
    io::stdin().read_line(&mut current_age).expect("Failed to read line");
    let current_age: u32 = match current_age.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a positive integer!!");
            return;
        },
    };
    println!("Current age is {current_age}");
    let future_age: u32 = age(current_age);
    println!("Age in 5 years is {future_age}");

    // Statements and Expressiions
    /*
        Statements: Instructions that perform some action and does not return a value.
        e.g let y = 6;

        Expressions: Evaluate to a resultant value
        e.g let y = {
            let x = 3;
            x + 1
        };
            The expression in the above code: {
                                                let x = 3;
                                                x + 1
                                            };
        println!("The value of y is: {y}");
    */

    // CONTROL FLOW
    // if Expressions
    let number = 3;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Repition with loops
    /*  
        Rust has 3 kinds of loop:
            - loop
            - while
            - for
    */
    // Repeating code with loop
    let mut counter = 0;

    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter*2;
        }
    };
    println!("The result is {result}");

    // Loop labels: To disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    // Conditional loop with while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // The above while loop can be rewritten using for loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // rev(): To reverse the range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


fn age(x:u32)-> u32{
    x+5 //Same as return x+5;
}