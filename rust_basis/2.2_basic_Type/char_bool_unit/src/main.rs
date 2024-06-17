/*
Rust 的字符不仅仅是 ASCII, 所有 Unicode 字符都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等
字符并不是 Unicode 中的一个概念，所以直觉上可能认为字符与 Rust 字符的概念并不一致。
Rust 字符占 4 个字节
*/
fn output_char(){
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{} {} {} {}", c, z, g, heart_eyed_cat);
}

fn output_charsize(){
    let x = '中';
    println!("字符'中'占用了 {} 字节内存大小",std::mem::size_of_val(&x));
}

fn output_boolsize(){
    let _t = true;

    let f: bool = false;
    if f {
        println!("this is meaningless code!");
    }
}

fn main(){
    output_char();
    output_charsize();

    output_boolsize();
}