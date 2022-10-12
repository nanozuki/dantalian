use super::types::{
    BgmError, Character, Characters, Episode, Person, Persons, Subject, SubjectBase, SubjectType,
};
use anyhow::{Context, Result};
use hyper::http::request;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use log::{debug, trace};
use once_cell::sync::OnceCell;
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use std::fmt;
use std::str::FromStr;

pub(crate) static ACCESS_TOKEN: OnceCell<String> = OnceCell::new();

const BASE_URL: &str = "https://api.bgm.tv/v0";

pub fn set_access_token(token: String) {
    // Should only set once. Set twice is a bug.
    ACCESS_TOKEN.set(token).unwrap();
}

#[derive(Serialize)]
struct SearchSubjectFilter {
    #[serde(rename = "type")]
    subject_type: Vec<SubjectType>,
}

#[derive(Serialize)]
struct SearchSubjectRequest<'a> {
    pub keyword: &'a str,
    pub filter: SearchSubjectFilter,
}

impl<'a> BangumiRequest for SearchSubjectRequest<'a> {
    fn uri(&self) -> Result<Uri> {
        Ok(Uri::from_str(&[BASE_URL, "/search/subjects"].concat())?)
    }

    fn body(&self) -> Result<Option<Body>> {
        let body = Body::from(serde_json::to_vec(&self)?);
        Ok(Some(body))
    }

    fn modify(&self, req: request::Builder) -> Result<request::Builder> {
        Ok(req.uri(self.uri()?).method(Method::POST))
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub data: Vec<SubjectBase>,
}

pub async fn search_anime(keyword: &str) -> Result<SearchResponse> {
    let search = SearchSubjectRequest {
        keyword,
        filter: SearchSubjectFilter {
            subject_type: vec![SubjectType::Anime],
        },
    };
    trace!("request url {}", search.uri()?.to_string());
    let res = request(search)
        .await
        .with_context(|| "request search anime")?;
    debug!("obj: {:?}", &res);
    Ok(res)
}

pub struct BgmAnime {
    pub subject: Subject,
    pub episodes: Vec<Episode>,
    pub persons: Vec<Person>,
    pub characters: Vec<Character>,
}

pub async fn get_anime_data(id: u32) -> Result<BgmAnime> {
    let subject = get_subject(id).await?;
    let persons = get_subject_persons(id).await?.0;
    let characters = get_subject_characters(id).await?.0;
    let episodes = get_subject_episodes(id).await?.data;
    Ok(BgmAnime {
        subject,
        episodes,
        persons,
        characters,
    })
}

pub async fn get_subject(id: u32) -> Result<Subject> {
    let path = format!("/subjects/{}", id);
    let subject = request(path)
        .await
        .with_context(|| format!("request get subject: {}", id))?;
    debug!("subject: {:#?}", &subject);
    Ok(subject)
}

pub async fn get_subject_persons(id: u32) -> Result<Persons> {
    let path = format!("/subjects/{}/persons", id);
    let persons = request(path)
        .await
        .with_context(|| format!("request get subject persons: {}", id))?;
    debug!("persons: {:#?}", &persons);
    Ok(Persons(persons))
}

pub async fn get_subject_characters(id: u32) -> Result<Characters> {
    let path = format!("/subjects/{}/characters", id);
    let characters = request(path)
        .await
        .with_context(|| format!("request get subject characters: {}", id))?;
    debug!("characters: {:#?}", &characters);
    Ok(Characters(characters))
}

pub async fn get_subject_episodes(id: u32) -> Result<EpisodeResponse> {
    trace!("get_subject_info: {}", id);
    let path = format!("/episodes?subject_id={}", id);
    let res: EpisodeResponse = request(path)
        .await
        .with_context(|| format!("get subject episode {}", id))?;
    for ep in &res.data {
        debug!("subject ep: {:#?}", &ep);
    }
    Ok(res)
}

#[derive(Deserialize, Debug)]
pub struct EpisodeResponse {
    pub data: Vec<Episode>,
}

impl fmt::Display for EpisodeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or(0);
        let strings: Vec<String> = self
            .data
            .iter()
            .map(|ep| format!("{:>width$}", ep, width = width))
            .collect();
        write!(f, "{}", strings.join("\n"))
    }
}

trait BangumiRequest {
    fn body(&self) -> Result<Option<Body>> {
        Ok(None)
    }

    fn uri(&self) -> Result<Uri>;

    fn modify(&self, req: request::Builder) -> Result<request::Builder> {
        Ok(req.uri(self.uri()?).method(Method::GET))
    }
}

impl BangumiRequest for String {
    fn uri(&self) -> Result<Uri> {
        Ok(Uri::from_str(&[BASE_URL, self].concat())?)
    }
}

async fn request<T: DeserializeOwned, Req: BangumiRequest>(bgm_req: Req) -> Result<T> {
    let user_agent = format!(
        "Dantalian/{} (https://github.com/nanozuki/dantalian)",
        env!("CARGO_PKG_VERSION")
    );

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut req = Request::builder().header("User-Agent", user_agent);
    if let Some(access_token) = ACCESS_TOKEN.get() {
        req = req.header("Authorization", format!("Bearer {}", access_token));
    }
    req = bgm_req.modify(req)?;
    debug!(
        "url = {}",
        req.uri_ref()
            .ok_or_else(|| anyhow::anyhow!("No Uri Setted."))?
    );
    let body = bgm_req.body()?.unwrap_or_default();
    let req = req.body(body)?;

    let res = client.request(req).await.with_context(|| "get request")?;
    let is_ok = res.status().is_success();
    debug!("status: {}", res.status());

    let buf = hyper::body::to_bytes(res)
        .await
        .with_context(|| "read body")?;

    if !is_ok {
        let err: BgmError = serde_json::from_slice(&buf).with_context(|| "deserialize error")?;
        Err(err)?;
    }

    let res_obj: T = serde_json::from_slice(&buf).with_context(|| {
        let body = String::from_utf8(buf.to_vec()).unwrap_or_else(|_| "not utf8".to_string());
        format!("get body: {}", body)
    })?;
    Ok(res_obj)
}
