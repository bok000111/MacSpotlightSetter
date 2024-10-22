use miniserde::{json, Deserialize};
use std::{
    env, eprintln, error::Error, format, fs, io::Write, path::PathBuf, println, process::Command,
};

pub fn get_spotlight(
    count: Option<i32>,
    country: Option<&str>,
    locale: Option<&str>,
) -> Result<Vec<Spotlight>, Box<dyn Error>> {
    let count = count.unwrap_or(SPOTLIGHT_API_BCOUNT);
    let country = country.unwrap_or(SPOTLIGHT_API_COUNTRY);
    let locale = locale.unwrap_or(SPOTLIGHT_API_LOCALE);

    let url = format!(
        "{}?placement={}&bcnt={}&country={}&locale={}&fmt={}",
        SPOTLIGHT_ENDPOINT, SPOTLIGHT_API_PLACEMENT, count, country, locale, SPOTLIGHT_API_FORMAT
    );

    let resp =
        json::from_str::<SpotlightResponse>(&ureq::get(&url).call()?.into_string()?).unwrap();

    let raw_ads = resp
        .batchrsp
        .items
        .into_iter()
        .map(|item| item.raw_ad)
        .collect::<Vec<String>>();
    let spotlights = raw_ads
        .into_iter()
        .map(|raw_ad| {
            let inter: SpotlightJson = json::from_str(&raw_ad).unwrap();

            Spotlight::from(inter.ad)
        })
        .collect();

    Ok(spotlights)
}

pub fn download_spotlight(spotlight: &Spotlight) -> Result<PathBuf, Box<dyn Error>> {
    let dist = env::temp_dir().join("mac_spotlight_setter");
    if !dist.exists() {
        fs::create_dir_all(&dist)?;
    }
    println!("Downloading spotlight image... {}", spotlight.landscape);
    let resp = ureq::get(&spotlight.landscape).call()?;

    let mut data: Vec<u8> = Vec::with_capacity(1e8 as usize); // 100MB
    resp.into_reader().read_to_end(&mut data)?;

    let filename = spotlight
        .landscape
        .rsplit_once('/')
        .unwrap_or(("", &spotlight.landscape))
        .1;
    let filename = filename.split_once('?').unwrap_or((filename, "")).0;
    let filename = if filename.is_empty() {
        "spotlight.jpg"
    } else if !filename.contains(".jpg") {
        &format!("{}.jpg", filename)
    } else {
        filename
    };
    let filename = dist.join(filename);

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&filename)?;
    f.write_all(&data)?;
    f.flush()?;

    Ok(filename)
}

