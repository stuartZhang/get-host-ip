//! 获取宿主机器的 IP 地址
use ::regex::Regex;
use ::std::{collections::HashMap, error::Error, process::Command};
fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("ipconfig.exe").output()?.stdout;
    let ip_config_str = iconv::decode(output.as_slice(), "cp936")?;
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
    let ip_config_pojo: HashMap<String, HashMap<String, String>> = ip_config_str.split("\r\n").fold(HashMap::new(), |mut map, line| {
        if section.is_match(line) {
            let section_key = rear_colon.replace_all(line, "").into_owned();
            map.entry(section_key.clone()).or_insert(HashMap::new());
            last_section_key.replace(section_key);
        } else if record.is_match(line) {
            if let Some(section_key) = last_section_key.as_ref() {
                if let Some(sub_map) = map.get_mut(section_key) {
                    if let Some(groups) = partition.captures(line) {
                        let key = groups["key"].trim();
                        let value = &groups["value"];
                        sub_map.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
        map
    });
    if let Some(section) = ip_config_pojo.get("无线局域网适配器 WLAN") {
        if let Some(value) = section.get("IPv4 地址") {
            print!("{value}");
        }
    }
    Ok(())
}
