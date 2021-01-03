use super::types::{Episode, SubjectBase, SubjectMedium};
use anyhow::{Context, Result};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{de::DeserializeOwned, Deserialize};

pub async fn search_anime(keyword: &str) -> Result<Vec<SubjectBase>> {
    println!("search_subject: {}", keyword);
    let encoded_keyword = utf8_percent_encode(&keyword, NON_ALPHANUMERIC);
    let path = format!("/search/subject/{}?type=2", encoded_keyword);
    println!("request url {}", path);
    let res_obj: SearchResponse = request(&path)
        .await
        .with_context(|| "request search anime")?;
    println!("obj: {:?}", &res_obj);
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
    println!("get_subject_info: {}", id);
    let path = format!("/subject/{}?responseGroup=medium", id);
    let subject: SubjectMedium = request(&path)
        .await
        .with_context(|| format!("request get subject: {}", id))?;
    println!("subject: {:#?}", &subject);
    Ok(subject)
}

pub async fn get_subject_episodes(id: u32) -> Result<Vec<Episode>> {
    println!("get_subject_info: {}", id);
    let path = format!("/subject/{}/ep", id);
    let res: EpisodeResponse = request(&path)
        .await
        .with_context(|| format!("get subject episode {}", id))?;
    for ep in &res.eps {
        println!("subject ep: {:#?}", &ep);
    }
    Ok(res.eps)
}

// mirror of "https://api.bgm.tv" for scripts.
const BASE_URL: &str = "https://mirror.api.bgm.rin.cat";

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

async fn request<T: DeserializeOwned>(path: &str) -> Result<T> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url: Uri = format!("{}{}", BASE_URL, path)
        .parse()
        .with_context(|| "parse url")?;
    println!("url = {}", &url);
    let res = client.get(url).await.with_context(|| "get request")?;
    println!("status: {}", res.status());
    let buf = hyper::body::to_bytes(res)
        .await
        .with_context(|| "read body")?;
    let res_obj: T = serde_json::from_slice(&buf).with_context(|| {
        let body = String::from_utf8(buf.to_vec()).unwrap_or_else(|_| "not utf8".to_string());
        format!("get body: {}", body)
    })?;
    Ok(res_obj)
}
