# 1、目录结构
 - capabilities：由 tauri 提供的能力目录
 - gen：schemas 存放描述文件
 - icons：tauri 的系统图标
 - src：tauri 后端 rust 源代码文件
    - lib.rs：安装依赖的配置文件
    - main.rs：程序主入口
 - build.rs：build 的入口文件
 - tauri.conf.json：tauri 配置文件

# 2、核心概念
Tauri 主要分为两个大内容组成：Upstream crates 和 core ecosystem。

## 2.1、Upstream creates
- Tao：用于创建和管理应用窗体，他是Rust 中跨平台应用程序窗口的创建库，支持Windows、MacOs、Linux、IOS 和 Android。是 winit 的一个分支，在 tauri中进行了扩展增加了菜单和系统托盘。
- Wry：作为接口，用于连接 WebView 和窗体，Wry 是 Rust 中的跨平台 WebView 渲染库，支持所有主流桌面平台，Tauri 使用 Wry 作为抽象层，负责确定使用哪个 WebView 进行交互，抹除了平台差异，暴露统一的上层 API。

## 2.2、Core ecosystem
- tauri-runtime：它负责与较低级别的 webView 库之间交互的粘合层。
- tauri-macros：将 upstream crate 作为上下文，处理程序和创建宏。
- tauri-utils：提供通用的工具，如：解析配置文件、检测平台三元组等。
- tauri-runtime-wry：与 Wry 通过接口直接进行系统级交互，如：打印、监视器等。
- tauri-codegen：负责创建应用内容，如：应用程序图标以及系统托盘等，tauri.conf.json 在编译时解析并生成 Config 结构。
- tauri-build 应用构建

其中 tauri-runtime、tauri-macros和 tauri-utils 构成了基本的 tauri 结构。

# 3、tauri.conf.json 文件结构
tauri 的默认配置格式为 JSON。并且同时支持json5或toml 的配置格式，你可以在Cargo.toml 文件中，修改 tauri 和 tauri-build 的配置来开启对 conf-json5 或 config-toml的启用。

```json
{
  "$schema": "https://schema.tauri.app/config/2", ## 配置文件的官方描述说明
  "productName": "tauri-test",  ## 应用的名称，程序的标题
  "version": "0.1.0",   ## 应用的版本号
  "identifier": "com.time_travel.tauri-test", ## 应用唯一标识
  "build": {    ## 应用构建配置
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist",
    "removeUnusedCommands": true
  },
  "app": {  ## 应用的配置
    "windows": [
      {
        "title": "tauri-test",
        "width": 800,
        "height": 600
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {   ## 应用安装配置
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  },
  "plugins": {} ## 插件配置
}
```
## 3.1、App 配置
```json
{
  "enableGTKAppId": false,
  "macOSPrivateApi": false,
  "security": {
    "assetProtocol": {
      "enable": false,
      "scope": []
    },
    "capabilities": [],
    "dangerousDisableAssetCspModification": false,
    "freezePrototype": false,
    "pattern": {
      "use": "brownfield"
    }
  },
  "windows": [
    {
      "title": "tauri-test",
      "width": 800,
      "height": 600
    }
  ],
  "withGlobalTauri": false
}
```

