fn main() {
    let user1 = User {
        username: String::from("pauls"),
        age: 27,
        active: true,
    };
    println!(" username: {} age: {} active: {}", user1.username, user1.age, user1.active);
}
struct User {
    username: String,
    age: i8,
    active: bool,
}