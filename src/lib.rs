const SPOTLIGHT_ENDPOINT: &str = "https://fd.api.iris.microsoft.com/v4/api/selection";
const SPOTLIGHT_API_PLACEMENT: &str = "88000820";
const SPOTLIGHT_API_BCOUNT: i32 = 1;
const SPOTLIGHT_API_COUNTRY: &str = "US";
const SPOTLIGHT_API_LOCALE: &str = "en-US";
const SPOTLIGHT_API_FORMAT: &str = "json";

#[derive(Debug, serde::Deserialize)]
struct SpotlightResponse {
    batchrsp: BatchResponse,
}

#[derive(Debug, serde::Deserialize)]
struct BatchResponse {
    items: Vec<SpotlightItem>,
}

#[derive(Debug, serde::Deserialize)]
struct SpotlightItem {
    #[serde(rename = "item")]
    raw_ad: String,
}

#[derive(Debug, serde::Deserialize)]
struct Image {
    pub asset: String,
}

#[derive(Debug, serde::Deserialize)]
struct Ad {
    #[serde(rename = "landscapeImage")]
    pub landscape_image: Image,
    // #[serde(rename = "portraitImage")]
    // pub portrait_image: Image,
    #[serde(rename = "iconHoverText")]
    pub icon_hover_text: String,
    pub copyright: String,
}

#[derive(Debug, serde::Deserialize)]
struct SpotlightJson {
    pub ad: Ad,
}

#[derive(Debug, serde::Deserialize)]
pub struct Spotlight {
    pub landscape: String,
    // pub portrait: String,
    pub description: String,
    pub copyright: String,
}

impl From<Ad> for Spotlight {
    fn from(ad: Ad) -> Self {
        Self {
            landscape: ad.landscape_image.asset,
            // portrait: ad.portrait_image.asset,
            description: ad.icon_hover_text,
            copyright: ad.copyright,
        }
    }
}

pub async fn get_spotlight(
    count: Option<i32>,
    country: Option<&str>,
    locale: Option<&str>,
) -> Result<Vec<Spotlight>, Box<dyn std::error::Error>> {
    let count = count.unwrap_or(SPOTLIGHT_API_BCOUNT);
    let country = country.unwrap_or(SPOTLIGHT_API_COUNTRY);
    let locale = locale.unwrap_or(SPOTLIGHT_API_LOCALE);

    let url = format!(
        "{}?placement={}&bcnt={}&country={}&locale={}&fmt={}",
        SPOTLIGHT_ENDPOINT, SPOTLIGHT_API_PLACEMENT, count, country, locale, SPOTLIGHT_API_FORMAT
    );

    let response: SpotlightResponse = reqwest::get(&url).await?.json().await?;
    let raw_ads = response
        .batchrsp
        .items
        .into_iter()
        .map(|item| item.raw_ad)
        .collect::<Vec<String>>();
    let spotlights = raw_ads
        .into_iter()
        .map(|raw_ad| {
            let inter: SpotlightJson = serde_json::from_str(&raw_ad).unwrap();

            Spotlight::from(inter.ad)
        })
        .collect();

    Ok(spotlights)
}

pub async fn download_spotlight(
    spotlights: &Vec<Spotlight>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if tokio::fs::metadata("/tmp/spotlight").await.is_err() {
        tokio::fs::create_dir("/tmp/spotlight").await?;
    }

    let mut spotlight_files = vec![];

    for spotlight in spotlights {
        let landscape = reqwest::get(&spotlight.landscape).await?.bytes().await?;

        let landscape_filename = std::path::Path::new(&spotlight.landscape)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        tokio::fs::write(format!("/tmp/spotlight/{}", landscape_filename), landscape).await?;

        spotlight_files.push(format!("/tmp/spotlight/{}", landscape_filename));
    }

    Ok(spotlight_files)
}

pub async fn set_wallpaper(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let apple_script = format!(
        r#"tell application "System Events" to set picture of every desktop to POSIX file "{}""#,
        path
    );

    // osascript를 호출해서 배경화면을 설정
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(&apple_script)
        .output()?;

    if output.status.success() {
        println!("Wallpaper successfully set to: {}", path);
    } else {
        eprintln!(
            "Failed to set wallpaper. Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
