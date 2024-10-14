#!/bin/bash

mkdir -p ~/bin
cp ./mac_spotlight_setter ~/bin
chmod +x ~/bin/mac_spotlight_setter
mkdir -p ~/Library/LaunchAgents
echo "<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <!-- Label: unique identifier for the job -->
    <key>Label</key>
    <string>com.$USER.setwallpaper</string>

    <!-- ProgramArguments: full path to the shell script -->
    <key>ProgramArguments</key>
    <array>
        <string>/bin/bash</string>
        <string>~/bin/mac_spotlight_setter</string>
    </array>

    <!-- RunAtLoad: run the script when the agent/daemon is loaded -->
    <key>RunAtLoad</key>
    <true/>

    <!-- KeepAlive: if true, the script will run again if it fails -->
    <key>KeepAlive</key>
    <false/>

    <!-- StartInterval: interval to run the script (in seconds) -->
    <key>StartInterval</key>
    <integer>21600</integer> <!-- 6시간마다 실행 -->
</dict>
</plist>
" > ~/Library/LaunchAgents/com.$USER.setwallpaper.plist
launchctl unload ~/Library/LaunchAgents/com.$USER.setwallpaper.plist
launchctl load ~/Library/LaunchAgents/com.$USER.setwallpaper.plist

echo "설정이 완료되었습니다. 6시간마다 배경화면이 변경됩니다."
exec ~/bin/mac_spotlight_setter