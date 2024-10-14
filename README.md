# Mac Spotlight Background Setter

Windows 10/11에서 제공하는 스포트라이트 배경화면을 자동으로 다운로드하고, macOS에서 배경화면으로 설정해주는 프로그램입니다.

## 기능

- **Windows 스포트라이트 이미지 다운로드**: Microsoft API를 통해 스포트라이트 이미지를 다운로드.
- **macOS 배경화면 자동 설정**: 다운로드한 이미지를 macOS의 모든 데스크탑 배경화면으로 자동 설정.
- **자동화**: 주기적으로 배경화면을 갱신할 수 있도록 자동화.

## 사용법

1. **최신 릴리즈에서 바이너리와 스크립트 다운로드**:

    최신 릴리즈 페이지에서 macOS용 바이너리 파일과 자동화 설정에 필요한 `plist` 파일 및 스크립트를 다운로드하세요.

    [Latest Release](https://github.com/your-username/mac-spotlight-setter/releases/latest)

2. **설정 스트립트 실행**:

    mac_spotlight_setter와 set_plist.sh 파일을 같은 디렉토리에 다운로드하고, set_plist.sh 파일을 실행하세요.

    ```bash
    ./set_plist.sh
    ```

## Acknowledgements

- This project utilizes the Windows Spotlight API endpoint discovered and analyzed by [ORelio](https://github.com/ORelio) as part of the [Spotlight Downloader](https://github.com/ORelio/Spotlight-Downloader) project.
- APIv4 analysis conducted by ORelio. Endpoint found through the analysis of network traffic originating from a Windows 11 virtual machine.
- Special thanks to ORelio and the Spotlight Downloader project. If you are using this documentation or the API discovery in your own project, please credit ORelio and the Spotlight Downloader project accordingly.