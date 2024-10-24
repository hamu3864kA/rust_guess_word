fn main() {
    let a = 1; // リテラルの1(Copy)を束縛
    let b = a; // b は a のコピーで束縛される 所有権はムーブしない
    println!("{}, {}", a, b);

    let a = 1; // リテラルの1(Copyトレイトの実装)をaに束縛
    let b: &i32 = &a; // aの不変参照をbに束縛（あまり意味はない）
    // let b: i32 = &a; 当然これはエラーになる
    println!("{}, {}", a, b);

    let mut a = 1; // リテラルの1(Copy)を可変で束縛
    let b: i32 = a; // b は a のコピーで束縛される 所有権はムーブしない
    println!("{}, {}", a, b);

    let mut a = 1;
    a = 3;
    let b: &i32 = &a;
    println!("{}, {}", a, b);

    // 可変束縛されているときに
    let mut a = 1;
    {
        let b: &i32 = &a;
        println!("{}", b);
    }
    a = 3;
    println!("{}", a);

    let mut a = 1;
    let b: &i32 = &a;
    // a = 4;　借用が存在するので変更できない
    println!("{}, {}", a, b);

    let mut a = 1;
    {
        let b: &i32 = &a;
    }
    a = 4; // 借用されたものはDROPされているので大丈夫
    println!("{}", a);
}