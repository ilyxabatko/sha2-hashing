use lib::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough args!");
    }

    if let Ok(hash_array) = hash_file(&args[1]) {
        print!("Hash: {}", hex_to_string(&hash_array));
    }

    // adding the file path print to the output as if we'd 
    // call the "sha256sum [filepath]" CLI command
    println!("  {}", &args[1]);

}
