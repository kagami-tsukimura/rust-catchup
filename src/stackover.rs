pub fn run() {
    println!("vars.rs is start!");

    // stack overflow
    // 配列のstack要件(8MB)を超えると発生

    // 7MB(safe)
    let _a1: [u8; 7000000] = [1; 7000000];

    // 9MB(stack overflow)
    let _a2: [u8; 9000000] = [1; 9000000];

    println!("vars.rs is done!\n------");
}
