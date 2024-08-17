use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub fn load_json_file<T>(filename: &str) -> Result<T, Box<dyn Error>> 
where
    T:DeserializeOwned
{
    // Open the file
    let mut file = File::open(filename)?;
    
    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let json = load_json(&contents)?;
    Ok(json)
    
    // Parse the JSON string into the struct
    // let json: T = serde_json::from_str(&contents)?;
    // Ok(json)
}

pub fn load_json<T>(content: &str) -> Result<T, Box<dyn Error>> 
where
    T:DeserializeOwned
{
    let json: T = serde_json::from_str(content)?;
    Ok(json)
}