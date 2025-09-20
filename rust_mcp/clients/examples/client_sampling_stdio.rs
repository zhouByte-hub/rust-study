use rmcp::model::CallToolRequestParam;
use rmcp::model::{Content, CreateMessageResult, Role, SamplingMessage};
use rmcp::object;
use rmcp::transport::{ConfigureCommandExt, TokioChildProcess};
use rmcp::{ClientHandler, ErrorData, ServiceExt};
use tokio::process::Command;

pub struct SimpleClient;

impl SimpleClient {
    fn mock_llm_response(
        &self,
        _message: &[SamplingMessage],
        _system_prompt: Option<&str>,
    ) -> String {
        "It just a mock response".to_string()
    }
}

impl ClientHandler for SimpleClient {
    async fn create_message(
        &self,
        params: rmcp::model::CreateMessageRequestParam,
        // 接收客户端角色的请求上下文
        _context: rmcp::service::RequestContext<rmcp::RoleClient>,
    ) -> Result<CreateMessageResult, ErrorData> {
        // 调用 mock_llm_response 方法，传入 params 中的消息和系统提示
        let response_txt =
            self.mock_llm_response(&params.messages, params.system_prompt.as_deref());
        // 创建一个包含响应的 CreateMessageResult 结构体
        let result = CreateMessageResult {
            // 创建一个具有 Assistant 角色和响应文本的 SamplingMessage
            message: SamplingMessage {
                role: Role::Assistant,
                content: Content::text(response_txt),
            },
            // 设置模型名称为 "mock_llm"
            model: "mock_llm".to_string(),
            // 设置停止原因为结束轮次
            stop_reason: Some(CreateMessageResult::STOP_REASON_END_TURN.to_string()),
        };
        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimpleClient;
    // 获取当前目录，并与 "rust_mcp" 和 "servers" 连接，得到服务器目录
    let servers_dir = std::env::current_dir()?
        .join("rust_mcp")
        .join("servers");

    // 使用 TokioChildProcess 传输来服务客户端
    let client = client
        .serve(TokioChildProcess::new(Command::new("cargo").configure(
            // 配置 cargo 命令
            |cmd| {
                // 向命令添加 "run" 参数
                cmd.arg("run")
                    // 向命令添加 "--example" 参数
                    .arg("--example")
                    // 向命令添加 "servers_sampling_stdio" 参数
                    .arg("servers_sampling_stdio")
                    // 设置命令的当前目录为服务器目录
                    .current_dir(servers_dir);
            },
        ))?)
        .await?;
    // Wait for initialization
    // 等待初始化，休眠 1 秒
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    // 尝试列出服务器上的所有工具
    match client.list_all_tools().await {
        // 如果列出工具成功
        Ok(_tools) => {
            // 尝试调用 "ask_LLM" 工具并提问
            match client
                .call_tool(CallToolRequestParam {
                    // 设置工具名称为 "ask_LLM"
                    name: "ask_LLM".into(),
                    // 设置参数为一个包含 "question" 键和 "Hello world" 值的对象
                    arguments: Some(object!({
                        "question": "Hello world"
                    })),
                })
                .await
            {
                Ok(result) => println!("Ask LLM result: {:?}", result),
                Err(e) => println!("Ask LLM error: {e}"),
            }
        }
        Err(e) => eprintln!("Failed to list tools: {e}"),
    }
    // 在取消前等待 1 秒
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    // 取消客户端连接
    client.cancel().await?;
    Ok(())
}
