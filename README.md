
# Tauri 🦀

Getting started with **Tauri** 🦀🦀🦀🦀🦀🦀 hand's on 👐 


## Installation Steps 🛠️

1. **Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Select "1) Proceed with installation (default)" by typping 1
    
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

## Tutorial Steps 🪜

1. **Download the base project**

```bash
git clone https://github.com/ivoafonsobispo/tauri-getting-started.git
```

2. **Install the pre-requisites**

```bash
sudo apt install cargo
cargo install tauri-cli
```

3. **Run the project**

```bash
cargo tauri dev
```

## Apprendix 🗄️

[Presentation](https://www.overleaf.com/read/fvbfxqjcpdzh#0ba971)

## Logo so pretty ✨✨

![Logo](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/th5xamgrr6se0x5ro4g6.png)

