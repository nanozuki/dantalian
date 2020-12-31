// TVShow file is for overall show informaion.
// TVShow file name must actully be tvshow.nfo.
// This file must be tv show's folder's root.
pub struct TVShow {
    pub uid: String,
    pub title: String,
    pub original_title: String,
    pub rating_value: f64,
    pub rating_votes: u32,
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
    pub actor: Vec<Actor>,
    pub episode: Vec<Episode>,
}

pub struct Actor {
    pub name: String,
    pub role: String,
    pub order: u32,
    pub thumb: String,
}

pub const TVSHOW_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<tvshow>
    <title></title>
    <originaltitle></originaltitle>
    <ratings>
        <rating name="bangumi" max="10" default="true">
            <value></value>
            <votes></votes>
        </rating>
    </ratings>
    <season>2</season>
    <episode>12</episode>
    <plot></plot>
    <thumb aspect="poster" preview="1.jpg">1.jpg</thumb>
    <uniqueid type="bangumi" default="true">id</uniqueid>
    <genre></genre>
    <tag></tag>
    <premiered>2019-01-13</premiered>
    <status>Air/Ended</status>
    <studio></studio>
    <actor>
        <name></name>
        <role></role>
        <order></order>
        <thumb></thumb>
    </actor>
</tvshow>
"#;

// Episode file is for single episode, this file must
// place alongside of media file, and use same file name.
pub struct Episode {
    pub file_name: String,

    pub uid: String,
    pub title: String,
    pub original_title: String,
    pub show_title: String,
    pub rating_value: f64,
    pub rating_votes: u32,
    pub ep_index: u32,
    pub is_sp: bool,
    pub plog: String,
    pub director: Vec<String>,
    pub credits: Vec<String>,
    pub premiered: String,
    pub status: Option<String>,
    pub aired: Option<String>,
    pub studio: Option<String>,
    pub actor: Vec<Actor>,
}

pub const EPISODE_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<episodedetails>
    <title></title>
    <originaltitle></originaltitle>
    <showtitle></showtitle>
    <ratings>
        <rating name="bangumi" max="10" default="true">
            <value></value>
            <votes></votes>
        </rating>
    </ratings>
    <season>2</season>
    <episode>12</episode>
    <displayseason>2</displayseason>
    <displayepisode>12</displaypisode>
    <plot></plot>
    <uniqueid type="bangumi" default="true">id</uniqueid>
    <credits></credits>
    <director></director>
    <premiered>2019-01-13</premiered>
    <status>Air/Ended</status>
    <aired>2019-01-13</aired>
    <studio></studio>
    <actor>
        <name></name>
        <role></role>
        <order></order>
        <thumb></thumb>
    </actor>
</episodedetails>
"#;
