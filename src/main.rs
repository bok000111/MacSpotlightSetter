use mac_spotlight_setter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 && args[1] == "install" {
        // 이전에 생성된 plist 파일을 삭제합니다.
        mac_spotlight_setter::uninstall().ok();
        // plist 파일을 생성합니다.
        mac_spotlight_setter::install()?;
    } else if args.len() > 1 && args[1] == "uninstall" {
        // plist 파일을 삭제합니다.
        mac_spotlight_setter::uninstall()?;
        println!("Spotlight setter uninstalled successfully.");
    } else {
        // TODO: 지역 설정을 변경할 수 있도록 인자를 받아서 처리할 수 있도록 수정
        let spotlights = mac_spotlight_setter::get_spotlight(Some(1), Some("KR"), Some("ko-KR"))?;
        let spotlight = &spotlights[0];
        let downloaded = mac_spotlight_setter::download_spotlight(&spotlight)?;

        mac_spotlight_setter::set_wallpaper(&downloaded)?;
    }
    Ok(())
}
