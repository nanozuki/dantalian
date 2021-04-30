use super::types::{Episode, SubjectBase, SubjectMedium};
use crate::bangumi::EpisodeStatus;
use anyhow::{Context, Result};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use log::{debug, info, trace};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{de::DeserializeOwned, Deserialize};
use std::fmt;
use std::time::SystemTime;

pub async fn search_anime(keyword: &str) -> Result<Vec<SubjectBase>> {
    info!("search anime '{}':", keyword);
    let encoded_keyword = utf8_percent_encode(&keyword, NON_ALPHANUMERIC);
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let path = format!(
        "/search/subject/{}?type=2&chii_searchDateLine={}",
        encoded_keyword, ts,
    );
    trace!("request url {}", path);
    let res_obj: SearchResponse = request(&path)
        .await
        .with_context(|| "request search anime")?;
    debug!("obj: {:?}", &res_obj);
    info!("found {} result(s):\n", &res_obj.results);
    for item in res_obj.list.iter() {
        info!("    * {} / {}", item.name, item.name_cn);
        info!("      Subject ID: {}", item.id);
        info!("      URL: {}", item.url);
    }
    Ok(res_obj.list)
}

pub struct BgmAnime {
    pub subject: SubjectMedium,
    pub episodes: Vec<Episode>,
}

pub async fn get_anime_data(id: u32) -> Result<BgmAnime> {
    let subject = get_subject_info(id).await?;
    let episodes = get_subject_episodes(id).await?;
    Ok(BgmAnime { subject, episodes })
}

pub async fn get_subject_info(id: u32) -> Result<SubjectMedium> {
    let path = format!("/subject/{}?responseGroup=medium", id);
    let subject: SubjectMedium = request(&path)
        .await
        .with_context(|| format!("request get subject: {}", id))?;
    debug!("subject: {:#?}", &subject);
    info!("{}", &subject);
    Ok(subject)
}

pub async fn get_subject_episodes(id: u32) -> Result<Vec<Episode>> {
    trace!("get_subject_info: {}", id);
    let path = format!("/subject/{}/ep", id);
    let res: EpisodeResponse = request(&path)
        .await
        .with_context(|| format!("get subject episode {}", id))?;
    for ep in &res.eps {
        debug!("subject ep: {:#?}", &ep);
    }
    info!("{}", &res);
    Ok(res.eps)
}

const BASE_URL: &str = "https://api.bgm.tv";

#[derive(Deserialize, Debug)]
struct SearchResponse {
    results: u32,
    list: Vec<SubjectBase>,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeResponse {
    // ignore SubjectBase
    pub eps: Vec<Episode>,
}

impl fmt::Display for EpisodeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ep in self.eps.iter().filter(|ep| ep.status != EpisodeStatus::NA) {
            writeln!(
                f,
                "{} {}\t{} / {}",
                ep.episode_type, ep.sort, ep.name, ep.name_cn
            )?;
        }
        Ok(())
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
