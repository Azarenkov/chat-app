use std::time::Duration;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

pub async fn connect(db_env: &str) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(db_env).await?;

    client_options.server_selection_timeout = Option::from(Duration::from_secs(4));

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(client)
}
