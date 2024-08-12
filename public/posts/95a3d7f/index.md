# Git基础配置




## Git 基本配置
安装 `Git`,以及本地使用`git bash`
1. 下载，并在本地安装git
从这里 https://git-scm.com/downloads ，下载安装适配的git bash版本;

2. 配置用户名和邮箱
   
随便一个位置鼠标右键打开`Git Bash命令行终端`，执行下列命令，配置用户名和邮箱。
```bash
git config --global user.name &#34;用户名&#34;
git config --global user.email &#34;你的邮箱&#34;
```
3. 查看你的用户名和邮箱配置信息，确认`用户名`和`你的邮箱`都填写无误。
可以在 Git bash 使用如下命令，
```bash
git config --global user.name
git config --global user.email
```
 

## 与 Github 创建连接
&gt; 创建链接，用于本地与Github服务端进行交互。
&gt; 不进行该操作，无法git clone 操作。

如果网站部署在 Github 上的话，需要本地与 Github服务端 创建链接。
应该进行：


1. 生成本地本机公钥文件；
在本地打开`Git Bash命令行终端`，输入这行指令，创建公钥文件；
```BASH
ssh-keygen -t rsa 
```

将SSH key 添加到 ssh-agent
```BASH
ssh-add ~/.ssh/id_rsa
```
&gt; 其中 `ssh-keygen -t rsa  -C &#34;***@163.com&#34;`
&gt; -C 参数并不是必需的;
&gt; -C 参数是用来添加一个`注释`到你的密钥中的，通常可以用来标识密钥的所有者或者用途，例如使用`你的电子邮件地址`作为注释。

2. 在`Git Bash命令行终端`输入这条指令查看`公钥`，并==复制保存`公钥`==。

```BASH
cat ~/.ssh/id_rsa.pub
```
&gt; 也可以直接到C:\Users\你的用户名\.ssh\文件夹下查看 id_rsa.pub 文件

3. 然后,在Github网页端， 添加 `SSH公钥`。
登录 GitHub，点击右上角头像，进入设置，把 SSH 公钥填进去就 OK 了。
  
4. 最后，在本地打开 Git Bash，输入下面的命令,验证连接 状态。
```BASH
ssh -T git@github.com
```

## Git clone项目
1. 在Github上，找到需要clone的项目，点击`Code`按钮，复制`SSH`链接。
```BASH
git clone  ssh链接
```
2. 如果clone的项目有子模块，需要使用`--recurse-submodules`参数。
```BASH
git clone --recurse-submodules  ssh链接 
```

## 参考文章
&#43; [&lt;零基础学习Git&gt; -一条狗的传说](https://www.cnblogs.com/legenddog/p/17632687.html)
&lt;!--more--&gt;

---

> 作者: Dawn  
> URL: http://localhost:1313/CTGU-Hugo-Blog.io/posts/95a3d7f/  

