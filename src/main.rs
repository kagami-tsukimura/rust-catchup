mod ownership;
mod vars;

fn main() {
    // println!("Hello, world!");
    // println!("Rust is awesome!");
    // vars.rsのrun()を実行
    vars::run();
    // println!("------");
    // vars::sub_a::func_a();
    ownership::run();
}
