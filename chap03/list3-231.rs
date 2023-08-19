async fn http_get(url: &str) -> Resutl<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = http_get("https://rust-lang.org/ja").await?;
    println!("body = {}", body);
    Ok(())
}
