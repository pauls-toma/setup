fn main(){ 
    let x: i32 = 5; 
    let y: i32 = 10; 
    let z = math(x, y); 
    println!("{z}");

fn math(a: i32, b: i32) -> i32{ 
    let result: i32 = a + b; 
    result
}

}

// This program demonstrates how Rust manages function calls and stack frames during execution. When main() begins, it becomes the first stack frame, and its local variables x, y, and later z are all stored inside that frame. When main() calls math(x, y), the program pauses main() and pushes a new stack frame for the math function. Inside that frame, the parameters a and b and the local variable result are created and used to compute the return value. After math() finishes, its entire stack frame is popped, control returns to main(), the value is printed, and once main() reaches the end of its scope, its own frame is also popped and the program ends. 
// This flow lines up directly with Rust’s ownership rules. Each variable in the program has exactly one owner tied to the stack frame in which it is created. When a function goes out of scope, all the values it owns are automatically dropped and their memory is reclaimed. In this program, the math() function owns its parameters and its result value, and they are all released the moment the function returns. Likewise, the variables in main() are owned by main() and are dropped when main() exits.
