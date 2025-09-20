# RMCP 使用说明

## 什么是 RMCP

RMCP (Rust Model Context Protocol) 是 Model Context Protocol (MCP) 的官方 Rust 语言 SDK 实现。它是一个基于 tokio 异步运行时构建的高性能工具包，专门用于构建 MCP 服务器和客户端。

RMCP 提供了一套完整的 API，使开发者能够轻松创建 MCP 兼容的应用程序，实现与 AI 模型的标准化交互。通过 RMCP，开发者可以定义工具和资源，使 AI 模型能够执行各种任务，如文件操作、网络请求、计算等。

## RMCP 与 MCP 的关系

MCP (Model Context Protocol) 是一个开放标准，旨在为 AI 模型提供与外部工具和资源的标准化接口。它定义了一套协议，使 AI 模型能够安全、可控地与外部系统交互。

RMCP 是 MCP 协议在 Rust 生态系统中的官方实现，它提供了：

- 完整的 MCP 协议支持
- 高性能的异步处理能力
- 类型安全的 API 设计
- 灵活的扩展机制

## 核心概念

### 1. 服务器 (Server)

MCP 服务器是提供工具和资源的核心组件。它负责处理来自客户端的请求，执行相应的操作，并返回结果。在 RMCP 中，服务器通过实现 `ServerHandler` trait 来定义其行为。

### 2. 客户端 (Client)

MCP 客户端是连接到服务器并调用其工具和资源的组件。客户端可以是 AI 模型、应用程序或其他系统。

### 3. 工具 (Tools)

工具是服务器提供的可调用功能，类似于函数或方法。每个工具有名称、描述和输入参数定义。客户端可以调用这些工具来执行特定任务。

### 4. 资源 (Resources)

资源是服务器管理的可访问数据或对象。资源可以是文件、数据库记录、API 端点等。客户端可以列出、读取和更新资源。

### 5. 传输层 (Transport)

传输层负责服务器和客户端之间的通信。RMCP 支持多种传输方式，包括标准输入输出 (Stdio)、服务器发送事件 (SSE) 和 TCP。

## 安装与配置

要在项目中使用 RMCP，首先需要在 `Cargo.toml` 文件中添加依赖：

```toml
[dependencies]
rmcp = "0.6.4"
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
anyhow = "1.0"
```

## 基本使用

### 创建简单的 MCP 服务器

下面是一个简单的 MCP 服务器示例，实现了两个工具：`echo` 和 `add`：

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, Tool, CallToolResultContent};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::{json, Value};

struct MyServer;

