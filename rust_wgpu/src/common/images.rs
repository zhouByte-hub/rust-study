/**
 * image = "0.25.8"
 */

#[cfg(test)]
mod images_test {
    use image::ImageReader;

    #[test]
    fn load_images() {
        // 使用正确的文件扩展名
        let image = ImageReader::open("src/common/1.png")
            .unwrap()
            .decode()
            .unwrap();
        println!("{}, {}", image.width(), image.height());
        let bytes = image.as_bytes();
        println!("{:?}", bytes);
    }

    #[test]
    fn write_images() {
        let image = ImageReader::open("src/common/1.png")
            .unwrap()
            .decode()
            .unwrap();
        image.save("src/common/2.jpg").unwrap();
    }
}
