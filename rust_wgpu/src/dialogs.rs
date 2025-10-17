/**
 * rfd 库无法在非窗口环境中创建对话框。
 */
#[cfg(test)]
mod dialog_test{

    #[test]
    fn simple(){
        let path = std::env::current_dir().unwrap();

        let res = rfd::FileDialog::new()
            .add_filter("text", &["txt", "rs"])
            .add_filter("rust", &["rs", "toml"])
            .set_directory(&path)
            .pick_files();

        println!("{:?}", res);
    }
}