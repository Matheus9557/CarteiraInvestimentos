use askama::Template;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard {
    pub name: String,
    pub balance: f64,
}
