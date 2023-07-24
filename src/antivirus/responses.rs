use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize)]
pub struct AnalysisIdResponse {
    pub data: AnalysisIdData,
}
#[derive(Debug, Default, Serialize)]
pub struct AnalysisIdData {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
}

impl AnalysisIdData {
    pub fn new(t: Option<String>, id: Option<String>) -> Self {
        AnalysisIdData {
            _type: t.unwrap_or_default(),
            id: id.unwrap_or_default(),
        }
    }
}
impl AnalysisIdResponse {
    pub fn new(data: AnalysisIdData) -> Self {
        AnalysisIdResponse { data }
    }
}

impl<'de> serde::Deserialize<'de> for AnalysisIdResponse {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let data = AnalysisIdData::deserialize(d)?;
        Ok(AnalysisIdResponse::new(data))
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisIdData {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let (_type, id) = Deserialize::deserialize(d)?;
        Ok(AnalysisIdData::new(_type, id))
    }
}

#[derive(Debug, Default, Serialize)]
pub struct AnalysisReportResponse {
    pub data: AnalysisReportData,
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
}
impl AnalysisReportResponse {
    pub fn new(
        data: Option<AnalysisReportData>,
        _type: Option<String>,
        id: Option<String>,
    ) -> Self {
        AnalysisReportResponse {
            data: data.unwrap_or_default(),
            _type: _type.unwrap_or_default(),
            id: id.unwrap_or_default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisReportResponse {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let (data, _type, id) = Deserialize::deserialize(d)?;

        Ok(AnalysisReportResponse::new(
            Some(data),
            Some(_type),
            Some(id),
        ))
    }
}

#[derive(Debug, Default, Serialize)]
pub struct AnalysisReportData {
    pub attributes: AnalysisReportAttribute,
}
impl AnalysisReportData {
    pub fn new(attributes: Option<AnalysisReportAttribute>) -> Self {
        AnalysisReportData {
            attributes: attributes.unwrap_or_default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisReportData {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let attributes = Deserialize::deserialize(d)?;

        Ok(AnalysisReportData::new(Some(attributes)))
    }
}

#[derive(Debug, Default, Serialize)]
pub struct AnalysisReportAttribute {
    pub date: i64, //timestamp
    pub results: HashMap<String, Analysis>,
    pub stats: ReportStats,
    pub status: String,
}
impl AnalysisReportAttribute {
    pub fn new(
        date: Option<i64>,
        results: Option<HashMap<String, Analysis>>,
        stats: Option<ReportStats>,
        status: Option<String>,
    ) -> Self {
        AnalysisReportAttribute {
            date: date.unwrap_or_default(),
            status: status.unwrap_or_default(),
            results: results.unwrap_or_default(),
            stats: stats.unwrap_or_default(),
        }
    }
}

impl<'de> serde::Deserialize<'de> for AnalysisReportAttribute {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let (date, results, stats, status) = Deserialize::deserialize(d)?;

        Ok(AnalysisReportAttribute::new(
            Some(date),
            Some(results),
            Some(stats),
            Some(status),
        ))
    }
}

#[derive(Debug, Default, Serialize)]
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
impl ReportStats {
    pub fn new(
        confirmed_timeout: Option<u64>,
        failure: Option<u64>,
        harmless: Option<u64>,
        malicious: Option<u64>,
        suspicious: Option<u64>,
        timeout: Option<u64>,
        type_unsupported: Option<u64>,
        undetected: Option<u64>,
    ) -> Self {
        ReportStats {
            confirmed_timeout: confirmed_timeout.unwrap_or_default(),
            failure: failure.unwrap_or_default(),
            harmless: harmless.unwrap_or_default(),
            malicious: malicious.unwrap_or_default(),
            suspicious: suspicious.unwrap_or_default(),
            timeout: timeout.unwrap_or_default(),
            type_unsupported: type_unsupported.unwrap_or_default(),
            undetected: undetected.unwrap_or_default(),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ReportStats {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let (
            confirmed_timeout,
            failure,
            harmless,
            malicious,
            suspicious,
            timeout,
            type_unsupported,
            undetected,
        ) = Deserialize::deserialize(d)?;

        Ok(ReportStats::new(
            Some(confirmed_timeout),
            Some(failure),
            Some(harmless),
            Some(malicious),
            Some(suspicious),
            Some(timeout),
            Some(type_unsupported),
            Some(undetected),
        ))
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Analysis {
    pub category: String,
    pub engine_name: String,
    pub engine_update: String,
    pub engine_version: String,
    pub method: String,
    pub result: Option<String>,
}

impl Analysis {
    pub fn new(
        category: Option<String>,
        engine_name: Option<String>,
        engine_update: Option<String>,
        engine_version: Option<String>,
        method: Option<String>,
        result: Option<Option<String>>,
    ) -> Self {
        Analysis {
            category: category.unwrap_or_default(),
            engine_name: engine_name.unwrap_or_default(),
            engine_update: engine_update.unwrap_or_default(),
            engine_version: engine_version.unwrap_or_default(),
            method: method.unwrap_or_default(),
            result: result.unwrap_or_default(),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Analysis {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let (category, engine_name, engine_update, engine_version, method, result) =
            Deserialize::deserialize(d)?;

        Ok(Analysis::new(
            Some(category),
            Some(engine_name),
            Some(engine_update),
            Some(engine_version),
            Some(method),
            Some(result),
        ))
    }
}
