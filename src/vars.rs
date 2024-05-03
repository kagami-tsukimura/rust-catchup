// mainでsub_aとsub_bを呼び出せるように`pub`を付ける
pub mod sub_a;
pub mod sub_b;

// デフォルトはprivate
pub fn run() {
    sub_a::func_a();
    println!("Here is vars module!");
    sub_b::func_b();
}
