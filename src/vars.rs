// // mainでsub_aとsub_bを呼び出せるように`pub`を付ける
// pub mod sub_a;
// pub mod sub_b;

// 定数
// Rustでは代入をバインドと呼ぶ
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

    // ポインタごとバインド
    let mut t2: ((i32, i32), (i32, i32)) = ((0, 1), (2, 3));
    // x1_ptr = 0, y1_ptr = 1, (2, 3) = _
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    println!("Stack address of x1, y1 is: {:p}, {:p}", &x1_ptr, &y1_ptr);
    println!("The value 'x1_ptr' = {}, 'y1_ptr' = {}", *x1_ptr, *y1_ptr);
    // 同じポインタを書き換え
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("Stack address of x1, y1 is: {:p}, {:p}", &x1_ptr, &y1_ptr);
    println!("The value 'x1_ptr' = {}, 'y1_ptr' = {}", *x1_ptr, *y1_ptr);
    // 複雑な型は{:?}を使う
    println!("{:?}", t2);

    // 配列(サイズ(要素数)はコンパイル時に決定)
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    // [3, 3, 3, 3, 3]
    let a2: [i32; 5] = [3; 5];
    println!("The value 'a1' = {:?}", a1);
    println!("The value 'a2' = {:?}", a2);
    // 各要素にアクセス
    println!("The value 'a1[0]' = {}, 'a2[2]' = {}", a1[0], a2[2]);

    // 文字列リテラル
    // 26bytes（アルファベット1byte, ひらがな漢字3byte）
    let s1: &str = "helloこんにちは挨拶";
    // 5bytes（アルファベット1byte, ひらがな漢字3byte）
    let s2: &str = "hello";
    println!("Stack address of\ns1 is: {:p}\ns2 is: {:p}", &s1, &s2);
    println!("The value 's1' = {}", s1);

    // ポインタアドレス（領域の先頭アドレス）を取得: as_ptr()
    println!(
        "Stack memory address of \ns1 is: {:?}\ns2 is: {:?}",
        &s1.as_ptr(),
        &s2.as_ptr()
    );

    // length取得
    println!("Len of \ns1 is: {}\ns2 is: {}", &s1.len(), &s2.len());
}
