use std::{
    process::{Command, ExitCode, ExitStatus},
    sync::Arc,
};

use actix_session::Session;
use actix_web::web;
use askama::Template;
use async_sqlite::Pool;
use reqwest::StatusCode;

use crate::{db, templates::ProjectPartTemplate};

#[macro_export]
macro_rules! ternary {
    ($condition: expr => $true_expr: expr , $false_expr: expr) => {
        if $condition {
            $true_expr
        } else {
            $false_expr
        }
    };
}

pub async fn fetch_spotify_endpoint() -> reqwest::Result<String> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    let api_update_token =
        std::env::var("API_UPDATE_TOKEN").unwrap_or_else(|_| "beans".to_string());
    let client = reqwest::Client::new();
    let http_req = client
        .get(format!("http://{host}:{port}/api/spotify"))
        .bearer_auth(api_update_token)
        .send()
        .await?;
    Ok(match http_req.status() {
        StatusCode::OK => http_req.text().await.unwrap(),
        _ => String::new(),
    })
}

pub async fn verify_admin_authentication(
    session: &Session,
    pool: &web::Data<Arc<Pool>>,
) -> Result<bool, anyhow::Error> {
    let mut session_id = match session.get::<String>("session_id") {
        Ok(Some(id)) => id,
        _ => {
            log::debug!("Missing Session token");
            return Ok(false);
        }
    };
    log::debug!("ID: {}", session_id);
    if db::AdminSession::verify(&pool, session_id).await? {
        log::debug!("Authenticated");
        return Ok(true);
    } else {
        log::debug!("Unknown Session");
        return Ok(false);
    };
}

pub async fn calculate_experience_tree(
    pool: &web::Data<Arc<Pool>>,
) -> Result<String, async_sqlite::Error> {
    log::info!("Calculating Experience");
    let mut records = crate::db::Experience::all(&pool, true).await?;
    log::debug!("Records: {:#?}", records);
    records.sort_unstable_by_key(|item| (item.start_date.clone(), item.end_date.clone()));
    log::debug!("Sorted Records: {:#?}", records);
    let mut sorted = OrganisedExperience { company: vec![] };

    for record in records {
        let mut attached = false;
        for company in &mut sorted.company {
            if company.company == record.company {
                attached = true;
                company.experience.push(record.clone());
                company
                    .experience
                    .sort_unstable_by_key(|item| (item.start_date.clone(), item.end_date.clone()));
                log::debug!("Sorted Records (per push): {:#?}", company.experience);
                break;
            }
        }
        if !attached {
            sorted.company.push(OrganizedExperienceCompany {
                company: record.company.clone(),
                experience: vec![record.clone()],
                e_type: ternary!(record.e_type == "Education".to_string() => "Education".to_string(), "Work".to_string()),
            });
        }
    }
    Ok(serde_json::to_string(&sorted).unwrap())
}

pub async fn render_project_tree(pool: &web::Data<Arc<Pool>>) -> String {
    let records = db::Project::all(&pool).await.unwrap();
    let mut active_records: Vec<db::Project> = vec![];
    let mut technologies: Vec<(String, i64)> = vec![];
    for record in records {
        for tech in record.clone().technologies {
            match technologies.binary_search_by_key(&tech, |&(ref tech, _count)| tech.to_string()) {
                Ok(index) => {
                    log::debug!("Incremented Technology");
                    technologies[index].1 += 1;
                }
                Err(index) => {
                    log::debug!("Saved new technology");
                    technologies.insert(index, (tech, 1));
                }
            }
        }
        if record.favourite {
            active_records.push(record);
        }
    }
    let html = ProjectPartTemplate {
        technologies,
        records: active_records,
    }
    .render()
    .expect("template should be valid");

    html
}

pub fn run_station_parser() {
    let db_url = std::env::var("DB_URL").unwrap_or_else(|_| String::from("./db.sqlite3"));
    let toc_api_key = std::env::var("NR_TOC_API_KEY").unwrap_or_else(|_| String::from(""));
    let station_api_key = std::env::var("NR_STATION_API_KEY").unwrap_or_else(|_| String::from(""));
    let parser_dist = std::env::var("NR_STATION_PARSER_DIST")
        .unwrap_or_else(|_| String::from("./dist/nr-station-parser"));

    Command::new(parser_dist)
        .env("DB_URL", db_url)
        .env("NR_TOC_API_KEY", toc_api_key)
        .env("NR_STATION_API_KEY", station_api_key)
        .status()
        .expect("Command has executed");
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct OrganisedExperience {
    pub company: Vec<OrganizedExperienceCompany>,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct OrganizedExperienceCompany {
    pub company: String,
    pub experience: Vec<crate::db::Experience>,
    pub e_type: String,
}