impl ServerHandler for MyServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        async move {
            match request.name.as_str() {
                "echo" => {
                    let message = request.arguments
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Hello, World!");
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Echo: {}", message),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                "add" => {
                    let a = request.arguments
                        .get("a")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let b = request.arguments
                        .get("b")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Result: {}", a + b),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                _ => Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建服务器实例
    let server = Server::new(MyServer);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.handle_stream(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
```

### 创建 MCP 客户端

下面是一个简单的 MCP 客户端示例，连接到服务器并调用工具：

```rust
use rmcp::{Client, ClientHandler};
use rmcp::model::{CallToolRequest, CallToolResult};
use std::collections::HashMap;
use serde_json::json;

#[derive(Debug, Clone)]
struct MyClientHandler;

impl ClientHandler for MyClientHandler {
    // 实现客户端处理逻辑
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = Client::new(MyClientHandler);
    
    // 连接到服务器
    let stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;
    let connection = client.connect(stream).await?;
    
    // 调用 echo 工具
    let echo_request = CallToolRequest {
        params: rmcp::model::CallToolRequestParam {
            name: "echo".to_string(),
            arguments: json!({
                "message": "Hello from client!"
            }),
        },
    };
    
    let echo_result = connection.call_tool(echo_request).await?;
    println!("Echo result: {:?}", echo_result);
    
    // 调用 add 工具
    let add_request = CallToolRequest {
        params: rmcp::model::CallToolRequestParam {
            name: "add".to_string(),
            arguments: json!({
                "a": 5,
                "b": 3
            }),
        },
    };
    
    let add_result = connection.call_tool(add_request).await?;
    println!("Add result: {:?}", add_result);
    
    Ok(())
}
```

## 完整的 MCP 服务器实现

下面是一个更完整的 MCP 服务器实现，包含工具和资源管理：

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, Tool, CallToolResultContent, ListToolsRequest, ListToolsResult};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// 定义服务器状态
#[derive(Debug, Clone)]
struct ServerState {
    resources: Arc<RwLock<HashMap<String, String>>>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Debug, Clone)]
struct MyServer {
    state: ServerState,
}

impl MyServer {
    fn new() -> Self {
        Self {
            state: ServerState::new(),
        }
    }
}

impl ServerHandler for MyServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        let state = self.state.clone();
        
        async move {
            match request.name.as_str() {
                "echo" => {
                    let message = request.arguments
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Hello, World!");
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Echo: {}", message),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                "add" => {
                    let a = request.arguments
                        .get("a")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let b = request.arguments
                        .get("b")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Result: {}", a + b),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                "update_resource" => {
                    let name = request.arguments
                        .get("name")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'name' parameter".to_string(),
                            data: None,
                        })?;
                    
                    let content = request.arguments
                        .get("content")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'content' parameter".to_string(),
                            data: None,
                        })?;
                    
                    let mut resources = state.resources.write().await;
                    resources.insert(name.to_string(), content.to_string());
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Resource '{}' updated", name),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                _ => Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
    
    fn list_tools(
        &self,
        _request: ListToolsRequest,
    ) -> impl Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        
        async move {
            let tools = vec![
                Tool {
                    name: "echo".to_string(),
                    description: "Echo a message".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "message": {
                                "type": "string",
                                "description": "Message to echo"
                            }
                        },
                        "required": ["message"]
                    }),
                },
                Tool {
                    name: "add".to_string(),
                    description: "Add two numbers".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "a": {
                                "type": "number",
                                "description": "First number"
                            },
                            "b": {
                                "type": "number",
                                "description": "Second number"
                            }
                        },
                        "required": ["a", "b"]
                    }),
                },
                Tool {
                    name: "update_resource".to_string(),
                    description: "Update a resource".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Resource name"
                            },
                            "content": {
                                "type": "string",
                                "description": "Resource content"
                            }
                        },
                        "required": ["name", "content"]
                    }),
                },
            ];
            
            Ok(ListToolsResult { tools })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建服务器实例
    let server = Server::new(MyServer::new());
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.handle_stream(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
```

## MCP 客户端实现

下面是一个完整的 MCP 客户端实现，包括初始化、工具调用和资源访问：

```rust
use anyhow::Result;
use rmcp::{Client, ClientHandler};
use rmcp::model::{CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult};
use serde_json::json;

#[derive(Debug, Clone)]
struct MyClientHandler;

impl ClientHandler for MyClientHandler {
    // 实现客户端处理逻辑
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = Client::new(MyClientHandler);
    
    // 连接到服务器
    let stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;
    let connection = client.connect(stream).await?;
    
    // 初始化连接
    connection.initialize().await?;
    
    // 列出可用工具
    let tools_request = ListToolsRequest {};
    let tools_result = connection.list_tools(tools_request).await?;
    println!("Available tools:");
    for tool in tools_result.tools {
        println!("- {}: {}", tool.name, tool.description);
    }
    
    // 调用 echo 工具
    let echo_request = CallToolRequest {
        params: rmcp::model::CallToolRequestParam {
            name: "echo".to_string(),
            arguments: json!({
                "message": "Hello from client!"
            }),
        },
    };
    
    let echo_result = connection.call_tool(echo_request).await?;
    if let Some(content) = echo_result.content.first() {
        match content {
            rmcp::model::CallToolResultContent::Text { text, .. } => {
                println!("Echo result: {}", text);
            }
        }
    }
    
    // 调用 add 工具
    let add_request = CallToolRequest {
        params: rmcp::model::CallToolRequestParam {
            name: "add".to_string(),
            arguments: json!({
                "a": 5,
                "b": 3
            }),
        },
    };
    
    let add_result = connection.call_tool(add_request).await?;
    if let Some(content) = add_result.content.first() {
        match content {
            rmcp::model::CallToolResultContent::Text { text, .. } => {
                println!("Add result: {}", text);
            }
        }
    }
    
    // 更新资源
    let update_resource_request = CallToolRequest {
        params: rmcp::model::CallToolRequestParam {
            name: "update_resource".to_string(),
            arguments: json!({
                "name": "test_resource",
                "content": "This is a test resource"
            }),
        },
    };
    
    let update_result = connection.call_tool(update_resource_request).await?;
    if let Some(content) = update_result.content.first() {
        match content {
            rmcp::model::CallToolResultContent::Text { text, .. } => {
                println!("Update resource result: {}", text);
            }
        }
    }
    
    Ok(())
}
```

## 传输层支持

RMCP 支持多种传输方式，包括标准输入输出 (Stdio)、服务器发送事件 (SSE) 和 TCP。下面分别介绍这些传输方式的使用方法。

### Stdio 传输

Stdio 传输使用标准输入输出进行通信，适用于本地进程间通信：

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, CallToolResultContent};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::json;

#[derive(Debug, Clone)]
struct StdioServer;

impl ServerHandler for StdioServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        async move {
            match request.name.as_str() {
                "echo" => {
                    let message = request.arguments
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Hello, World!");
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Echo: {}", message),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                _ => Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(StdioServer);
    
    // 使用 stdio 传输
    server.handle_stdio().await?;
    
    Ok(())
}
```

### SSE 传输

SSE (Server-Sent Events) 传输使用 HTTP 服务器发送事件进行通信，适用于 Web 应用：

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, CallToolResultContent};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::json;

#[derive(Debug, Clone)]
struct SseServer;

impl ServerHandler for SseServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        async move {
            match request.name.as_str() {
                "echo" => {
                    let message = request.arguments
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Hello, World!");
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Echo: {}", message),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                _ => Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(SseServer);
    
    // 使用 SSE 传输
    server.handle_sse("127.0.0.1:8080").await?;
    
    Ok(())
}
```

### TCP 传输

TCP 传输使用 TCP 套接字进行通信，适用于网络通信：

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, CallToolResultContent};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::json;

#[derive(Debug, Clone)]
struct TcpServer;

impl ServerHandler for TcpServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        async move {
            match request.name.as_str() {
                "echo" => {
                    let message = request.arguments
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Hello, World!");
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Echo: {}", message),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                _ => Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(TcpServer);
    
    // 启动 TCP 服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("TCP Server running on 127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.handle_stream(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
```

## 工具管理

RMCP 提供了灵活的工具管理机制，使开发者能够动态添加、删除和修改工具。下面是一个工具管理的示例：

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, Tool, CallToolResultContent, ListToolsRequest, ListToolsResult};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// 工具定义
#[derive(Debug, Clone)]
struct ToolDefinition {
    name: String,
    description: String,
    input_schema: Value,
    handler: fn(HashMap<String, Value>) -> Result<String, String>,
}

// 工具服务器
#[derive(Debug, Clone)]
struct ToolServer {
    tools: Arc<RwLock<HashMap<String, ToolDefinition>>>,
}

impl ToolServer {
    fn new() -> Self {
        let mut tools = HashMap::new();
        
        // 添加 echo 工具
        tools.insert("echo".to_string(), ToolDefinition {
            name: "echo".to_string(),
            description: "Echo a message".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "Message to echo"
                    }
                },
                "required": ["message"]
            }),
            handler: |args| {
                let message = args.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Hello, World!");
                Ok(format!("Echo: {}", message))
            },
        });
        
        // 添加 calculate 工具
        tools.insert("calculate".to_string(), ToolDefinition {
            name: "calculate".to_string(),
            description: "Perform a calculation".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "Mathematical expression to evaluate"
                    }
                },
                "required": ["expression"]
            }),
            handler: |args| {
                let expression = args.get("expression")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'expression' parameter".to_string())?;
                
                // 简单的表达式计算（仅作示例，实际应用中应使用更安全的表达式求值库）
                let result = match expression {
                    "1+1" => "2".to_string(),
                    "2*3" => "6".to_string(),
                    "10/2" => "5".to_string(),
                    _ => format!("Unknown expression: {}", expression),
                };
                
                Ok(format!("Result: {}", result))
            },
        });
        
        Self {
            tools: Arc::new(RwLock::new(tools)),
        }
    }
    
    async fn add_tool(&self, tool: ToolDefinition) {
        let mut tools = self.tools.write().await;
        tools.insert(tool.name.clone(), tool);
    }
    
    async fn remove_tool(&self, name: &str) -> bool {
        let mut tools = self.tools.write().await;
        tools.remove(name).is_some()
    }
}

impl ServerHandler for ToolServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        let tools = self.tools.clone();
        
        async move {
            let tools_guard = tools.read().await;
            
            if let Some(tool_def) = tools_guard.get(&request.name) {
                let result = (tool_def.handler)(request.arguments.clone());
                
                match result {
                    Ok(text) => Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text,
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    }),
                    Err(error_msg) => Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Error: {}", error_msg),
                            type_: "text".to_string(),
                        }],
                        is_error: Some(true),
                    }),
                }
            } else {
                Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                })
            }
        }
    }
    
    fn list_tools(
        &self,
        _request: ListToolsRequest,
    ) -> impl Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        
        let tools = self.tools.clone();
        
        async move {
            let tools_guard = tools.read().await;
            let tools_list: Vec<Tool> = tools_guard.values().map(|tool_def| Tool {
                name: tool_def.name.clone(),
                description: tool_def.description.clone(),
                input_schema: tool_def.input_schema.clone(),
            }).collect();
            
            Ok(ListToolsResult { tools: tools_list })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(ToolServer::new());
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Tool Server running on 127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.handle_stream(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
```

## 资源管理

RMCP 提供了完整的资源管理功能，使开发者能够创建、读取、更新和删除资源。下面是一个资源管理的示例：

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, Tool, CallToolResultContent, ListToolsRequest, ListToolsResult};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// 资源数据
#[derive(Debug, Clone)]
struct ResourceData {
    name: String,
    content: String,
    metadata: HashMap<String, String>,
}

// 资源服务器
#[derive(Debug, Clone)]
struct ResourceServer {
    resources: Arc<RwLock<HashMap<String, ResourceData>>>,
}

impl ResourceServer {
    fn new() -> Self {
        Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    async fn add_resource(&self, name: String, content: String, metadata: HashMap<String, String>) {
        let mut resources = self.resources.write().await;
        resources.insert(name.clone(), ResourceData { name, content, metadata });
    }
    
    async fn get_resource(&self, name: &str) -> Option<ResourceData> {
        let resources = self.resources.read().await;
        resources.get(name).cloned()
    }
    
    async fn update_resource(&self, name: &str, content: String, metadata: Option<HashMap<String, String>>) -> bool {
        let mut resources = self.resources.write().await;
        if let Some(resource) = resources.get_mut(name) {
            resource.content = content;
            if let Some(new_metadata) = metadata {
                resource.metadata = new_metadata;
            }
            true
        } else {
            false
        }
    }
    
    async fn remove_resource(&self, name: &str) -> bool {
        let mut resources = self.resources.write().await;
        resources.remove(name).is_some()
    }
    
    async fn list_resources(&self) -> Vec<String> {
        let resources = self.resources.read().await;
        resources.keys().cloned().collect()
    }
}

impl ServerHandler for ResourceServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        let server = self.clone();
        
        async move {
            match request.name.as_str() {
                "add_resource" => {
                    let name = request.arguments
                        .get("name")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'name' parameter".to_string(),
                            data: None,
                        })?;
                    
                    let content = request.arguments
                        .get("content")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'content' parameter".to_string(),
                            data: None,
                        })?;
                    
                    let metadata = request.arguments
                        .get("metadata")
                        .and_then(|v| v.as_object())
                        .map(|obj| {
                            obj.iter()
                                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                                .collect::<HashMap<String, String>>()
                        })
                        .unwrap_or_else(HashMap::new);
                    
                    server.add_resource(name.to_string(), content.to_string(), metadata).await;
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Resource '{}' added successfully", name),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                "get_resource" => {
                    let name = request.arguments
                        .get("name")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'name' parameter".to_string(),
                            data: None,
                        })?;
                    
                    match server.get_resource(name).await {
                        Some(resource) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: format!("Resource '{}': {}", resource.name, resource.content),
                                type_: "text".to_string(),
                            }],
                            is_error: None,
                        }),
                        None => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: format!("Resource '{}' not found", name),
                                type_: "text".to_string(),
                            }],
                            is_error: Some(true),
                        }),
                    }
                }
                "update_resource" => {
                    let name = request.arguments
                        .get("name")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'name' parameter".to_string(),
                            data: None,
                        })?;
                    
                    let content = request.arguments
                        .get("content")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'content' parameter".to_string(),
                            data: None,
                        })?;
                    
                    let metadata = request.arguments
                        .get("metadata")
                        .and_then(|v| v.as_object())
                        .map(|obj| {
                            obj.iter()
                                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                                .collect::<HashMap<String, String>>()
                        });
                    
                    let updated = server.update_resource(name, content.to_string(), metadata).await;
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: if updated {
                                format!("Resource '{}' updated successfully", name)
                            } else {
                                format!("Resource '{}' not found", name)
                            },
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                "remove_resource" => {
                    let name = request.arguments
                        .get("name")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'name' parameter".to_string(),
                            data: None,
                        })?;
                    
                    let removed = server.remove_resource(name).await;
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: if removed {
                                format!("Resource '{}' removed successfully", name)
                            } else {
                                format!("Resource '{}' not found", name)
                            },
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                _ => Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
    
    fn list_tools(
        &self,
        _request: ListToolsRequest,
    ) -> impl Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        
        async move {
            let tools = vec![
                Tool {
                    name: "add_resource".to_string(),
                    description: "Add a new resource".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Resource name"
                            },
                            "content": {
                                "type": "string",
                                "description": "Resource content"
                            },
                            "metadata": {
                                "type": "object",
                                "description": "Resource metadata (optional)"
                            }
                        },
                        "required": ["name", "content"]
                    }),
                },
                Tool {
                    name: "get_resource".to_string(),
                    description: "Get a resource by name".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Resource name"
                            }
                        },
                        "required": ["name"]
                    }),
                },
                Tool {
                    name: "update_resource".to_string(),
                    description: "Update a resource".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Resource name"
                            },
                            "content": {
                                "type": "string",
                                "description": "New resource content"
                            },
                            "metadata": {
                                "type": "object",
                                "description": "New resource metadata (optional)"
                            }
                        },
                        "required": ["name", "content"]
                    }),
                },
                Tool {
                    name: "remove_resource".to_string(),
                    description: "Remove a resource".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Resource name"
                            }
                        },
                        "required": ["name"]
                    }),
                },
            ];
            
            Ok(ListToolsResult { tools })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(ResourceServer::new());
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Resource Server running on 127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.handle_stream(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
```

## 高级用法

### 自定义工具实现

RMCP 允许开发者实现自定义工具，以满足特定需求。下面是一个自定义工具的示例：

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, Tool, CallToolResultContent, ListToolsRequest, ListToolsResult};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::{json, Value};
use std::process::Command;

#[derive(Debug, Clone)]
struct CustomToolServer;

impl ServerHandler for CustomToolServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        async move {
            match request.name.as_str() {
                "execute_command" => {
                    let command = request.arguments
                        .get("command")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'command' parameter".to_string(),
                            data: None,
                        })?;
                    
                    let args = request.arguments
                        .get("args")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str())
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_else(Vec::new);
                    
                    // 执行命令
                    let output = Command::new(command)
                        .args(args)
                        .output();
                    
                    match output {
                        Ok(output) => {
                            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                            
                            Ok(CallToolResult {
                                content: vec![CallToolResultContent::Text {
                                    text: format!("Command executed successfully.\nExit code: {}\nStdout:\n{}\nStderr:\n{}", 
                                        output.status.code().unwrap_or(-1), stdout, stderr),
                                    type_: "text".to_string(),
                                }],
                                is_error: None,
                            })
                        }
                        Err(e) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: format!("Failed to execute command: {}", e),
                                type_: "text".to_string(),
                            }],
                            is_error: Some(true),
                        }),
                    }
                }
                "read_file" => {
                    let path = request.arguments
                        .get("path")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'path' parameter".to_string(),
                            data: None,
                        })?;
                    
                    match tokio::fs::read_to_string(path).await {
                        Ok(content) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: content,
                                type_: "text".to_string(),
                            }],
                            is_error: None,
                        }),
                        Err(e) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: format!("Failed to read file: {}", e),
                                type_: "text".to_string(),
                            }],
                            is_error: Some(true),
                        }),
                    }
                }
                "write_file" => {
                    let path = request.arguments
                        .get("path")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'path' parameter".to_string(),
                            data: None,
                        })?;
                    let content = request.arguments
                        .get("content")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'content' parameter".to_string(),
                            data: None,
                        })?;
                    
                    match tokio::fs::write(path, content).await {
                        Ok(_) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: "File written successfully".to_string(),
                                type_: "text".to_string(),
                            }],
                            is_error: None,
                        }),
                        Err(e) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: format!("Failed to write file: {}", e),
                                type_: "text".to_string(),
                            }],
                            is_error: Some(true),
                        }),
                    }
                }
                _ => Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
    
    fn list_tools(
        &self,
        _request: ListToolsRequest,
    ) -> impl Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        
        async move {
            let tools = vec![
                Tool {
                    name: "execute_command".to_string(),
                    description: "Execute a system command".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "command": {
                                "type": "string",
                                "description": "Command to execute"
                            },
                            "args": {
                                "type": "array",
                                "items": {
                                    "type": "string"
                                },
                                "description": "Command arguments"
                            }
                        },
                        "required": ["command"]
                    }),
                },
                Tool {
                    name: "read_file".to_string(),
                    description: "Read a file's content".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "Path to the file"
                            }
                        },
                        "required": ["path"]
                    }),
                },
                Tool {
                    name: "write_file".to_string(),
                    description: "Write content to a file".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "Path to the file"
                            },
                            "content": {
                                "type": "string",
                                "description": "Content to write"
                            }
                        },
                        "required": ["path", "content"]
                    }),
                },
            ];
            
            Ok(ListToolsResult { tools })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(CustomToolServer);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Custom Tool Server running on 127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.handle_stream(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
```

### 异步处理

RMCP 基于 tokio 异步运行时构建，完全支持异步操作。下面是一个展示异步处理的示例：

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, Tool, CallToolResultContent, ListToolsRequest, ListToolsResult};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
struct AsyncServer;

