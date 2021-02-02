# dantalian

dantalian 是一个以 [bangumi](https://bangumi.tv/) 为数据源的动画视频的 NFO 文件生成器，
可以用于 [Jellyfin](https://jellyfin.org/)，[Kodi](https://kodi.tv/) 等媒体中心软件或播放器。

通常所用的刮削器例如，[The Movie DB](https://www.themoviedb.org)、[The TV DB](https://thetvdb.com/)，采用将所有非 TV
剧集（OVA，总集篇，特别篇）不加区分放在 S00 中，这对 OVA 经常自成系列的日本动画并不友好，
特别体现在[《物语系列》](https://www.themoviedb.org/tv/46195/season/0)这样的动画中。AniDB 将其进行了区分，
但数据只有英文版本。

因此 dantalian 采用了 bangumi 作为源来生成 nfo 文件，在 Jellyfin, Kodi 这样的媒体中心软件中，
nfo 文件的优先级大于刮削器。不过 MediaCenter 的文件数据库并非 bagnumi 的定位，因此部分数据有缺失。

大体上 dantalian 生成的数据有以下特点：

* 每一季、每个系列的动画单独成条目，仅仅将属于某一季的特别篇列合并在当季里
* 偏向使用角色的信息而非演员的信息
* 支持文件夹名称中包含自定义标签
* 缺少分类和标签，缺少单集演员和staff名单。这部分将使用全剧的数据代替
* 缺少单集封面，将有媒体中心自行抓取

## 文件结构

```
<media_root>/
├── ひぐらしのなく頃に 業
│   ├── tvshow.nfo
│   ├── ひぐらしのなく頃に 業 01.mp4
│   ├── ひぐらしのなく頃に 業 01.nfo
│   ├── ひぐらしのなく頃に 業 02.mp4
│   ├── ひぐらしのなく頃に 業 02.nfo
│   ├── ひぐらしのなく頃に 業 03.mp4
│   ├── ひぐらしのなく頃に 業 03.nfo
│   ├── ひぐらしのなく頃に 業 04.mp4
│   └── ひぐらしのなく頃に 業 04.nfo
├── 化物語 [2009][BDRip]
│   ├── tvshow.nfo
│   ├── 化物語 01.mp4
│   ├── 化物語 01.nfo
│   ├── 化物語 02.mp4
│   ├── 化物語 02.nfo
│   ├── 化物語 SP5_5.mp4
│   └── 化物語 SP5_5.nfo
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

将同一个动画条目的所有文件放在同于个文件夹中，并将这些文件夹放在同一个媒体库文件夹（`media_root`）下。
文件夹的命名遵守如下形式：

```
动画名 [自定义tag][tag1][tag2]
```

请于 bangumi.tv （或使用 `dantalian bgm search <keyword>`）确认动画名称，
可以使用中文或者日文名。剧集的文件遵守如下的形式：

```
与文件夹相同的动画名 12.mp4
```

特别篇在数字前加 `SP`, 如果有小数，将小数点写作 `_`：

```
化物语 SP5_5
```

剧集数字前可以加 0，也可以不加。

## 生成 nfo 文件

使用如下命令即可生成 nfo 文件：

```
dantalian gen --root <medie_root_path>
```

伴随文件的使用和播放，媒体中心软件可能修改 nfo 文件的内容存储动态数据，因此 dantalian 在检测到 nfo 文件后，
将跳过对应文件。如果想强制重新生成的话，可以添加选项 `--force <动画名>`，`--root` 和 `--force` 选项都可以重复：

```
dantalian gen --root <path1> --root <path2> --force 化物语
```

## Roadmap

- [ ] 动画电影/剧场版
- [ ] BD 文件
- [ ] DVD 文件
- [ ] 自定义文件名或模糊匹配
