use rmcp::model::{Implementation, ProtocolVersion, ServerCapabilities, ServerInfo};
use rmcp::{model::{CallToolResult, Content}, tool_handler, tool_router, transport::{SseServer}, ErrorData, ServerHandler};
use rmcp::tool;
use rmcp::handler::server::tool::ToolRouter;
use serde::{Deserialize, Serialize};
use rmcp::schemars::JsonSchema;
use rmcp::handler::server::wrapper::Parameters;

/// 发送邮件的参数
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "发送邮件的参数")]
pub struct SendEmailArgs {
    #[schemars(description = "收件人邮箱地址")]
    pub to: String,
    #[schemars(description = "邮件主题")]
    pub subject: String,
    #[schemars(description = "邮件内容")]
    pub body: String,
}

/// 获取联系人列表的参数
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "获取联系人列表的参数")]
pub struct ContactListArgs {
    #[schemars(description = "联系人分组名称，可选")]
    pub group: Option<String>,
    #[schemars(description = "返回的联系人数量限制，默认为10")]
    pub limit: Option<i32>,
}

/// 获取工具列表的参数（空参数）
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "获取工具列表的参数")]
pub struct ListToolArgs {
    // 空结构体，不需要任何字段
}


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

    /**
     * 在 MCP 框架中，所有使用 #[tool] 宏标记的方法都需要有参数结构体，即使这些参数是空的。这是因为：
     *      1、协议规范要求 ：MCP 协议要求每个工具都必须有一个输入模式（inputSchema），且类型必须是 "object"。
     *      2、框架设计 ： #[tool] 宏会为每个工具方法生成相应的 schema，如果方法没有参数，就无法生成符合协议要求的 schema。
     */
    #[tool(description = "发送邮件")]
    async fn send_email(&self, Parameters(args): Parameters<SendEmailArgs>) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![Content::text(format!(
            "邮件已发送至: {}, 主题: {}, 内容: {}", 
            args.to, args.subject, args.body
        ))]))
    }

    #[tool(description = "获取联系人列表")]
    async fn contact_list(&self, Parameters(args): Parameters<ContactListArgs>) -> Result<CallToolResult, ErrorData> {
        let limit = args.limit.unwrap_or(10);
        let group_info = args.group.map_or("所有联系人".to_string(), |g| format!("{}分组", g));
        
        let mut list = Vec::new();
        for i in 0..limit {
            let contact = Content::text(format!("{}{}", group_info, i));
            list.push(contact);
        }
        Ok(CallToolResult::success(list))
    }

    #[tool(description = "获取所有工具列表")]
    async fn list_tool(&self, Parameters(_args): Parameters<ListToolArgs>) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![
            Content::text("send_email".to_string()),
            Content::text("contact_list".to_string()),
        ]))
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