impl ServerHandler for AsyncServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        async move {
            match request.name.as_str() {
                "delayed_echo" => {
                    let message = request.arguments
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Hello, World!");
                    
                    let delay_secs = request.arguments
                        .get("delay_seconds")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(1);
                    
                    // 模拟异步延迟
                    sleep(Duration::from_secs(delay_secs)).await;
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Delayed Echo ({}s): {}", delay_secs, message),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                "fetch_url" => {
                    let url = request.arguments
                        .get("url")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ErrorData {
                            code: -32602,
                            message: "Missing 'url' parameter".to_string(),
                            data: None,
                        })?;
                    
                    // 异步获取 URL 内容
                    match fetch_url_async(url).await {
                        Ok(content) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: content,
                                type_: "text".to_string(),
                            }],
                            is_error: None,
                        }),
                        Err(e) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: format!("Failed to fetch URL: {}", e),
                                type_: "text".to_string(),
                            }],
                            is_error: Some(true),
                        }),
                    }
                }
                "parallel_tasks" => {
                    let count = request.arguments
                        .get("count")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(3);
                    
                    // 并行执行多个任务
                    let tasks: Vec<_> = (0..count)
                        .map(|i| async move {
                            sleep(Duration::from_millis(100 * i)).await;
                            format!("Task {} completed", i)
                        })
                        .collect();
                    
                    let results = futures::future::join_all(tasks).await;
                    let combined_results = results.join("\n");
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: combined_results,
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                _ => Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
    
    fn list_tools(
        &self,
        _request: ListToolsRequest,
    ) -> impl Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        
        async move {
            let tools = vec![
                Tool {
                    name: "delayed_echo".to_string(),
                    description: "Echo a message after a delay".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "message": {
                                "type": "string",
                                "description": "Message to echo"
                            },
                            "delay_seconds": {
                                "type": "integer",
                                "description": "Delay in seconds",
                                "default": 1
                            }
                        },
                        "required": ["message"]
                    }),
                },
                Tool {
                    name: "fetch_url".to_string(),
                    description: "Fetch content from a URL".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "url": {
                                "type": "string",
                                "description": "URL to fetch"
                            }
                        },
                        "required": ["url"]
                    }),
                },
                Tool {
                    name: "parallel_tasks".to_string(),
                    description: "Execute multiple tasks in parallel".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "count": {
                                "type": "integer",
                                "description": "Number of tasks to execute",
                                "default": 3
                            }
                        }
                    }),
                },
            ];
            
            Ok(ListToolsResult { tools })
        }
    }
}

