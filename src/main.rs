//! 获取宿主机器的 IP 地址
use ::clap::Parser;
use ::regex::Regex;
use ::std::{collections::HashMap, error::Error, process::Command};
fn main() -> Result<(), Box<dyn Error>> {
    /// 获取 wsl 宿主机器的物理 IP 地址
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct CliArgs {
        /// ipconfig.exe 返回结果中的【主分类】标题
        #[arg(short, long, default_value_t = String::from("无线局域网适配器 WLAN"))]
        section: String,
        /// ipconfig.exe 返回结果中的【主分类】下各个条目的标签名
        #[arg(short, long, default_value_t = String::from("IPv4 地址"))]
        entry: String,
    }
    let cli_args = CliArgs::parse();
    let output = Command::new("ipconfig.exe").output()?.stdout;
    let ip_config_str = iconv_sys::decode(output.as_slice(), "cp936")?;
    #[cfg(debug_assertions)]
    println!("{ip_config_str}");
    let ip_config_pojo = build_map(&ip_config_str[..], None)?;
    #[cfg(debug_assertions)]
    dbg!(&ip_config_pojo);
    ip_config_pojo.get(&cli_args.section[..]).and_then(|section| {
        section.get(&cli_args.entry[..])
    }).map(|value| {
        print!("{value}");
    });
    Ok(())
}
fn build_map(input_str: &str, separator: Option<&str>) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn Error>> {
    let section = Regex::new(r"^\S+")?;
    let record = Regex::new(r"^\s+")?;
    let rear_colon = Regex::new(r"^\s*|:\s*$")?;
    let partition = Regex::new(r"(?x)
        \s*
        (?<key>[^\.]+)
        (?:\.\s)+:\s*
        (?<value>.+)
        \s*
    ")?;
    let mut last_section_key: Option<String> = None;
    Ok(input_str.split(separator.or(Some("\r\n")).unwrap()).fold(HashMap::new(), |mut map, line| {
        if section.is_match(line) {
            let section_key = rear_colon.replace_all(line, "").into_owned();
            map.entry(section_key.clone()).or_insert(HashMap::new());
            last_section_key.replace(section_key);
        } else if record.is_match(line) {
            last_section_key.as_ref().and_then(|section_key| {
                map.get_mut(section_key)
            }).map(|sub_map| {
                partition.captures(line).map(|groups| {
                    let key = groups["key"].trim();
                    let value = &groups["value"];
                    sub_map.insert(key.to_string(), value.to_string());
                })
            });
        }
        map
    }))
}
#[cfg(test)]
mod test {
    #[test]
    fn test_build_map(){
        const SOURCE_TEXT: &str = r"
Windows IP 配置


以太网适配器 以太网:

    媒体状态  . . . . . . . . . . . . : 媒体已断开连接
    连接特定的 DNS 后缀 . . . . . . . :

无线局域网适配器 本地连接* 1:

    媒体状态  . . . . . . . . . . . . : 媒体已断开连接
    连接特定的 DNS 后缀 . . . . . . . :

无线局域网适配器 本地连接* 2:

    媒体状态  . . . . . . . . . . . . : 媒体已断开连接
    连接特定的 DNS 后缀 . . . . . . . :

以太网适配器 VMware Network Adapter VMnet1:

    连接特定的 DNS 后缀 . . . . . . . :
    本地链接 IPv6 地址. . . . . . . . : fe80::7276:59f4:8f49:3b04%10
    IPv4 地址 . . . . . . . . . . . . : 192.168.88.1
    子网掩码  . . . . . . . . . . . . : 255.255.255.0
    默认网关. . . . . . . . . . . . . :

以太网适配器 VMware Network Adapter VMnet8:

    连接特定的 DNS 后缀 . . . . . . . :
    本地链接 IPv6 地址. . . . . . . . : fe80::ba20:90bb:a5cf:5f76%4
    IPv4 地址 . . . . . . . . . . . . : 192.168.23.1
    子网掩码  . . . . . . . . . . . . : 255.255.255.0
    默认网关. . . . . . . . . . . . . :

以太网适配器 以太网 3:

    媒体状态  . . . . . . . . . . . . : 媒体已断开连接
    连接特定的 DNS 后缀 . . . . . . . :

无线局域网适配器 WLAN:

    连接特定的 DNS 后缀 . . . . . . . :
    IPv6 地址 . . . . . . . . . . . . : 2408:8210:3013:430:ea70:b108:de91:f06f
    临时 IPv6 地址. . . . . . . . . . : 2408:8210:3013:430:3d81:f4d1:86f3:5ae6
    本地链接 IPv6 地址. . . . . . . . : fe80::ab5f:4811:e399:9c2c%16
    IPv4 地址 . . . . . . . . . . . . : 192.168.18.12
    子网掩码  . . . . . . . . . . . . : 255.255.255.0
    默认网关. . . . . . . . . . . . . : fe80::1%16
                                        192.168.18.1

以太网适配器 以太网 2:

    媒体状态  . . . . . . . . . . . . : 媒体已断开连接
    连接特定的 DNS 后缀 . . . . . . . :
";
        let pojo = super::build_map(SOURCE_TEXT, Some("\n")).unwrap();
        assert_eq!(pojo["无线局域网适配器 WLAN"]["IPv4 地址"], "192.168.18.12");
    }
}
