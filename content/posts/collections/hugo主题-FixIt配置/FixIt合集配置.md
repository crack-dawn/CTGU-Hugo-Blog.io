---
title: 合集配置踩坑记录
date: 2024-06-28T22:29:03+08:00

tags: [ 'draft' ]
categories: [ 'hugo主题-FixIt配置']
collections: ['hugo主题-FixIt配置']
keywords:
  - CSS
  - scrollbar-styling
  - scrollbar-color
  - scrollbar-width
---

+ FixIt内容的目录结构
    ``` TEXT
    -|(内容)content
            -|posts
                    -|分类(categeries) 
                    -|合集(collections)
                            -|index.md     标注合集大类的名称
                            -|合集1文章存放处
                                -|index.md  (标注合集1的名称，一般和文件夹同名)
                                -|文章1.md
                            -|合集2文章存放处
                                -|index.md  (标注合集2的名称  一般和文件夹同名)
                                -|文章6.md
            -|friends
            -|about
            -|reward
    ```
---
## `FixIt`主题，建立`文档合集`
+ 最好将`合集文章`放入 `content/posts/collections/` 目录下💡；或是通过，在`合集文章`开头加上 `collections: ["集合1"]` 将文章归入对应`合集`。

### `文档合集`Fixt主题的相关设置
+ 开启`[page]`的这些设置项。
    ```TEXT
    # FixIt 0.3.0 | 新增 是否在侧边栏显示集合列表
    collectionList = true
    # FixIt 0.3.0 | 新增 是否在文章末尾显示集合导航
    collectionNavigation = true
    ```

+ 注意`taxonomies.toml`配置项,务必添加 `collection = "collections"`，不要少s，也不要多s,如下配置

    ```toml
    # -------------------------------------------------------------------------------------
    # Taxonomies Configuration
    # `taxonomies.toml`
    # See: https://gohugo.io/content-management/taxonomies/#configure-taxonomies
    # -------------------------------------------------------------------------------------

    collection = "collections"
    category = "categories"
    tag = "tags"
    series = 'series'
    ```
+ 最后，`合集文章`开头记得加上 `collections: ["集合1"]` 将文章归入对应`合集`。=（不要少写s）=



## `FixIt` 的`markup.toml`的`[goldmark]`导致的错误：
```text
ERROR error calling resources.GetRemote: Get "https://publish.twitter.com/oembed?dnt=true&url=https%3A%2F%2Ftwitter.com%2FSanDiegoZoo%2Fstatus%2F1453110110599868418": dial tcp 174.36.228.136:443: connectex: A connection attempt failed because the connected party did not properly respond after a period of time, or established connection failed because connected host has failed to respond.
Built in 22619 ms
Error: error building site: logged 1 error(s)
```

```TOML
# -------------------------------------------------------------------------------------
# Markup related configuration in Hugo
# See: https://gohugo.io/getting-started/configuration-markup/
# -------------------------------------------------------------------------------------
# Syntax Highlighting (https://gohugo.io/content-management/syntax-highlighting)
# Table Of Contents settings
[tableOfContents]
  ordered = true
  startLevel = 2
  endLevel = 7
########## necessary configurations ##########
[highlight]

  # https://github.com/Lruihao/FixIt/issues/43
  codeFences = true
  lineNos = true
  lineNumbersInTable = true
  noClasses = false
  guessSyntax = true
########## necessary configurations ##########

# [goldmark] # 默认 FixIt 的配置，用户不适宜改动，不然莫名出现如上的错误
    # _merge = "shallow"
# # Goldmark is from Hugo 0.60 the default library used for Markdown
# [goldmark]
#   [goldmark.extensions]
#     definitionList = true
#     footnote = true
#     linkify = true
#     strikethrough = true
#     table = true
#     taskList = true
#     typographer = true
#   [goldmark.parser]
#     autoHeadingID = true
#     autoHeadingIDType = "github"
#     wrapStandAloneImageWithinParagraph = true
#     [goldmark.parser.attribute]
#       block = true
#       title = true
#   [goldmark.renderer]
#     # By default, Goldmark ignores newlines within a paragraph. Set to true to render newlines as <br> elements.
#     hardWraps = false
#     # whether to use HTML tags directly in the document
#     unsafe = true
```

踩坑有点多了😶
