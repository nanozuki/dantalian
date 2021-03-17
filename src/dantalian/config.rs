use crate::bangumi::{get_subject_info, search_anime};
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct ConfigFile {
    subject_id: u32,
    episode_re: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub subject_id: u32,
    pub episode_re: Regex,
}

const DIR_CONFIG_NAME: &str = "dantalian.toml";

impl Config {
    pub async fn parse(path: &Path) -> Result<Config> {
        let filepath = path.join(DIR_CONFIG_NAME);
        match filepath.exists() {
            true => Self::parse_from_file(&filepath).await,
            false => Self::parse_from_dirname(path).await,
        }
    }

    async fn parse_from_file(filepath: &Path) -> Result<Config> {
        let file = std::fs::read_to_string(filepath)?;
        let cf: ConfigFile = toml::from_str(file.as_ref())?;
        match cf.episode_re {
            Some(re) => Ok(Config {
                subject_id: cf.subject_id,
                episode_re: Regex::new(&re)?,
            }),
            None => {
                let subject = get_subject_info(cf.subject_id).await?;
                Ok(Config {
                    subject_id: cf.subject_id,
                    episode_re: default_ep_regex(&subject.name, &subject.name_cn)?,
                })
            }
        }
    }

    async fn parse_from_dirname(path: &Path) -> Result<Config> {
        let anime_name = path.to_str().and_then(|f| cap_anime_name(f));
        match anime_name {
            Some(name) => {
                let subjects = search_anime(&name).await?;
                if subjects.is_empty() {
                    bail!("not found");
                }
                Ok(Config {
                    subject_id: subjects[0].id,
                    episode_re: default_ep_regex(&name)?,
                })
            }
            None => bail!("invalid name"),
        }
    }

    async fn save(&self, dir: &Path) -> Result<()> {
        let file_content = toml::to_string(&ConfigFile {
            subject_id: self.subject_id,
            episode_re: Some(self.episode_re.to_string()),
        })?;
        std::fs::write(dir.join(DIR_CONFIG_NAME), file_content)?;
        Ok(())
    }
}

fn cap_anime_name(dir_name: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<name>.+?)(?P<tags> (\[[^\s]+\])+)?$").unwrap();
    }
    RE.captures(dir_name)
        .and_then(|cap| cap.name("name"))
        .map(|mat| String::from(mat.as_str()))
}

fn default_ep_regex(name: &str, name_cn: &str) -> Result<Regex> {
    Ok(Regex::new(&format!(
        r"^(?P<name>{}|{}) (?P<sp>SP)?(?P<ep>[_\d]+)\.",
        name, name_cn
    ))?)
}
