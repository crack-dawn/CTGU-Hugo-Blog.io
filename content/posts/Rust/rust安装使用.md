---
title: rust安装使用
subtitle:
date: 2024-08-17T22:05:52+08:00
slug: ca1d32f
draft: false
comment: true
summary:
description: 在windows下安装rust。

keywords:

message:   # 密码输入框预设提示
password:  # 解锁密码

layout:     # 布局归类：posts, archives, tags, categories等，文件分散在不同文件夹需要注明
archives:
tags: ['rust']
categories: ['rust']

author:
  name:
  link:
  email:
  avatar:
license:
repost:
  enable: true
  url:

weight: 0
hiddenFromHomePage: false
hiddenFromSearch: false
hiddenFromRss: false
hiddenFromRelated: false

resources:
  - name: featured-image
    src: featured-image.jpg
  - name: featured-image-preview
    src: featured-image-preview.jpg
toc: true
math: true
lightgallery: true

# See details front matter: https://fixit.lruihao.cn/documentation/content-management/introduction/#front-matter
---

<!--more-->

## windows下安装rust
### 1. 安装前准备：
+ [下载mingw64](https://github.com/niXman/mingw-builds-binaries/releases),下载后解压，`mingw64/bin/`目录加入环境变量
+ [下载rust_init.exe](https://www.rust-lang.org/tools/install) 


+ Rust编译链工具安装目录修改：

    Rust需要安装两个东西，一个是rustup，一个是cargo。所以你需要设置两个环境变量来分别指定他们的安装目录。

    通过`RUSTUP_HOME`指定`rustup`的安装目录。
    通过`CARGO_HOME`指定`cargo`的安装目录。

+ 切换清华镜像,网络加速
    ```text
    配置加速安装地址: 直接从官方网站下载会很慢，改用国内镜像加速安装，设置以下`变量`，并一起加入`环境变量`：

    RUSTUP_DIST_SERVER https://mirrors.tuna.tsinghua.edu.cn/rustup
    RUSTUP_UPDATE_ROOT https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
    
    ```
+ 配置镜像配置库镜像
    在“C:\Users\用户名\”下创建“.cargo”文件夹，在文件夹内创建“config”文件，如下图：
    ```text
    [source.crates-io]
    registry = “https://github.com/rust-lang/crates.io-index”
    replace-with = ‘tuna’
    [source.tuna]
    registry = https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git
    ```
 
### 2. 安装开始：
+ 打开预先下载好的[rust_init.exe](https://www.rust-lang.org/tools/install) ，按照安装提示操作。
+  在弹出的命令窗中 选择2,回车，然后输入`stable-x86_64-pc-windows-gnu`
    ```text
    You can uninstall at any time with rustup self uninstall and
    these changes will be reverted.

    Current installation options:


    default host triple: x86_64-pc-windows-msvc
        default toolchain: stable (default)
                profile: default
    modify PATH variable: yes

    1) Proceed with standard installation (default - just press enter)
    2) Customize installation
    3) Cancel installation
    >2

    I'm going to ask you the value of each of these installation options.
    You may simply press the Enter key to leave unchanged.

    Default host triple? [x86_64-pc-windows-msvc]
    stable-x86_64-pc-windows-gnu        后续一直回车即可
    ```
---

### 3. 安装完成，提示如下安装成功
```
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload its PATH environment variable to include
Cargo's bin directory (D:\toolchains\Rust\.cargo\bin).

Press the Enter key to continue.
```

### 4. 验证安装是否成功
```
rustc --version
cargo --version
```
## 安装日志：

```text
Rust Visual C++ prerequisites

Rust requires a linker and Windows API libraries but they don't seem to be
available.

These components can be acquired through a Visual Studio installer.

1) Quick install via the Visual Studio Community installer
   (free for individuals, academic uses, and open source).

2) Manually install the prerequisites
   (for enterprise and advanced users).

3) Don't install the prerequisites
   (if you're targeting the GNU ABI).

>2


You can acquire the build tools by installing Microsoft Visual Studio.

  https://visualstudio.microsoft.com/downloads/

Check the box for "Desktop development with C++" which will ensure that the
needed components are installed. If your locale language is not English,
then additionally check the box for English under Language packs.

For more details see:

  https://rust-lang.github.io/rustup/installation/windows-msvc.html

Install the C++ build tools before proceeding.

If you will be targeting the GNU ABI or otherwise know what you are
doing then it is fine to continue installation without the build
tools, but otherwise, install the C++ build tools before proceeding.

Continue? (y/N)
y


Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  D:\toolchains\Rust\.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  D:\toolchains\Rust\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  D:\toolchains\Rust\.cargo\bin

This path will then be added to your PATH environment variable by
modifying the HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>2

I'm going to ask you the value of each of these installation options.
You may simply press the Enter key to leave unchanged.

Default host triple? [x86_64-pc-windows-msvc]
stable-x86_64-pc-windows-gnu        后续一直回车即可

..........

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>

info: profile set to 'default'
info: default host triple is x86_64-pc-windows-gnu
info: syncing channel updates for 'stable-x86_64-pc-windows-gnu'
759.8 KiB / 759.8 KiB (100 %) 287.7 KiB/s in  2s ETA:  0s
info: latest update on 2024-08-08, rust version 1.80.1 (3f5fd8dd4 2024-08-06)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-mingw'
info: downloading component 'rust-std'
info: downloading component 'rustc'
 74.7 MiB /  74.7 MiB (100 %) 272.0 KiB/s in  4m 55s ETA:  0s
info: downloading component 'rustfmt'
  3.2 MiB /   3.2 MiB (100 %) 269.5 KiB/s in 12s ETA:  0s
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 15.7 MiB /  15.7 MiB (100 %)   1.5 MiB/s in 11s ETA:  0s
info: installing component 'rust-mingw'
info: installing component 'rust-std'
 22.5 MiB /  22.5 MiB (100 %)  11.4 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 74.7 MiB /  74.7 MiB (100 %)  13.5 MiB/s in  5s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-pc-windows-gnu'

  stable-x86_64-pc-windows-gnu installed - rustc 1.80.1 (3f5fd8dd4 2024-08-06)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload its PATH environment variable to include
Cargo's bin directory (D:\toolchains\Rust\.cargo\bin).

Press the Enter key to continue.
```

