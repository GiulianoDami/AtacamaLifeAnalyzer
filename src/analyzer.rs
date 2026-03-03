use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct SoilSample {
    pub moisture: f64,
    pub elevation: f64,
    pub temperature: f64,
    pub nematode_count: u32,
}

#[derive(Debug, Serialize)]
pub struct AnalysisResult {
    pub average_nematode_density: f64,
    pub moisture_optimum: f64,
    pub elevation_impact: f64,
    pub survival_strategy: String,
}

fn analyze_data(input_file: &str, output_file: &str) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(input_file)?;
    let mut samples: Vec<SoilSample> = Vec::new();

    for result in reader.deserialize() {
        let sample: SoilSample = result?;
        samples.push(sample);
    }

    if samples.is_empty() {
        return Err("No data found in input file".into());
    }

    let total_nematodes: u64 = samples.iter().map(|s| s.nematode_count as u64).sum();
    let total_samples = samples.len() as f64;
    let average_nematode_density = total_nematodes as f64 / total_samples;

    let moisture_optimum = samples
        .iter()
        .max_by(|a, b| a.moisture.partial_cmp(&b.moisture).unwrap())
        .map(|s| s.moisture)
        .unwrap_or(0.0);

    let elevation_impact = samples
        .iter()
        .map(|s| s.elevation)
        .sum::<f64>()
        / total_samples;

    let survival_strategy = if moisture_optimum > 0.5 {
        "Sexual reproduction".to_string()
    } else {
        "Asexual reproduction".to_string()
    };

    let result = AnalysisResult {
        average_nematode_density,
        moisture_optimum,
        elevation_impact,
        survival_strategy,
    };

    let json_result = serde_json::to_string_pretty(&result)?;
    std::fs::write(output_file, json_result)?;

    Ok(result)
}