use actix_web::{
    get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use zlib_searcher::{Book, Searcher};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Clone)]
struct AppState {
    searcher: Arc<Searcher>,
}

impl AppState {
    pub fn init() -> Self {
        info!("AppState init!");
        AppState {
            searcher: Arc::new(Searcher::new()),
        }
    }
}

fn default_limit() -> usize {
    30
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
    #[serde(default = "default_limit")]
    limit: usize,
}

#[derive(Serialize)]
struct SearchResult {
    books: Vec<Book>,
}

#[get("/search")]
async fn search(query: web::Query<SearchQuery>, state: web::Data<AppState>) -> impl Responder {
    let books = state.searcher.search(&query.query, query.limit);
    let result = SearchResult { books };

    return HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .json(result);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("zlib-searcher started!");
    let app_state = AppState::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(index)
            .service(search)
    })
    .bind(("127.0.0.1", 7070))?
    .run()
    .await
}