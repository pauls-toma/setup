//borrowing

fn main(){
    let s1 = String::from("hello");
    consume(&s1);
    println!("{s1}");
}

fn consume(_s: &String){
}