fn main(){
    let B = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve K"),
        pages: 500,
        published: true,
    };

    println!("{}", B.title);
}

struct Book{
    title: String,
    author: String,
    pages: u32,
    published: bool,
}