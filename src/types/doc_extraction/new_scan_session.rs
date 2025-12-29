use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct NewScanSession {
    pub session_id:i64
}