use mac_spotlight_setter;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spotlight = mac_spotlight_setter::get_spotlight(Some(1), Some("KR"), Some("ko-KR")).await?;

    let files = mac_spotlight_setter::download_spotlight(&spotlight).await?;

    for file in &files {
        println!("Downloaded: {}", file);
    }

    mac_spotlight_setter::set_wallpaper(&files[0]).await?;
    Ok(())
}
