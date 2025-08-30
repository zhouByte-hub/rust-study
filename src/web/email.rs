/**
 * lettre = "0.11"
 *
 * Rust 生态中一个功能强大、异步友好的 邮件发送库 (email sending library)。它允许你的 Rust 程序通过多种方式（最常见的是 SMTP）发送电子邮件。
 */

#[cfg(test)]
mod email_test {
    use lettre::{
        AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
        message::{
            Attachment, MultiPart, SinglePart,
            header::{self, ContentType},
        },
        transport::smtp::{
            authentication::Credentials,
            client::{Tls, TlsParameters},
        },
    };

    // 发送文本邮件
    #[tokio::test]
    async fn test_async_email() {
        // 构建邮件消息
        let message = Message::builder()
            .from("sender@qq.com".parse().unwrap()) // 设置发件人
            .reply_to("sender@qq.com".parse().unwrap()) // 设置收件人回复的地址
            .to("receiver@163.com".parse().unwrap()) // 设置收件人
            .subject("Rust异步普通文本测试邮件") // 设置邮件主题
            .header(header::ContentType::TEXT_PLAIN) // 设置邮件内容类型为文本
            .body("这是一封使用异步方式发送的Rust测试邮件，来自tokio1运行时。".to_string())
            .unwrap(); // 设置邮件内容

        // 配置SMTP认证凭据
        let credentials = Credentials::new("sender@qq.com".to_string(), "123123123".to_string());

        // 创建异步SMTP传输器
        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.qq.com")
            .unwrap()
            .port(587) // QQ邮箱SMTP TLS端口
            .credentials(credentials)
            // 使用Required而不是Wrapper，避免TLS配置冲突
            .tls(Tls::Required(
                TlsParameters::builder("smtp.qq.com".to_string())
                    .dangerous_accept_invalid_certs(false) // 开发环境可以接受无效证书
                    .build()
                    .unwrap(),
            ))
            .timeout(Some(std::time::Duration::from_secs(30))) // 设置超时时间
            .build();

        // 异步发送邮件
        match smtp_transport.send(message).await {
            Ok(_) => println!("异步发送邮件成功"),
            Err(e) => println!("异步发送邮件失败: {:?}", e),
        }
    }

    // 发送 HTML 内容邮件
    #[tokio::test]
    async fn send_html_email() {
        let message = Message::builder()
            .from("sender@qq.com".parse().unwrap()) // 设置发件人
            .reply_to("sender@qq.com".parse().unwrap()) // 设置收件人回复的地址
            .to("receiver@163.com".parse().unwrap()) // 设置收件人
            .subject("Rust异步HTML测试邮件") // 设置邮件主题
            .header(header::ContentType::TEXT_HTML) // 设置邮件内容类型为HTML
            .body("<h1>这是一封HTML邮件</h1>".to_string())
            .unwrap(); // 设置邮件内容

        let credentials = Credentials::new("sender@qq.com".to_string(), "123123123".to_string());

        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.qq.com")
            .unwrap()
            .port(587) // QQ邮箱SMTP TLS端口
            .credentials(credentials)
            // 使用Required而不是Wrapper，避免TLS配置冲突
            .tls(Tls::Required(
                TlsParameters::builder("smtp.qq.com".to_string())
                    .dangerous_accept_invalid_certs(false) // 开发环境可以接受无效证书
                    .build()
                    .unwrap(),
            ))
            .timeout(Some(std::time::Duration::from_secs(30))) // 设置超时时间
            .build();

        match smtp_transport.send(message).await {
            Ok(_) => println!("异步发送邮件成功"),
            Err(e) => println!("异步发送邮件失败: {:?}", e),
        }
    }

    // 发送附件
    #[tokio::test]
    async fn send_email_with_attachment() {
        let file_content = tokio::fs::read_to_string("src/web/jwt.rs").await.unwrap();
        let message = Message::builder()
            .from("sender@qq.com".parse().unwrap()) // 设置发件人
            .reply_to("sender@qq.com".parse().unwrap()) // 设置收件人回复的地址
            .to("receiver@163.com".parse().unwrap()) // 设置收件人
            .subject("Rust 异步附件测试邮件")
            .multipart(
                MultiPart::mixed()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body("Please find the attached document.".to_string()),
                    )
                    .singlepart(
                        Attachment::new("jwt.rs".to_string())
                            .body(file_content, "application/plain".parse().unwrap()),
                    ),
            )
            .unwrap();

        let credentials = Credentials::new("sender@qq.com".to_string(), "123123123".to_string());

        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("smtp.qq.com")
            .unwrap()
            .port(587) // QQ邮箱SMTP TLS端口
            .credentials(credentials)
            // 使用Required而不是Wrapper，避免TLS配置冲突
            .tls(Tls::Required(
                TlsParameters::builder("smtp.qq.com".to_string())
                    .dangerous_accept_invalid_certs(false) // 开发环境可以接受无效证书
                    .build()
                    .unwrap(),
            ))
            .timeout(Some(std::time::Duration::from_secs(30))) // 设置超时时间
            .build();

        match smtp_transport.send(message).await {
            Ok(_) => println!("异步发送邮件成功"),
            Err(e) => println!("异步发送邮件失败: {:?}", e),
        }
    }
}
