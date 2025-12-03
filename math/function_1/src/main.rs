fn main(){
    let z = add(6, 7);
    println!("{z}");
}

fn add(x: i32, y: i32) -> i32{
    let result = x + y;
    result
}

//basic add function, passing argument, declaring parameters, and return types.