use rmcp::{
    RoleServer, ServerHandler, ServiceExt, handler::server::router::prompt::PromptRouter, model::*,
    prompt_handler, prompt_router, schemars::JsonSchema, service::RequestContext, transport::stdio,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Code review parameters")]
pub struct CodeReviewArgs {
    #[schemars(description = "Programming language of the code")]
    pub language: String,
    #[schemars(description = "Path to the file or code snippet")]
    pub file_path: String,
    #[schemars(description = "Focus areas for the review")]
    pub focus_areas: Option<Vec<String>>,
}

/// Arguments for the data analysis prompt
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Data analysis parameters")]
pub struct DataAnalysisArgs {
    #[schemars(description = "Type of data: 'csv', 'json', 'logs', etc.")]
    pub data_type: String,
    #[schemars(description = "What kind of analysis to perform")]
    pub analysis_type: String,
    #[schemars(description = "Additional context about the data")]
    pub context: Option<String>,
}

/// Arguments for the writing assistant prompt
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Writing assistant parameters")]
pub struct WritingAssistantArgs {
    #[schemars(description = "Type of content: 'email', 'documentation', 'blog', etc.")]
    pub content_type: String,
    #[schemars(description = "Target audience")]
    pub audience: String,
    #[schemars(description = "Writing tone: 'formal', 'casual', 'technical', etc.")]
    pub tone: Option<String>,
    #[schemars(description = "Key points to cover")]
    pub key_points: Vec<String>,
}

/// Arguments for the debug assistant prompt
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Debug assistant parameters")]
pub struct DebugAssistantArgs {
    #[schemars(description = "Error message or symptom")]
    pub error_message: String,
    #[schemars(description = "Technology stack involved")]
    pub stack: Vec<String>,
    #[schemars(description = "Steps already tried")]
    pub tried_solutions: Option<Vec<String>>,
}

/// Simple prompt server demonstrating various prompt patterns
#[derive(Clone)]
pub struct PromptServer {
    /// Stores user preferences that can be used in prompts
    user_preferences: Arc<RwLock<UserPreferences>>,
    prompt_router: PromptRouter<PromptServer>,
}

#[derive(Debug, Clone)]
struct UserPreferences {
    preferred_language: String,
    expertise_level: String,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            preferred_language: "English".to_string(),
            expertise_level: "intermediate".to_string(),
        }
    }
}

impl PromptServer {
    pub fn new() -> Self {
        Self {
            user_preferences: Arc::new(RwLock::new(UserPreferences::default())),
            prompt_router: Self::prompt_router(),
        }
    }
}

impl Default for PromptServer {
    fn default() -> Self {
        Self::new()
    }
}

#[prompt_router]
impl PromptServer {}

#[prompt_handler]
impl ServerHandler for PromptServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_prompts().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provides various prompt templates for code review, data analysis, \
                 writing assistance, debugging help, and personalized learning paths. \
                 All prompts are designed to provide structured, context-aware assistance."
                    .to_string(),
            ),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MCP Prompt Server Example");
    println!("=======================");
    println!();
    println!("This server demonstrates various prompt patterns:");
    println!("- Simple prompts without parameters");
    println!("- Prompts with typed parameters");
    println!("- Prompts using server state");
    println!("- Multi-turn conversation prompts");
    println!("- Error handling in prompts");
    println!();
    println!("To test with MCP Inspector:");
    println!(
        "npx @modelcontextprotocol/inspector cargo run -p mcp-server-examples --example servers_prompt_stdio"
    );
    println!();

    let server = PromptServer::new();
    let service = server.serve(stdio()).await?;

    service.waiting().await?;
    Ok(())
}
