# CirQuare

Arch Linux 기반의 커스텀 리눅스 배포판입니다. XFCE4 데스크톱 환경과 Calamares 설치 프로그램을 사용합니다.

## 특징

- **베이스**: Arch Linux
- **데스크톱 환경**: XFCE4
- **설치 프로그램**: Calamares (자체 브랜딩 적용)
- **부트로더**: GRUB

## 빌드 방법

이 프로젝트는 [archiso](https://wiki.archlinux.org/title/Archiso)를 기반으로 합니다. Arch Linux 환경(또는 WSL2 Arch Linux)에서 빌드합니다.

### 요구 사항

```bash
sudo pacman -S archiso
```

### 로컬 패키지 저장소 준비

Calamares는 AUR 전용 패키지이므로, 빌드 전에 로컬 저장소를 먼저 구성해야 합니다.

```bash
# yay 등 AUR 헬퍼로 calamares 빌드
yay -S calamares

# 로컬 저장소에 패키지 등록
repo-add ~/local-repo/cirquare-local.db.tar.gz ~/local-repo/*.pkg.tar.zst
```

`work/pacman.conf`에 아래 내용이 포함되어 있어야 합니다:

```ini
[cirquare-local]
SigLevel = Optional TrustAll
Server = file:///home/admin/local-repo
```

### ISO 빌드

```bash
sudo mkarchiso -v -w work -o out work
```

빌드가 완료되면 `out/cirquare-YYYY.MM.DD-x86_64.iso` 파일이 생성됩니다.

## 프로젝트 구조

```
.
├── airootfs/                          # 라이브 이미지에 포함될 루트 파일시스템
│   ├── etc/
│   │   ├── calamares/                 # Calamares 설정 (branding, modules 등)
│   │   ├── os-release                 # 배포판 식별 정보
│   │   ├── passwd, group, shadow      # liveuser 계정 정보
│   │   └── systemd/system/            # 활성화된 서비스 심볼릭 링크
│   ├── home/liveuser/
│   │   └── Desktop/install-cirquare.desktop
│   └── usr/local/bin/
│       └── cirquare-postinstall.sh    # 설치 후 커널/initramfs 복구 스크립트
├── packages.x86_64                    # 설치될 패키지 목록
├── profiledef.sh                      # archiso 프로필 설정 (iso 이름, 이미지 타입 등)
└── pacman.conf                        # 사용할 저장소 목록 (로컬 저장소 포함)
```

## 알려진 이슈 / 주의사항

- **커널 설치 문제**: archiso 라이브 이미지는 커널을 airootfs 안에 포함하지 않기 때문에, Calamares로 설치만 하면 `/boot`에 커널이 없어 GRUB rescue로 빠집니다. 이를 해결하기 위해 `shellprocess` 모듈로 설치 마지막 단계에 `cirquare-postinstall.sh`를 실행해 커널과 initramfs를 재설치합니다.
- **로컬 저장소 보안**: 현재 로컬 저장소는 `SigLevel = Optional TrustAll`로 설정되어 있어 GPG 서명 검증을 하지 않습니다. 프로덕션 배포 전에는 반드시 서명 체계를 갖추는 것을 권장합니다.
- **Calamares shellprocess 변수 치환**: shellprocess 모듈의 인라인 명령어에 `$repo`, `$arch` 같은 bash 변수를 직접 쓰면 오류가 발생하므로, 반드시 별도 스크립트 파일로 분리해야 합니다.

## 로드맵

- [ ] 브랜딩 마무리 (배경화면, 로그인 화면, 아이콘 테마)
- [ ] GPG 서명 등 로컬 저장소 보안 강화
- [ ] 다른 VM 및 실제 하드웨어에서 추가 테스트
- [ ] 배포용 문서 및 다운로드 페이지 준비

## 라이선스

TODO: 라이선스를 선택하세요 (예: GPLv3, MIT 등)
