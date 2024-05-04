pub fn run() {
    println!("vars.rs is start!");

    // stack overflow
    // 配列のstack要件(8MB)を超えると発生

    // // 7MB(safe)
    // let _a1: [u8; 7000000] = [1; 7000000];

    // // 9MB(stack overflow)
    // let _a2: [u8; 9000000] = [1; 9000000];

    // Vector型
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    // Vector型とString型はメモリ構造が同じ(ptr(8bytes), len(8bytes), capacity(8bytes))
    println!(
        "Stack address of\nv1 is: {:p}\nv2 is: {:p}\nv3 is: {:p}",
        &v1, &v2, &v3
    );

    // アドレス
    println!(
        "Heap memory address of\nv1 is: {:?}\nv2 is: {:?}",
        &v1.as_ptr(),
        &v2.as_ptr()
    );
    // Len
    println!("Len of \nv1 is: {}\nv2 is: {}", &v1.len(), &v2.len());
    // Cap
    println!(
        "Capacity of \nv1 is: {}\nv2 is: {}",
        &v1.capacity(),
        &v2.capacity()
    );

    // insert(indexの後ろにelementを追加: [1, 2, 3, 4] => [1, 10, 2, 3, 4])
    v1.insert(1, 10);
    println!("{:?}", v1);

    // Len
    println!("Len of \nv1 is: {}\nv2 is: {}", &v1.len(), &v2.len());
    // Cap
    println!(
        "Capacity of \nv1 is: {}\nv2 is: {}",
        &v1.capacity(),
        &v2.capacity()
    );

    // remove
    v1.remove(0);
    println!("{:?}", v1);

    // Len
    println!("Len of \nv1 is: {}\nv2 is: {}", &v1.len(), &v2.len());
    // Cap
    println!(
        "Capacity of \nv1 is: {}\nv2 is: {}",
        &v1.capacity(),
        &v2.capacity()
    );

    // append
    v1.append(&mut v3);
    // 連結したv3は空配列になる
    println!("{:?}", v1);
    println!("{:?}", v3);

    println!("vars.rs is done!\n------");
}
