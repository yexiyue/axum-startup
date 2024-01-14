use mongodb::Database;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::MongoDb] db: Database) -> shuttle_axum::ShuttleAxum {
    let router=shuttle_mongodb::start(db).await;
    Ok(router.into())
}
