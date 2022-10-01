use crate::logger::indent_display;
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

const BGM_WEB: &str = "https://bgm.tv";

#[derive(Deserialize_repr, Debug, Serialize_repr)]
#[repr(u32)]
pub enum SubjectType {
    Book = 1,
    Anime = 2,
    Music = 3,
    Game = 4,
    Real = 6,
}

#[derive(Deserialize, Debug)]
pub struct SubjectImage {
    pub large: String,
    pub common: String,
    pub medium: String,
    pub small: String,
    pub grid: String,
}

#[derive(Deserialize, Debug)]
pub struct SubjectRatingCount {
    #[serde(rename = "1")]
    pub s1: u32,
    #[serde(rename = "2")]
    pub s2: u32,
    #[serde(rename = "3")]
    pub s3: u32,
    #[serde(rename = "4")]
    pub s4: u32,
    #[serde(rename = "5")]
    pub s5: u32,
    #[serde(rename = "6")]
    pub s6: u32,
    #[serde(rename = "7")]
    pub s7: u32,
    #[serde(rename = "8")]
    pub s8: u32,
    #[serde(rename = "9")]
    pub s9: u32,
    #[serde(rename = "10")]
    pub s10: u32,
}

#[derive(Deserialize, Debug)]
pub struct SubjectRating {
    pub rank: u32,
    pub total: u32,
    pub score: f64,
    pub count: SubjectRatingCount,
}

#[derive(Deserialize, Debug)]
pub struct SubjectCollection {
    pub wish: u32,
    pub collect: u32,
    pub doing: u32,
    pub on_hold: u32,
    pub dropped: u32,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub count: u32,
}

/// Subject in search.
#[derive(Deserialize, Debug)]
pub struct SubjectBase {
    pub id: u32,
    #[serde(rename = "type")]
    pub subject_type: Option<SubjectType>,
    pub name: String,
    pub name_cn: String,
    pub summary: String,
    pub date: String,
    pub score: f32,
    pub rank: u32,
    pub images: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

/// New subjectbase has no url field.
impl SubjectBase {
    fn url(&self) -> String {
        format!("{}/subject/{}", BGM_WEB, self.id)
    }
}

impl fmt::Display for SubjectBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = indent_display(f);
        let strings = vec![
            format!("{}* {} / {}", prefix, self.name, self.name_cn),
            format!("{}  Subject ID: {}", prefix, self.id),
            format!("{}  Air Date: {}", prefix, self.date),
            format!("{}  URL: {}", prefix, self.url()),
        ];
        write!(f, "{}", strings.join("\n"))
    }
}

/// There is no SubjectMedium/Base or else.
/// infobox is too big and contains things we don't need.
/// volumes is for book, has no relations with anime.
#[derive(Deserialize, Debug)]
pub struct Subject {
    pub id: u32,
    #[serde(rename = "type")]
    pub subject_type: SubjectType,
    pub name: String,
    pub name_cn: String,
    pub summary: String,
    pub nsfw: bool,
    pub date: String,
    /// TV, Web, 欧美剧, PS4...
    pub platform: String,
    pub images: Option<SubjectImage>,
    pub eps: Option<u32>,
    pub total_episodes: Option<u32>,
    pub rating: SubjectRating,
    pub collection: SubjectCollection,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

impl Subject {
    pub fn url(&self) -> String {
        format!("{}/subject/{}", BGM_WEB, self.id)
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = indent_display(f);
        let strings = vec![
            format!("{}* {} / {}", prefix, self.name, self.name_cn),
            format!("{}* {}", prefix, self.url()),
            format!("{}* Air Date: {}", prefix, self.date),
            format!("{}* {}", prefix, self.summary),
        ];
        write!(f, "{}", strings.join("\n"))
    }
}

#[derive(Deserialize, Debug)]
pub struct Character {
    pub id: u32,
    pub url: String,
    pub name: String,
    pub images: CharacterImage,
    pub name_cn: String,
    pub comment: u32,
    pub collects: u32,
    pub actors: Option<Vec<Actor>>,
    pub role_name: String, // example: 主角
}

#[derive(Deserialize, Debug)]
pub struct CharacterImage {
    pub large: String,
    pub medium: String,
    pub small: String,
    pub grid: String,
}

#[derive(Deserialize, Debug)]
pub struct Actor {
    pub id: u32,
    pub url: String,
    pub name: String,
    pub images: Option<CharacterImage>,
}

#[derive(Deserialize, Debug)]
pub struct Staff {
    pub id: u32,
    pub url: String,
    pub name: String,
    pub images: Option<CharacterImage>,
    pub name_cn: String,
    pub comment: u32,
    pub collects: u32,
    pub role_name: String,
    pub jobs: Vec<String>,
}

#[derive(Deserialize_repr, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum EpisodeType {
    Honpen = 0,
    Sp = 1,
    OP = 2,
    ED = 3,
    CM = 4,
    MAD = 5,
    Other = 6,
}

impl fmt::Display for EpisodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EpisodeType::Honpen => write!(f, ""),
            EpisodeType::Sp => write!(f, "SP"),
            EpisodeType::OP => write!(f, "OP"),
            EpisodeType::ED => write!(f, "ED"),
            EpisodeType::CM => write!(f, "CM"),
            EpisodeType::MAD => write!(f, "MAD"),
            EpisodeType::Other => write!(f, "Other"),
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub enum EpisodeStatus {
    Air,
    Today,
    NA,
}

#[derive(Deserialize, Debug)]
pub struct Episode {
    pub id: u32,
    pub url: String,
    #[serde(rename = "type")]
    pub episode_type: EpisodeType,
    pub sort: f64,
    pub name: String,
    pub name_cn: String,
    pub duration: String,
    pub airdate: String,
    pub comment: u32,
    pub desc: String,
    pub status: EpisodeStatus,
}

impl Episode {
    pub fn is_empty(&self) -> bool {
        self.status == EpisodeStatus::NA && self.name.is_empty() && self.name_cn.is_empty()
    }
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = indent_display(f);
        let idx = format!("{:>3}{:02}", self.episode_type, self.sort);
        write!(f, "{}{:>6}: {} / {}", prefix, idx, self.name, self.name_cn)
    }
}
