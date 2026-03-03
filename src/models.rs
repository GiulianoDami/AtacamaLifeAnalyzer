use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilSample {
    pub location_id: String,
    pub elevation: f64,
    pub moisture_content: f64,
    pub temperature: f64,
    pub ph_level: f64,
    pub organic_matter: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub sample_id: String,
    pub predicted_nematode_diversity: f64,
    pub survival_probability: f64,
    pub reproduction_strategy: ReproductionStrategy,
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReproductionStrategy {
    Asexual,
    Sexual,
    Mixed,
}