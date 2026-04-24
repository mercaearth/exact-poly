use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WasmDecomposeResult {
    pub parts: Vec<Vec<i64>>,
    pub steiner_points: Vec<i64>,
    pub strategy: crate::types::Strategy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Vec<crate::types::Attempt>>,
}

#[derive(Serialize, Deserialize)]
pub struct WasmIndexPair {
    pub a_index: usize,
    pub b_index: usize,
}
