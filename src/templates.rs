use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub title: &'a str,
    pub spotify_widget: String,
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
}
#[derive(Template)]
#[template(path = "admin/experience/new.html")]
pub struct AdminExperienceNewTemplate<'a> {
    pub title: &'a str,
    pub error: Option<&'a str>,
    pub e_type: Vec<&'a str>,
}
