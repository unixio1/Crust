use crust::args::parse_arguments;

#[tokio::main] 
async fn main() {
    let scanner = parse_arguments();
    scanner.scan().await;
}
