use sea_orm::{ConnectOptions, Database};
use tide_sea_orm_example::create_app;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut opts = ConnectOptions::new("postgres://tide:tide_pass@localhost:5432/tide_db".into());
    opts.min_connections(5).max_connections(100);
    let db = Database::connect(opts)
        .await
        .expect("Failed to connect to the database");
    let app = create_app(db).await;
    app.listen("127.0.0.1:4000").await?;
    Ok(())
}
