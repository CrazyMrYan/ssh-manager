# Git Key Manager

<div align="center">
  <img src="icons/128x128@2x.png" alt="Git Key Manager Logo" width="128" height="128">
  <p>一个简单易用的 Git SSH 密钥管理工具</p>
</div>

## 功能特点

- 🔑 SSH 密钥管理
  - 生成 ED25519/RSA SSH 密钥
  - 查看密钥详情和指纹
  - 一键复制公钥
  - 删除密钥对

- ⚙️ Git 全局配置
  - 管理 Git 全局配置
  - 添加/修改/删除配置项
  - 快速搜索配置

- 🌍 多语言支持
  - 中文
  - English

- 🎨 主题支持
  - 亮色主题
  - 暗色主题
  - 跟随系统

## 安装

### macOS

下载最新的 `.dmg` 文件并安装：

```bash
# 使用 Homebrew
brew install --cask git-key-manager
```

### Windows

下载最新的 `.msi` 或 `.exe` 安装包并安装。

### Linux

下载最新的 `.deb` 或 `.AppImage` 文件并安装：

```bash
# Debian/Ubuntu
sudo dpkg -i git-key-manager_1.0.0_amd64.deb

# 其他发行版
chmod +x git-key-manager_1.0.0_amd64.AppImage
./git-key-manager_1.0.0_amd64.AppImage
```

## 开发

### 环境要求

- Node.js >= 16
- Rust >= 1.70
- Git
- Tauri CLI (`npm install -g @tauri-apps/cli`)

### 本地开发

```bash
# 克隆仓库
git clone https://github.com/yourusername/git-key-manager.git
cd git-key-manager

# 安装依赖
yarn install # 或 npm install

# 启动开发服务器
yarn tauri dev  # 或 npm run tauri dev
```

### 构建

Git Key Manager 提供了多种构建命令，用于生成不同平台的安装包。

#### 基本构建命令

```bash
# 构建当前平台的应用（自动检测并使用当前系统平台）
yarn build:app
```

#### macOS 构建命令

```bash
# 构建适用于 Apple Silicon (M1/M2) Mac 的 DMG 安装包
yarn build:mac

# 构建适用于 Intel Mac 的 DMG 安装包
yarn build:mac-intel

# 构建通用 Mac DMG 安装包（同时支持 Intel 和 Apple Silicon）
yarn build:mac-universal
```

#### Windows 构建命令

```bash
# 构建 Windows 的 EXE 和 MSI 安装包
yarn build:windows
```

#### Linux 构建命令

```bash
# 构建 Linux 的 DEB 和 AppImage 安装包
yarn build:linux
```

#### 跨平台构建

> **重要提示**：跨平台构建非常复杂，尤其是在 macOS 上构建 Windows 应用或在 Windows 上构建 macOS 应用。强烈建议使用 CI/CD 服务（如 GitHub Actions）在各自的原生平台上构建应用。

##### 在 macOS 上构建 Windows 应用的已知问题

在 macOS 上构建 Windows 应用需要完整的 Windows 开发环境，包括：

1. **安装 Windows 交叉编译工具链**：
   ```bash
   brew install mingw-w64
   ```

2. **安装 vcpkg 并设置环境变量**：
   ```bash
   git clone https://github.com/microsoft/vcpkg
   cd vcpkg
   ./bootstrap-vcpkg.sh
   export VCPKG_ROOT=$(pwd)
   ```

3. **安装 Windows 依赖**：
   ```bash
   $VCPKG_ROOT/vcpkg install zlib:x64-windows
   ```

4. **配置 Cargo**：
   在 `~/.cargo/config.toml` 中添加：
   ```toml
   [target.x86_64-pc-windows-msvc]
   linker = "x86_64-w64-mingw32-gcc"
   ar = "x86_64-w64-mingw32-ar"
   ```

即使完成上述所有步骤，仍可能遇到各种兼容性问题和缺失的系统头文件。**这是因为 macOS 和 Windows 的系统架构和库存在根本性差异**。

##### 推荐的跨平台构建方法

对于需要构建多平台应用的开发者，我们强烈推荐以下方法：

1. **使用 GitHub Actions 进行自动化构建**（推荐）：
   - 在 GitHub 上设置工作流，在各自的原生平台上构建应用
   - 自动生成所有平台的安装包
   - 无需处理复杂的交叉编译问题

2. **使用虚拟机或容器**：
   - 在 macOS 上使用 Parallels 或 VMware 运行 Windows 虚拟机
   - 在 Windows 上使用 Docker 容器运行 Linux 构建环境
   - 在各自的原生环境中构建应用

3. **使用云构建服务**：
   - 使用 Azure DevOps、CircleCI 或其他云构建服务
   - 配置多平台构建管道

#### 使用 GitHub Actions 进行跨平台构建

对于正式发布，我们强烈建议使用 GitHub Actions 工作流程来自动构建所有平台的安装包。在项目根目录创建 `.github/workflows/build.yml` 文件，内容如下：

```yaml
name: Build

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-tauri:
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, windows-latest, ubuntu-latest]

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
          
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Install dependencies (ubuntu only)
        if: matrix.platform == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
          
      - name: Install frontend dependencies
        run: yarn install
        
      - name: Build Tauri App
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: v__VERSION__
          releaseName: 'Git Key Manager v__VERSION__'
          releaseBody: 'See the assets to download this version and install.'
          releaseDraft: true
          prerelease: false
```

## 技术栈

- [Tauri](https://tauri.app) - 跨平台应用框架
- [Svelte](https://svelte.dev) - 前端框架
- [TailwindCSS](https://tailwindcss.com) - CSS 框架
- [Rust](https://www.rust-lang.org) - 后端语言

## 项目结构

```
git-key-manager/
├── src/                 # Svelte 前端代码
├── src-tauri/           # Rust 后端代码
│   ├── src/             # Rust 源代码
│   └── Cargo.toml       # Rust 依赖配置
├── public/              # 静态资源
├── icons/               # 应用图标
├── dist/                # 构建输出目录
└── package.json         # 项目配置和依赖
```

## 常见问题

### 构建时遇到 "Option::unwrap() on a None value" 错误

这通常是由于 Tauri 配置问题导致的。请确保：
1. `tauri.conf.json` 中的配置正确
2. 所有必要的依赖项已安装
3. 尝试清理构建缓存 (`yarn clean`)

### 跨平台构建失败

跨平台构建失败是常见问题，特别是在 macOS 上构建 Windows 应用时。常见错误包括：

1. **找不到系统头文件**（如 `sys/types.h` 或 `stdio.h`）：
   - 这是因为 macOS 上的交叉编译工具链不包含完整的 Windows 系统头文件
   - 解决方案：使用 GitHub Actions 在 Windows 环境中构建

2. **VCPKG 相关错误**：
   - 确保已安装 vcpkg 并设置了 `VCPKG_ROOT` 环境变量
   - 安装所需的 Windows 库：`vcpkg install zlib:x64-windows`

3. **链接器错误**：
   - 确保 `~/.cargo/config.toml` 中正确配置了 Windows 目标
   - 检查是否安装了 MinGW-w64 工具链

**最佳解决方案**：使用 GitHub Actions 或其他 CI/CD 服务在各自的原生平台上构建应用。

## 贡献

欢迎提交 Pull Request 或创建 Issue！

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开一个 Pull Request

## 许可证

[MIT License](LICENSE)

## 联系方式

如有问题或建议，请通过 Issues 或以下方式联系我们：

- 邮箱：crazymryan@gmail.com
- 网站：https://giao.club
