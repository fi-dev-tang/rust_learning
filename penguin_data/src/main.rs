fn output_penguin(){
    let penguin_data = "\
    common name, length(cm)
    Little penguin, 33
    Yellow-Eyed penguin, 56
    Forid penguin,75
    Invalid, data
    ";

    // 处理 penguin 数据，先将其一行一行处理
    let records = penguin_data.lines();

    // 对 records 中的每一行数据进行枚举，遍历的元素是: (第 i 行数据，第 i 行记录)
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;  // 排除第 1 行 和长度为 0 的行
        }


        // 将 records 按照 , 分割 ， 放入一个变长数组中，例如 ["Little penguin","33"]
        let fields: Vec<_> = record.split(',').map(|fields| fields.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{},{}cm", name, length);
        }
    }
}

fn main(){
    output_penguin();
}