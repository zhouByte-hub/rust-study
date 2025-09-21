# Schemars 使用指南

## 简介

Schemars 是一个 Rust 库，用于从 Rust 数据结构生成 JSON Schema 文档。该库构建在 Rust 的 trait 系统之上 - 任何实现了 `JsonSchema` trait 的类型都可以生成描述该类型的 JSON Schema。Schemars 为许多标准库类型实现了这个 trait，并提供了一个 derive 宏来自动为自定义类型实现它。

该库的主要目标之一是与 Serde 兼容。任何生成的 schema 都应该与 `serde_json` 序列化/反序列化 JSON 的方式相匹配。为了支持这一点，Schemars 会检查派生 `JsonSchema` 的类型上的任何 `#[serde(...)]` 属性，并相应地调整生成的 schema。

## 安装

在 `Cargo.toml` 文件中添加 Schemars 依赖：

```toml
[dependencies]
schemars = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 基本用法

### 简单示例

如果你不太关心具体细节，为你的类型生成 JSON Schema 的最简单方法是使用 `#[derive(JsonSchema)]` 和 `schema_for!` 宏。该类型的所有字段也必须实现 `JsonSchema` - Schemars 为许多标准库类型实现了这一点。

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct MyStruct {
    pub my_int: i32,
    pub my_bool: bool,
    pub my_nullable_enum: Option<MyEnum>,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub enum MyEnum {
    StringNewType(String),
    StructVariant { floats: Vec<f32> },
}

fn main() {
    let schema = schema_for!(MyStruct);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
```

输出结果：

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "MyStruct",
  "type": "object",
  "properties": {
    "my_bool": {
      "type": "boolean"
    },
    "my_int": {
      "type": "integer",
      "format": "int32"
    },
    "my_nullable_enum": {
      "anyOf": [
        {
          "$ref": "#/$defs/MyEnum"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "my_int",
    "my_bool"
  ],
  "$defs": {
    "MyEnum": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "StringNewType": {
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "StringNewType"
          ]
        },
        {
          "type": "object",
          "properties": {
            "StructVariant": {
              "type": "object",
              "properties": {
                "floats": {
                  "type": "array",
                  "items": {
                    "type": "number",
                    "format": "float"
                  }
                }
              },
              "required": [
                "floats"
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "StructVariant"
          ]
        }
      ]
    }
  }
}
```

## 高级用法

### 自定义 Schema

你可以通过添加属性来自定义生成的 schema。Schemars 支持多种属性来控制 schema 的生成。

#### `#[schemars(...)]` 属性

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
#[schemars(title = "User", description = "A user account")]
pub struct User {
    #[schemars(description = "The user's unique identifier")]
    pub id: u64,
    
    #[schemars(length(min = 1, max = 100))]
    pub username: String,
    
    #[schemars(email)]
    pub email: String,
    
    #[schemars(url)]
    pub website: Option<String>,
    
    #[schemars(range(min = 18, max = 120))]
    pub age: u8,
}
```

#### 与 Serde 属性集成

Schemars 会自动考虑 Serde 属性，例如重命名字段、标记字段为可选等。

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "firstName")]
    pub first_name: String,
    
    #[serde(skip)]
    pub internal_id: String,
    
    #[serde(default)]
    pub last_login: Option<String>,
    
    #[serde(rename(serialize = "age", deserialize = "years"))]
    pub age: u8,
}
```

### 验证规则

Schemars 支持在 schema 中包含验证规则，这些规则可以用于验证 JSON 数据。

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct Product {
    #[schemars(length(min = 3, max = 100))]
    pub name: String,
    
    #[schemars(range(min = 0))]
    pub price: f64,
    
    #[schemars(regex = r"^[A-Z]{2}-[0-9]{4}$")]
    pub sku: String,
    
    #[schemars(contains = "example.com")]
    pub website: String,
}
```

### 枚举处理

Schemars 提供了多种方式来处理枚举类型的 schema 生成。

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

// 外部标记枚举
#[derive(Debug, JsonSchema, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExternalTagged {
    TypeA { a: i32 },
    TypeB { b: String },
}

// 内部标记枚举
#[derive(Debug, JsonSchema, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum InternalTagged {
    TypeA(i32),
    TypeB(String),
}

// 相邻标记枚举
#[derive(Debug, JsonSchema, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum AdjacentTagged {
    TypeA { a: i32 },
    TypeB { b: String },
}

// 无标记枚举
#[derive(Debug, JsonSchema, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Untagged {
    Int(i32),
    String(String),
    Array(Vec<i32>),
}
```

### 泛型支持

Schemars 支持泛型类型的 schema 生成。

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct Response<T> {
    pub success: bool,
    pub data: T,
    pub error: Option<String>,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}

fn main() {
    let schema = schema_for!(Response<User>);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
```

### 自定义 Schema 生成

你可以为自定义类型实现 `JsonSchema` trait，以完全控制 schema 的生成方式。

```rust
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDateTime(String);

impl JsonSchema for CustomDateTime {
    fn schema_name() -> String {
        "CustomDateTime".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("date-time".to_string()),
            ..Default::default()
        }
        .into()
    }
}
```

## 实际应用示例

### API 请求/响应验证

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct CreateUserRequest {
    #[schemars(length(min = 3, max = 50))]
    pub username: String,
    
    #[schemars(email)]
    pub email: String,
    
    #[schemars(length(min = 8))]
    pub password: String,
    
    #[schemars(range(min = 13, max = 120))]
    pub age: u8,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub age: u8,
    pub created_at: String,
}

fn main() {
    let request_schema = schema_for!(CreateUserRequest);
    let response_schema = schema_for!(UserResponse);
    
    println!("Request Schema:\n{}", serde_json::to_string_pretty(&request_schema).unwrap());
    println!("\nResponse Schema:\n{}", serde_json::to_string_pretty(&response_schema).unwrap());
}
```

### 配置文件 Schema

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct DatabaseConfig {
    #[schemars(url)]
    pub url: String,
    
    pub max_connections: u32,
    
    #[schemars(range(min = 1, max = 3600))]
    pub connection_timeout_seconds: u32,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct ServerConfig {
    #[schemars(range(min = 1, max = 65535))]
    pub port: u16,
    
    pub host: String,
    
    #[schemars(url)]
    pub base_url: String,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    
    #[schemars(regex = r"^(debug|info|warn|error)$")]
    pub log_level: String,
}

fn main() {
    let config_schema = schema_for!(AppConfig);
    println!("Config Schema:\n{}", serde_json::to_string_pretty(&config_schema).unwrap());
}
```

### 复杂嵌套结构

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct ContactInfo {
    #[schemars(email)]
    pub email: String,
    
    #[schemars(regex = r"^\+?[0-9]{10,15}$")]
    pub phone: String,
    
    #[schemars(url)]
    pub website: Option<String>,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct Employee {
    pub id: u64,
    pub name: String,
    
    #[schemars(email)]
    pub work_email: String,
    
    pub address: Address,
    pub contact: ContactInfo,
    
    #[schemars(range(min = 0))]
    pub salary: f64,
    
    #[schemars(regex = r"^(full-time|part-time|contract)$")]
    pub employment_type: String,
    
    pub skills: Vec<String>,
    
    #[schemars(contains = "company.com")]
    pub company_email: Option<String>,
}

fn main() {
    let schema = schema_for!(Employee);
    println!("Employee Schema:\n{}", serde_json::to_string_pretty(&schema).unwrap());
}
```

## 高级特性

### 条件 Schema

你可以使用 `#[schemars(...)]` 属性来创建条件 schema。

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct ConditionalExample {
    #[schemars(required_if(r#""role" = "admin""#))]
    pub admin_key: Option<String>,
    
    #[schemars(required_if(r#""status" = "active""#))]
    pub activation_date: Option<String>,
}
```

### 自定义 Schema 转换

你可以使用 `#[schemars(with = "...")]` 属性来指定自定义的 schema 转换函数。

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

fn custom_string_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = <String as JsonSchema>::json_schema(gen);
    if let schemars::schema::Schema::Object(ref mut obj) = schema {
        obj.metadata().description = Some("A custom string schema".to_string());
        obj.metadata().title = Some("CustomString".to_string());
    }
    schema
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct CustomSchemaExample {
    #[schemars(with = "custom_string_schema")]
    pub custom_field: String,
}
```

### Schema 扩展

你可以使用 `#[schemars(...)]` 属性来扩展 schema。

```rust
use schemars::{schema_for, JsonSchema};
use serde::{Serialize, Deserialize};

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
#[schemars(
    title = "Extended Example",
    description = "An example with extended schema properties",
    example = r#"{"name": "Example", "value": 42}"#
)]
pub struct ExtendedExample {
    #[schemars(
        title = "Name Field",
        description = "The name of the example",
        example = "\"Example\""
    )]
    pub name: String,
    
    #[schemars(
        title = "Value Field",
        description = "The value of the example",
        example = "42",
        default = "0"
    )]
    pub value: i32,
}
```

## 最佳实践

1. **保持一致性**：确保你的 Rust 类型定义与 JSON Schema 保持一致，特别是在使用 Serde 属性时。

2. **添加描述**：为你的类型和字段添加描述，这有助于生成更易读的文档。

3. **使用验证规则**：利用 Schemars 提供的验证规则来确保数据的有效性。

4. **测试你的 Schema**：生成 schema 后，使用 JSON Schema 验证器来测试它是否能正确验证你的数据。

5. **考虑性能**：对于大型或复杂的类型，考虑 schema 生成的性能影响。

6. **版本控制**：将生成的 schema 纳入版本控制，以便跟踪 API 或配置的变化。

## 总结

Schemars 是一个强大的库，可以轻松地从 Rust 数据结构生成 JSON Schema。它与 Serde 的紧密集成使得它成为 Rust 生态系统中处理 JSON 数据的理想选择。通过使用 Schemars，你可以：

- 为你的 API 自动生成文档
- 验证配置文件
- 创建动态表单
- 实现数据验证

希望这个指南能帮助你开始使用 Schemars。更多信息和高级用法，请参考 [官方文档](https://graham.cool/schemars/)。