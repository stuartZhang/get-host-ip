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
    let ip_config_pojo = ip_config_str.split("\r\n").fold(HashMap::new(), |mut map, line| {
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
                    ()
                })
            });
        }
        map
    });
    #[cfg(debug_assertions)]
    dbg!(&ip_config_pojo);
    if let Some(section) = ip_config_pojo.get(&cli_args.section[..]) {
        if let Some(value) = section.get(&cli_args.entry[..]) {
            print!("{value}");
        }
    }
    Ok(())
}