|属性|类型|描述|
|--|--|--|
|active|boolean|是否启用打包功能，默认值为 false|
|android.minSdkVersion|number|Android最小SDK版本，默认值为 24|
|createUpdaterArtifacts|boolean|是否创建更新器工件，默认值为 false|
|iOS.minimumSystemVersion|string|iOS最小系统版本，默认值为 "14.0"|
|icon|array|应用图标列表，默认值为空数组|
|linux.appimage.bundleMediaFramework|boolean|Linux AppImage是否捆绑媒体框架，默认值为 false|
|linux.appimage.files|object|Linux AppImage文件配置，默认值为空对象|
|linux.deb.files|object|Linux DEB文件配置，默认值为空对象|
|linux.rpm.epoch|number|Linux RPM的epoch值，默认值为 0|
|linux.rpm.files|object|Linux RPM文件配置，默认值为空对象|
|linux.rpm.release|string|Linux RPM的发布版本，默认值为 "1"|
|macOS.dmg.appPosition.x|number|macOS DMG中应用位置的X坐标，默认值为 180|
|macOS.dmg.appPosition.y|number|macOS DMG中应用位置的Y坐标，默认值为 170|
|macOS.dmg.applicationFolderPosition.x|number|macOS DMG中应用程序文件夹位置的X坐标，默认值为 480|
|macOS.dmg.applicationFolderPosition.y|number|macOS DMG中应用程序文件夹位置的Y坐标，默认值为 170|
|macOS.dmg.windowSize.height|number|macOS DMG窗口高度，默认值为 400|
|macOS.dmg.windowSize.width|number|macOS DMG窗口宽度，默认值为 660|
|macOS.files|object|macOS文件配置，默认值为空对象|
|macOS.hardenedRuntime|boolean|是否启用macOS强化运行时，默认值为 true|
|macOS.minimumSystemVersion|string|macOS最小系统版本，默认值为 "10.13"|
|targets|string或array|构建目标，默认值为 "all"|
|useLocalToolsDir|boolean|是否使用本地工具目录，默认值为 false|
|windows.allowDowngrades|boolean|Windows是否允许降级，默认值为 true|
|windows.certificateThumbprint|null|string|Windows证书指纹，默认值为 null|
|windows.digestAlgorithm|null|string|Windows摘要算法，默认值为 null|
|windows.nsis|null|object|Windows NSIS配置，默认值为 null|
|windows.signCommand|null|string|Windows签名命令，默认值为 null|
|windows.timestampUrl|null|string|Windows时间戳URL，默认值为 null|
|windows.tsp|boolean|Windows是否使用时间戳协议，默认值为 false|
|windows.webviewInstallMode.silent|boolean|Windows WebView安装模式是否静默，默认值为 true|
|windows.webviewInstallMode.type|string|Windows WebView安装模式类型，默认值为 "downloadBootstrapper"|
|windows.wix|null|object|Windows WiX配置，默认值为 null|
|enableGTKAppId|boolean|是否启用 GTK 应用 ID，默认值为 false|
|macOSPrivateApi|boolean|是否启用 macOS 私有 API，默认值为 false|
|security.assetProtocol.enable|boolean|是否启用资产协议，默认值为 false|
|security.assetProtocol.scope|array|资产协议的作用域，默认值为空数组|
|security.capabilities|array|应用的能力，默认值为空数组|
|security.dangerousDisableAssetCspModification|boolean|是否禁用资产 CSP 修改，默认值为 false|
|security.freezePrototype|boolean|是否冻结原型，默认值为 false|
|security.pattern.use|string|应用的模式，默认值为 "brownfield"|
|windows|array|应用窗口配置|
|withGlobalTauri|boolean|是否启用全局 Tauri 实例，默认值为 false|

## 3.2、windows 配置
|属性|类型|描述|
|--|--|--|
|title|string|窗口的标题|
|width|number|窗口的宽度，默认值为 800|
|height|number|窗口的高度，默认值为 600|
|resizable|boolean|窗口是否可调整大小，默认值为 true|
|fullscreen|boolean|窗口是否全屏，默认值为 false|
|transparent|boolean|窗口是否透明，默认值为 false|
|alwaysOnTop|boolean|窗口是否始终保持在顶部，默认值为 false|
|visible|boolean|窗口是否可见，默认值为 true|
|minWidth|number|窗口的最小宽度，默认值为 0|
|minHeight|number|窗口的最小高度，默认值为 0|
|maxWidth|number|窗口的最大宽度，默认值为 0|
|maxHeight|number|窗口的最大高度，默认值为 0|
|x|number|窗口的 X 坐标，默认值为 0|
|y|number|窗口的 Y 坐标，默认值为 0|
|alwaysOnBottom|boolean|窗口是否始终保持在底部，默认值为 false|
|alwaysOnTop|boolean|窗口是否始终保持在顶部，默认值为 false|
|backgroundColor|string|窗口的背景颜色，默认值为 "#000000"|
|center|boolean|窗口是否居中，默认值为 false|
|closable|boolean|窗口是否可关闭，默认值为 true|
|contentProtected|boolean|窗口是否受内容保护,防止窗口内容被其他应用程序捕获，默认值为 false|
|create|boolean|窗口是否在创建时立即显示，默认值为 true|
|decorations|boolean|窗口是否有装饰(边框和条)，默认值为 true|
|dragDropEnabled|boolean|窗口是否启用拖拽和drop，默认值为 true|
|hiddenTitle|boolean|窗口是否隐藏标题，默认值为 false|
|resizable|boolean|窗口是否可调整大小，默认值为 true|
|shadow|boolean|窗口是否有阴影，默认值为 true|
|transparent|boolean|窗口是否透明，默认值为 false|
|visible|boolean|窗口是否可见，默认值为 true|

