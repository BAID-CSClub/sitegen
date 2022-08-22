# SiteGen - 页面生成器

## Usage

For simplify, this tool has *no* subcommands. The only function is to generate a static site from articles.

This tool *do* need some arguments:

- `--frontend <REPO>:<BRANCH>`: The repo of the frontend project, by default is: `https://github.com/BAID-CSClub/baid-website-next.git:build`

- `--out <DIR>`: The output dir, by default is: `./dist`

- `--articles <DIR>`: The articles dir, by default is: `./`

## 流程

1. 下载存储库至 *output dir*

2. 遍历 *articles dir* 并解析文章

3. 构建文章的静态文件依赖树，处理后全部放到 `<OUTPUT>/assets/` 里面

4. 在 *output dir* 和 *articles dir* 中寻找 `routes.toml`

5. 构建静态路由，为 `<OUTPUT>/index.html` 添加元数据

6. 生成文章目录结构，生成文章元数据

7. 生成 `sitemap.xml`

## 注意事项

- 所有图片文件会被转换为 `webp` 格式