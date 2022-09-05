use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use serde::Deserialize;

use crate::accountbook::{AccountBook, Summary};
use crate::db::*;
use crate::item::Item;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    items: Vec<Item>,
}

#[derive(Template)]
#[template(path = "summary.html")]
struct SummaryTemplate {
    summaries: Vec<Summary>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FormInput {
    category: String,
    price: i32,
}

struct HtmlTemplate<T>(T);

pub async fn index() -> impl IntoResponse {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut book = AccountBook::new(make_db_conn(&database_url));
    let items = book.fetch_items(10).unwrap();
    let template = IndexTemplate { items };
    HtmlTemplate(template)
}

pub async fn save(Form(input): Form<FormInput>) -> impl IntoResponse {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut book = AccountBook::new(make_db_conn(&database_url));
    let res = book.add_item(&input.category.as_str(), input.price);
    match res {
        Ok(_) => Redirect::to("/").into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save item. Error: {}", err),
        )
            .into_response(),
    }
}
pub async fn summary() -> impl IntoResponse {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut book = AccountBook::new(make_db_conn(&database_url));
    let res = book.get_summaries().unwrap();
    let template = SummaryTemplate { summaries: res };
    HtmlTemplate(template)
}

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => {
                return Html(html).into_response();
            }
            Err(err) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template. Error: {}", err),
                )
                    .into_response();
            }
        }
    }
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "not found").into_response()
}