## 3.3、Build 配置
|属性|类型|描述|
|--|--|--|
|beforeDevCommand|string|开发模式下，应用启动前执行的命令|
|devUrl|string|开发模式下，应用启动的 URL，默认值为 "http://localhost:1420"|
|beforeBuildCommand|string|构建应用前执行的命令|
|frontendDist|string|前端代码的输出目录，默认值为 "../dist"|
|removeUnusedCommands|boolean|是否移除未使用的命令，默认值为 true|

## 3.4、Bundle 配置
```json
{
  "active": false,
  "android": {
    "minSdkVersion": 24
  },
  "createUpdaterArtifacts": false,
  "iOS": {
    "minimumSystemVersion": "14.0"
  },
  "icon": [],
  "linux": {
    "appimage": {
      "bundleMediaFramework": false,
      "files": {}
    },
    "deb": {
      "files": {}
    },
    "rpm": {
      "epoch": 0,
      "files": {},
      "release": "1"
    }
  },
  "macOS": {
    "dmg": {
      "appPosition": {
        "x": 180,
        "y": 170
      },
      "applicationFolderPosition": {
        "x": 480,
        "y": 170
      },
      "windowSize": {
        "height": 400,
        "width": 660
      }
    },
    "files": {},
    "hardenedRuntime": true,
    "minimumSystemVersion": "10.13"
  },
  "targets": "all",
  "useLocalToolsDir": false,
  "windows": {
    "allowDowngrades": true,
    "certificateThumbprint": null,
    "digestAlgorithm": null,
    "nsis": null,
    "signCommand": null,
    "timestampUrl": null,
    "tsp": false,
    "webviewInstallMode": {
      "silent": true,
      "type": "downloadBootstrapper"
    },
    "wix": null
  }
}
```
|属性|类型|描述|
|--|--|--|
|active|boolean|是否启用打包功能，默认值为 false|
|android.minSdkVersion|number|Android最小SDK版本，默认值为 24|
|createUpdaterArtifacts|boolean|是否创建更新器工件，默认值为 false|
|iOS.minimumSystemVersion|string|iOS最小系统版本，默认值为 "14.0"|
|icon|array|应用图标列表，默认值为空数组|
|linux.appimage.bundleMediaFramework|boolean|Linux AppImage是否捆绑媒体框架，默认值为 false|
|linux.appimage.files|object|Linux AppImage文件配置，默认值为空对象|
|linux.deb.files|object|Linux DEB文件配置，默认值为空对象|
|linux.rpm.epoch|number|Linux RPM的epoch值，默认值为 0|
|linux.rpm.files|object|Linux RPM文件配置，默认值为空对象|
|linux.rpm.release|string|Linux RPM的发布版本，默认值为 "1"|
|macOS.dmg.appPosition.x|number|macOS DMG中应用位置的X坐标，默认值为 180|
|macOS.dmg.appPosition.y|number|macOS DMG中应用位置的Y坐标，默认值为 170|
|macOS.dmg.applicationFolderPosition.x|number|macOS DMG中应用程序文件夹位置的X坐标，默认值为 480|
|macOS.dmg.applicationFolderPosition.y|number|macOS DMG中应用程序文件夹位置的Y坐标，默认值为 170|
|macOS.dmg.windowSize.height|number|macOS DMG窗口高度，默认值为 400|
|macOS.dmg.windowSize.width|number|macOS DMG窗口宽度，默认值为 660|
|macOS.files|object|macOS文件配置，默认值为空对象|
|macOS.hardenedRuntime|boolean|是否启用macOS强化运行时，默认值为 true|
|macOS.minimumSystemVersion|string|macOS最小系统版本，默认值为 "10.13"|
|targets|string或array|构建目标，默认值为 "all"|
|useLocalToolsDir|boolean|是否使用本地工具目录，默认值为 false|
|windows.allowDowngrades|boolean|Windows是否允许降级，默认值为 true|
|windows.certificateThumbprint|null|string|Windows证书指纹，默认值为 null|
|windows.digestAlgorithm|null|string|Windows摘要算法，默认值为 null|
|windows.nsis|null|object|Windows NSIS配置，默认值为 null|
|windows.signCommand|null|string|Windows签名命令，默认值为 null|
|windows.timestampUrl|null|string|Windows时间戳URL，默认值为 null|
|windows.tsp|boolean|Windows是否使用时间戳协议，默认值为 false|
|windows.webviewInstallMode.silent|boolean|Windows WebView安装模式是否静默，默认值为 true|
|windows.webviewInstallMode.type|string|Windows WebView安装模式类型，默认值为 "downloadBootstrapper"|
|windows.wix|null|object|Windows WiX配置，默认值为 null|
```




