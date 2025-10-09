use crate::{db, utils::OrganizedExperienceCompany};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub title: &'a str,
    pub spotify_widget: String,
    pub experiences: Vec<OrganizedExperienceCompany>,
    pub education: Vec<OrganizedExperienceCompany>,
    pub project: String,
    pub guestlog: String,
}

#[derive(Template)]
#[template(path = "parts/spotify.html")]
pub struct SpotifyPartTemplate {
    pub is_playing: bool,
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub album_image_url: String,
    pub song_url: String,
    pub device: String,
}

#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct AdminLoginTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "admin/experience/list.html")]
pub struct AdminExperienceListTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
    pub experiences: Vec<db::Experience>,
}
#[derive(Template)]
#[template(path = "admin/experience/new.html")]
pub struct AdminExperienceNewTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
    pub e_type: Vec<&'a str>,
}
#[derive(Template)]
#[template(path = "admin/experience/edit.html")]
pub struct AdminExperienceEditTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
    pub e_type: Vec<&'a str>,
    pub experience: db::Experience,
}

#[derive(Template)]
#[template(path = "admin/index.html")]
pub struct AdminOptionsTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "admin/project/list.html")]
pub struct AdminProjectListTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
    pub projects: Vec<db::Project>,
}
#[derive(Template)]
#[template(path = "admin/project/new.html")]
pub struct AdminProjectNewTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "admin/project/edit.html")]
pub struct AdminProjectEditTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
    pub project: db::Project,
}

#[derive(Template)]
#[template(path = "parts/project.html")]
pub struct ProjectPartTemplate {
    pub technologies: Vec<(String, i64)>,
    pub records: Vec<db::Project>,
}

#[derive(Template)]
#[template(path = "parts/guestlog.html")]
pub struct GuestlogPartTemplate {
    pub guestlogs: Vec<db::Guestlog>,
}

#[derive(Template)]
#[template(path = "admin/guestlog/list.html")]
pub struct AdminGuestlogListTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
    pub guestlogs: Vec<db::Guestlog>,
}
