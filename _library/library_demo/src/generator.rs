use rand::Rng;
pub fn gen_ran() -> u8 {
    // 乱数機を生成
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen();
    n
}
