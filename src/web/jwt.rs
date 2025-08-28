/**
 * jsonwebtoken = "9.3.1"
 *
 * Rust 生态中一个非常流行且成熟的 JSON Web Token (JWT) 库。它允许你在 Rust 程序中方便地创建（编码/签名）、验证（解码/验证）和解析 JWT。
 *
 * 支持多种签名算法，最常用的是 HMAC (HS256, HS384, HS512) 和 RSA/ECDSA (RS256, RS384, RS512, ES256, ES384, ES512)。
 * 使用流程通常包括三个步骤：定义载荷 (Claims) -> 编码 (Encode) -> 解码 (Decode)。
 *
 * JWT 的核心是 Claims，即你想在 Token 中包含的信息。
 *
 * 定义载荷 (Claims)：
 * 载荷是 JWT 中包含的实际数据，通常包括用户 ID、角色、过期时间等信息。
 * 你可以使用 jsonwebtoken 库提供的 Claims 结构体来定义自己的载荷。
 *
 * 编码 (Encode)：
 * 编码是将载荷转换为 JWT 字符串的过程。
 * 你需要提供一个密钥（Secret）来对载荷进行签名，确保只有知道密钥的人才能解码和验证 JWT。
 *
 * 解码 (Decode)：
 * 解码是将 JWT 字符串转换为载荷的过程。
 * 你需要提供相同的密钥来验证 JWT 的签名，确保 JWT 没有被篡改。
 */

#[cfg(test)]
mod jwt_test {
    use std::time::{SystemTime, UNIX_EPOCH};

    use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        // 标准字段，必须是缩写，否则会报错：Error(InvalidToken)
        #[serde(rename = "iss")]
        issuer: String, // 签发人
        #[serde(rename = "sub")]
        subject: String, // 主题
        #[serde(rename = "exp")]
        expiration: usize, // 过期时间
        #[serde(rename = "iat")]
        issued_at: usize, // 签发时间

        // 自定义字段
        user_id: String, // 用户 ID
    }

    #[test]
    fn encode_test() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let claim = Claims {
            issuer: "issuer".to_string(),
            subject: "subject".to_string(),
            expiration: now + 60 * 60, // 一个小时后过期
            issued_at: now,
            user_id: "123123".to_string(),
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claim,
            &EncodingKey::from_secret("123123".as_bytes()),
        )
        .unwrap();
        println!("{}", token);
    }

    #[test]
    fn decode_test() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJpc3N1ZXIiLCJzdWIiOiJzdWJqZWN0IiwiZXhwIjoxNzU2MzUwMDAxLCJpYXQiOjE3NTYzNDY0MDEsInVzZXJfaWQiOiIxMjMxMjMifQ.tic9_hBDUFeOe4GIZeoyaIJ90bqRNrXi1son6s11QtU";

        // 创建验证对象
        let mut validate = Validation::new(Algorithm::HS256);
        // 自定义验证信息
        validate.set_issuer(&["issuer"]); // 验证发布者是不是 issuer，否则就会报错：Error(InvalidIssuer)

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret("123123".as_bytes()),
            &validate,
        );
        match token_data {
            Ok(data) => println!("{:?}", data.claims),
            Err(e) => println!("{:?}", e),
        }
    }
}
