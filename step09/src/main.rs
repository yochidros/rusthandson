mod accountbook;
mod error;
mod item;
mod schema;
use std::io::Write;
use std::net::SocketAddr;

use accountbook::*;
use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{routing, Router};
use diesel::{Connection, SqliteConnection};
use item::Item;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_tempalte=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/greet", routing::get(hoge));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // loop {
    //     let n: usize = input_string("n >").parse().unwrap();
    //     match n {
    //         1 => {
    //             let item = input_item();
    //             let mut book = AccountBook::new(make_db_conn(&database_url));
    //             book.add_item(item.0.as_str(), item.1).unwrap();
    //         }
    //         2 => {
    //             let mut book = AccountBook::new(make_db_conn(&database_url));
    //             let items = book.fetch_items(10).unwrap();
    //             show_items(&items);
    //         }
    //         3 => {
    //             let mut book = AccountBook::new(make_db_conn(&database_url));
    //             let summaries = book.get_summaries().unwrap();
    //             show_summries(&summaries);
    //         }
    //         _ => break,
    //     }
    // }
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    items: Vec<Item>,
}

struct HtmlTemplate<T>(T);

async fn hoge() -> impl IntoResponse {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut book = AccountBook::new(make_db_conn(&database_url));
    let items = book.fetch_items(10).unwrap();
    let template = IndexTemplate { items };
    HtmlTemplate(template)
}

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => {
                tracing::debug!("html render");
                return Html(html).into_response();
            }
            Err(err) => {
                tracing::debug!("html error");

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template. Error: {}", err),
                )
                    .into_response();
            }
        }
    }
}

fn make_db_conn(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .expect(&format!("Error connection to {}", database_url))
}

fn input_item() -> (String, i32) {
    let category: String = input_string("category >");
    let price: i32 = input_string("price > ").parse().unwrap();
    (category, price)
}

fn input_string(help_text: &'static str) -> String {
    let mut input = String::new();
    print!("{} ", help_text);
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}

fn show_items(items: &Vec<Item>) {
    println!("===========================");
    for item in items.iter() {
        println!(
            "[{:04}] {}: {}円",
            item.id.unwrap_or(0),
            item.category,
            item.price
        );
    }
    println!("===========================");
}
fn show_summries(summaries: &Vec<Summary>) {
    println!("===========================");
    println!("category\tcount\tsum\tavg");
    for item in summaries.iter() {
        println!(
            "{}\t{}\t{}円\t{:.2}円",
            item.category,
            item.count,
            item.sum,
            item.avg()
        );
    }
    println!("===========================");
}
