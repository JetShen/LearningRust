struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
// }



fn main() { // update syntax without repeating the other values that haven’t changed from user1
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );



    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}