#!/bin/bash
# 현재 연결된 모든 모니터에 CirQuare 기본 배경화면을 설정
WALLPAPER="/usr/share/backgrounds/cirquare/cirquare-default.png"

sleep 2

if command -v xrandr &>/dev/null; then
    RAW_MONITORS=$(xrandr --listmonitors | grep -oP '(?<=\d: \+\*?)[^ ]+')
    MONITORS=""
    for m in $RAW_MONITORS; do
        MONITORS="$MONITORS monitor${m}"
    done
else
    MONITORS=$(xfconf-query -c xfce4-desktop -p /backdrop/screen0 -l 2>/dev/null \
        | grep -oP '(?<=/backdrop/screen0/)[^/]+' | sort -u)
fi

for MON in $MONITORS; do
    xfconf-query -c xfce4-desktop -p "/backdrop/screen0/${MON}/workspace0/last-image" \
        --create -t string -s "$WALLPAPER" 2>/dev/null
    xfconf-query -c xfce4-desktop -p "/backdrop/screen0/${MON}/workspace0/image-style" \
        --create -t int -s 5 2>/dev/null
    xfconf-query -c xfce4-desktop -p "/backdrop/screen0/${MON}/workspace0/color-style" \
        --create -t int -s 0 2>/dev/null
done
