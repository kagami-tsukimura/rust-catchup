mod sub_a;
mod sub_b;

// デフォルトはprivate
pub fn run() {
    sub_a::func_a();
    println!("Here is vars module!");
    sub_b::func_b();
}
