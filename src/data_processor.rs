use csv;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SoilSample {
    pub sample_id: String,
    pub moisture_content: f64,
    pub elevation: f64,
    pub temperature: f64,
    pub nematode_count: u32,
}

pub fn process_csv(data: &str) -> Result<Vec<SoilSample>, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let mut samples = Vec::new();
    
    for result in reader.deserialize() {
        let sample: SoilSample = result?;
        samples.push(sample);
    }
    
    Ok(samples)
}