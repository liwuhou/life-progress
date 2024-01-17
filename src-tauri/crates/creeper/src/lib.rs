extern crate dirs;

use anyhow::Result;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs::{self, metadata, File};
use std::io::Write;
use std::path::PathBuf;

const FETCH_URL: &str = "https://en.wikipedia.org/wiki/List_of_countries_by_life_expectancy";

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryInfo {
    pub all: f32,
    pub male: f32,
    pub female: f32,
}

fn ensure_tmp_exist() -> Result<()> {
    if let Some(tmp_path) = get_tmp_file_path().parent() {
        fs::create_dir_all(tmp_path)?;
    }
    Ok(())
}

fn get_tmp_file_path() -> PathBuf {
    let home_dir = dirs::home_dir().or(Some(PathBuf::from("."))).unwrap();

    home_dir
        .join(".config")
        .join("live_progress")
        .join(".tmp_expectancy.json")
}

fn set_tmp_file_path(content: &HashMap<String, CountryInfo>) -> Result<()> {
    ensure_tmp_exist()?;
    let mut output = File::create(get_tmp_file_path())?;
    output.write_all(serde_json::to_string(content)?.as_bytes())?;

    Ok(())
}

fn has_cache() -> Result<bool> {
    if let Ok(metadata) = metadata(get_tmp_file_path()) {
        return Ok(metadata.is_file());
    }

    Ok(false)
}

pub fn get_data() -> Result<HashMap<String, CountryInfo>> {
    match has_cache() {
        Ok(true) => {
            let json = fs::read_to_string(get_tmp_file_path())?;
            Ok(serde_json::from_str::<HashMap<String, CountryInfo>>(&json)?)
        }
        _ => {
            if let Ok(hashmap) = fetch() {
                set_tmp_file_path(&hashmap)?;
                Ok(hashmap)
            } else {
                // Network error, use default expectancy data
                Ok(receive_default_expectancy()?)
            }
        }
    }
}

fn receive_default_expectancy() -> Result<HashMap<String, CountryInfo>> {
    let default_expectancy_path = env::current_dir()?.join("default_expectancy.json");
    let json = fs::read_to_string(default_expectancy_path)?;
    Ok(serde_json::from_str::<HashMap<String, CountryInfo>>(&json)?)
}

fn fetch() -> Result<HashMap<String, CountryInfo>> {
    let mut result: HashMap<String, CountryInfo> = HashMap::new();
    let resp = reqwest::blocking::get(FETCH_URL)?.text()?;
    let document = Document::from(resp.as_str());
    if let Some(target_table) = document.find(Class("wikitable")).nth(3) {
        let tbody = target_table.find(Name("tbody")).next().unwrap();
        println!("{:?}", tbody);

        for tr in tbody.find(Name("tr")).skip(2) {
            let mut tds = tr.find(Name("td")).take(4);
            if let Some(country_name) = extract_country_name(tds.next()) {
                let all = tds.next().unwrap().text().trim().parse::<f32>()?;
                let male = tds.next().unwrap().text().trim().parse::<f32>()?;
                let female = tds.next().unwrap().text().trim().parse::<f32>()?;
                result.insert(country_name, CountryInfo { all, male, female });
            }
        }
    }

    Ok(result)
}

fn extract_country_name(node: Option<Node>) -> Option<String> {
    if let Some(node) = node {
        Some(node.find(Name("a")).next().unwrap().text().to_string())
    } else {
        None
    }
}