// 异步获取 URL 内容
async fn fetch_url_async(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(AsyncServer);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Async Server running on 127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.handle_stream(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
```

### 错误处理

RMCP 提供了丰富的错误处理机制，使开发者能够优雅地处理各种错误情况。

```rust
use anyhow::Result;
use rmcp::{Server, ServerHandler};
use rmcp::model::{CallToolRequestParam, CallToolResult, Tool, CallToolResultContent, ListToolsRequest, ListToolsResult};
use rmcp::service::RequestContext;
use rmcp::RoleServer;
use rmcp::ErrorData;
use std::future::Future;
use serde_json::{json, Value};

// 自定义错误类型
#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}

impl From<MyError> for ErrorData {
    fn from(err: MyError) -> Self {
        match err {
            MyError::Io(_) => ErrorData {
                code: -32603,
                message: "Internal error".to_string(),
                data: Some(json!(err.to_string())),
            },
            MyError::Parse(_) => ErrorData {
                code: -32700,
                message: "Parse error".to_string(),
                data: Some(json!(err.to_string())),
            },
            MyError::Validation(_) => ErrorData {
                code: -32602,
                message: "Invalid params".to_string(),
                data: Some(json!(err.to_string())),
            },
            MyError::NotFound(_) => ErrorData {
                code: -32601,
                message: "Method not found".to_string(),
                data: Some(json!(err.to_string())),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct ErrorHandlingServer;

impl ServerHandler for ErrorHandlingServer {
    fn call_tool(
            &self,
            request: CallToolRequestParam,
            _context: RequestContext<RoleServer>,
        ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        
        async move {
            match request.name.as_str() {
                "divide" => {
                    let a = request.arguments
                        .get("a")
                        .and_then(|v| v.as_f64())
                        .ok_or_else(|| MyError::Validation("Missing or invalid 'a' parameter".to_string()))?;
                    
                    let b = request.arguments
                        .get("b")
                        .and_then(|v| v.as_f64())
                        .ok_or_else(|| MyError::Validation("Missing or invalid 'b' parameter".to_string()))?;
                    
                    if b == 0.0 {
                        return Err(MyError::Validation("Division by zero".to_string()).into());
                    }
                    
                    let result = a / b;
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Result: {}", result),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                "parse_json" => {
                    let json_str = request.arguments
                        .get("json")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| MyError::Validation("Missing 'json' parameter".to_string()))?;
                    
                    match serde_json::from_str::<serde_json::Value>(json_str) {
                        Ok(value) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: format!("Parsed JSON: {}", value),
                                type_: "text".to_string(),
                            }],
                            is_error: None,
                        }),
                        Err(e) => Err(MyError::Parse(format!("Failed to parse JSON: {}", e)).into()),
                    }
                }
                "read_config" => {
                    let path = request.arguments
                        .get("path")
                        .and_then(|v| v.as_str())
                        .unwrap_or("config.json");
                    
                    match tokio::fs::read_to_string(path).await {
                        Ok(content) => Ok(CallToolResult {
                            content: vec![CallToolResultContent::Text {
                                text: content,
                                type_: "text".to_string(),
                            }],
                            is_error: None,
                        }),
                        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                            Err(MyError::NotFound(format!("Config file '{}' not found", path)).into())
                        }
                        Err(e) => Err(MyError::Io(e).into()),
                    }
                }
                "validate_email" => {
                    let email = request.arguments
                        .get("email")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| MyError::Validation("Missing 'email' parameter".to_string()))?;
                    
                    // 简单的邮箱验证
                    if !email.contains('@') || !email.contains('.') {
                        return Err(MyError::Validation("Invalid email format".to_string()).into());
                    }
                    
                    Ok(CallToolResult {
                        content: vec![CallToolResultContent::Text {
                            text: format!("Email '{}' is valid", email),
                            type_: "text".to_string(),
                        }],
                        is_error: None,
                    })
                }
                _ => Err(ErrorData {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
    
    fn list_tools(
        &self,
        _request: ListToolsRequest,
    ) -> impl Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        
        async move {
            let tools = vec![
                Tool {
                    name: "divide".to_string(),
                    description: "Divide two numbers".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "a": {
                                "type": "number",
                                "description": "Numerator"
                            },
                            "b": {
                                "type": "number",
                                "description": "Denominator"
                            }
                        },
                        "required": ["a", "b"]
                    }),
                },
                Tool {
                    name: "parse_json".to_string(),
                    description: "Parse a JSON string".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "json": {
                                "type": "string",
                                "description": "JSON string to parse"
                            }
                        },
                        "required": ["json"]
                    }),
                },
                Tool {
                    name: "read_config".to_string(),
                    description: "Read a configuration file".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "Path to the config file",
                                "default": "config.json"
                            }
                        }
                    }),
                },
                Tool {
                    name: "validate_email".to_string(),
                    description: "Validate an email address".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "email": {
                                "type": "string",
                                "description": "Email address to validate"
                            }
                        },
                        "required": ["email"]
                    }),
                },
            ];
            
            Ok(ListToolsResult { tools })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(ErrorHandlingServer);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Error Handling Server running on 127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.handle_stream(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}
