# SiteGen - 页面生成器

## Usage

For simplify, this tool has *no* subcommands. The only function is to generate a static site from articles.

This tool *do* need some arguments:

- `--frontend <REPO>:<BRANCH>`: The repo of the frontend project, by default is: `https://github.com/BAID-CSClub/baid-website-next.git:build`

- `--out <DIR>`: The output dir, by default is: `./dist`

- `--articles <DIR>`: The articles dir, by default is: `./`

## 流程

1. 下载存储库至 *output dir*

下面的步骤会同时执行：

> 1. 遍历 *articles dir* 并解析文章
> 2. 构建文章的静态文件依赖树，处理后全部放到 `<OUTPUT>/assets/` 里面
> 3. 生成文章元数据（`<OUTPUT>/articles/<id>.json`），编译文章（`<OUTPUT>/articles/<id>.html`）

---

> 1. 在 *output dir* 和 *articles dir* 中寻找 `routes.toml`
> 2. 构建静态路由，为模板添加元数据
> 3. 写入 `<OUTPUT>/index.html`

最后的步骤：

1. 生成 `sitemap.xml`

## 注意事项

- 所有图片文件会被转换为 `webp` 格式，并保留原始格式（命名相同）

- 目前仅支持 `jpeg` 和 `png` 格式的图片，其他格式的图片不会被转换（但是可以显示）

- 所有静态文件的命名方式为：`<ORIGIN_FILE_HASH>.<EXT>`（因此不会有重复的文件）

## routes.toml

```toml
[global]

[global.zh-CN]
title_prefix = "北京中学国际部 - "
title_suffix = " - 后缀示例"

[global.en-US]
title_prefix = "Baid Website - "
title_suffix = " - Suffix Example"

[[routes]]
path = "/index.html"
robots = "index, follow"

[routes.zh-CN]
title = "首页"
description = "这是一个首页"

[routes.en-US]
title = "Home"
description = "This is a home page"

# More routes...
```
