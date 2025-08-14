
use include_dir::{include_dir, Dir};

// 在编译时就会检测文件是否存在，以便加载到内存中。
static FILE_DIR:Dir = include_dir!("E:\\project\\rust\\rust-study\\rust_include_dir\\src\\config");

fn main() {
    let settings = FILE_DIR.get_file("settings.ini").unwrap();
    let content = settings.contents_utf8().unwrap();

    println!("{}", content);
}
