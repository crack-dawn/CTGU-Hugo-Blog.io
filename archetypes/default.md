---
title: {{ replace .TranslationBaseName "-" " " | title }}
subtitle:
date: {{ .Date }}
slug: {{ substr .File.UniqueID 0 7 }}
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

<!--more-->