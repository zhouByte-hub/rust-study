use std::sync::Arc;

use rmcp::model::{
    CallToolResult, Content, ContextInclusion, CreateMessageRequestParam, ErrorCode,
    Implementation, ListToolsResult, ModelHint, ModelPreferences, ProtocolVersion, SamplingMessage,
    ServerCapabilities, ServerInfo, Tool,
};
use rmcp::transport::stdio;
use rmcp::{ErrorData, ServerHandler, ServiceExt};

/// 一个简单的MCP（Model Context Protocol）服务器，演示了工具的使用
/// 和LLM采样功能。
#[derive(Clone, Debug, Default)]
struct SimpleServer;

impl ServerHandler for SimpleServer {
    /// 返回服务器信息，包括协议版本、功能、实现详情和使用说明。
    fn get_info(&self) -> ServerInfo {
        ServerInfo{
            protocol_version: ProtocolVersion::default(),   // MCP协议版本
            capabilities: ServerCapabilities::builder().enable_tools().build(),    // 定义MCP服务器支持的功能和能力
            server_info: Implementation::from_build_env(),  // 服务器实现信息，包括名称、版本、作者等
            // 提供服务器的使用说明或指导信息
            instructions: Some(concat!(
                "This is a demo server that requests sampling from clients. It provides tools that use LLM capabilities.\n\n",
                "IMPORTANT: This server requires a client that supports the 'sampling/createMessage' method. ",
                "Without sampling support, the tools will return errors."
            ).into()),
        }
    }

    /// 处理来自客户端的工具调用。目前支持'ask_LLM'工具，
    /// 该工具通过客户端采样向LLM发送提示。
    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        match request.name.as_ref() {
            "ask_LLM" => {
                // 从请求参数中提取问题，如果没有提供则使用默认值
                let prompt = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("question"))
                    .and_then(|q| q.as_str())
                    .unwrap_or("Hello, LLM!");

                // 创建用于LLM采样的消息请求
                let message_request = CreateMessageRequestParam {
                    messages: vec![SamplingMessage {
                        role: rmcp::model::Role::User,  // 消息发送者的角色，这里表示用户消息
                        content: Content::text(prompt), // 消息内容，将提示文本转换为内容格式
                    }],
                    // 模型偏好设置，用于指导客户端选择合适的LLM模型
                    model_preferences: Some(ModelPreferences {
                        // 模型提示，指定偏好使用的模型
                        hints: Some(vec![ModelHint {
                            name: Some("claude".to_string()), // 指定使用Claude模型
                        }]),
                        cost_priority: Some(0.3), // 成本优先级，值越低表示越注重成本控制
                        speed_priority: Some(0.8), // 速度优先级，值越高表示越注重响应速度
                        intelligence_priority: Some(0.7), // 智能优先级，值越高表示越注重回答质量
                    }),
                    // 系统提示，用于设定LLM的角色和行为准则
                    system_prompt: Some("You are a helpful assistant.".to_string()),
                    // 是否包含上下文信息，None表示不包含之前的对话上下文
                    include_context: Some(ContextInclusion::None),
                    // 温度参数，控制LLM输出的随机性，0.7表示中等偏上的随机性
                    temperature: Some(0.7),
                    // 最大令牌数，限制LLM响应的长度，256个令牌约等于200-300个英文单词
                    max_tokens: 256,
                    // 停止序列，当LLM生成这些序列时停止生成，None表示不设置停止序列
                    stop_sequences: None,
                    // 元数据，可以包含任何额外的请求信息，None表示没有额外的元数据
                    metadata: None,
                };

                // 向客户端发送采样请求并处理响应
                match context.peer.create_message(message_request).await {
                    Ok(response) => {
                        let text = format!(
                            "Question: {}\nAnswer:{}",
                            prompt,
                            response
                                .message
                                .content
                                .as_text()
                                .map(|t| &t.text)
                                .unwrap_or(&"No text response".to_string())
                        );
                        Ok(CallToolResult::success(vec![Content::text(text)]))
                    }
                    Err(e) => Err(ErrorData::new(
                        ErrorCode::INTERNAL_ERROR,
                        format!("Sampling request failed: {}", e),
                        None,
                    )),
                }
            }
            // 对未知工具返回错误
            _ => Err(ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Unknown tool: {}", request.name),
                None,
            )),
        }
    }

    /// 返回此服务器提供的可用工具列表。
    /// 目前只提供用于LLM交互的'ask_LLM'工具。
    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        // 定义'ask_LLM'工具及其模式
        let tool = Tool {
            name: "ask_LLM".into(),
            title: Some("Ask LLM".into()),
            description: Some("Ask a question to the LLM through sampling".into()),
            input_schema: Arc::new(
                serde_json::from_value(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "question": {
                            "type": "string",
                            "description": "The question to ask the LLM"
                        }
                    },
                    "required": ["question"]
                }))
                .unwrap(),
            ),
            output_schema: None,
            annotations: None,
            icons: None,
        };
        Ok(ListToolsResult {
            tools: vec![tool],
            next_cursor: None,
        })
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    // 使用stdio传输创建并服务SimpleServer
    let server = SimpleServer.serve(stdio()).await.unwrap();
    // 等待服务器完成（这将阻塞直到服务器停止）
    server.waiting().await.unwrap();
}
