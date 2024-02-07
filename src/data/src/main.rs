use data::run;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenvy::var("DATABASE_URL").unwrap();
    run(&database_uri).await;
}
