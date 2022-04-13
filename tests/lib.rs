use tide_sea_orm_example::create_app;
use sea_orm::{MockDatabase, DatabaseBackend};
use tide::StatusCode;
use tide_testing::TideTestingExt;

#[async_std::test]
async fn test_health_check() -> tide::Result<()> {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let app = create_app(db).await;
    assert_eq!(app.get("/health_check").await?.status(), StatusCode::Ok);
    Ok(())
}
