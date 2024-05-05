pub fn run() {
    // Boxポインタ
    // スタックに存在するデータをそのままヒープに移動
    // 移動したデータのポインタを、Boxポインタとしてスタックに保持

    // コンパイル時にサイズが決まらないデータに使用
    // ポインタ情報だけにすることで、8bytes固定のためコンパイルが通る

    // Boxポインタ前：メモリ表現の確認
    // スタック：Boxポインタ、データ、ptr(8bytes), len(8bytes), capacity(8bytes)
    // ヒープ：データ
    // Boxポインタ後：メモリ表現の確認
    // スタック：Boxポインタ（heapデータの先頭アドレスポインタ）
    // ヒープ：データ、スタックデータ、ptr(8bytes), len(8bytes), capacity(8bytes)

    println!("pointer.rs is start!");

    println!("pointer.rs is done!\n------");
}
