pub fn run() {
    // Boxポインタ
    // スタックに存在するデータをそのままヒープに移動
    // 移動したデータのポインタを、Boxポインタとしてスタックに保持

    // コンパイル時にサイズが決まらないデータに使用
    // ポインタ情報だけにすることで、8bytes固定のためコンパイルが通る

    println!("pointer.rs is start!");

    println!("pointer.rs is done!\n------");
}
