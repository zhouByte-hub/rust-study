/**
 * clipboard = "0.5.0"
 * 
 * rust-clipboard 是一个跨平台的库，用于获取和设置操作系统级别的剪贴板内容。
 */

#[cfg(test)]
mod clipboard_test{
    use clipboard::ClipboardProvider;


    #[test]
    fn get_test(){
        let mut ctx = clipboard::ClipboardContext::new().unwrap();
        let content = ctx.get_contents().unwrap();
        println!("{}", content);
    }


    #[test]
    fn set_test() {
        let mut ctx = clipboard::ClipboardContext::new().unwrap();
        ctx.set_contents("hello world".to_string()).unwrap();
    }

}