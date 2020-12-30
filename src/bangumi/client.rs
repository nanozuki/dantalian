use anyhow::Result;
use reqwest;
use super::types::{SubjectSmall};

pub struct Bangumi {}

const BASE_URL: &str = "https://api.bgm.tv";


impl Bangumi {
    pub async fn search_subject(&self, keyword: String) -> Result<Vec<SubjectSmall>> {
        let url = format!("{}/search/subject/{}?type=2", BASE_URL, keyword);
        println!("get request, to {}", &url);
        let body = reqwest::get(&url).await?.text().await?;
        println!("body = {:?}", body);
        Ok(vec![])
    }
}
