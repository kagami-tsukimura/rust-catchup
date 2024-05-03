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
    let mut x: i32 = 5;
    println!("The value 'x' = {}", x);
    x = 6;
    println!("The value 'x' = {}", x);

    // 使用しない変数は”_”で明示的に指定→コンパイル時のwarningを避ける
    let _i1: i32 = 3;
    let _f1: f64 = 0.1;
    // 実行環境のサイズ（64bit）
    println!("{}", usize::BITS);
    // {:p}:ポインタ形式でメモリアドレスを表示
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    // 変数の領域（異なるアドレスに格納）
    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    let y: i32 = 5;
    println!("Stack address of y is: {:p}", &y);
    let y: i32 = y + 1;
    println!("Stack address of y + 1 is: {:p}", &y);
    let y: i32 = y * 2;
    println!("Stack address of y * 2 is: {:p}", &y);
    println!("The value 'y' = {}", y);
    {
        let y: i32 = 0;
        println!("The value local scope 'y' = {}", y);
    }
    println!("The value 'y' = {}", y);

    // タプル型
    let t1: (i32, f64, &str) = (500, 6.4, "dummy");
    let (x, y, z) = t1;
    println!("The value 't1' = ({}, {}, {})", t1.0, t1.1, t1.2);
    println!("The value 't1' = ({}, {}, {})", x, y, z);
    println!("The value 't1' = {}", t1.0);
}
