/**
 * qrcode-generator = "5.0.0"
 *
 * 在 RAW、PNG 和 SVG 格式中生成 QR 码矩阵和图像。
 * qrcode-generator: 这是一个 Rust 库（crate）的名称。它的功能是生成 QR Code（二维码）
 * 除了用来生成二维码之外还可以用来进行简单的修图
 */

#[cfg(test)]
mod qrcode_test {
    use qrcode_generator::QrCodeEcc;
    use std::io::{BufWriter, Write};

    #[test]
    fn png_test() {
        let content = "https://www.baidu.com";
        // 生成二维码输出到文件
        qrcode_generator::to_png_to_file(
            &content,
            QrCodeEcc::Medium,
            300,
            "src/web/images/png_qrcode.png",
        )
        .unwrap();

        // 将生成的二维码输出到矩阵
        let list = qrcode_generator::to_png_to_vec(&content, QrCodeEcc::High, 300).unwrap();
        println!("{:?}", list);

        // 将生成的二维码输出到输出流
        let mut writer =
            BufWriter::new(std::fs::File::create("src/web/images/png_qrcode.png").unwrap());
        qrcode_generator::to_png_to_writer(content, QrCodeEcc::High, 300, &mut writer).unwrap();
        writer.flush().unwrap();
    }

    #[test]
    fn svg_test() {
        let content = "https://www.baidu.com";
        // 将生成的二维码写入文件
        qrcode_generator::to_svg_to_file(
            content,
            QrCodeEcc::High,
            300,
            Some("svg qrcode"),
            "src/web/images/svg_qrcode.svg",
        )
        .unwrap();

        // 将生成的二维码写入字符串
        let svg =
            qrcode_generator::to_svg_to_string(content, QrCodeEcc::High, 300, Some("svg qrcode"))
                .unwrap();
        println!("{:?}", svg); // 是一个XML文件，内部有一个元素<svg>

        // 将生成的二维码写入输出流
        let mut writer =
            BufWriter::new(std::fs::File::create("src/web/images/svg_qrcode.svg").unwrap());
        qrcode_generator::to_svg_to_writer(
            content,
            QrCodeEcc::High,
            300,
            Some("svg qrcode"),
            &mut writer,
        )
        .unwrap();
    }
}
