/**
 * sha2 = "0.10.9"
 *
 * SHA-2 哈希函数家族的纯 Rust 实现，包括 SHA-224、SHA-256、SHA-384 和 SHA-512。
 */

#[cfg(test)]
mod sha_test {
    use sha2::{Digest, Sha256};

    #[test]
    fn test_1() {
        let mut sha256 = Sha256::new();
        sha256.update(b"hello world");
        println!("{:?}", sha256.finalize());

        let sha = Sha256::new()
            .chain_update(b"hello")
            .chain_update(b"world")
            .finalize();
        println!("{:?}", sha);
    }

    #[test]
    fn test_2() {
        let mut file = std::fs::File::open("src/other/mod.rs").unwrap();
        let mut sha = Sha256::new();

        let length = std::io::copy(&mut file, &mut sha).unwrap();

        println!("length: {}", length);
        println!("{:?}", sha.finalize());
    }
}
