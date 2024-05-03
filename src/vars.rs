// // mainでsub_aとsub_bを呼び出せるように`pub`を付ける
// pub mod sub_a;
// pub mod sub_b;

// 定数
const MAX_POINTS: u32 = 100_000;

// デフォルトはprivate
pub fn run() {
    // sub_a::func_a();
    println!("Here is vars module!");
    // sub_b::func_b();
    let mut x = 5;
    println!("The value 'x' = {}", x);
    x = 6;
    println!("The value 'x' = {}", x);

    // 使用しない変数は”_”で明示的に指定→コンパイル時のwarningを避ける
    let _i1 = 3;
    let _f1 = 0.1;
}
