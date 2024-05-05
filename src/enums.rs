#[derive(Debug)]
enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn run() {
    println!("enums.rs is start!");
    let linux = OS::Linux(1991, String::from("Linus"));
    let windows = OS::Windows(1985, String::from("Microsoft"));
    let mac = OS::Mac(1985, String::from("Apple"));

    println!("windows = {:?}", windows);
    println!("mac = {:?}", mac);
    println!("linux = {:?}", linux);
}
