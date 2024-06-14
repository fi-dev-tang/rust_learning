// 定义主函数 main
fn main(){
    // 定义一个包含企鹅数据的多行字符串，每行代表一种企鹅的名称和长度
    let penguin_data = "\
    common name, length(cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid, data
    ";

    // 使用 lines 方法将字符串分割成一个迭代器，每个元素是一行
    let records = penguin_data.lines();

    // 遍历每一行，并同时获取行索引和行内容
    for (i, record) in records.enumerate() {
        // 跳过第一行(标题行) 和空白行
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 使用 split(',') 方法按逗号分割每一行，然后对每个部分应用 trim 去除空白，
        // 最后收集到一个 Vec 中， Vec 元素类型由编译器根据上下文推断
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        // 当 debug_assertions 配置启用时(通常在调试构建中)，打印当前处理的行和分割后的字段到标准错误输出
        if cfg!(debug_assertions){
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        // 获取分割后的第一个字段作为企鹅名称
        let name = fields[0];

        // 尝试将第二个字段 (fields[1]) 转换为 f32 类型的浮点数
        // 使用 if let 匹配 Result::Ok 类型，表示转换成功，并将值绑定到 length 变量
        if let Ok(length) = fields[1].parse::<f32>(){
            // 如果转换成功，将企鹅名称和长度打印到标准输出
            println!("{},{}", name, length);
        }
    }
}