// 列挙型 enum
enum List {
    // (4bytes, List: コンパイラが無限に再帰でサイズ参照→エラー)
    // ListをBoxポインタ化すると、サイズがBoxポインタと等しくなる→値の確定
    Node(i32, Box<List>),
    // 0byte
    Nil,
}

pub fn run() {
    println!("pointer.rs is start!");
    // Boxポインタ
    // スタックに存在するデータをそのままヒープに移動
    // 移動したデータのポインタを、Boxポインタとしてスタックに保持

    // コンパイル時にサイズが決まらないデータに使用
    // ポインタ情報だけにすることで、8bytes固定のためコンパイルが通る
    // →リストの値が定まらないときに役に立つ

    // Boxポインタ前：メモリ表現の確認
    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of t1 is: {:p}", &t1);

    // スタック：Boxポインタ、データ、ptr(8bytes), len(8bytes), capacity(8bytes)
    // ヒープ：データ
    // ヒープのアドレス（"hello"）のptr(8bytes), len(8bytes), capacity(8bytes)取得
    println!("Heap memory address of t1 is: {:?}", &t1.1.as_ptr());
    println!("Len of t1 is: {}", &t1.1.len());
    println!("Capacity of t1 is: {}", &t1.1.capacity());

    // Boxポインタ後：メモリ表現の確認
    // スタック：Boxポインタ（heapデータの先頭アドレスポインタ）
    // ヒープ：データ、スタックデータ、ptr(8bytes), len(8bytes), capacity(8bytes)
    let mut b1 = Box::new(t1);
    println!("Stack address of b1 is: {:p}", &b1);
    // (*b1): 参照外し→スタックの実際の値にアクセス
    (*b1).1 += " world!";
    println!("b1.0: {}, b1.1: {}", b1.0, b1.1);
    println!("Heap address of b1 is: {:p}", b1);
    println!("Heap memory address of b1 is: {:?}", &b1.1.as_ptr());
    println!("Len of b1 is: {}", &b1.1.len());
    println!("Capacity of b1 is: {}", &b1.1.capacity());

    println!("pointer.rs is done!\n------");
}
