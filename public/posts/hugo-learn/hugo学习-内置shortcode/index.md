# Hugo学习-内置Shortcode


**Hugo** 提供了多个内置的 Shortcodes, 以方便作者保持 Markdown 内容的整洁。

&lt;!--more--&gt;

Hugo 使用 Markdown 为其简单的内容格式。但是，Markdown 在很多方面都无法很好地支持。你可以使用纯 HTML 来扩展可能性。

但这恰好是一个坏主意。大家使用 Markdown, 正是因为它即使不经过渲染也可以轻松阅读。应该尽可能避免使用 HTML 以保持内容简洁。

为了避免这种限制，Hugo 创建了 [shortcodes][shortcodes]。
shortcode 是一个简单代码段，可以生成合理的 HTML 代码，并且符合 Markdown 的设计哲学。

Hugo 附带了一组预定义的 shortcodes, 它们实现了一些非常常见的用法。
提供这些 shortcodes 是为了方便保持你的 Markdown 内容简洁。

{{&lt; admonition tip &#34;使用 Shortcodes&#34; &gt;}}

1. 带有原始字体串格式的 Shortcodes \` \`
2. 带有 Markdown 的 Shortcodes `% %`
3. 不带有 Markdown 的 Shortcodes `&lt; &gt;`

详见 [shortcodes/#use-shortcodes](https://gohugo.io/content-management/shortcodes/#use-shortcodes)

{{&lt; /admonition &gt;}}

## figure {#figure}

[`figure` 的文档][figure]

一个 `figure` 示例：

```markdown
{{&lt;/* figure src=&#34;/images/lighthouse.jpg&#34; title=&#34;Lighthouse (figure)&#34; */&gt;}}
```

呈现的输出效果如下：

{{&lt; figure src=&#34;/images/lighthouse.jpg&#34; title=&#34;Lighthouse (figure)&#34; &gt;}}

输出的 HTML 看起来像这样：

```html
&lt;figure&gt;
  &lt;img src=&#34;/images/lighthouse.jpg&#34; /&gt;
  &lt;figcaption&gt;
    &lt;h4&gt;Lighthouse (figure)&lt;/h4&gt;
  &lt;/figcaption&gt;
&lt;/figure&gt;
```

## gist

[`gist` 的文档][gist]

一个 `gist` 示例：

```markdown
{{&lt;/* gist spf13 7896402 */&gt;}}
```

呈现的输出效果如下：

{{&lt; gist spf13 7896402 &gt;}}

输出的 HTML 看起来像这样：

```html
&lt;script type=&#34;application/javascript&#34; src=&#34;https://gist.github.com/spf13/7896402.js&#34;&gt;&lt;/script&gt;
```

## highlight

[`highlight` 的文档][highlight]

一个 `highlight` 示例：

```markdown
{{&lt;/* highlight html */&gt;}}
&lt;section id=&#34;main&#34;&gt;
    &lt;div&gt;
        &lt;h1 id=&#34;title&#34;&gt;{{ .Title }}&lt;/h1&gt;
        {{ range .Pages }}
            {{ .Render &#34;summary&#34;}}
        {{ end }}
    &lt;/div&gt;
&lt;/section&gt;
{{&lt;/* /highlight */&gt;}}
```

呈现的输出效果如下：

{{&lt; highlight html &gt;}}
&lt;section id=&#34;main&#34;&gt;
    &lt;div&gt;
        &lt;h1 id=&#34;title&#34;&gt;{{ .Title }}&lt;/h1&gt;
        {{ range .Pages }}
            {{ .Render &#34;summary&#34;}}
        {{ end }}
    &lt;/div&gt;
&lt;/section&gt;
{{&lt; /highlight &gt;}}

## param
&lt;!--
[`param` 的文档][param]

一个 `param` 示例：

```markdown
{{&lt;/* param description */&gt;}}
```

呈现的输出效果如下：

{{&lt; param description &gt;}}

## ref 和 relref {#ref-and-relref}

[`ref` 和 `relref` 的文档][ref-and-relref]

## tweet

[`tweet` 的文档][tweet]

一个 `tweet` 示例：

```markdown
{{&lt;/* tweet user=&#34;SanDiegoZoo&#34; id=&#34;1453110110599868418&#34; */&gt;}}
```

呈现的输出效果如下：

{{&lt; tweet user=&#34;SanDiegoZoo&#34; id=&#34;1453110110599868418&#34; &gt;}}

## vimeo

[`vimeo` 的文档][vimeo]

一个 `vimeo` 示例：

```markdown
{{&lt;/* vimeo 146022717 */&gt;}}
```

呈现的输出效果如下：

{{&lt; vimeo 146022717 &gt;}}

## youtube

[`youtube` 的文档][youtube]

一个 `youtube` 示例：

```markdown
{{&lt;/* youtube w7Ft2ymGmfc */&gt;}}
```

呈现的输出效果如下：

{{&lt; youtube w7Ft2ymGmfc &gt;}}

[shortcodes]: https://gohugo.io/extras/shortcodes/
[figure]: https://gohugo.io/content-management/shortcodes#figure
[gist]: https://gohugo.io/content-management/shortcodes#gist
[highlight]: https://gohugo.io/content-management/shortcodes#highlight
[param]: https://gohugo.io/content-management/shortcodes#param
[ref-and-relref]: https://gohugo.io/content-management/shortcodes#ref-and-relref
[tweet]: https://gohugo.io/content-management/shortcodes#tweet
[vimeo]: https://gohugo.io/content-management/shortcodes#vimeo
[youtube]: https://gohugo.io/content-management/shortcodes#youtube --&gt;

---

> 作者: [Lruihao](https://lruihao.cn)  
> URL: http://localhost:1313/CTGU-Hugo-Blog.io/posts/hugo-learn/hugo%E5%AD%A6%E4%B9%A0-%E5%86%85%E7%BD%AEshortcode/  

