use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;

mod podcasts;
use podcasts::read_podcasts_from_xml;
use crate::podcasts::Podcast;

type AppState = Arc<Vec<Podcast>>;

async fn root(
    State(app_state): State<AppState>
) -> impl IntoResponse {
    let response = format!(
        r#"
<html>
    <head>
        <title>My Podcasts</title>
    </head>
    <body>
        <h1>My Podcasts</h1>
        <ul>
            {}
        </ul>
    </body>
</html>
    "#,
        app_state
            .iter()
            .enumerate()
            .map(|(id, podcast)| {
                format!(r#"<li><a href="/{}">{}</a></li>"#, id, podcast.title)
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
    Html(response)
}

async fn podcast(
    State(app_state): State<AppState>, Path(id): Path<usize>
) -> impl IntoResponse {
    let podcast = app_state.get(id);
    Html(match podcast {
        Some(podcast) => podcast.to_html(),
        None => "No podcast found".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let podcasts =
        read_podcasts_from_xml("https://feeds.megaphone.fm/darknetdiaries").await.expect("Cant fetch podcasts");
    let app_state = Arc::new(podcasts);
    let app = Router::new()
        .route("/", get(root))
        .route("/:id", get(podcast))
        .with_state(app_state);

    let addr = SocketAddr::new([0, 0, 0, 0].into(), 3000);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    //use super::*;


}