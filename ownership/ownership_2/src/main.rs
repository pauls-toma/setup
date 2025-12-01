//concepts: Copy, Move, Clone

fn main(){
    let x: i32 = 10;
    let y = x;
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{x} and {y} ");
    println!("{s1} and {s2}");
}

// This program shows that values with a known, fixed size (like i32) are automatically copied on the stack during assignment, so both variables (x and y) own their own independent data on the stack.
// This program also shows that heap-allocated String values do not copy their heap data automatically; instead, only the stack pointer is moved to the new variable. Without .clone(), s1 would immediately fall out of scope because its pointer is transferred to s2, making s2 the sole owner of the underlying string.
