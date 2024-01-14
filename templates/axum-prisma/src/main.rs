#[tokio::main]
async fn main() {
    axum_prisma::start().await.unwrap();
}
