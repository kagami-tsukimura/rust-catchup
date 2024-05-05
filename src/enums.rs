#[derive(Debug)]
enum OS {
    Windows(u32, String),
    MacOS(u32, String),
    Linux(u32, String),
}

pub fn run() {
    println!("enums.rs is start!");
    let linux = OS::Linux(1991, String::from("Linus"));

    println!("linux = {:?}", linux);
}
