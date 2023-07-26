use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisIdResponse {
    pub data: AnalysisIdData,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisIdData {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub links: AnalysisLink,
    pub attributes: Option<AnalysisAttributes>,
    pub relationships: Option<AnalysisRelationships>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisLink {
    #[serde(rename = "self")]
    pub _self: String,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisAttributes {
    #[serde(rename = "self")]
    pub integer_attribute: u64,
    pub string_attribute: String,
    pub dictionary_attribute: HashMap<String, u64>,
    pub list_attribute: Vec<String>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisRelationships {
    pub x: String, // not a string but it's will never appear in response
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisReportResponse {
    #[serde(rename = "type")]
    pub _type: Option<String>,

    pub data: AnalysisReportData,
    pub id: Option<String>,
    pub links: Option<AnalysisReportLinks>,
    pub meta: Option<AnalysisReportMeta>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisReportLinks {
    #[serde(rename = "self")]
    pub selff: String,
    pub item: String,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisReportMeta {
    pub file_info: MetaFileInfo,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MetaFileInfo {
    pub size: Option<u64>,
    pub sha256: Option<String>,
    pub sha1: Option<String>,
    pub md5: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisReportData {
    pub attributes: AnalysisReportAttribute,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisReportAttribute {
    pub date: i64, //timestamp
    pub results: HashMap<String, Analysis>,
    pub stats: ReportStats,
    pub status: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportStats {
    #[serde(rename = "confirmed-timeout")]
    pub confirmed_timeout: u64,
    pub failure: u64,
    pub harmless: u64,
    pub malicious: u64,
    pub suspicious: u64,
    pub timeout: u64,
    #[serde(rename = "type-unsupported")]
    pub type_unsupported: u64,
    pub undetected: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Analysis {
    pub category: Option<String>,
    pub engine_name: Option<String>,
    pub engine_update: Option<String>,
    pub engine_version: Option<String>,
    pub method: Option<String>,
    pub result: Option<String>,
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorContent
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ErrorContent {
    pub message: String,
    pub code: String
}