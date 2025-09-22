use std::sync::Arc;
use std::sync::Mutex;

use rmcp::ErrorData as McpError;
use rmcp::ServerHandler;
use rmcp::ServiceExt;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use rmcp::model::Implementation;
use rmcp::model::ProtocolVersion;
use rmcp::model::ServerCapabilities;
use rmcp::model::ServerInfo;
use rmcp::tool;
use rmcp::tool_handler;
use rmcp::tool_router;
use rmcp::transport::stdio;

#[derive(Clone)]
struct Counter {
    counter: Arc<Mutex<i32>>,
    tool_router: ToolRouter<Counter>,
}

/**
 * #[tool_router] 宏是 rmcp (Rust Model Context Protocol) 库中的一个过程宏，
 * 用于在 MCP (Model Context Protocol) 服务器实现中自动生成工具路由逻辑。它的主要作用包括：
 *      1、该宏会扫描 impl 块中所有使用 #[tool] 标记的方法，并为它们自动生成路由逻辑。
 *      2、创建ToolRouter 实例。
 *      3、将所有标记为 #[tool] 的方法注册到路由器中，使得当客户端请求特定的工具时，服务器能够正确地调用对应的处理函数。
 */
#[tool_router]
impl Counter {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            tool_router: Self::tool_router(),
        }
    }

    /**
     * tool宏用于将方法标记为 MCP 工具。
     */
    #[tool(description = "将计数器增加 1")]
    async fn increment(&self) -> Result<CallToolResult, McpError> {
        let mut count = self.counter.lock().unwrap();
        *count += 1;
        Ok(CallToolResult::success(vec![Content::text(format!(
            "{}",
            count
        ))]))
    }

    #[tool(description = "获取当前计数器值")]
    async fn get_value(&self) -> Result<CallToolResult, McpError> {
        let count = self.counter.lock().unwrap();
        Ok(CallToolResult::success(vec![Content::text(format!(
            "{}",
            count
        ))]))
    }
}

/**
 * #[tool_handler] 宏是 rmcp (Rust Model Context Protocol) 库中的一个过程宏，
 * 用于在 MCP (Model Context Protocol) 服务器实现中自动处理工具调用的逻辑，主要作用包括：
 *      1、该宏为 Counter 结构体自动实现 ServerHandler trait，这是处理 MCP 服务器请求所需的核心 trait。
 *      2、#[tool_handler] 宏与 #[tool_router] 宏协同工作。
 *         #[tool_router] 负责创建工具路由逻辑，而 #[tool_handler] 则负责将这些路由集成到服务器的请求处理流程中，
 *         使得服务器能够正确地接收和分发工具调用请求。
 *      3、当客户端发起工具调用请求时， #[tool_handler] 宏生成的代码会自动将请求路由到正确的处理函数（如 increment 或 get_value 方法），而不需要开发者手动编写请求分发逻辑。
 *      4、使用这个宏可以大大简化 MCP 服务器的实现，开发者只需要专注于实现具体的工具逻辑和服务器基本信息（如 get_info 方法），而不需要手动处理底层协议细节。
 */
#[tool_handler]
impl ServerHandler for Counter {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::default(),
            instructions: Some("此服务器提供一个计数器工具。计数器从 0 开始，可以增加。使用 'get_value' 检查当前计数。".to_string()),
        }
    }
}

#[tokio::main]
async fn main() {
    let server = Counter::new().serve(stdio()).await.unwrap();
    server.waiting().await.unwrap();
}
