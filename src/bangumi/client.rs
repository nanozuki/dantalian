use super::types::SubjectSmall;
use anyhow::{Context, Result};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use percent_encoding::{utf8_percent_encode, CONTROLS};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json;

pub struct Bangumi {}

const BASE_URL: &str = "https://api.bgm.tv";

impl Bangumi {
    pub async fn search_subject(&self, keyword: String) -> Result<Vec<SubjectSmall>> {
        println!("search_subject: {}", keyword);
        let encoded_keyword = utf8_percent_encode(&keyword, &CONTROLS);
        let path = format!("/search/subject/{}?type=2", encoded_keyword);
        let res_obj: SearchResponse = request(&path).await?;
        println!("obj: {:?}", &res_obj);
        Ok(res_obj.list)
    }
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    results: u32,
    list: Vec<SubjectSmall>,
}

async fn request<T: DeserializeOwned>(path: &String) -> Result<T> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url: Uri = format!("{}{}", BASE_URL, path)
        .parse()
        .with_context(|| format!("parse url"))?;
    println!("url = {}", url);
    let res = client.get(url).await.with_context(|| "get request")?;
    println!("status: {}", res.status());
    let buf = hyper::body::to_bytes(res).await?;
    let res_obj: T = serde_json::from_slice(&buf)?;
    Ok(res_obj)
}
