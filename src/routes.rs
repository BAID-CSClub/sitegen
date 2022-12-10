use serde::Deserialize;
use std::{io::Write, path::Path};

#[derive(Debug, Deserialize)]
struct GlobalConfig {
    title_prefix: String,
    title_suffix: String,
}

#[derive(Debug, Deserialize)]
struct Lang<T> {
    #[serde(rename = "zh-CN")]
    zh: T,
    #[serde(rename = "en-US")]
    en: T,
}

#[derive(Debug, Deserialize)]
struct RouteSubConfig {
    title: String,
    description: String,
    default: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct RouteConfig {
    path: String,
    robots: String,
    #[serde(rename = "zh-CN")]
    zh: RouteSubConfig,
    #[serde(rename = "en-US")]
    en: RouteSubConfig,
}

#[derive(Debug, Deserialize)]
struct Config {
    global: Lang<GlobalConfig>,
    routes: Vec<RouteConfig>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        let config = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }
    pub fn decorate_title(&self, lang: &str, title: &str) -> String {
        let global = &self.global;
        let prefix = if lang == "zh-CN" {
            &global.zh.title_prefix
        } else {
            &global.en.title_prefix
        };
        let suffix = if lang == "zh-CN" {
            &global.zh.title_suffix
        } else {
            &global.en.title_suffix
        };
        format!("{}{}{}", prefix, title, suffix)
    }
}

pub fn minify(template: String) -> anyhow::Result<String> {
    let template = minify_html::minify(template.as_bytes(), &minify_html::Cfg::new());
    String::from_utf8(template).map_err(|e| anyhow::anyhow!(e))
}

pub fn build(routes_path: &Path, out: &Path) -> anyhow::Result<()> {
    let config = Config::load(routes_path)?;

    let template = std::fs::read_to_string(out.join("index.html"))?;

    for route in &config.routes {
        for lang in ["zh-CN", "en-US"] {
            let path = route.path.trim_matches('/');
            let out_path = out.join(lang).join(path);
            // 另一种语言的页面，用于生成link rel="alternate"
            let alternate_link = format!(
                "/{}/{}",
                if lang == "zh-CN" { "en-US" } else { "zh-CN" },
                path
            );

            // Check if the path ends with .html
            if !out_path.extension().eq(&Some(std::ffi::OsStr::new("html"))) {
                anyhow::bail!("The path {} must end with .html", out_path.display());
            }
            std::fs::create_dir_all(out_path.parent().unwrap())?;
            let mut out_file = std::fs::File::create(out_path)?;
            let sub_config = if lang == "zh-CN" {
                &route.zh
            } else {
                &route.en
            };
            let temp = template
                .replace(
                    "{{title}}",
                    config.decorate_title(&lang, &sub_config.title).as_str(),
                )
                .replace("{{description}}", &sub_config.title)
                .replace("{{robots}}", &route.robots)
                .replace("{{lang}}", lang)
                .replace("{{alternate}}", &alternate_link)
                .replace(
                    "{{alternateLang}}",
                    if lang == "zh-CN" { "en-US" } else { "zh-CN" },
                );
            let minified = minify(temp)?.into_bytes();
            if sub_config.default.unwrap_or(false) {
                std::fs::File::create(out.join("index.html"))?.write(&minified)?;
            }
            out_file.write_all(&minified)?;
        }
    }

    Ok(())
}

#[test]
fn test_build_routes() -> anyhow::Result<()> {
    info!("Testing build routes");
    build(Path::new("./articles/routes.toml"), Path::new("./dist"))
}
