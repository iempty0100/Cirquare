#!/bin/bash

# CirQuare - 배경화면 + 테마(GTK/아이콘/xfwm4) 자동 적용 스크립트
# 로그인 시 autostart로 실행됨 (~/.config/autostart/cirquare-theme.desktop)

set -u

WALLPAPER="/usr/share/backgrounds/cirquare/cirquare-default.png"
GTK_THEME="CirQuare"
ICON_THEME="CirQuare"
WM_THEME="CirQuare"

# --- 배경화면 적용 ---

# 모니터 이름은 하드웨어마다 다르므로
# xrandr로 실제 연결된 출력 이름을 런타임에 감지한다.

if command -v xrandr >/dev/null 2>&1; then
    MONITORS=$(xrandr --listmonitors 2>/dev/null | grep -oP '(?<=\s)[A-Za-z0-9_-]+$')

    if [ -n "$MONITORS" ]; then
        while IFS= read -r MON; do
            PROP="/backdrop/screen0/monitor${MON}/workspace0/last-image"

            xfconf-query \
                -c xfce4-desktop \
                -p "$PROP" \
                -n \
                -t string \
                -s "$WALLPAPER" \
                2>/dev/null || \
            xfconf-query \
                -c xfce4-desktop \
                -p "$PROP" \
                -s "$WALLPAPER"
        done <<< "$MONITORS"
    else
        echo "cirquare-set-theme: xrandr에서 모니터를 찾지 못함, 기존 xfconf 키 재사용" >&2
    fi
else
    echo "cirquare-set-theme: xrandr 없음, 배경화면 자동 감지 생략" >&2
fi

# --- GTK / 아이콘 / 창 관리자 테마 적용 ---

xfconf-query \
    -c xsettings \
    -p /Net/ThemeName \
    -n \
    -t string \
    -s "$GTK_THEME" \
    2>/dev/null || \
xfconf-query \
    -c xsettings \
    -p /Net/ThemeName \
    -s "$GTK_THEME"

xfconf-query \
    -c xsettings \
    -p /Net/IconThemeName \
    -n \
    -t string \
    -s "$ICON_THEME" \
    2>/dev/null || \
xfconf-query \
    -c xsettings \
    -p /Net/IconThemeName \
    -s "$ICON_THEME"

xfconf-query \
    -c xfwm4 \
    -p /general/theme \
    -n \
    -t string \
    -s "$WM_THEME" \
    2>/dev/null || \
xfconf-query \
    -c xfwm4 \
    -p /general/theme \
    -s "$WM_THEME"

exit 0
