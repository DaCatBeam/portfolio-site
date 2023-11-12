
use axum::{routing, Router, response::Html};
use std::error::Error;
use std::net::SocketAddr;

pub async fn run(socket_address: SocketAddr) -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/", routing::get(root_handler));

    axum::Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root_handler() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <div>
                    Hello World!
                </div>
            </body>
        </html>
        "#,
    )
}
