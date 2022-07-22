use std::fs;
use std::io::{Result, Read, BufReader};

use serde::{Serialize, Deserialize};
use serde_json;

mod core;
mod cli_display;

fn main() -> Result<()> {
    let example_frame = get_first_frame_from_file("../../patterns_examples.json")?;

    cli_display::render(example_frame, None);

    Ok(())
}


fn get_first_frame_from_file(file_name: &str) -> Result<Vec<Vec<u8>>> {
    let file = fs::File::open(file_name)?;
    let mut data_io = BufReader::new(file);

    let mut data = String::new();
    data_io.read_to_string(&mut data)?;
    let mut json_data: serde_json::Value = serde_json::from_str(&data)?;
    
    let example_frame: Vec<Vec<u8>> = json_data["Oscillators"]["Penta-decathlon"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .map(|vec| vec
            .as_array_mut()
            .unwrap()
            .iter()
            .map(|value| value.as_i64().unwrap() as u8)
            .collect())
        .collect();
    
    Ok(example_frame)
}