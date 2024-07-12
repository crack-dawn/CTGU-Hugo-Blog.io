[TOC]

## 1.如何快速创建hugo博客仓库/个人笔记
> 请看： [hugo quick start](https://gohugo.io/getting-started/quick-start/)
>

## 2.我的操作步骤
1. 新建一个hugo博客仓库
```
hugo new site quickstart
cd quickstart
git init
```

2. 我使用的主题是[FixIt](https://github.com/hugo-fixit/FixIt)
添加并配置自己的主题：
```
git submodule add https://github.com/hugo-fixit/FixIt.git ./themes/
echo "theme = 'FixIt'" >> hugo.toml
hugo server
```
> tips: 如果因为编码问题乱码，可以尝试：直接打开hugo.toml文件，编辑内容，添加一行`theme = 'FixIt'` 保存即可
基础样例添加主题后修改为：
```
baseURL = 'https://example.org/'
languageCode = 'en-us'
title = 'My New Hugo Site'
theme = 'FixIt'
```

3. 添加自己的markdown笔记
操作命令如下
```
hugo new content content/posts/my-first-post.md
```
Start Hugo’s development server to see your changes, remembering to include draft content.


4. 好了，接下来我们测试下hugo的博客网站
```
hugo server -D
```

5. 测试完成后，我们使用git提交到github上
```
git add .
git commit -m "first commit"
git add remote origin https://github.com/你的仓库地址.git 
git push -u origin master
```

## 3.部署到github pages
1. 添加github pages
![QQ图片](noteImage/mk-2024-07-12-13-06-06-markdown.png)
