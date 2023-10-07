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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
