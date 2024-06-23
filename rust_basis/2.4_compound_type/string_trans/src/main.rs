fn trans_example1(){
    // \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F(\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {}(U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式，使用 \ 忽略换行符
    let long_string = "String literals
                       can span multiple lines.
                       The linebreak and indentation here -> \
                       <- can be escaped too!";
    println!("{}", long_string);
}

fn trans_example2(){
    // 希望保留字符串的原样，不要转义
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escape don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

// 操作字符串的两种方式
fn multiple_string_in_2_ways(){
    let chinese = String::from("中国人");
    // Unicode 方式遍历字符串
    for c in chinese.chars(){
        print!("{} ", c);
    }

    // 返回字符串底层字节数组表现形式
    for c in chinese.bytes() {
        print!("{} ", c);
    }
}

fn main(){
    trans_example1();
    trans_example2();

    multiple_string_in_2_ways();
}