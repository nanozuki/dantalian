# dantalian

[简体中文](./README_cn.md)

Dantalian is a nfo file generator for your anime, source from [bangumi](https://bangumi.tv/). You can use these nfo
files with media center software such as [Jellyfin](https://jellyfin.org/), [Kodi](https://kodi.tv/).

Some Popular scrapers, ["The Movie DB"](https://www.themoviedb.org), ["The TV DB"](https://thetvdb.com/) put all
episodes not published in TV into Season 00, including all SPs and OVAs. It is not suitable for anime, which usually
has multiple OVAs or SPs series, especially the ["Monogatari (series)"](https://www.themoviedb.org/tv/46195/season/0).
[AniDB](https://anidb.net/) put episodes in the right place, but It only has English Infos.

So, Dantalian uses bangumi as animes source to generate nfo files. In media center software like Jellyfin or Kodi, nfo
files have higher priority than scrapers. As bangumi is not design for anime media files database, some of the data has
lacked.

In general, this thing has the following peculiarity:

* One subject only has one season's episodes and SPs.
* It prefers anime characters' info to actors' info.
* Support custom tags in folder
* Lack of genres, tags. Lack of actors and staff info for a single episode
(Dantalian will use the whole series info as an alternative).
* Lack of a poster of a single episode. The media center software will capture it.

## File structure

```
<media_root>/
├── ひぐらしのなく頃に 業
│   ├── tvshow.nfo
│   ├── ひぐらしのなく頃に 業 01.mp4
│   ├── ひぐらしのなく頃に 業 01.nfo
│   ├── ひぐらしのなく頃に 業 02.mp4
│   ├── ひぐらしのなく頃に 業 02.nfo
│   ├── ひぐらしのなく頃に 業 03.mp4
│   ├── ひぐらしのなく頃に 業 03.nfo
│   ├── ひぐらしのなく頃に 業 04.mp4
│   └── ひぐらしのなく頃に 業 04.nfo
├── 化物語 [2009][BDRip]
│   ├── tvshow.nfo
│   ├── 化物語 01.mp4
│   ├── 化物語 01.nfo
│   ├── 化物語 02.mp4
│   ├── 化物語 02.nfo
│   ├── 化物語 SP5_5.mp4
│   └── 化物語 SP5_5.nfo
└── 进击的巨人 最终季
    ├── tvshow.nfo
    ├── 进击的巨人 最终季 60.mp4
    ├── 进击的巨人 最终季 60.nfo
    ├── 进击的巨人 最终季 61.mp4
    ├── 进击的巨人 最终季 61.nfo
    ├── 进击的巨人 最终季 62.mp4
    ├── 进击的巨人 最终季 62.nfo
    ├── 进击的巨人 最终季 63.mp4
    └── 进击的巨人 最终季 63.nfo
```

Put all episodes of one anime subject into one folder and put the folder in your media lib's root directory ("media
root"). The name of the folder should follow this format below:

```
Anime Name [自定义tag][tag1][tag2]
```

It's better to check the anime name first. You can check anime name by bangumi. (or use `dantalian bgm search <keyword>`
). Episode files' names should follow this format:

```
Same name of the folder 12.mp4
```

For SPs file, you should add "SP" before the episode number. If the episode number is not an integer, you should
translate the `.` to `_`:

```
化物语 SP5_5
```

You can add '0' before the episode number or not.

## Generate NFO files

Use this command to generate nfo files:

```
dantalian gen --root <medie_root_path>
```

As the file is used and played, the media center software will modify nfo files to store dynamic data. Because of this
situation, dantalian will not re-generate nfo files if it already exists. If you want to force re-generate, you can add
a `--force` option to the command. Both the `--root` and `--force` option are repeatable.

```
dantalian gen --root <path1> --root <path2> --force 化物语
```

## Roadmap

- [ ] Anime Movie / "theater edition"
- [ ] BD file
- [ ] DVD file
- [ ] Custom file pattern or fuzzy match.
