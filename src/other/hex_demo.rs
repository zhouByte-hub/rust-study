/**
 * hex = "0.4.3"
 * 
 * 将数据编码/解码为十六进制表示形式。
 * features:
 *      1、std：默认启用
 *      2、serde：默认禁用
 */
#[cfg(test)]
mod hex_test{

    #[test]
    fn test_1(){
        let encode_data = hex::encode("hello world");
        println!("{}", encode_data);
        let decode_data = hex::decode(encode_data).unwrap();
        println!("{}", String::from_utf8(decode_data).unwrap());
    }
}