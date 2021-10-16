use serde::Serialize;
use std::rc::Rc;

pub const TVSHOW_NFO_NAME: &str = "tvshow.nfo";

// TVShow file is for overall show informaion.
// TVShow file name must actully be tvshow.nfo.
// This file must be tv show's folder's root.
#[derive(Serialize, Debug)]
pub struct TVShow {
    pub uid: u32,
    pub title: String,
    pub original_title: String,
    pub rating_value: f64,
    pub rating_votes: u32,
    pub has_sp: bool,
    pub eps_count: Option<u32>,
    pub plot: String,
    pub poster: String,
    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub premiered: String,
    pub status: Option<String>,
    pub studio: Option<String>,
    pub actors: Rc<[Actor]>,
}

#[derive(Serialize, Debug)]
pub struct Actor {
    pub name: String,
    pub role: String,
    pub order: u32,
    pub thumb: String,
}

pub const TVSHOW_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<tvshow>
    <title>{title}</title>
    <originaltitle>{original_title}</originaltitle>
    <ratings>
        <rating name="bangumi" max="10" default="true">
            <value>{rating_value}</value>
            <votes>{rating_votes}</votes>
        </rating>
    </ratings>
    <season>{{ if has_sp }}2{{ else }}1{{ endif }}</season>
    {{ if eps_count }}<episode>{eps_count}</episode>{{ endif }}
    <plot>{plot}</plot>
    <thumb aspect="poster" preview="{poster}">{poster}</thumb>
    <uniqueid type="bangumi" default="true">{uid}</uniqueid>{{ for g in genres }}
    <genre>{g}</genre>{{ endfor }}{{ for t in tags }}
    <tag>{t}</tag>{{ endfor }}
    <premiered>{premiered}</premiered>{{ if status }}
    <status>{status}</status>{{ endif }}{{ if studio }}
    <studio>{studio}</studio>{{ endif }}{{ for a in actors }}
    <actor>
        <name>{a.name}</name>
        <role>{a.role}</role>
        <order>{a.order}</order>
        <thumb>{a.thumb}</thumb>
    </actor>{{ endfor }}
</tvshow>
"#;

// Episode file is for single episode, this file must
// place alongside of media file, and use same file name.
#[derive(Serialize, Debug)]
pub struct Episode {
    pub uid: u32,
    pub title: String,
    pub original_title: String,
    pub show_title: String,
    pub rating_value: Option<f64>,
    pub rating_votes: Option<u32>,
    pub ep_index: String,
    pub is_sp: bool,
    pub plot: String,
    pub directors: Rc<[String]>,
    pub credits: Rc<[String]>,
    pub premiered: String,
    pub status: Option<String>,
    pub aired: Option<String>,
    pub studio: Option<String>,
    pub actors: Rc<[Actor]>,
}

pub const EPISODE_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<episodedetails>
    <title>{title}</title>
    <originaltitle>{original_title}</originaltitle>
    <showtitle>{show_title}</showtitle>{{ if rating_value }}
    <ratings>
        <rating name="bangumi" max="10" default="true">
            <value>{rating_value}</value>
            {{ if rating_votes }}<votes>{rating_votes}</votes>{{ endif }}
        </rating>
    </ratings>{{ endif }}
    <season>{{ if is_sp }}0{{ else }}1{{ endif }}</season>
    <episode>{ep_index}</episode>
    <plot>{plot}</plot>
    <uniqueid type="bangumi" default="true">{uid}</uniqueid>{{ for c in credits }}
    <credits>{c}</credits>{{ endfor }}{{ for d in directors }}
    <director>{d}</director>{{ endfor }}
    <premiered>{premiered}</premiered>{{ if status }}
    <status>{status}</status>
    {{ endif }}<aired>{aired}</aired>{{ if studio }}
    <studio>{studio}</studio>{{ endif }}{{ for a in actors }}
    <actor>
        <name>{a.name}</name>
        <role>{a.role}</role>
        <order>{a.order}</order>
        <thumb>{a.thumb}</thumb>
    </actor>{{ endfor }}
</episodedetails>
"#;
