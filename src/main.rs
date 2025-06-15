mod gemini;

#[tokio::main]
async fn main() {
	gemini::main().await;
}
