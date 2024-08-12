# Hugo仓库维护与博客更新

## 如何hugo建站可以参考：
1. https://www.cnblogs.com/legenddog/p/17632687.html
2. 

## 从github拉取hugo博客仓库，本地部署

### 配置本地hugo环境
1. 安装hugo

参考地址：https://gohugo.io/installation/



### 拉取仓库到本地
直接拉取&#43;clone子模块
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
    &#34;version&#34;: &#34;2.0.0&#34;,
    &#34;tasks&#34;: [
        {//
            &#34;type&#34;: &#34;shell&#34;,
            &#34;label&#34;: &#34;1.hugo serve(本地构建测试)&#34;,
            &#34;command&#34;: &#34;hugo serve -D -e production --gc&#34;,
            &#34;isBackground&#34;: true,
            &#34;group&#34;: {
                &#34;kind&#34;: &#34;build&#34;,
                &#34;isDefault&#34;: true
            }
        },
        {//
            &#34;label&#34;: &#34;2.open browser(打开本地浏览器)&#34;,
            &#34;type&#34;: &#34;shell&#34;,
            &#34;windows&#34;:{
                &#34;command&#34;: &#34;start http://localhost:1313/CTGU-Hugo-Blog.io/&#34;,
            },
            &#34;linux&#34;:{
                &#34;command&#34;: &#34;xdg-open http://localhost:1313/CTGU-Hugo-Blog.io/&#34;,
            },
            &#34;osx&#34;:{
                &#34;command&#34;: &#34;open http://localhost:1313/CTGU-Hugo-Blog.io/&#34;,
            },
        },
        {//
            &#34;label&#34;: &#34;hugo new file(新建博客文件)&#34;,
            &#34;type&#34;: &#34;shell&#34;,
            &#34;command&#34;:&#34;hugo new ./content/posts/${input:newFileName}&#34;
        },
    ],

    &#34;inputs&#34;: [
        {
        &#34;id&#34;: &#34;newFileName&#34;,
        &#34;type&#34;: &#34;promptString&#34;,
        &#34;default&#34;: &#34;*.md&#34;,
        &#34;description&#34;: &#34;输入要创建的文件名&#34;
        }
    ],
}
```


&lt;!--more--&gt;

---

> 作者:   
> URL: http://localhost:1313/CTGU-Hugo-Blog.io/posts/a116deb/  

