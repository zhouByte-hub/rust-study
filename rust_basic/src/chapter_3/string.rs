pub fn string_add_operator() {
    let mut content = String::from("abc");
    content.push('a'); // 追加一个字符
    content.push_str("abcdef"); // 追加字符串
}

pub fn string_insert_operator() {
    let mut content = String::from("abc");
    content.insert(0, 'h'); // 指定位置插入单个字符
    content.insert_str(0, "hello"); // 指定位置插入字符串
}

pub fn string_replice_operator() {
    let mut content = String::from("ddd");
    let new_content = content.replace("d", "aaa"); // 替换字符
    let _ = content.replacen("d", "dd", 1); // 替换指定个数
    let _range = content.replace_range(0..2, "a"); // 替换指定范围的字符
    println!("{}", new_content);
}

pub fn string_delete_operator() {
    let mut content = String::from("rust study progrem");
    let last = content.pop(); // 删除并返回字符串的最后一个字符，该方法是直接操作原来的字符串
    println!("{:?}", last);

    let remove_chat = content.remove(3); // 删除并返回字符串中指定位置的字符，该方法是直接操作原来的字符串
    println!("{}", remove_chat);

    content.truncate(5); // 删除字符串中从指定位置开始到结尾的全部字符，该方法是直接操作原来的字符串

    content.clear(); // 清空字符串，该方法是直接操作原来的字符串、
}

pub fn string_plus_operator() {
    // 使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型，+ 是返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰。
    let content = String::from("abc");
    let a = content + "abc";
    println!("{}", a);

    let format_result = format!("{}-{}-{}", "mac", "host", "64");
    println!("{}", format_result);
}
