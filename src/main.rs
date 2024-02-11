use axum::{
    extract::{Query, State},
    routing::post,
    Router,
};
pub mod configuration;
pub mod render;

use serenity::prelude::*;
use serenity::builder::{CreateAttachment, CreateMessage};
use serenity::model::id::ChannelId;
use std::collections::HashMap;
use std::error::Error;
use render::get_screenshot;
use configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = configuration::read_config()?;
    /*let data = fs::read_to_string("test.html")?;
    let bytes = get_screenshot(data).await?;*/
    let app = Router::new()
        .route("/", post(submit))
        // provide the state so the router can access it
        .with_state(settings.clone());

    let bindaddr = format!("0.0.0.0:{}",settings.port);
    
    let listener = tokio::net::TcpListener::bind(&bindaddr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    return Ok(());
}

async fn submit(Query(params): Query<HashMap<String, String>>, State(state): State<Configuration>, body: String ) {
    let image = get_screenshot(body).await.unwrap();
    //params.get
    let token = state.discord_token;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let client = Client::builder(token, intents).await.unwrap();
    let cid = ChannelId::from(state.discord_channel);
    let ckey = match params.get("ckey") {
        Some(x) => x.clone(),
        None => "Unknown".to_string()
    };
    let name = match params.get("name") {
        Some(x) => x.clone(),
        None => "Unknown".to_string()
    };
    cid.send_message(client.http,CreateMessage::new().content(format!("Fax from {}/({})",name,ckey)).add_file(CreateAttachment::bytes(image,"fax.png"))).await.unwrap();
    //client.http.send_message()
}