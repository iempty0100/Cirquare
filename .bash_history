mv work/.git .git
git status
ls -la
cd /
ls -la
cd . .
cd ~
cd . .
mv ~/git .git
git status
cd . .
cd ..
git status
pwd
ls -la
ls -la ~/작업폴더
git pull
cd /home/admin
ls -la
git status
cd /home/admin
git reset --hard HEAD
git clean -fd
git pull
git status
cd ~/work
rm -rf work
git status
git pull
sudo rm -rf /work
sudo rm -rf /out
sudo mkarchiso -v -o /out .
git pull
cd ~
cd /work
cd work
ls -la
sudo rm -rf out
sudo rm -rf work
sudo mkarchiso -v -o /out .
sudo mkarchiso -v -o /out .
cd ~/work
sudo mkarchiso -v -o /out .
sudo rm -rf work
sudo rm -rf out
sudo mkarchiso -v -o /out .
cd ~/work
find . -type f -name "unpackfs.conf"
cat airootfs/etc/calamares/modules/unpackfs.conf
sudo nano airootfs/etc/calamares/modules/unpackfs.conf
grep -n "source:" airootfs/etc/calamares/modules/unpackfs.conf
grep -n "source:" airootfs/etc/calamares/modules/unpackfs.conf
sudo rm -rf x86_64
history | grep -E 'mkarchiso|build'
sudo rm -rf work
sudo rm -rf out
sudo mkarchiso -v -o /out .
pwd
ls -la
find . -maxdepth 2 -type d | sort
cat profiledef.sh
grep -nE 'work|output|iso|install' profiledef.sh
mkarchiso -v -w work -o . .
sudo ls -lah work
ls -lh iso/
ls -lh work/iso/
find iso work/iso -maxdepth 2 -type f \( -name "*.iso" -o -name "grubenv" \) -ls
history | grep mkarchiso | tail -10
   sudo rm -rf work
   sudo mkarchiso -v -w work -o . .
echo $?
ls -lh *.iso
   sudo mkarchiso -v -w work -o . .
# 1) 시스템 로그에서 이 시간대에 뭔가 있었는지 확인
sudo journalctl --since "10 minutes ago" | grep -iE "sigint|kill|oom|suspend"
# 2) dmesg 확인 (OOM killer 등)
dmesg | tail -50
tmux new -s build
sudo mkarchiso -v -w work -o . .
uptime -s
free -h
df -h .
  wsl --status
  wsl --version
  wsl --status
  wsl --version
ps aux | grep -E "mkarchiso|mkfs.erofs|mksquashfs"
top
cd /work
cd ~/work
ls -la
sudo rm -rf work
sudo rm -rf out
sudo mkarchiso -v -o ~/out .
# 터미널 B: 네트워크 연결 감시 (2초마다 갱신)
watch -n 2 'ss -tnp 2>/dev/null | grep -v "127.0.0.1\|::1"'
cd ~/work
sudo mkarchiso -v -o /out .
sudo rm -rf work
sudo rm -rf out
sudo mkarchiso -v -o /out .
   sudo mkarchiso -v -w work -o . .
sudo rm -rf out .
   sudo mkarchiso -v -w work -o . .
sudo rm -rf work
sudo rm -rf out
sudo mkarchiso -v -w work -o . .
# 터미널 A: 빌드 실행
sudo mkarchiso -v -w work -o . . 2>&1 | tee build_log_$(date +%s).log
sudo rm -rf work
sudo rm -rf out
sudo mkarchiso -v -o /out .
echo $?
ls -lh *.iso
sudo rm -rf work
sudo rm -rf out
sudo mkarchiso -v -w work -o . . > build_full.log 2>&1
while true; do echo "--- $(date +%T) ---" >> netwatch.log; ss -tnp 2>/dev/null | grep -v "127.0.0.1\|::1" >> netwatch.log; sleep 1; done
# 터미널 B: 네트워크 연결 감시 (2초마다 갱신)
watch -n 2 'ss -tnp 2>/dev/null | grep -v "127.0.0.1\|::1"'
cd work
sudo rm -rf work
sudo rm -rf out
sudo mkarchiso -v -o out .
sudo mkarchiso -v -w work -o . . 2>&1 | tee build_full.log
echo "EXIT CODE: ${PIPESTATUS[0]}"
sudo rm -rf work
sudo rm -rf out
sudo mkarchiso -v -w work -o . . 2>&1 | tee build_full.log
echo "EXIT CODE: ${PIPESTATUS[0]}"
grep -i "error\|permission denied\|cannot" build_full.log
sudo rm -rf work
sudo mkarchiso -v -w work -o . .
pacman -Q archiso
sudo pacman -Syu archiso
grep -rn "will not overwrite\|grubenv" /usr/lib/archiso/ 2>/dev/null
grep -rn "grubenv" $(pacman -Ql archiso | awk '{print $2}') 2>/dev/null
pacman -Ql archiso | grep -i grub
grep -n "cp " /usr/bin/mkarchiso | grep -i "grub\|bios\|uefi\|efi"
grep -n "isofs_dir}/boot/grub\|work_dir}/grub" /usr/bin/mkarchiso
ls -la ~/work/grub/
rm -f ~/work/grub/grubenv
ls -la ~/work/grub/
sudo rm -rf work
sudo mkarchiso -v -w work -o . .
