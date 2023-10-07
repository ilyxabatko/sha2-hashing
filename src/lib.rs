use anyhow::Result;
use sha2::{Sha256, Digest};
use std::{fs::File, io::Read};

pub fn hash_file(file_name: &str) -> Result<[u8; 32]> {
    // creates a new hasher instance
    let mut hasher = Sha256::new();

    let mut file = File::open(file_name)?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)?;

    // updates the internal state and returns a GenericArray (hash)
    hasher.update(&file_data);
    let hash = hasher.finalize();

    // get a common Rust array that we want to work with
    let mut return_array = <[u8; 32]>::default();
    return_array.copy_from_slice(&hash);

    Ok(return_array)
}

pub fn hex_to_string(data: &[u8]) -> String {
    let mut result = String::new();

    for d in data {
        let x = format!("{:02x}", d);
        result.push_str(&x);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_sum() {
        let hash = hash_file("test_data/test1.bin").expect("Can't hash the file data!");
        let hash_string = hex_to_string(&hash);

        let expected_message_digest = "3c593aa539fdcdae516cdf2f15000f6634185c88f505b39775fb9ab137a10aa2";
        assert_eq!(&hash_string, expected_message_digest);
    }
}