```

## 最佳实践

### 1. 设计良好的工具接口

- **清晰的命名**：使用描述性的名称来明确工具的功能。
- **详细的文档**：为每个工具提供详细的描述和参数说明。
- **输入验证**：对所有输入参数进行验证，确保它们符合预期格式和范围。
- **错误处理**：提供有意义的错误消息，帮助客户端理解问题所在。

### 2. 资源管理

- **合理设计资源结构**：根据应用需求设计合理的资源层次结构。
- **缓存策略**：对于频繁访问的资源，考虑实现缓存机制以提高性能。
- **访问控制**：实现适当的访问控制，确保只有授权的客户端才能访问敏感资源。

### 3. 异步处理

- **避免阻塞操作**：尽量使用异步版本的函数和方法，避免阻塞事件循环。
- **合理的超时设置**：为长时间运行的操作设置合理的超时时间。
- **并发控制**：对于高并发场景，实现适当的并发控制机制，如信号量或限流器。

### 4. 安全考虑

- **输入验证**：对所有输入进行严格验证，防止注入攻击。
- **敏感信息保护**：避免在日志或错误消息中暴露敏感信息。
- **认证与授权**：实现适当的认证和授权机制，确保只有合法用户可以访问系统。

### 5. 性能优化

- **连接池**：对于数据库连接等资源，使用连接池减少创建和销毁的开销。
- **批处理**：对于大量小请求，考虑实现批处理机制以提高效率。
- **监控与日志**：实现完善的监控和日志系统，帮助识别和解决性能问题。

## 常见问题与解决方案

### 1. 连接问题

**问题**：客户端无法连接到服务器。

**可能原因**：
- 服务器未启动或端口被占用
- 网络配置问题
- 防火墙阻止连接

**解决方案**：
- 确保服务器已启动并监听正确的端口
- 检查网络配置，确保客户端可以访问服务器
- 检查防火墙设置，确保允许相关端口的通信

### 2. 协议版本不匹配

**问题**：客户端和服务器使用不同版本的 MCP 协议，导致通信失败。

**解决方案**：
- 确保客户端和服务器使用相同版本的 RMCP 库
- 在初始化时检查协议版本，如果不兼容则提供明确的错误消息
- 考虑实现向后兼容性，支持多个协议版本

### 3. 工具调用失败

**问题**：工具调用返回错误或意外结果。

**可能原因**：
- 参数验证失败
- 工具实现中的错误
- 资源不可用

**解决方案**：
- 检查工具调用的参数是否符合预期格式
- 查看服务器日志，了解详细的错误信息
- 确保所需的资源（如文件、数据库连接等）可用

### 4. 性能问题

**问题**：服务器响应缓慢或无法处理高并发请求。

**解决方案**：
- 优化工具实现，减少不必要的计算和 I/O 操作
- 使用异步操作和非阻塞 I/O
- 考虑实现缓存机制，减少重复计算
- 对于高并发场景，考虑使用连接池和负载均衡

### 5. 内存泄漏

**问题**：服务器运行一段时间后内存使用量不断增加。

**解决方案**：
- 检查代码中是否有未释放的资源，如文件句柄、数据库连接等
- 使用 Rust 的所有权和借用检查器确保资源正确释放
- 使用内存分析工具（如 Valgrind）检测内存泄漏

## 总结

RMCP (Rust Model Context Protocol) 是 MCP 协议在 Rust 生态系统中的官方实现，为开发者提供了构建 MCP 兼容应用的强大工具。通过 RMCP，开发者可以轻松创建 MCP 服务器和客户端，实现与 AI 模型的标准化交互。

本文档详细介绍了 RMCP 的基本概念、使用方法和最佳实践，包括：

1. RMCP 与 MCP 的关系和核心概念
2. 如何创建和配置 MCP 服务器和客户端
3. 工具和资源的管理方法
4. 不同传输方式的使用
5. 自定义工具实现和异步处理
6. 错误处理和最佳实践
7. 常见问题的解决方案

通过遵循本文档中的指导，开发者可以充分利用 RMCP 的功能，构建高效、可靠的 MCP 应用程序，为 AI 模型提供丰富的工具和资源访问能力。