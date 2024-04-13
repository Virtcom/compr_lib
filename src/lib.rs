use std::fs::File;
use std::io::{Read, Write, Result};

pub fn read_data_bin(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let binary_string: String = buffer.iter().map(|byte| format!("{:08b}", byte)).collect();
    Ok(binary_string)
}

pub fn write_data_bin(binding: String, outfilename: &str) -> Result<()> {
    let binary_string = binding.as_str();
    let chunks: Vec<&str> = binary_string.as_bytes().chunks(8).map(|chunk| std::str::from_utf8(chunk).unwrap()).collect();
    let mut file = File::create(outfilename)?;
    for chunk in chunks {
        let byte = u8::from_str_radix(chunk, 2).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to convert binary string to byte"))?;
        file.write_all(&[byte])?;
    }
    Ok(())
}

pub fn write_data(input: String, outfilename: &str) -> Result<()> {
    let mut file = File::create(outfilename)?;
    file.write_all(input.as_bytes())?;
    Ok(())
}

pub fn read_data(file_path: &str) -> Result<String> {
    let mut f = File::open(file_path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let contents = String::from_utf8(buffer).expect("Found invalid UTF-8");
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let file_path = "Cargo.toml";
        let binding = read_data_bin(file_path).expect("Failed to read binary data");
        write_data(binding, "temp").expect("Failed to write binary data");
        let binding = read_data("temp").expect("Failed to read binary data from temp file");
        write_data_bin(binding, "Output").expect("Failed to write binary data to Output");
    }
}
