use serde::Serialize;

// TVShow file is for overall show informaion.
// TVShow file name must actully be tvshow.nfo.
// This file must be tv show's folder's root.
#[derive(Serialize, Debug)]
pub struct TVShow<'a> {
    pub uid: u32,
    pub title: String,
    pub original_title: String,
    pub rating_value: f64,
    pub rating_votes: u32,
    pub has_sp: bool,
    pub eps_count: u32,
    pub plot: String,
    pub director: Vec<String>,
    pub credits: Vec<String>,
    pub poster: String,
    pub genre: Vec<String>,
    pub tag: Vec<String>,
    pub premiered: String,
    pub status: Option<String>,
    pub studio: Option<String>,
    pub actor: &'a Vec<Actor<'a>>,
}

#[derive(Serialize, Debug)]
pub struct Actor<'a> {
    pub name: &'a String,
    pub role: &'a String,
    pub order: u32,
    pub thumb: &'a String,
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
    <episode>{eps_count}</episode>
    <plot>{plot}</plot>
    <thumb aspect="poster" preview="{poster}">{poster}</thumb>
    <uniqueid type="bangumi" default="true">{uid}</uniqueid>
    {{ for g in genre }}
    <genre>{g}</genre>
    {{ endfor }}
    {{ for t in tag }}
    <tag>{t}</tag>
    {{ endfor }}
    <premiered>{premiered}</premiered>
    {{ if status }}<status>{status}</status>{{ endif }}
    {{ if studio }}<studio>{studio}</studio>{{ endif }}
    {{ for a in actor }}
    <actor>
        <name>{a.name}</name>
        <role>{a.role}</role>
        <order>{a.order}</order>
        <thumb>{a.thumb}</thumb>
    </actor>
    {{ endfor }}
</tvshow>
"#;

// Episode file is for single episode, this file must
// place alongside of media file, and use same file name.
#[derive(Serialize, Debug)]
pub struct Episode<'a> {
    pub uid: u32,
    pub title: String,
    pub original_title: String,
    pub show_title: String,
    pub rating_value: Option<f64>,
    pub rating_votes: Option<u32>,
    pub ep_index: u32,
    pub is_sp: bool,
    pub plog: String,
    pub director: Vec<String>,
    pub credits: Vec<String>,
    pub premiered: String,
    pub status: Option<String>,
    pub aired: Option<String>,
    pub studio: Option<String>,
    pub actor: &'a Vec<Actor<'a>>,
}

pub const EPISODE_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<episodedetails>
    <title>{title}</title>
    <originaltitle>{original_title}</originaltitle>
    <showtitle>{show_title}</showtitle>
    {{ if rating_value }}
    <ratings>
        <rating name="bangumi" max="10" default="true">
            <value>{rating_value}</value>
            {{ if rating_votes }}<votes>{rating_votes}</votes>{{ endif }}
        </rating>
    </ratings>
    {{ endif }}
    <season>{{ if is_sp }}0{{ else }}1{{ endif }}</season>
    <episode>{ep_index}</episode>
    <plot>{plot}</plot>
    <uniqueid type="bangumi" default="true">{uid}</uniqueid>
    {{ for c in credits }}
    <credits>{c}</credits>
    {{ endfor }}
    {{ for d in director }}
    <director>{d}</director>
    {{ endfor }}
    <premiered>{premiered}</premiered>
    {{ if status }}<status>{status}</status>{{ endif }}
    <aired>{aired}</aired>
    {{ if studio }}<studio>{studio}</studio>{{ endif }}
    {{ for a in actor }}
    <actor>
        <name>{a.name}</name>
        <role>{a.role}</role>
        <order>{a.order}</order>
        <thumb>{a.thumb}</thumb>
    </actor>
    {{ endfor }}
</episodedetails>
"#;
