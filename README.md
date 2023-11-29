
# Tauri 🦀

Getting started with **Tauri** 🦀🦀🦀🦀🦀🦀 hand's on 👐 


## Installation Steps 🛠️

1. **Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **NodeJS**

```bash
sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key sudo gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
```

```bash
NODE_MAJOR=20
echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | sudo tee /etc/apt/sources.list.d/nodesource.list
```

```bash
sudo apt-get update
sudo apt-get install nodejs -y
```

3. **Pre-requisites**

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

```bash
npm install --save-dev @tauri-apps/cli
```

## Tutorial Steps 🪜

1. **Download the base project**

```bash
git clone https://github.com/ivoafonsobispo/tauri-getting-started.git
```

2. **Run the project**

```bash
npm install
npm run tauri dev
```

3. **Try to solve the ordered steps in the base project**

😶‍🌫️ [Base Project](https://github.com/ivoafonsobispo/tauri-getting-started/tree/main/tauri-dex-base)

4. **Build Project**
- Linux
```bash
npm run tauri build
```

```bash
cd src-tauri/target/release/
./tauri-dex
```

- Windows

```bash
sudo apt-get install mingw-w64
```

```bash
rustup target add x86_64-pc-windows-gnu
```

```bash
npm run tauri build -- --target x86_64-pc-windows-gnu
```
- Window's built project:
    - Directory: src-tauri/target/release/
    - Executable file: tauri-dex.exe

## Appendix 🗄️

🧐 [Presentation](https://www.overleaf.com/read/fvbfxqjcpdzh#0ba971)

🥳 [Solution](https://github.com/ivoafonsobispo/tauri-getting-started/tree/main/tauri-dex-solution)

## Pretty ✨✨

![Logo](https://tauri.app/img/social.png)

