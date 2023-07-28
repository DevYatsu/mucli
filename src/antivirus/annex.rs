use std::io::{Cursor, Read};
use std::{io::Error, path::PathBuf};

use super::responses::{AnalysisReportData, ErrorResponse};
use crate::antivirus::responses::{AnalysisIdResponse, AnalysisReportResponse};
use crate::file_as_bytes;
use base64_stream::ToBase64Reader;
use custom_error::custom_error;
use dotenv_codegen::dotenv;
use reqwest::{self, multipart};
use serde_json::Error as SerdeError;

custom_error! {pub AntivirusError
    Io{source: Error} = "{source}",
    ReqWest{source: reqwest::Error } = "{source}",
    Serde{source: SerdeError } = "{source}",
    InvalidApiResponse = "API response is invalid",
    ErrorApiResponse{message: String} = "{message}",
    ApiReponseAnalyseFailed = "Failed to analyse API Response",
    CannotProcessEmptyFile = "Cannot process empty file",
    ApiCallQueued = "API cannot process file yet. Try again later!"
}

pub async fn is_dangerous(file_path: &PathBuf) -> Result<bool, AntivirusError> {
    println!("Analysing file");
    println!("It can take some time...");
    let id = get_analysis_id(file_path).await?;

    let report = get_analysis_report(&id).await?;

    let (malicious_number, suspicious_number) = reports_key_data(report);

    let malicious_display = if malicious_number > 0 {
        format!("\x1B[38;5;88m{}\x1B[0m", malicious_number)
    } else {
        format!("\x1B[32m{}\x1B[0m", malicious_number)
    };
    let suspicious_display = if suspicious_number > 0 {
        format!("\x1B[38;5;88m{}\x1B[0m", suspicious_number)
    } else {
        format!("\x1B[32m{}\x1B[0m", suspicious_number)
    };

    println!("malicious threat detected: {}", malicious_display);
    println!("suspicious threat detected: {}", suspicious_display);

    Ok(malicious_number + suspicious_number > 0)
}

async fn get_analysis_id(file_path: &PathBuf) -> Result<String, AntivirusError> {
    const API_KEY: &str = dotenv!("VIRUSTOTAL_API_KEY");
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
        let error_reponse: ErrorResponse = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(e.into()),
        };

        return Err(AntivirusError::ErrorApiResponse {
            message: error_reponse.error.message,
        });
    }
}

async fn get_analysis_report(id: &str) -> Result<AnalysisReportData, AntivirusError> {
    const API_KEY: &str = dotenv!("VIRUSTOTAL_API_KEY");
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
        let api_response: AnalysisReportResponse = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(e.into()),
        };
        if api_response.data.attributes.status != "completed" {
            return Err(AntivirusError::ApiCallQueued);
        }
        let response_data = api_response.data;

        Ok(response_data)
    } else {
        let error_reponse: ErrorResponse = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(e.into()),
        };

        return Err(AntivirusError::ErrorApiResponse {
            message: error_reponse.error.message,
        });
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

    if file_content.len() == 0 {
        return Err(AntivirusError::CannotProcessEmptyFile);
    }

    let mut reader = ToBase64Reader::new(Cursor::new(file_content));

    let mut base64 = String::new();

    reader.read_to_string(&mut base64).unwrap();

    Ok(base64)
}
