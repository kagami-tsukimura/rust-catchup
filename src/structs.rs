struct User {
    // String型
    username: String,
    email: String,
    // 整数型
    sign_in_count: u64,
    // 真偽値型
    active: bool,
}

pub fn run() {
    println!("struct.rs is start!");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1.email = {}", user1.email);

    let user2 = user1;
    // // 所有権: user1→user2のためエラー
    // println!("user1 = {}", user1);
}
