use super::types::{Episode, SubjectBase, SubjectMedium};
use anyhow::{Context, Result};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use log::{debug, trace};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{de::DeserializeOwned, Deserialize};
use std::fmt;
use std::time::SystemTime;

pub async fn search_anime(keyword: &str) -> Result<SearchResponse> {
    let encoded_keyword = utf8_percent_encode(&keyword, NON_ALPHANUMERIC);
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let path = format!(
        "/search/subject/{}?type=2&chii_searchDateLine={}",
        encoded_keyword, ts,
    );
    trace!("request url {}", path);
    let res: SearchResponse = request(&path)
        .await
        .with_context(|| "request search anime")?;
    debug!("obj: {:?}", &res);
    Ok(res)
}

pub struct BgmAnime {
    pub subject: SubjectMedium,
    pub episodes: Vec<Episode>,
}

pub async fn get_anime_data(id: u32) -> Result<BgmAnime> {
    let subject = get_subject_info(id).await?;
    let episodes = get_subject_episodes(id).await?.eps;
    Ok(BgmAnime { subject, episodes })
}

pub async fn get_subject_info(id: u32) -> Result<SubjectMedium> {
    let path = format!("/subject/{}?responseGroup=medium", id);
    let subject: SubjectMedium = request(&path)
        .await
        .with_context(|| format!("request get subject: {}", id))?;
    debug!("subject: {:#?}", &subject);
    Ok(subject)
}

pub async fn get_subject_episodes(id: u32) -> Result<EpisodeResponse> {
    trace!("get_subject_info: {}", id);
    let path = format!("/subject/{}/ep", id);
    let res: EpisodeResponse = request(&path)
        .await
        .with_context(|| format!("get subject episode {}", id))?;
    for ep in &res.eps {
        debug!("subject ep: {:#?}", &ep);
    }
    Ok(res)
}

const BASE_URL: &str = "https://api.bgm.tv";

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub results: u32,
    pub list: Vec<SubjectBase>,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeResponse {
    // ignore SubjectBase
    pub eps: Vec<Episode>,
}

impl fmt::Display for EpisodeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let strings: Vec<String> = self.eps
            .iter()
            .map(|ep| format!("{:>width$}", ep, width=width))
            .collect();
        write!(f, "{}", strings.join("\n"))
    }
}

async fn request<T: DeserializeOwned>(path: &str) -> Result<T> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url: Uri = format!("{}{}", BASE_URL, path)
        .parse()
        .with_context(|| "parse url")?;
    debug!("url = {}", &url);
    let res = client.get(url).await.with_context(|| "get request")?;
    debug!("status: {}", res.status());
    let buf = hyper::body::to_bytes(res)
        .await
        .with_context(|| "read body")?;
    let res_obj: T = serde_json::from_slice(&buf).with_context(|| {
        let body = String::from_utf8(buf.to_vec()).unwrap_or_else(|_| "not utf8".to_string());
        format!("get body: {}", body)
    })?;
    Ok(res_obj)
}
