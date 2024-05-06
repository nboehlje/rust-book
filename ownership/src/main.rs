/*

    Undefined Behavior
    -------------------------------------------------
    (1) Using a pointer that points to freed memory
    (2) Using a non-boolean value as an if condition
    (3) Freeing the same memory a second time


    EXAMPLE
    --------------------------------------------------
    Rust will not compile any of the code snippets below. 
    One snippet will not cause undefined behavior. 


    fun move_a_box(b: Box<i32>) { 
        // do something
    }

    (1) ------------------------
        let b = Box::new(0); 
        let b2 = b; 
        move_a_box(b); 
    
    (2) ------------------------
        let b = Box::new(0); 
        move_a_box(b); 
        println!("{}", b); 
    
    (3) ------------------------
        let b = Box::new(0); 
        let b2 = b; 
        println!("{}", b); 
        move_a_box(b2); 
    
    (4) ------------------------
        let b = Box::new(0); 
        move_a_box(b); 
        let b2 = b; 

    ANS: 
        Snippet (3) will not cause undefined behavior. 

        CONTEXT: The key idea is that when a box is passed to move_a_box, 
        its memory is deallocated after move_a_box ends. Therefore:

        - Reading b via println after move_a_box is undefined behavior, 
          as it reads freed memory.
        
        - Giving b a second owner is undefined behavior, as it would cause Rust 
          to free the box a second time on behalf of b2. It doesn't matter whether 
          the let b2 = b binding happens before or after move_a_box.
        
        However, doing let b2 = b and then println is not undefined behavior. 
        Although b is moved, its data is not deallocated until move_a_box is called at the end. 
        Therefore this program is technically safe, although still rejected by Rust.
*/



fn main() {
   println!("hi there");

   
}
