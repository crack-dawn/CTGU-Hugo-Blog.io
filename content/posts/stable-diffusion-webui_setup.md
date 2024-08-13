---
title: Stable Diffusion Webui_setup
subtitle:
date: 2024-08-12T23:25:20+08:00
slug: 4d19e60
draft: false
comment: true
summary:
description:
keywords:

message:   # 密码输入框预设提示
password:  # 解锁密码

layout:     # 布局归类：posts, archives, tags, categories等，文件分散在不同文件夹需要注明
archives:
tags: ['stable-diffusion-webui']
categories: ['stable-diffusion']

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

## `stable-diffusion-webui`下载安装
首先请访问github项目：https://github.com/AUTOMATIC1111/stable-diffusion-webui ，

### Windows下的安装方式一
> 优点：安装简单，无需网络，直接下载安装，安装过程简单
> 缺点：版本可能滞后一些，但更加稳定可靠
Installation on `Windows 10/11` with `NVidia-GPUs` using release package
[Download sd.webui.zip from v1.0.0-pre and extract its contents:https://github.com/AUTOMATIC1111/stable-diffusion-webui/releases/tag/v1.0.0-pre](https://github.com/AUTOMATIC1111/stable-diffusion-webui/releases/tag/v1.0.0-pre)
+ Unzip `sd.webui.zip` to a folder of your choice.
+ Run `update.bat`. 双击两次
+ Run `run.bat`.    双击两次
完成初始化安装操
###  Windows下的安装方式二
> 缺点：需要网络良好，拉取并构建一堆python包，过程漫长，25~40分钟
> 优点：可以获取最新的 `stable-diffusion-webui` 版本
+ Install [Python 3.10.6](https://www.python.org/downloads/release/python-3106/) (Newer version of Python does not support torch), checking "Add Python to PATH".
+ Install [Git](https://git-scm.com/downloads).
+ Download the stable-diffusion-webui repository, for example by running 
```bash
git clone https://github.com/AUTOMATIC1111/stable-diffusion-webui.git
```
+ Then, run `webui-user.bat` from `Windows Explorer` as normal, non-administrator, user.Just click `webui-user.bat` twice to run it.
完成初始化安装操作
---

## [如何使用](https://www.cnblogs.com/deali/p/17275651.html#docker-build-%E4%BB%A3%E7%90%86)
> 刚开始肯定一脸懵逼，咋画不出高大上的效果啊？
> 提示：可以用[`colab`](https://www.bilibili.com/video/BV1Qz421Q7yn/?share_source=copy_web&vd_source=2ef13f5d59f10f604d872c33f966a20b)练手，使用python编程进行AI绘画。
1. prompt提示词

首先，得学会使用 prompt，也就是生成图片的描述，Stable Diffusion 通过英文文字内容来描述场景或物体，以此来决定生成的图像中会出现什么。文字描述是决定图像生成质量的关键因素。具体如何写 prompt 不在本文讨论范围，请自行搜索相关文章，网上很多。

这几个网站可以按提示组合生成 prompt：
> https://promptomania.com/stable-diffusion-prompt-builder/
> https://weirdwonderfulai.art/resources/disco-diffusion-modifiers/

这几个网站有很多人分享的成品图和描述文案：

> https://prompthero.com/
> https://openart.ai/


2. 模型
   
模型是个统称，其中包含了Checkpoints、LORA、Texture之类的，不细说了，按照模型网站上的教程安装使用就行。
然后模型的话，可以去这几个网站下载：
+ liblib绘画：[liblib绘画](https://www.liblib.art/) 即 https://www.liblib.art/
+ huggingface抱脸:[huggingface](https://huggingface.co/models?sort=trending&search=stable-diffusion)，即 https://civitai.com/
+ civitai:[civitai](https://civitai.com/search/models?sortBy=models_v9&query=nahida) 即 https://civitai.com/




---
<!-- 
nahidaGenshinImpactV3-LyCORIS  
```
NAHIDA \(GENSHIN IMPACT\)
IN-GAME DRESS
HAIR ORNAMENT
BLOOMERS
BRACELET
CAPE
STIRRUP LEGWEAR
CROSS-SHAPED PUPILS
```

 -->
<!-- 
## `stable-diffusion-webui`项目文件结构
stable-diffusion-webui项目文件结构：
```BASH
/stable-diffusion-webui (master)
$ tree -L 1
.
|-- CHANGELOG.md 记录了项目的变更历史。
|-- CITATION.cff 提供了如何引用此项目的规范。
|-- CODEOWNERS 定义了仓库中不同部分的所有者和审查者。
|-- LICENSE.txt 包含项目的许可协议。
|-- README.md 通常包含项目的简介、安装指南和使用说明。
|-- __pycache__ Python 缓存文件夹，存放编译后的 .pyc 文件。
|-- _typos.toml 可能是用于拼写检查的配置文件。
|-- cache  存储临时缓存数据。
|-- config_states 可能存储了配置状态文件。
|-- configs 配置文件目录，可能包括不同的模型配置。
|-- embeddings 
|-- environment-wsl2.yaml 针对 WSL2 环境的依赖配置文件。
|-- extensions 用户安装的扩展插件目录。
|-- extensions-builtin 内置扩展插件目录。
|-- html 存放 HTML 文件，可能是用户界面的一部分
|-- javascript 存放 JavaScript 文件，用于前端交互。
|-- launch.py 启动脚本，用于启动 Web UI。
|-- localizations 国际化和本地化文件，如翻译字符串。
|-- models 存储预训练模型文件。
|-- modules Python 模块文件夹，包含了项目的逻辑实现。
|-- outputs 生成的输出文件夹，如生成的图像。
|-- package.json Node.js 项目的配置文件，用于管理前端依赖。
|-- params.txt  可能是模型参数或默认参数的文本文件。
|-- pyproject.toml 项目配置文件，用于构建系统和依赖管理。
|-- repositories 存储外部依赖的克隆仓库。
|-- requirements-test.txt
|-- requirements.txt Python项目依赖列表，用于构建和运行项目。
|-- requirements_npu.txt
|-- requirements_versions.txt
|-- screenshot.png 
|-- script.js 可能是前端脚本文件。
|-- scripts 存放脚本文件 
|-- style.css  CSS 样式表文件，用于前端样式。
|-- test 测试文件夹，存放单元测试或其他测试相关文件。
|-- textual_inversion_templates 文本反转模板，用于文本到图像的转换
|-- tmp 临时文件夹，用于存储临时文件。
|-- ui-config.json  用户界面配置文件。
|-- venv Python 虚拟环境文件夹,默认3.10.6版本，且该项目的python包默认储存在这里，不污染全局环境
|-- webui-macos-env.sh MacOS操作系统的用户初始化，安装和启动脚本。
|-- webui-user.bat windows操作系统的用户初始化，安装和启动脚本。
|-- webui-user.sh  linux操作系统的用户初始化，安装和启动脚本。
|-- webui.bat windows操作系统的启动脚本。
|-- webui.py 启动脚本，用于启动 Web UI。
`-- webui.sh linux操作系统的启动脚本。
``` -->

<!--more-->