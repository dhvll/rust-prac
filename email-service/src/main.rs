use email_service::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
