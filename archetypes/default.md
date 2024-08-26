---
title: {{ replace .TranslationBaseName "-" " " | title }}
subtitle:
date: {{ .Date }}
slug: {{ substr .File.UniqueID 0 7 }}
summary:
message:   # 密码预设提示
password:  # 解锁密码

type: posts 
tags: [ ]
categories: ['draft']
collocations: ['draft']

repost: # 转载链接
  enable: true
  url:
toc: true
math: true
comment: true
---
