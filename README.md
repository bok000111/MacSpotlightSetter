# Mac Spotlight Background Setter

Windows 10/11에서 제공하는 스포트라이트 배경화면을 자동으로 다운로드하고, macOS에서 배경화면으로 설정해주는 프로그램입니다.

## 기능

- **Windows 스포트라이트 이미지 다운로드**: Microsoft API를 통해 스포트라이트 이미지를 다운로드.
- **macOS 배경화면 자동 설정**: 다운로드한 이미지를 macOS의 모든 데스크탑 배경화면으로 자동 설정.
- **자동화**: 주기적으로 배경화면을 갱신할 수 있도록 자동화.

## 사용법

1. **최신 릴리즈에서 아키텍쳐에 맞는 바이너리 다운로드**:

    애플 칩을 사용하는 Mac을 사용하고 있다면, aarch64 아키텍쳐의 바이너리를 다운로드하세요.

    인텔 칩을 사용하는 Mac을 사용하고 있다면, x86_64 아키텍쳐의 바이너리를 다운로드하세요.

    [Latest Release](https://github.com/your-username/mac-spotlight-setter/releases/latest)

2. **바이너리 설치**:

    install 인자를 사용하면 자동으로 plist를 생성하고 설정합니다.
    
    기본값은 4시간마다 배경화면을 갱신합니다.

    ```bash
    ./set_spotlight-[your-architecture] install
    ```

3. **제거**:

    만약 설정한 plist를 제거하고 싶다면, 다음 명령어를 실행하세요.

    ```bash
    ./set_spotlight-[your-architecture] uninstall
    ```

4. **수동으로 실행**:

    인자 없이 실행하면 즉시 배경화면을 갱신합니다.

    ```bash
    ./set_spotlight-[your-architecture]
    ```

## Acknowledgements

- This project utilizes the Windows Spotlight API endpoint discovered and analyzed by [ORelio](https://github.com/ORelio) as part of the [Spotlight Downloader](https://github.com/ORelio/Spotlight-Downloader) project.
- APIv4 analysis conducted by ORelio. Endpoint found through the analysis of network traffic originating from a Windows 11 virtual machine.
- Special thanks to ORelio and the Spotlight Downloader project. If you are using this documentation or the API discovery in your own project, please credit ORelio and the Spotlight Downloader project accordingly.
