# npm proxy‘ config is set unproperly. See: ‘npm help config‘

问题：使用 npm publish发布项目依赖失败，报错 ` proxy' config is set properly. See: 'npm help config`

1、先查找一下自己的代理

```shell
npm config get proxy
npm config get https-proxy
npm config get registry
```

2、然后将代理和缓存置空
方式一：

```shell
npm config set proxy false
npm cache clean --force
```

方式二：

```shell
npm config set proxy null
npm config set https-proxy null
```

3、配置新的镜像源，选一个就行

## 淘宝源（推荐）

```shell
npm config set registry http://registry.npm.taobao.org/
```

## 官方源

```shell
npm config set registry http://registry.npmjs.org/
npm config set registry https://registry.npmjs.org/
```

## cnpm 源

```shell
npm config set registry https://registry.cnpmjs.org/
# 如果使用 cnpm，注意是否安装了 cnpm，cnpm 走推荐走的也是淘宝源
npm install -g cnpm --registry=https://registry.npm.taobao.org/
```

4、查看镜像使用状态

```shell
npm config get registry
```

* [ ] ty

> [青衫折扇 原文链接](https://blog.csdn.net/qq_42543264/article/details/140668297)
> 版权声明：本文为博主原创文章，遵循 CC 4.0 BY-SA 版权协议，转载请附上原文出处链接和本声明。

# 'vuepress' 不是内部或外部命令，也不是可运行的程序

```shell
npm install -g vuepress
```
