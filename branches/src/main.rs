use std::char::from_digit;


fn main() {
    /* Because if is an expression, we can use it on the right side
    of a 'let' statement to assing the outcome to a variable */

    let condition = true; 
    let number = if condition { 5 } else { 6 }; 

    println!("The value of number is: {number}"); 

    /*
        Returning values from loops 
        ----------------------------------------------------------------
        You can return a value from the result of a loop by appending the 
        value after the 'break' expression

        NOTE: 
        The semicolon after 'break counter * 2' is optional. Break is very 
        similar to return. The Rust compiler treats a break expression and 
        a return expression as having the value unit, or (). 
     */
    let mut counter = 0; 

    let result = loop { 
        counter += 1; 

        if counter == 10 { 
            break counter * 2;
        }
    }; 

    println!("The result is {result}"); 


    /*
        Loop labels 
        ----------------------------------------------------------------
        You can break or continue an outer loop by using its label 
    */
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


    /*
        while loops 
        -----------------------------------------
     */
    let mut number1 = 3; 

    while number1 != 0 { 
        println!("{number1}"); 

        number1 -= 1; 
    }
    println!("LIFTOFF!!!"); 

    /*
        Looping through collections 
        -----------------------------------------
        The first example is slower and error prone. 
        The compiler adds runtime code to perform the 
        conditional check of whether the index is within 
        the bounds of the array on every iteration through
        the loop. 
     */
    let arr = [10, 20, 30, 40, 50]; 
    let mut index = 0; 

    // should use arr.len() instead
    while index < 5   { 
        println!("the value is: {}", arr[index]); 
        index += 1; 
    }

    for element in arr { 
        println!("the value is: {element}"); 
    }

    // a for loop can be used with a 'Range' to make 
    // iterating a certain number of times more concise 
    // and less error prone
    // 
    // NOTE: the rev() method reverses the collection
    for num in (1..4).rev() { 
        println!("{num}"); 
    }
    println!("LIFTOFF!!!"); 

    /* 
        RANDOM NOTE:  terminal plotting tool

        $ asciiplot <MATH_EXPRESSION> [-xn | --xname <NAME>] [-yn | --yname <NAME>]
     */


    /*
        PROGRAMS TO TEST MY SKILLS
        -----------------------------------------------------------------------------
        (1) Convert temperatures between Fahrenheit and Celsius.
        (2) Generate the nth Fibonacci number.
        (3) Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
           taking advantage of the repetition in the song.
     */

    // (1) 
    celsius_vs_fahrenheit_graph(); 

    // (2) 
    

    // (3)
 

}


/**
 * Convert temperatures between Fahrenheit and Celsius
 * FORMULA: F = C * (9/5) + 32
 */
fn celsius_vs_fahrenheit_graph() { 
    const BLUE: (u32, u32, u32) = (0, 69, 252); 
    const GREEN: (u32, u32, u32) = (0, 252, 143); 
    const C_MAX: usize = 33; 
    const F_MAX: usize = 90; 
 
    let mut grid: [[bool; C_MAX]; F_MAX] = [[false; C_MAX]; F_MAX];

    // populate grid with values
    for degrees_c in 0..C_MAX { 
        let degrees_f: f32 = ((degrees_c as f32) * 9.0 / 5.0) + 32.0; 
        grid[degrees_f as usize][degrees_c as usize] = true; 
    }

    let mut str_buffer: String = String::new(); 
    println!("{}", print_color(BLUE, "\n\n(\u{00b0}F)\n"));
    for i in (32..F_MAX).rev() {
        // for printing the y-axis
        print!("{}", print_color(BLUE, &format!("{} |", i)));
        for j in 0..C_MAX {
            let point = format!("*  ({}, {})", j, i); 
            let marker = if grid[i][j] { &point } else { "  " }; 
            str_buffer.push_str(marker);
        }
        println!("{}", print_color(GREEN, &str_buffer)); 
        str_buffer.clear();
    }

    // print the x-axis 
    for _ in 0..C_MAX { 
        str_buffer.push_str("----"); 
    }
    str_buffer.push_str(" (\u{00b0}C)"); 
    println!("{}", print_color(BLUE, &str_buffer)); 
}

/**
 * Generates colored text output given an rgb value
 */
fn print_color(color: (u32, u32, u32), text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", color.0, color.1, color.2, text);
}