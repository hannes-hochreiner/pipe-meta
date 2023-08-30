use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipeMetaError {
    #[error("pipe meta io error")]
    Disconnect(#[from] std::io::Error),
}

#[derive(Debug, serde::Serialize)]
pub struct MetaData {
    pub length: usize,
    pub sha256: String,
}

pub fn generate_metadata(
    mut input: impl Read,
    mut output: impl Write,
) -> Result<MetaData, PipeMetaError> {
    let mut buffer = [0u8; 1024];
    let mut content_length = 0usize;
    let mut hasher = Sha256::new();

    loop {
        let length = input.read(&mut buffer)?;

        match length {
            0 => break,
            _ => {
                content_length += length;
                hasher.update(&buffer[..length]);
                output.write(&buffer[..length])?;
            }
        }
    }

    let result = hasher.finalize();

    Ok(MetaData {
        length: content_length,
        sha256: hex::encode(result),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "Test1";
        let mut output = [0u8; 5];

        let result = generate_metadata(input.as_bytes(), &mut output[..]).unwrap();

        assert_eq!(result.length, 5);
        assert_eq!(output, [84, 101, 115, 116, 49]);
        assert_eq!(
            result.sha256,
            "8a863b145dc6e4ed7ac41c08f7536c476ebac7509e028ed2b49f8bd5a3562b9f"
        );
    }

    #[test]
    fn test_2() {
        let input = "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";
        let mut output = [0u8; 2000];

        let result = generate_metadata(input.as_bytes(), &mut output[..]).unwrap();

        assert_eq!(result.length, 2000);
        assert_eq!(output, input.as_bytes());
        assert_eq!(
            result.sha256,
            "37aceb0e214aa5af389c76b31e0a9b3d40c560c42a7523304236bb9b02561ccc"
        );
    }
}
