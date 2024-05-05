pub fn run() {
    println!("debug.rs is start!");
    let mut x = 10;
    x = 20;
    {
        let mut y = 30;
        y = 40;
    }
    x = 4;
    x = 5;
    println!("{}", x);

    println!("debug.rs is done!\n------");
}
