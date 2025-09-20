#[cfg(test)]
mod simple_client_test {
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
            _context: rmcp::service::RequestContext<rmcp::RoleClient>,
        ) -> Result<CreateMessageResult, ErrorData> {
            let response_txt =
                self.mock_llm_response(&params.messages, params.system_prompt.as_deref());
            let result = CreateMessageResult {
                message: SamplingMessage {
                    role: Role::Assistant,
                    content: Content::text(response_txt),
                },
                model: "mock_llm".to_string(),
                stop_reason: Some(CreateMessageResult::STOP_REASON_END_TURN.to_string()),
            };
            Ok(result)
        }
    }

    #[tokio::test]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let client = SimpleClient;
        let servers_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("mcp")
            .join("servers");

        let client = client
            .serve(TokioChildProcess::new(Command::new("cargo").configure(
                |cmd| {
                    cmd.arg("run")
                        .arg("--example")
                        .arg("servers_sampling_stdio")
                        .current_dir(servers_dir);
                },
            ))?)
            .await?;
        // Wait for initialization
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        match client.list_all_tools().await {
            Ok(_tools) => {
                match client
                    .call_tool(CallToolRequestParam {
                        name: "ask_llm".into(),
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
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        client.cancel().await?;
        Ok(())
    }
}
