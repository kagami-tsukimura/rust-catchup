// 構造体そのままコンソール出力のためにDebugトレイトを実装(println!("user1 = {:?}", user1);)
#[derive(Debug)]
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
    // {:#?}: "#"で整形
    println!("user1 = {:#?}", user1);
    // 構造体のような複雑なデータ型: {:?}とする
    // println!("{:?}", user1);

    // let mut user2 = user1;
    // // 所有権: user1→user2のためエラー
    // println!("user1 = {:?}", user1);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("someone_update1@example.com");
    user1.username = String::from("someusername123_update1");
    user1.active = false;
    user1.sign_in_count += 1;
    // {:#?}: "#"で整形
    println!("{:#?}", user1);
}

fn build_user(email: String, username: String) -> User {
    // 受け取った引数（email, username）でUser構造体を作成
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
