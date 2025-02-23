use crate::bangumi::{get_subject, search_anime};
use crate::info;
use anyhow::{Result, anyhow, bail};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct ConfigFile {
    subject_id: u32,
    episode_re: Option<String>,
    episode_offset: Option<i32>,
}

#[derive(Debug)]
pub struct Config {
    pub subject_id: u32,
    pub episode_re: Regex,
    pub episode_offset: i32,
}

const DIR_CONFIG_NAME: &str = "dantalian.toml";

impl Config {
    pub async fn parse(path: &Path) -> Result<Config> {
        let filepath = path.join(DIR_CONFIG_NAME);
        let config = match filepath.exists() {
            true => Self::parse_from_file(&filepath).await?,
            false => Self::parse_from_dirname(path).await?,
        };
        config.save(filepath.as_path())?;
        Ok(config)
    }

    async fn parse_from_file(filepath: &Path) -> Result<Config> {
        info!(ind: 2, "Parse config file");
        let file = std::fs::read_to_string(filepath)?;
        let cf: ConfigFile = toml::from_str(file.as_ref())?;
        let episode_re = match cf.episode_re {
            Some(re) => Regex::new(&re)?,
            None => {
                let subject = get_subject(cf.subject_id).await?;
                let name_qry = format!("{}|{}", subject.name, subject.name_cn);
                default_ep_regex(&name_qry)?
            }
        };
        let episode_offset = cf.episode_offset.unwrap_or(0);
        Ok(Config {
            subject_id: cf.subject_id,
            episode_re,
            episode_offset,
        })
    }

    async fn parse_from_dirname(path: &Path) -> Result<Config> {
        info!(ind: 2, "Not found config file, create one");
        let dirname = path
            .file_name()
            .ok_or_else(|| anyhow!("invalid path"))?
            .to_str()
            .ok_or_else(|| anyhow!("invalid path"))?;
        let anime_name = cap_anime_name(dirname);
        match anime_name {
            Some(name) => {
                let subjects = search_anime(&name).await?.data;
                if subjects.is_empty() {
                    bail!("not found");
                }
                Ok(Config {
                    subject_id: subjects[0].id,
                    episode_re: default_ep_regex(&name)?,
                    episode_offset: 0,
                })
            }
            None => bail!("invalid name"),
        }
    }

    fn save(&self, filepath: &Path) -> Result<()> {
        let file_content = toml::to_string(&ConfigFile {
            subject_id: self.subject_id,
            episode_re: Some(self.episode_re.to_string()),
            episode_offset: Some(self.episode_offset),
        })?;
        let mut f = File::create(filepath)?;
        f.write_all(&file_content.into_bytes())?;
        Ok(())
    }
}

static DEFAULT_DIR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<name>.+?)(?P<tags> (\[[^\s]+\])+)?$").unwrap());

fn cap_anime_name(dir_name: &str) -> Option<String> {
    DEFAULT_DIR_RE
        .captures(dir_name)
        .and_then(|cap| cap.name("name"))
        .map(|mat| String::from(mat.as_str()))
}

fn default_ep_regex(name_qry: &str) -> Result<Regex> {
    Ok(Regex::new(&format!(
        r"^(?P<name>{}) (?P<sp>SP)?(?P<ep>[.\d]+)\.",
        name_qry
    ))?)
}
