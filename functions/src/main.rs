
fn main() {
    println!("hi there"); 

    another_function(5, 'h'); 

    /*
        Statements and Expressions
        -----------------------------------------
        Expression do not include semicolons. If you 
        add a semicolon to the end of an expression, 
        you turn it into a statement, and it will not 
        return a value. 
     */
    let y = { 
        let x = 3; 
        x + 1
    }; 
    println!("The value of y is: {y}");

    /*
        Functions with Return Values 
        -----------------------------------------
        - declare returns typed with (->)
        - you can early return by using the "return"
          keyword and specifying a value, but most 
          functions return the last expression implicitly
    
     */
    let x = five(); 
    println!("The value of x is: {x}"); 

    let x2 = plus_one(5); 
    println!("The value of x2 is: {x2}"); 

}

fn plus_one(x: i32) -> i32 { 
    x + 1
}

fn five() -> i32 { 
    5
}

fn another_function(value: i32, unit_label: char) { 
    println!("The measurement is: {value}{unit_label}"); 
}