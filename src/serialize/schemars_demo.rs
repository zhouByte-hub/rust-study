/**
 * schemars = "1.0"
 *
 * Schemars主要用于从Rust数据结构生成JsonSchema文档
 * 任何实现了JsonSchema的类型都可以生成描述该类型的JsonSchema。
 *
 * 在使用Schemars库的基本要求是与Serde库集成。
 */

#[cfg(test)]
mod test {
    use schemars::{JsonSchema, schema_for};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, JsonSchema, Serialize, Deserialize)]
    struct User {
        username: String,
        address: String,
        age: u8,
    }

    #[test]
    fn test_1() {
        let schema = schema_for!(User);
        let schema_str = serde_json::to_string_pretty(&schema).unwrap();
        /* 生成的JsonSchema文档如下：
           {
               "$schema": "https://json-schema.org/draft/2020-12/schema",
               "title": "User",
               "type": "object",
               "properties": {
                   "address": {
                   "type": "string"
                   },
                   "age": {
                   "type": "integer",
                   "format": "uint8",
                   "maximum": 255,
                   "minimum": 0
                   },
                   "username": {
                   "type": "string"
                   }
               },
               "required": [
                   "username",
                   "address",
                   "age"
               ]
           }
        */
        println!("{}", schema_str);
    }

    /**
     * 自定义Schema
     */
    #[derive(Debug, JsonSchema, Serialize, Deserialize)]
    #[schemars(title = "User_double", description = "用户信息")]
    #[allow(dead_code)]
    struct User2 {
        #[serde(skip)]
        id: u64,

        #[schemars(description = "用户名", length(min = 3, max = 20))]
        username: String,

        #[schemars(description = "用户邮箱")]
        email: String,

        #[schemars(description = "用户网站")]
        websit: Option<String>,

        #[schemars(description = "用户年龄", range(min = 0, max = 150))]
        age: u8,

        #[schemars(description = "用户状态")]
        status: StatusEnum,
    }

    #[derive(Debug, JsonSchema, Serialize, Deserialize)]
    #[serde(tag = "status")]
    enum StatusEnum {
        ONLINE,
        OFFLINE,
    }

    #[test]
    fn test_2() {
        let schema = schema_for!(User2);
        let schema_str = serde_json::to_string_pretty(&schema).unwrap();
        /*
           {
               "$schema": "https://json-schema.org/draft/2020-12/schema",
               "title": "User_double",
               "description": "用户信息",
               "type": "object",
               "properties": {
                   "age": {
                   "description": "用户年龄",
                   "type": "integer",
                   "format": "uint8",
                   "maximum": 150,
                   "minimum": 0
                   },
                   "email": {
                   "description": "用户邮箱",
                   "type": "string"
                   },
                   "status": {
                   "description": "用户状态",
                   "$ref": "#/$defs/StatusEnum"
                   },
                   "username": {
                   "description": "用户名",
                   "type": "string",
                   "maxLength": 20,
                   "minLength": 3
                   },
                   "websit": {
                   "description": "用户网站",
                   "type": [
                       "string",
                       "null"
                   ]
                   }
               },
               "required": [
                   "username",
                   "email",
                   "age",
                   "status"
               ],
               "$defs": {
                   "StatusEnum": {
                   "oneOf": [
                       {
                       "type": "object",
                       "properties": {
                           "status": {
                           "type": "string",
                           "const": "ONLINE"
                           }
                       },
                       "required": [
                           "status"
                       ]
                       },
                       {
                       "type": "object",
                       "properties": {
                           "status": {
                           "type": "string",
                           "const": "OFFLINE"
                           }
                       },
                       "required": [
                           "status"
                       ]
                       }
                   ]
                   }
               }
               }
        */
        println!("{}", schema_str);
    }
}
