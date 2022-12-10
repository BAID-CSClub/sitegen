use std::io::Read;
use std::{fs, path::Path, path::PathBuf};
use std::collections::HashMap;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    description: String,
    id: String,
    time: String,
}

#[derive(Debug, Clone)]
struct SubArticle {
    path: PathBuf,
    description: String,
    title: String,
    id: String,
    time: String,
}

impl SubArticle {
    fn from_frontmatter(fm: Frontmatter, path: PathBuf) -> Self {
        Self {
            path,
            description: fm.description,
            title: fm.title,
            id: fm.id,
            time: fm.time,
        }
    }
}

#[derive(Debug)]
struct Article {
    id: String,
    year: String,
    month: String,
    day: String,
    cn: SubArticle,
    en: SubArticle
}

fn parse_frontmatter(path: &Path) -> anyhow::Result<SubArticle> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines = contents.lines();
    let mut frontmatter = String::new();
    let mut in_frontmatter = false;
    loop {
        let line = lines.next().unwrap_or_default();
        if line == "---" {
            if in_frontmatter {
                break;
            } else {
                in_frontmatter = true;
                continue;
            }
        }
        if in_frontmatter {
            frontmatter.push_str(&format!("{}\n", line));
        }
    }
    let fm: Frontmatter = serde_yaml::from_str(&frontmatter)?;
    Ok(SubArticle::from_frontmatter(fm, path.to_path_buf()))
}

fn find_all_articles(articles_dir: &Path) -> anyhow::Result<Vec<Article>> {
    let mut map: HashMap<String, HashMap<String, SubArticle>> = HashMap::new(); // { 'id': { 'cn': SubArticle, 'en': SubArticle } }
    for entry in fs::read_dir(articles_dir)? {
        // Check if file name matches <...>.<cn|en>.md
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let mut parts = file_name.split('.');
        parts.next().unwrap_or_default();
        let lang = parts.next().unwrap_or_default();
        if lang != "cn" && lang != "en" {
            continue;
        }
        if path.extension().unwrap_or_default() != "md" {
            continue;
        }
        let config = parse_frontmatter(&path)?;
        // Write to map
        let id = config.id.clone();
        let sub_map = map.entry(id).or_insert_with(HashMap::new);
        sub_map.insert(lang.to_string(), config);
    }


    let mut articles = vec![];
    for (id, sub_map) in map {
        let cn = sub_map.get("cn").unwrap();
        let en = sub_map.get("en").unwrap();
        let mut parts = cn.path.file_name().unwrap().to_str().unwrap().split('.');
        let year = parts.next().unwrap_or_default().to_string();
        let month = parts.next().unwrap_or_default().to_string();
        let day = parts.next().unwrap_or_default().to_string();
        articles.push(Article {
            id,
            year,
            month,
            day,
            cn: cn.to_owned(),
            en: en.to_owned(),
        });
    }

    Ok(articles)
}

fn transform_single(article: &Article, out: &Path) -> anyhow::Result<()> {
    // Article -> <out>/zh-CN,en-US/articles/:year/:month/:day/:id
    fn transform(article: &Article, out: &Path, lang: &str) -> anyhow::Result<()> {
        let html_dir = out
            .join(if lang == "cn" { "zh-CN" } else { "en-US" })
            .join("articles")
            .join(&article.year)
            .join(&article.month)
            .join(&article.day);
        fs::create_dir_all(html_dir);

        let sub_config = if lang == "cn" {
            &article.cn
        } else {
            &article.en
        };

        // Create html file
        // TODO: Create /<out>/articles/:id.json, which contains the article's metadata
        // TODO: Compile markdown to html, finds all static files, and copy them to /<out>/articles/:id/
        // TODO: Create /<out>/articles/:id.html, which contains compiled markdown


        Ok(())
    }
    transform(article, out, "cn")?;
    transform(article, out, "en")
}

pub fn build(articles: &Path, out: &Path) -> anyhow::Result<()> {
    let articles = find_all_articles(articles)?;
    for article in articles {
        transform_single(&article, out)?;
    }

    Ok(())
}

#[test]
fn test_parse_articles() {
    build(Path::new("./articles"), Path::new("./dist")).unwrap();
}
