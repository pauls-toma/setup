struct Book{
    title: String,
    number_of_pages: u32,
    chapters: u32,
}

impl Book {
    fn title(&self) -> &str{
        &self.title
    }
}

fn main(){
    let b = Book {
        title: String::from("Book1"),
        number_of_pages: 5,
        chapters: 6,
    };
    println!("{}", b.title());
}