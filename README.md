# CirQuare OS

**CirQuare OS (CQOS)** is an Arch Linux-based operating system focused on providing a simple, consistent, and modern Linux desktop experience.

CirQuare OS is being developed with its own desktop environment, **CQDE (CirQuare Desktop Environment)**, built from the ground up using **Wayland, Smithay, and Rust**.

> Linux, simplified.

## Features

* **Base**: Arch Linux
* **Desktop Environment**: CQDE
* **Display Protocol**: Wayland
* **Compositor Framework**: Smithay
* **Programming Language**: Rust
* **Installer**: Calamares
* **Bootloader**: GRUB
* **Build System**: archiso

## CQDE

**CQDE (CirQuare Desktop Environment)** is the desktop environment developed specifically for CirQuare OS.

Unlike traditional desktop environments that are built around an existing window system, CQDE is being developed around the modern Wayland architecture.

### Technology

* Rust
* Wayland
* Smithay
* Linux
* EGL / Mesa

### Components

CQDE is designed as a collection of components rather than a single monolithic application.

```text
CQDE
├── Compositor
├── Window Management
├── Shell
├── Panel / Launcher
├── Desktop UI
└── System Components
```

The compositor is currently being developed using Smithay.

## Architecture

```text
┌─────────────────────────────┐
│          CQOS Apps          │
├─────────────────────────────┤
│            CQDE             │
│   Shell · UI · Components   │
├─────────────────────────────┤
│       Wayland / Smithay     │
├─────────────────────────────┤
│        Linux / Mesa         │
├─────────────────────────────┤
│         Hardware           │
└─────────────────────────────┘
```

CirQuare OS uses Arch Linux as its base while developing its own user-facing desktop experience on top of the Linux stack.

## Building

