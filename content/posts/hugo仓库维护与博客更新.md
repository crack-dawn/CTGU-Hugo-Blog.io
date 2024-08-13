---
title: Hugo仓库维护与博客更新
subtitle:
date: 2024-08-11T22:56:40+08:00
slug: a116deb
draft: false
comment: true
summary:
description:
keywords:

message:   # 密码输入框预设提示
password:  # 解锁密码

layout:     # 布局归类：posts, archives, tags, categories等，文件分散在不同文件夹需要注明
archives:
tags: ['draft']
categories: ['draft']

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
## 如何hugo建站可以参考：
1. https://www.cnblogs.com/legenddog/p/17632687.html
2. 

## 从github拉取hugo博客仓库，本地部署

### 配置本地hugo环境
1. 安装hugo

参考地址：https://gohugo.io/installation/



### 拉取仓库到本地
直接拉取+clone子模块
```bash
git clone --recurse-submodules https://github.com/crack-dawn/CTGU-Hugo-Blog.io.git
``` 
忘记拉取子模块，可以进行
```bash
cd CTGU-Hugo-Blog.io
git submodule init
git submodule update --recursive
```


### vscode作为编辑器，配置hugo任务
```json
// https://code.visualstudio.com/docs/editor/tasks-appendix
{
    "version": "2.0.0",
    "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "new",
        "showReuseMessage": true,
        "clear": true
    },
    "options": {
        "cwd": "${workspaceFolder}",//默认进入到 hugo项目根目录
    },
    "problemMatcher": [],

    "tasks": [
        {//
            "type": "shell",
            "label": "1.hugo serve(本地构建测试)",
            "command": "hugo serve -D -e production --gc",
            "isBackground": true,
            "group": {
                "kind": "build",
                "isDefault": true
            }
        },
        {//
            "label": "2.open browser(打开本地浏览器)",
            "type": "shell",
            "windows":{
                "command": "start http://localhost:1313/CTGU-Hugo-Blog.io/",
            },
            "linux":{
                "command": "xdg-open http://localhost:1313/CTGU-Hugo-Blog.io/",
            },
            "osx":{
                "command": "open http://localhost:1313/CTGU-Hugo-Blog.io/",
            },
        },
        {//
            "label": "hugo new file(新建博客文件)",
            "type": "shell",
            "command":"hugo new ./content/posts/${input:newFileName}"
        },
        {
            "label": "3.open Document folder(打开./content/posts文件夹)",
            "type": "shell",
            "command": "code --goto ${workspaceFolder}/content/posts",
        }
    ],

    "inputs": [
        {
        "id": "newFileName",
        "type": "promptString",
        "default": "*.md",
        "description": "输入要创建的文件名"
        }
    ],
}

```


<!--more-->