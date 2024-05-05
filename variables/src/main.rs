fn main() {
    let mut x = 5; 
    println!("The value of x is: {x}"); 
    x = 6; 
    println!("The value of x is: {x}"); 

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 

    let y = 5; 
    let y = y + 1; 
    {
        let y = y * 2; 
        println!("The value of y in the inner scope is {y}"); 

    }
    println!("The value of y is: {y}"); 

    // more shadowing 
    let spaces = "   "; 
    let spaces = spaces.len(); 

    println!("number of spaces: {spaces}"); 

    /*
        Numeric literals 
        -----------------------------------------------
     */

    let decimal_num = 98_222;
    let hex_num = 0xff; 
    let octal_num = 0o77; 
    let binary_num = 0b1111_0000; 
    let byte_num = b'A';  

    /*
        Floating point numbers
        ----------------------------------------------
     */

    let x_float = 2.0; 
    let y_float: f32 = 3.0; 

    /*
        Math operations
        ----------------------------------------------
     */
    // addition 
    let sum = 5 + 10; 

    // subtraction 
    let difference = 95.5 - 4.3; 

    // division 
    let quotient = 56.7 / 32.2; 
    let truncated = -5 / 3; // Results in -1

    // remainder 
    let remainder = 43 % 5; 

    /*
        Booleans (1 byte) 
        -------------------------------------------------
     */

    let t = true; 
    let f: bool = false; // with explicit type annotation 

    /*
        Characters (char type is 4 bytes)
        --------------------------------------------------
     */
    let c = '\u{1F408}'; 
    let z: char = 'z'; 

    /*
        Tuples
        ---------------------------------------------------
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1); 
    let tup_wo_type_anno = (888, 5.4, true); 

    let (first, second, third) = tup; 
    println!("The value of second is: {second}"); 

    let five_hundred = tup.0; 
    let six_point_four = tup.1; 
    let one = tup.2; 

    // a tuple without any values is called a "unit"
    // and is denoted with an empty parenthesis "()"
    let unit = (); 

    /*
        Arrays 
        ----------------------------------------------- 
    */

    let a = [1, 2, 3, 4, 5]; 
    let months = ["January", "February", "March", "April", 
                "May", "June", "July", "August", "Septemper", 
                "October", "November", "December" ];

    let a_explicit: [i32; 5] = [1, 2, 3, 4, 5];
    
    // creates an array with 5 elements all of 
    // which are the number 3
    let a_w_same_values = [3, 5]; 

    let first = a[0]; 
    let second = a[1]; 

    let arr = [1, 2, 3, 4, 5]; 
    println!("Please enter an array index."); 

    let mut index = String::new(); 
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line"); 

    let index: usize = index 
        .trim()
        .parse()
        .expect("Index entered was not a number"); 

    let element = a[index]; 

    println!("The value of the element at index {index} is: {element}"); 
}
