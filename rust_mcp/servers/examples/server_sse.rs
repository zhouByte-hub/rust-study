use rmcp::model::{Implementation, ProtocolVersion, ServerCapabilities, ServerInfo};
use rmcp::{model::{CallToolResult, Content}, tool_handler, tool_router, transport::{SseServer}, ErrorData, ServerHandler};
use rmcp::tool;
use rmcp::handler::server::tool::ToolRouter;


#[tokio::main]
async fn main() {

    // 创建一个 SSE 服务器，绑定到指定的地址和端口，然后为其注册一个服务处理程序。SSE 是一种允许服务器向客户端推送事件的技术，在这里用于实现 MCP 协议的通信。
    let ct = SseServer::serve("127.0.0.1:8080".parse().unwrap()).await.unwrap().with_service(move || {
        EmailServer::new()
    });

    tokio::signal::ctrl_c().await.unwrap();
    ct.cancel();

}



struct EmailServer {
    // tool_router 字段是必须的，不然 tool_handler 会报错
    tool_router: ToolRouter<EmailServer>,
}

impl EmailServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router()
        }
    }
}

#[tool_router]
impl EmailServer {

    #[tool(description = "发送邮件")]
    async fn send_email(&self) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![Content::text("邮件发送成功")]))
    }

    #[tool(description = "获取联系人列表")]
    async fn contact_list(&self) -> Result<CallToolResult, ErrorData> {
        let mut list = Vec::new();
        for i in 0..10 {
            let contact = Content::text(format!("联系人{}", i));
            list.push(contact);
        }
        Ok(CallToolResult::success(list))
    }
}

#[tool_handler]
impl ServerHandler for EmailServer {
    
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::default(),
            instructions: Some("此服务器提供发送邮件和获取联系人列表的工具。".to_string()),
        }
    }
    
}
