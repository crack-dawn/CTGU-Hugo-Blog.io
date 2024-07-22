# 1.hugo尝试-Fixit插入视频


&lt;!--more--&gt;

## 快速使用

如果视频只有一个部分，则仅需要视频的 BV `id`, 例如：

```code
https://www.bilibili.com/video/BV1Sx411T7QQ
```

一个 `bilibili` 示例：

```go-html-template
{{&lt;/* bilibili BV1Sx411T7QQ */&gt;}}
或者
{{&lt;/* bilibili id=BV1Sx411T7QQ */&gt;}}
```

呈现的输出效果如下：

{{&lt; bilibili id=BV1Sx411T7QQ &gt;}}

### 视频包含多个部分
如果视频包含多个部分，则除了视频的 BV `id` 之外，还需要 `p`, 默认值为 `1`, 例如：

```code
https://www.bilibili.com/video/BV16y421v7hE?p=3
```

一个带有 `p` 参数的 `bilibili` 示例：

```go-html-template
{{&lt;/* bilibili BV16y421v7hE 3 */&gt;}}
或者
{{&lt;/* bilibili id=BV16y421v7hE p=3 */&gt;}}
```

呈现的输出效果如下：
##### man梗指南
{{&lt; bilibili id=BV16y421v7hE p=3 &gt;}}
##### 曼波梗指南
{{&lt; bilibili id=BV1Bt42137XA p=3 &gt;}}



---

> 作者: [crack-dawn](https://github.com/crack-dawn/)  
> URL: http://localhost:1313/CTGU-Hugo-Blog.io/posts/hugo-learn/1.hugo%E5%B0%9D%E8%AF%95-fixit%E6%8F%92%E5%85%A5%E8%A7%86%E9%A2%91/  

