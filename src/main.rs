#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pkg::app::AuthJwt::run().await
}
