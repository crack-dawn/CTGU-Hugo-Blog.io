---
title: 第一个尝试 关于FixIt插入视频
subtitle:
date: 2024-07-11T22:10:50+08:00
draft: false
comment: true
weight: 0
archives: [2024-07-11]
tags:  ["尝试","FixIt","插入视频"]
categories:
  - man!
summary:
resources:
  - name: featured-image
    src: featured-image.jpg
  - name: featured-image-preview
    src: featured-image-preview.jpg
content: true
toc: true
math: false
lightgallery: false
password:
message:
repost:
  enable: true
  url:

# See details front matter: https://fixit.lruihao.cn/documentation/content-management/introduction/#front-matter
---

<!--more-->

## 快速使用

如果视频只有一个部分，则仅需要视频的 BV `id`, 例如：

```code
https://www.bilibili.com/video/BV1Sx411T7QQ
```

一个 `bilibili` 示例：

```go-html-template
{{</* bilibili BV1Sx411T7QQ */>}}
或者
{{</* bilibili id=BV1Sx411T7QQ */>}}
```

呈现的输出效果如下：

{{< bilibili id=BV1Sx411T7QQ >}}

### 视频包含多个部分
如果视频包含多个部分，则除了视频的 BV `id` 之外，还需要 `p`, 默认值为 `1`, 例如：

```code
https://www.bilibili.com/video/BV16y421v7hE?p=3
```

一个带有 `p` 参数的 `bilibili` 示例：

```go-html-template
{{</* bilibili BV16y421v7hE 3 */>}}
或者
{{</* bilibili id=BV16y421v7hE p=3 */>}}
```

呈现的输出效果如下：
##### man梗指南
{{< bilibili id=BV16y421v7hE p=3 >}}
##### 曼波梗指南
{{< bilibili id=BV1Bt42137XA p=3 >}}

