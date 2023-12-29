mod bot;
mod server;

#[tokio::main]
async fn main() {
    loop {
        tokio::join!(
            crate::bot::start(),
            crate::server::start()
        );
    }
}
