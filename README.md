# get-host-ip

这是一款用`rust`制作的命令行工具。其被设计用来从`C:\Windows\System32\ipconfig.exe`的执行结果内提取出指定【网卡】的属性值。比如，从电脑本的【无线局域网适配器`WLAN`】中提取出【`IPv4`地址】字符串值。

## 使用场景

从`Windows Subsystem for Linux`（比如，`Ubuntu-20.04`）读取其宿主主机（比如，`Windows 11`）无线物理网卡的`IP`地址。进而，将其作为两项重要功能的配置项入参：

1. 投影`WSL2`图形界面至`Windows`宿主桌面系统显示。
   * 无论是将整个`Gnome`桌面系统投影作为`Windows`环境下的一个应用程序窗口，还仅只是投影某个`Linux GUI`应用程序，`X11 Server`都需要借助`$DISPLAY`环境变量明确地知晓`XSTATA`的准确`IP`位置。请不要自做聪明地认为`127.0.0.1`可能搪塞过去。`127.0.0.1`是指向`WSL2`子系统自身，而不是宿主主机。
   * 关于`WSL2`桌面投影的更多技术细节，可参见我早先的另一篇文章[为 Rust+QT 编程搭建【伪】win32 开发环境](https://rustcc.cn/article?id=96458b90-9e62-44fd-8155-afe9642d4170)。还算是详细吧！
2. 使`Windows`宿主操作系统共享`VPN`客户端给`WSL2`子系统。
   * 从操作上，执行`export HTTP_PROXY=***`指令要比在`Linux`系统里安装与配置另一个`Clash`实例要省心多了。
   * 从后续维护上，对`VPN`客户端的软件升级与配置更新仅需要做一遍操作更节省时间。
   * 从经济上，更少的在线客户端数量也更便宜。

## 曾经的纠结

早先我也曾经使用`nodejs`脚本程序实现了相同的功能。但，`node`虚拟机首次启动时间着实有些长了。虽然该长延时弊端在交互式`bash`上下文中并不明显（毕竟咱敲键盘也不快，还时不时地敲错字母），但当被置于`.bashrc`内执行时，其造成的`shell`会话初始化阻滞就让人有点不好接受了（毕竟，每次开个新`shell`窗口都得明显黑幕个把秒钟，体验着实有些儿差）。所以，我决心使用`rust`重构此“热”指令。效果明显，几乎一闪便可完成`shell`初始化。

## 命令行指令-用法

```shell
$ get-host-ip --help
获取 wsl 宿主机器的物理 IP 地址

Usage: get-host-ip [OPTIONS]

Options:
  -s, --section <SECTION>  ipconfig.exe 返回结果中的【主分类】标题 [default: "无线局域网适配器 WLAN"]
  -e, --entry <ENTRY>      ipconfig.exe 返回结果中的【主分类】下各个条目的标签名 [default: "IPv4 地址"]
  -h, --help               Print help
  -V, --version            Print version
```

### 更详细图例

![参数图](https://user-images.githubusercontent.com/13935927/234434832-a94dbc37-a40d-454e-bd7f-619e723ef671.png)

### 返回结果

`get-host-ip`执行输出就是没有结尾换行符的`IP`地址字符串。若将该指令添加入`$PATH`，那么在`Shell`求值表达式内可以直接

```shell
export HOST_IP=$(get-host-ip);
```

## 链接库依赖

因为`Windows cmd`指令输出文本内容的字符集是`cp936`，而不是`UTF-8`。所以，`get-host-ip`需要依赖操作系统预置的字符集转换动态链接库`libiconv`，来完成`cp936 -> UTF-8`的字符集转换。

### `Linux`操作系统

大部分主流`Linux OS`都包含有`libiconv`。若你的`Linux OS`版本比较早或是`compact`版而缺失了`libiconv`也不必慌。按如下方式补装即可：

```shell
wget http://ftp.gnu.org/pub/gnu/libiconv/libiconv-1.9.1.tar.gz
tar -xzvf libiconv-1.9.1.tar.gz
cd libiconv-1.9.1.tar.gz
./configure --prefix=/usr/local
sudo make -j8
sudo make install
sudo ln -s /usr/local/lib/libiconv.so /usr/lib/libiconv.so
sudo ln -s /usr/local/lib/libiconv.so.2 /usr/lib/libiconv.so.2
```

### `Windows`操作系统

要么，从[setup](https://sourceforge.net/projects/gnuwin32/files/libiconv/1.9.2-1/libiconv-1.9.2-1.exe/download?use_mirror=jaist&download=)直接下载安装包，并本地安装之。缺点就是会“污染”你的`PATH`环境变量。

要么，从[binary](https://sourceforge.net/projects/gnuwin32/files/libiconv/1.9.2-1/libiconv-1.9.2-1-bin.zip/download?use_mirror=jaist&download=)下载预编译包。在解压缩之后，将其下的`bin`目录添加到你的编译环境变量`RUST_FLAGS`内。比如，

```shell
set RUST_FLAGS=-L C:\libiconv-1.9.2-1-bin\bin
```

### 吐槽

同一款`libiconv`链接库怎么对`Linux`与`Windows`操作系统提供了**不同名**的导出函数呢？这个“缺德的”命名差异导致我在【编译期·动态链接】环节卡住了好几天。相对于`Linux`版的链接库导出函数名，`Windows`版的每个导出函数都有一个`lib`前缀 —— 故意的吧？真要命。