pub fn set_wallpaper(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let apple_script = format!(
        r#"tell application "System Events" to set picture of every desktop to POSIX file "{}""#,
        path.to_str().unwrap()
    );

    // osascript를 호출해서 배경화면을 설정
    let output = Command::new("osascript")
        .arg("-e")
        .arg(&apple_script)
        .output()?;

    if output.status.success() {
        println!("Wallpaper successfully set to: {}", path.display());
    } else {
        eprintln!(
            "Failed to set wallpaper. Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fs::remove_file(&path).ok();

    Ok(())
}

pub fn install() -> Result<(), Box<dyn Error>> {
    // ~/bin 디렉토리가 없으면 생성
    let bin_dir = dirs_next::home_dir().unwrap().join("bin");
    if !bin_dir.exists() {
        fs::create_dir(&bin_dir)?;
    }

    // ~/bin/mac_spotlight_setter 파일 생성
    let current_exe = env::current_exe()?;
    let dest = bin_dir.join("mac_spotlight_setter");
    fs::copy(&current_exe, &dest)?;

    // ~/Library/LaunchAgents/com.jbok.setwallpaper.plist 파일 생성
    let plist = dirs_next::home_dir()
        .unwrap()
        .join("Library/LaunchAgents/com.jbok.setwallpaper.plist");

    const INTERVAL: i32 = 60 * 60 * 6; // 6시간
    let plist_script = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
            <dict>
                <!-- Label: unique identifier for the job -->
                <key>Label</key>
                <string>com.jbok.setwallpaper</string>

                <!-- ProgramArguments: full path to the shell script -->
                <key>ProgramArguments</key>
                <array>
                    <string>{}</string>
                </array>

                <!-- RunAtLoad: run the script when the agent/daemon is loaded -->
                <key>RunAtLoad</key>
                <true/>

                <!-- KeepAlive: if true, the script will run again if it fails -->
                <key>KeepAlive</key>
                <false/>

                <!-- StartInterval: interval to run the script (in seconds) -->
                <!-- <key>StartInterval</key> -->
                <!-- <integer>{}</integer> -->
                <key>StartCalendarInterval</key>
                <array>
                    <dict>
                        <key>Hour</key>
                        <integer>0</integer>
                        <key>Minute</key>
                        <integer>0</integer>
                    </dict>
                    <dict>
                        <key>Hour</key>
                        <integer>4</integer>
                        <key>Minute</key>
                        <integer>0</integer>
                    </dict>
                    <dict>
                        <key>Hour</key>
                        <integer>8</integer>
                        <key>Minute</key>
                        <integer>0</integer>
                    </dict>
                    <dict>
                        <key>Hour</key>
                        <integer>12</integer>
                        <key>Minute</key>
                        <integer>0</integer>
                    </dict>
                    <dict>
                        <key>Hour</key>
                        <integer>16</integer>
                        <key>Minute</key>
                        <integer>0</integer>
                    </dict>
                    <dict>
                        <key>Hour</key>
                        <integer>20</integer>
                        <key>Minute</key>
                        <integer>0</integer>
                    </dict>
                </array>
            </dict>
        </plist>"#,
        dest.to_str().unwrap(),
        INTERVAL
    );

    fs::write(&plist, plist_script)?;

    // ~/Library/LaunchAgents/com.jbok.setwallpaper.plist 파일 로드
    let output = Command::new("launchctl")
        .arg("bootstrap")
        .arg(format!("gui/{}", get_id()?))
        .arg(&plist)
        .output()?;

    if output.status.success() {
        println!("Spotlight setter installed successfully.");
    } else {
        eprintln!(
            "Failed to install spotlight setter. Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub fn uninstall() -> Result<(), Box<dyn Error>> {
    let plist = dirs_next::home_dir()
        .unwrap()
        .join("Library/LaunchAgents/com.jbok.setwallpaper.plist");

    // ~/Library/LaunchAgents/com.jbok.setwallpaper.plist 파일 언로드
    let _output = Command::new("launchctl")
        .arg("remove")
        .arg("com.jbok.setwallpaper")
        .output()?;

    // ~/Library/LaunchAgents/com.jbok.setwallpaper.plist 파일 삭제
    fs::remove_file(&plist).ok();

    // ~/bin/mac_spotlight_setter 파일 삭제
    let bin = dirs_next::home_dir()
        .unwrap()
        .join("bin/mac_spotlight_setter");
    fs::remove_file(&bin).ok();

    Ok(())
}

fn get_id() -> Result<String, Box<dyn Error>> {
    let id = Command::new("id").arg("-u").output()?;

    Ok(String::from_utf8_lossy(&id.stdout).trim().to_string())
}

#[derive(Debug, Deserialize)]
struct SpotlightResponse {
    batchrsp: BatchResponse,
}
#[derive(Debug, Deserialize)]
struct BatchResponse {
    items: Vec<SpotlightItem>,
}
#[derive(Debug, Deserialize)]
struct SpotlightItem {
    #[serde(rename = "item")]
    raw_ad: String,
}
#[derive(Debug, Deserialize)]
struct Image {
    pub asset: String,
}
#[derive(Debug, Deserialize)]
struct Ad {
    #[serde(rename = "landscapeImage")]
    pub landscape_image: Image,
    #[serde(rename = "iconHoverText")]
    pub icon_hover_text: String,
    pub copyright: String,
}
#[derive(Debug, Deserialize)]
struct SpotlightJson {
    pub ad: Ad,
}
#[derive(Debug, Deserialize)]
pub struct Spotlight {
    pub landscape: String,
    pub description: String,
    pub copyright: String,
}
impl From<Ad> for Spotlight {
    fn from(ad: Ad) -> Self {
        Self {
            landscape: ad.landscape_image.asset,
            description: ad.icon_hover_text,
            copyright: ad.copyright,
        }
    }
}
const SPOTLIGHT_ENDPOINT: &str = "https://fd.api.iris.microsoft.com/v4/api/selection";
const SPOTLIGHT_API_PLACEMENT: &str = "88000820";
const SPOTLIGHT_API_BCOUNT: i32 = 1;
const SPOTLIGHT_API_COUNTRY: &str = "US";
const SPOTLIGHT_API_LOCALE: &str = "en-US";
const SPOTLIGHT_API_FORMAT: &str = "json";
