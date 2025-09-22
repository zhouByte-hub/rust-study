use rmcp::{
    ServiceExt,
    model::CallToolRequestParam,
    transport::{ConfigureCommandExt, TokioChildProcess},
};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("正在启动 Counter MCP 客户端...");

    let servers_dir = std::env::current_dir()?.join("rust_mcp").join("servers");

    // 使用子进程连接到服务器
    let client = ()
        .serve(TokioChildProcess::new(Command::new("cargo").configure(
            |cmd| {
                cmd.arg("run")
                    .arg("--example")
                    .arg("count_server_stdio")
                    .current_dir(servers_dir);
            },
        ))?)
        .await?;

    let peer_info = client.peer_info().unwrap();
    println!("info: {:?}", peer_info);

    let tools = client.list_tools(None).await?;

    for item in tools.tools {
        println!("{:?}", item);
    }

    // 调用 increment 工具
    println!("调用 increment...");
    let result = client
        .call_tool(CallToolRequestParam {
            name: "increment".into(),
            arguments: None,
        })
        .await?;
    println!("结果：{:?}", result);

    // 调用 get_value 工具
    println!("调用 get_value...");
    let result = client
        .call_tool(CallToolRequestParam {
            name: "get_value".into(),
            arguments: None,
        })
        .await?;
    println!("当前值：{:?}", result);

    Ok(())
}
