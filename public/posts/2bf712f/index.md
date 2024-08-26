# Vscode LSP补全

## [vsocde task Documents](https://code.visualstudio.com/Docs/editor/tasks)
&lt;!--more--&gt;


# markdown 与一些代码补全问题

&#43; 打开VScode的setting.json在[markdown]下新增或修改editor.quickSuggestions即可：

    ```json
    {
        &#34;[markdown]&#34;: {
            &#34;editor.quickSuggestions&#34;: {
                &#34;other&#34;: &#34;on&#34;,
                &#34;comments&#34;: &#34;on&#34;,
                &#34;strings&#34;: &#34;on&#34;
            },
        }
    }
    ```


 

---

> 作者:   
> URL: http://localhost:1313/CTGU-Hugo-Blog.io/posts/2bf712f/  