CirQuare OS is built using [archiso](https://wiki.archlinux.org/title/Archiso).

Builds are currently performed in an Arch Linux environment, including Arch Linux under WSL2.

### Requirements

```bash
sudo pacman -S archiso
```

Additional packages required by the current profile may need to be installed separately.

### ISO Build

From the project directory:

```bash
sudo mkarchiso -v -w work -o out work
```

The generated ISO will be placed in:

```text
out/
```

with a filename similar to:

```text
cirquare-YYYY.MM.DD-x86_64.iso
```

## Project Structure

```text
.
├── airootfs/
│   ├── etc/
│   │   ├── calamares/
│   │   ├── os-release
│   │   └── systemd/
│   ├── home/
│   └── usr/
│
├── packages.x86_64
├── profiledef.sh
├── pacman.conf
└── ...
```

The exact structure may change as CirQuare OS and CQDE continue to evolve.

## Project Status

CirQuare OS is currently under active development.

### Completed

* [x] Arch Linux-based system
* [x] Custom archiso build
* [x] Calamares integration
* [x] CirQuare branding
* [x] Custom icon system
* [x] Custom system theme
* [x] Boot and installation testing
* [x] Initial CQDE compositor prototype
* [x] Basic Wayland/Smithay integration

### In Progress

* [ ] CQDE window management
* [ ] CQDE shell
* [ ] Desktop UI
* [ ] System panel
* [ ] Application launcher
* [ ] System settings
* [ ] Hardware compatibility testing
* [ ] CQOS default applications

### Planned

* [ ] Stable CQDE release
* [ ] Complete CirQuare design system
* [ ] Installer refinement
* [ ] Production package repository
* [ ] GPG package signing
* [ ] Release infrastructure
* [ ] Public documentation
* [ ] First stable CQOS release

## Development

CirQuare OS consists of multiple components that are developed independently.

```text
CirQuare
├── CQOS
│   └── Operating System / Distribution
│
├── CQDE
│   └── Desktop Environment
│
└── Future Components
    └── CirQuare ecosystem
```

The project is currently focused on building the core operating system and CQDE before expanding into additional components.

## Philosophy

CirQuare OS aims to combine the flexibility of Linux with an experience that feels like a complete operating system rather than a collection of independently configured components.

The goal is not to hide Linux.

The goal is to make Linux easier to use.

## Roadmap

### Phase 1 — Foundation

* [x] Arch Linux base
* [x] ISO build system
* [x] Installer
* [x] Initial branding

### Phase 2 — CQDE

* [x] Wayland compositor prototype
* [x] Smithay integration
* [ ] Window management
* [ ] Shell
* [ ] Desktop UI
* [ ] System components

### Phase 3 — CQOS

* [ ] Stable CQDE integration
* [ ] Hardware compatibility
* [ ] Default applications
* [ ] System settings
* [ ] Release engineering

### Phase 4 — CirQuare Ecosystem

* [ ] Additional CirQuare software
* [ ] CirQuare services
* [ ] Cross-device integration

## License

License information will be added before the first public release.
# CirQuare OS

**CirQuare OS (CQOS)** is an Arch Linux-based operating system focused on providing a simple, consistent, and modern Linux desktop experience.

CirQuare OS is being developed with its own desktop environment, **CQDE (CirQuare Desktop Environment)**, built from the ground up using **Wayland, Smithay, and Rust**.

> Linux, simplified.

## Features

* **Base**: Arch Linux
* **Desktop Environment**: CQDE
* **Display Protocol**: Wayland
* **Compositor Framework**: Smithay
* **Programming Language**: Rust
* **Installer**: Calamares
* **Bootloader**: GRUB
* **Build System**: archiso

## CQDE

**CQDE (CirQuare Desktop Environment)** is the desktop environment developed specifically for CirQuare OS.

Unlike traditional desktop environments that are built around an existing window system, CQDE is being developed around the modern Wayland architecture.

### Technology

* Rust
* Wayland
* Smithay
* Linux
* EGL / Mesa

### Components

CQDE is designed as a collection of components rather than a single monolithic application.

```text
CQDE
├── Compositor
├── Window Management
├── Shell
├── Panel / Launcher
├── Desktop UI
└── System Components
```

The compositor is currently being developed using Smithay.

## Architecture

```text
┌─────────────────────────────┐
│          CQOS Apps          │
├─────────────────────────────┤
│            CQDE             │
│   Shell · UI · Components   │
├─────────────────────────────┤
│       Wayland / Smithay     │
├─────────────────────────────┤
│        Linux / Mesa         │
├─────────────────────────────┤
│         Hardware           │
└─────────────────────────────┘
```

CirQuare OS uses Arch Linux as its base while developing its own user-facing desktop experience on top of the Linux stack.

## Building

CirQuare OS is built using [archiso](https://wiki.archlinux.org/title/Archiso).

Builds are currently performed in an Arch Linux environment, including Arch Linux under WSL2.

### Requirements

```bash
sudo pacman -S archiso
```

Additional packages required by the current profile may need to be installed separately.

### ISO Build

From the project directory:

```bash
sudo mkarchiso -v -w work -o out work
```

The generated ISO will be placed in:

```text
out/
```

with a filename similar to:

```text
cirquare-YYYY.MM.DD-x86_64.iso
```

## Project Structure

```text
.
├── airootfs/
│   ├── etc/
│   │   ├── calamares/
│   │   ├── os-release
│   │   └── systemd/
│   ├── home/
│   └── usr/
│
├── packages.x86_64
├── profiledef.sh
├── pacman.conf
└── ...
```

The exact structure may change as CirQuare OS and CQDE continue to evolve.

## Project Status

CirQuare OS is currently under active development.

### Completed

* [x] Arch Linux-based system
* [x] Custom archiso build
* [x] Calamares integration
* [x] CirQuare branding
* [x] Custom icon system
* [x] Custom system theme
* [x] Boot and installation testing
* [x] Initial CQDE compositor prototype
* [x] Basic Wayland/Smithay integration

### In Progress

* [ ] CQDE window management
* [ ] CQDE shell
* [ ] Desktop UI
* [ ] System panel
* [ ] Application launcher
* [ ] System settings
* [ ] Hardware compatibility testing
* [ ] CQOS default applications

### Planned

* [ ] Stable CQDE release
* [ ] Complete CirQuare design system
* [ ] Installer refinement
* [ ] Production package repository
* [ ] GPG package signing
* [ ] Release infrastructure
* [ ] Public documentation
* [ ] First stable CQOS release

## Development

CirQuare OS consists of multiple components that are developed independently.

```text
CirQuare
├── CQOS
│   └── Operating System / Distribution
│
├── CQDE
│   └── Desktop Environment
│
└── Future Components
    └── CirQuare ecosystem
```

The project is currently focused on building the core operating system and CQDE before expanding into additional components.

## Philosophy

CirQuare OS aims to combine the flexibility of Linux with an experience that feels like a complete operating system rather than a collection of independently configured components.

The goal is not to hide Linux.

The goal is to make Linux easier to use.

## Roadmap

### Phase 1 — Foundation

* [x] Arch Linux base
* [x] ISO build system
* [x] Installer
* [x] Initial branding

### Phase 2 — CQDE

* [x] Wayland compositor prototype
* [x] Smithay integration
* [ ] Window management
* [ ] Shell
* [ ] Desktop UI
* [ ] System components

### Phase 3 — CQOS

* [ ] Stable CQDE integration
* [ ] Hardware compatibility
* [ ] Default applications
* [ ] System settings
* [ ] Release engineering

### Phase 4 — CirQuare Environment

* [ ] Additional CirQuare software
* [ ] CirQuare services
* [ ] Cross-device integration

## License

License information will be added before the first public release.
