# get-host-ip

这是一款用`rust`制作的命令行工具。其被设计用来从`C:\Windows\System32\ipconfig.exe`的执行结果内提取出指定【网卡】的属性值。比如，从电脑本的【无线局域网适配器`WLAN`】中提取出【`IPv4`地址】字符串值。

## 使用场景

从`Windows Subsystem for Linux`（比如，`Ubuntu-20.04`）读取其宿主主机（比如，`Windows 11`）无线物理网卡的`IP`地址值。进而，将其作为两项重要功能的配置项入参：

1. 投影`WSL2`图形界面至`Windows`宿主桌面系统显示。
   * 无论是将整个`Gnome`桌面系统投影作为`Windows`环境下的一个应用程序窗口，还仅只是投影某个`Linux GUI`应用程序，`X11 Server`都需要借助`$DISPLAY`环境变量明确地知晓`XSTATA`的准备`IP`位置。请不要自做聪明地认为`127.0.0.1`可能搪塞过去。`127.0.0.1`是指向`WSL2`子系统自身，而不是宿主主机。
   * 关于`WSL2`桌面投影的更多技术细节，可参见我早先的另一篇文章[为 Rust+QT 编程搭建【伪】win32 开发环境](https://rustcc.cn/article?id=96458b90-9e62-44fd-8155-afe9642d4170)。还算是详细吧！
2. 使`WSL2`子系统共用`Windows`宿主环境的`VPN`客户端。
   * 从操作上，`export HTTP_PROXY=***`要比在`Linux`系统里安装与配置`Clash`要省心多了。
   * 从后续维护上，升级与更新配置仅需要做一遍操作。
   * 从经济上，限制在线客户端数量也更便宜。

## 曾经的纠结

早先我也曾经使用`nodejs`脚本程序实现了相同的功能。但，`node`虚拟机首次启动时间着实有些长了。这个长延时在`bash`命令行交互过程中并不明显，毕竟咱敲键盘也不快，还时不时地敲错字母。而，我将该命令执行于`Shell`会话初始化的`.bashrc`脚本里了，以在每个`Shell`会话上下文中都预置好`$DISPLAY`与`$HTTP_PROXY`环境变量。这导致每次启动`WSL2`系统，甚至仅只是登录会话，都慢得叫人怀疑：“是不是哪里崩溃了？”。我掐表计时过大约`10`秒左右，不知道慢到哪里去了？

再更新到此原生版本的命令行的指令之后，半秒内就可完成`WSL2`的`Shell`会话登录。

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

## 进行中的纠结，求帮助
