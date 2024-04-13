use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn read_data_bin(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    let mut binary_string = String::new();
    for byte in buffer {
        binary_string.push_str(&format!("{:08b}", byte));
    }
    binary_string
}

pub fn write_data_bin(binding: String, outfilename: &str) {
    let binary_string = binding.as_str();
    let chunks: Vec<&str> = binary_string.as_bytes().chunks(8).map(|chunk| std::str::from_utf8(chunk).unwrap()).collect();
    let mut file = File::create(outfilename).expect("Failed to create file");
    for chunk in chunks {
        let byte = u8::from_str_radix(chunk, 2).expect("Failed to convert binary string to byte");
        file.write_all(&[byte]).expect("Failed to write to file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let file_path = "Cargo.toml";
        let binding = read_data_bin(file_path);
        write_data_bin(binding, "Output");
    }
}
