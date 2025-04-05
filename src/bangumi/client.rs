use super::types::{
    BgmError, Character, Characters, Episode, Person, Persons, Subject, SubjectBase, SubjectType,
};
use anyhow::{Context, Result};
use log::{debug, trace};
use once_cell::sync::OnceCell;
use reqwest::{Body, Client, RequestBuilder, Url};
use serde::Serialize;
use serde::{Deserialize, de::DeserializeOwned};
use std::fmt;

pub(crate) static ACCESS_TOKEN: OnceCell<String> = OnceCell::new();

// Trailing slash is necessary
const BASE_URL: &str = "https://api.bgm.tv/v0/";

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

impl BangumiRequest for SearchSubjectRequest<'_> {
    fn url(&self) -> Result<Url> {
        Ok(Url::parse(BASE_URL)?.join("search/subjects")?)
    }

    fn body(&self) -> Result<Option<Body>> {
        let body = Body::from(serde_json::to_vec(&self)?);
        Ok(Some(body))
    }

    fn modify(&self, client: &Client) -> Result<RequestBuilder> {
        Ok(client.post(self.url()?))
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
    trace!("request url {}", search.url()?.to_string());
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
    let path = format!("subjects/{}", id);
    let subject = request(path).await?;
    // .with_context(|| format!("request get subject: {}", id))?;
    debug!("subject: {:#?}", &subject);
    Ok(subject)
}

pub async fn get_subject_persons(id: u32) -> Result<Persons> {
    let path = format!("subjects/{}/persons", id);
    let persons = request(path)
        .await
        .with_context(|| format!("request get subject persons: {}", id))?;
    debug!("persons: {:#?}", &persons);
    Ok(Persons(persons))
}

pub async fn get_subject_characters(id: u32) -> Result<Characters> {
    let path = format!("subjects/{}/characters", id);
    let characters = request(path)
        .await
        .with_context(|| format!("request get subject characters: {}", id))?;
    debug!("characters: {:#?}", &characters);
    Ok(Characters(characters))
}

pub async fn get_subject_episodes(id: u32) -> Result<EpisodeResponse> {
    trace!("get_subject_info: {}", id);
    let path = format!("episodes?subject_id={}", id);
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

    fn url(&self) -> Result<Url>;

    fn modify(&self, client: &Client) -> Result<RequestBuilder> {
        Ok(client.get(self.url()?))
    }
}

impl BangumiRequest for String {
    fn url(&self) -> Result<Url> {
        Ok(Url::parse(BASE_URL)?.join(self)?)
    }
}

async fn request<T: DeserializeOwned, Req: BangumiRequest>(bgm_req: Req) -> Result<T> {
    let user_agent = format!(
        "Dantalian/{} (https://github.com/nanozuki/dantalian)",
        env!("CARGO_PKG_VERSION")
    );

    let client = Client::builder().user_agent(user_agent).build()?;
    let mut req_builder = bgm_req.modify(&client)?;
    if let Some(access_token) = ACCESS_TOKEN.get() {
        req_builder = req_builder.bearer_auth(access_token);
    }
    let body = bgm_req.body()?.unwrap_or_default();
    req_builder = req_builder.body(body);
    let req = req_builder.build()?;
    debug!("url = {}", req.url());

    let res = client.execute(req).await.with_context(|| "get request")?;
    debug!("status: {}", res.status());
    let is_ok = res.status().is_success();

    let buf = res.bytes().await.with_context(|| "read body")?;

    if !is_ok {
        let body = String::from_utf8(buf.to_vec()).unwrap_or_else(|_| "not utf8".to_string());
        let err: BgmError =
            serde_json::from_slice(&buf).with_context(|| format!("deserialize error: {}", body))?;
        Err(err)?;
    }

    let res_obj: T = serde_json::from_slice(&buf).with_context(|| {
        let body = String::from_utf8(buf.to_vec()).unwrap_or_else(|_| "not utf8".to_string());
        format!("get body: {}", body)
    })?;
    Ok(res_obj)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subject_get() {
        let result = get_subject(1).await.unwrap();
        // let Ok(result) = result else {
        //     let err = result.err().unwrap();

        //     panic!("Error {}", err);
        // };

        assert_eq!(result.id, 1);
    }
}
