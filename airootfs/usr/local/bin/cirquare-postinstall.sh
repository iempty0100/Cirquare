#!/bin/bash
set -e

cp /run/systemd/resolve/resolv.conf /etc/resolv.conf 2>/dev/null || cp /etc/resolv.conf /etc/resolv.conf.bak 2>/dev/null
echo "nameserver 8.8.8.8" > /etc/resolv.conf
echo "nameserver 1.1.1.1" >> /etc/resolv.conf

rm -f /etc/mkinitcpio.conf.d/archiso.conf

cat > /etc/mkinitcpio.d/linux.preset <<'EOF'
# mkinitcpio preset file for the linux package
ALL_kver="/boot/vmlinuz-linux"
PRESETS=("default" "fallback")
default_image="/boot/initramfs-linux.img"
fallback_image="/boot/initramfs-linux-fallback.img"
fallback_options="-S autodetect"
EOF

cat > /etc/pacman.d/mirrorlist <<'EOF'
Server = https://geo.mirror.pkgbuild.com/$repo/os/$arch
Server = https://mirrors.kernel.org/archlinux/$repo/os/$arch
EOF

pacman-key --init
pacman-key --populate archlinux
pacman -Sy --noconfirm
pacman -S --noconfirm --overwrite '*' linux linux-firmware mkinitcpio
mkinitcpio -P
