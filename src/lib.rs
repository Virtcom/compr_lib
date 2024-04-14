use std::fs::File;
use std::io::{Read, Write, Result};
use std::process::Output;

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
    let contents = String::from_utf8(buffer).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(contents)
}

pub fn packstream(filestream: String, packperbite: u16) -> String {
    let mut oldchunk = &filestream[0..packperbite.into()];
    let mut counter = 1;
    let size : usize = packperbite.into();
    let mut start: usize = packperbite.into();
    let mut end: usize = (packperbite*2).into();
    let mut outstring = String::new();
    while end <= filestream.len() {
        let chunk = &filestream[start..end];
        if chunk == oldchunk {
            counter += 1;
        }
        else {
            outstring += chunk;
            outstring += "x";
            outstring += &counter.to_string();
            outstring += "\n";
            counter = 1;
        }
        oldchunk = chunk;
        start = end;
        end += size;
    }
    if start < filestream.len() {
        outstring += &filestream[start..];
        outstring += "x1\n"; 
    }
    outstring
}

pub fn unpackstream(filestream: String) -> String {
    let mut oustring = String::new();
    let chunks: Vec<&str> = filestream.split('\n').collect();
    for chunk in chunks {
        let parts: Vec<&str> = chunk.split('x').collect();
        if parts.len() == 2 {
            if let Ok(num) = parts[1].parse::<i32>() {
                let multiplied = parts[0].repeat(num.try_into().unwrap());
                oustring += &multiplied.to_string();
            }
        println!("{}", parts[0]);
        }
    }
    oustring
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let file_path = "Cargo.toml";
        let binding = read_data_bin(file_path).expect("Failed to read binary data");
        write_data(packstream(binding, 8*10), "temp.sa").expect("Failed to write binary data");
        let binding = read_data("temp.sa").expect("Failed to read binary data from temp file");
        write_data_bin(unpackstream(binding), "Output").expect("Failed to write binary data to Output");
    }
}
