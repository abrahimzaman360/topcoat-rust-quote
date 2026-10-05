use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, component, view},
};

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<String>,
}

#[tokio::main]
async fn main() {
    topcoat::start(module_router!().build()).await.unwrap();
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <body style="
                background-color: #FFFF00; // yellow
                font-family: Arial, sans-serif;
            ">
                fetch_quote()
            </body>
        </html>
    })
}

#[component]
async fn fetch_quote() -> Result<impl View> {
    let client = Client::new();
    let response: ApiResponse = client
        .get("https://meowfacts.herokuapp.com/")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(view! { <h2>"Quote:"</h2> <p>(response.data[0].as_str())</p> })
}
