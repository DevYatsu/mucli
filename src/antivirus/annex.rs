use std::io::{Cursor, Read};
use std::{io::Error, path::PathBuf};

use base64_stream::ToBase64Reader;

use crate::antivirus::responses::{AnalysisIdResponse, AnalysisReportResponse};
use crate::file_as_bytes;
use crate::utils::terminal::arrow_progress;
use custom_error::custom_error;
use reqwest::{self, multipart};
use serde_json::Error as SerdeError;
const API_KEY: &str = "000";
use super::responses::AnalysisReportData;

custom_error! {pub AntivirusError
    Io{source: Error} = "{source}",
    ReqWest{source: reqwest::Error } = "{source}",
    Serde{source: SerdeError } = "{source}",
    InvalidApiResponse = "API response is invalid",
    ErrorRetrievingVersion = "Cannot find release version",
}

pub async fn is_dangerous(file_path: &PathBuf) -> Result<bool, AntivirusError> {
    let pb = arrow_progress(3);

    let id = get_analysis_id(file_path).await?;

    pb.inc(1);

    let report = get_analysis_report(&id).await?;

    pb.inc(2);

    let (malicious_number, suspicious_number) = reports_key_data(report);

    pb.finish_and_clear();

    let malicious_display = if malicious_number > 0 {
        format!("\x1B[41m{}\x1B[0m", malicious_number)
    } else {
        format!("\x1B[32m{}\x1B[0m", malicious_number)
    };
    let suspicious_display = if suspicious_number > 0 {
        format!("\x1B[41m{}\x1B[0m", suspicious_number)
    } else {
        format!("\x1B[32m{}\x1B[0m", suspicious_number)
    };

    println!("malicious threat detected: {}", malicious_display);
    println!("suspicious threat detected: {}", suspicious_display);

    Ok(malicious_number + suspicious_number > 0)
}

async fn get_analysis_id(file_path: &PathBuf) -> Result<String, AntivirusError> {
    const URL: &str = "https://www.virustotal.com/api/v3/files";
    let client = reqwest::Client::new();
    let body = file_to_base64(file_path)?;

    let form = multipart::Form::new().part(
        "file",
        multipart::Part::stream(body.clone())
            .file_name(file_path.file_name().unwrap().to_string_lossy().to_string())
            .mime_str("application/octet-stream")?,
    );

    let response = client
        .post(URL)
        .header("accept", "application/json")
        .header("x-apikey", API_KEY)
        .multipart(form)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if status.is_success() {
        let api_response: AnalysisIdResponse = serde_json::from_str(&text)?;
        let response_data = api_response.data;
        return Ok(response_data.id);
    } else {
        Err(AntivirusError::InvalidApiResponse)
    }
}

async fn get_analysis_report(id: &str) -> Result<AnalysisReportData, AntivirusError> {
    let url = String::from("https://www.virustotal.com/api/v3/analyses/") + id;
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("accept", "application/json")
        .header("x-apikey", API_KEY)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if status.is_success() {
        let api_response: AnalysisReportResponse = serde_json::from_str(&text).unwrap();
        let response_data = api_response.data;

        Ok(response_data)
    } else {
        return Err(AntivirusError::InvalidApiResponse);
    }
}

fn reports_key_data(report: AnalysisReportData) -> (u64, u64) {
    let results = report.attributes.stats;

    let malicious_number = results.malicious;
    let suspicious_number = results.suspicious;

    (malicious_number, suspicious_number)
}

fn file_to_base64(file_path: &PathBuf) -> Result<String, AntivirusError> {
    let (_, file_content) = file_as_bytes!(file_path);

    let mut reader = ToBase64Reader::new(Cursor::new(file_content));

    let mut base64 = String::new();

    reader.read_to_string(&mut base64).unwrap();

    Ok(base64)
}
