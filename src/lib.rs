use sea_orm::DbConn;
use tide::{Request, Response, Server, StatusCode};

#[derive(Clone)]
pub struct State {
    db: DbConn,
}

pub async fn create_app(db: DbConn) -> Server<State> {
    let state = State { db };
    let mut app = Server::with_state(state.clone());
    app.at("/health_check").get(health_check);
    app
}

async fn health_check(_req: Request<State>) -> tide::Result {
    let res = Response::new(StatusCode::Ok);
    Ok(res)
}
