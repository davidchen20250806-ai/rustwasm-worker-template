use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use cron::Schedule;
use csscolorparser::Color;
use html_escape::{decode_html_entities, encode_text};
use ipnetwork::Ipv4Network;
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use qrcode::render::svg;
use qrcode::QrCode;
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};
use sqlformat::{format, FormatOptions, Indent, QueryParams};
use std::net::Ipv4Addr;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

// --- 1. SQL Formatter ---
pub fn format_sql(sql: &str) -> String {
    let options = FormatOptions {
        indent: Indent::Spaces(4),
        uppercase: true,
        lines_between_queries: 1,
    };
    format(sql, &QueryParams::None, options)
}

// --- 2. Text Diff ---
#[derive(Serialize)]
pub struct DiffChunk {
    pub tag: String,
    pub text: String,
}
#[derive(Serialize)]
pub struct DiffResponse {
    pub chunks: Vec<DiffChunk>,
}
pub fn compute_diff(old_text: &str, new_text: &str) -> DiffResponse {
    let diff = TextDiff::from_lines(old_text, new_text);
    let mut chunks = Vec::new();
    for change in diff.iter_all_changes() {
        let tag = match change.tag() {
            ChangeTag::Delete => "delete",
            ChangeTag::Insert => "insert",
            ChangeTag::Equal => "equal",
        };
        chunks.push(DiffChunk {
            tag: tag.to_string(),
            text: change.value().to_string(),
        });
    }
    DiffResponse { chunks }
}

// --- 3. Cron Generator ---
#[derive(Serialize)]
pub struct CronResponse {
    pub valid: bool,
    pub next_runs: Vec<String>,
    pub error: String,
}
pub fn check_cron(expression: &str) -> CronResponse {
    // 兼容 Linux Crontab (5位) 格式，自动补全秒位 (6位)
    let expr = if expression.trim().split_whitespace().count() == 5 {
        format!("0 {}", expression)
    } else {
        expression.to_string()
    };

    match Schedule::from_str(&expr) {
        Ok(schedule) => {
            let next_runs = schedule
                .upcoming(Utc)
                .take(5)
                .map(|d| d.to_string())
                .collect();
            CronResponse {
                valid: true,
                next_runs,
                error: String::new(),
            }
        }
        Err(e) => CronResponse {
            valid: false,
            next_runs: vec![],
            error: e.to_string(),
        },
    }
}

// --- 4. Subnet Calculator ---
#[derive(Serialize)]
pub struct SubnetResponse {
    pub valid: bool,
    pub ip: String,
    pub cidr: String,
    pub mask: String,
    pub wildcard: String,
    pub network: String,
    pub broadcast: String,
    pub first_ip: String,
    pub last_ip: String,
    pub total_hosts: u64,
    pub usable_hosts: u64,
    pub ip_class: String,
    pub ip_type: String,
    pub binary_ip: String,
    pub binary_mask: String,
}
pub fn calculate_subnet(ip_str: &str, prefix: u8) -> SubnetResponse {
    let ip: Ipv4Addr = match ip_str.parse() {
        Ok(i) => i,
        Err(_) => return empty_subnet(),
    };
    let net = match Ipv4Network::new(ip, prefix) {
        Ok(n) => n,
        Err(_) => return empty_subnet(),
    };

    let mask = net.mask();
    let network_addr = net.network();
    let broadcast_addr = net.broadcast();
    let total = net.size() as u64;
    let usable = if total > 2 { total - 2 } else { 0 };

    let first = if total > 2 {
        Ipv4Addr::from(u32::from(network_addr) + 1)
    } else {
        network_addr
    };
    let last = if total > 2 {
        Ipv4Addr::from(u32::from(broadcast_addr) - 1)
    } else {
        broadcast_addr
    };
    let wildcard = Ipv4Addr::from(!u32::from(mask));

    let first_octet = ip.octets()[0];
    let class = if first_octet < 128 {
        "A 类"
    } else if first_octet < 192 {
        "B 类"
    } else if first_octet < 224 {
        "C 类"
    } else if first_octet < 240 {
        "D 类 (组播)"
    } else {
        "E 类 (保留)"
    };
    let ip_type = if ip.is_private() {
        "私有 (Private)"
    } else if ip.is_loopback() {
        "回环 (Loopback)"
    } else if ip.is_multicast() {
        "组播 (Multicast)"
    } else {
        "公网 (Public)"
    };

    let to_bin = |addr: Ipv4Addr| {
        addr.octets()
            .iter()
            .map(|&o| format!("{:08b}", o))
            .collect::<Vec<_>>()
            .join(".")
    };

    SubnetResponse {
        valid: true,
        ip: ip.to_string(),
        cidr: net.to_string(),
        mask: mask.to_string(),
        wildcard: wildcard.to_string(),
        network: network_addr.to_string(),
        broadcast: broadcast_addr.to_string(),
        first_ip: first.to_string(),
        last_ip: last.to_string(),
        total_hosts: total,
        usable_hosts: usable,
        ip_class: class.to_string(),
        ip_type: ip_type.to_string(),
        binary_ip: to_bin(ip),
        binary_mask: to_bin(mask),
    }
}
fn empty_subnet() -> SubnetResponse {
    SubnetResponse {
        valid: false,
        ip: "".into(),
        cidr: "".into(),
        mask: "".into(),
        wildcard: "".into(),
        network: "".into(),
        broadcast: "".into(),
        first_ip: "".into(),
        last_ip: "".into(),
        total_hosts: 0,
        usable_hosts: 0,
        ip_class: "".into(),
        ip_type: "".into(),
        binary_ip: "".into(),
        binary_mask: "".into(),
    }
}

// --- 5. Regex Tools ---
#[derive(Serialize)]
pub struct RegexResponse {
    pub matches: Vec<String>,
    pub count: usize,
    pub error: Option<String>,
}
pub fn test_regex(pattern: &str, text: &str) -> RegexResponse {
    match Regex::new(pattern) {
        Ok(re) => {
            let matches: Vec<String> = re.find_iter(text).map(|m| m.as_str().to_string()).collect();
            RegexResponse {
                matches: matches.clone(),
                count: matches.len(),
                error: None,
            }
        }
        Err(e) => RegexResponse {
            matches: vec![],
            count: 0,
            error: Some(e.to_string()),
        },
    }
}
pub fn get_common_regex(key: &str) -> &'static str {
    match key {
        "email" => r"(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$",
        "phone_cn" => r"^1[3-9]\d{9}$",
        "id_cn" => r"^[1-9]\d{5}(18|19|20)\d{2}(0[1-9]|1[0-2])(0[1-9]|[1-2]\d|3[0-1])\d{3}[\dXx]$",
        "ipv4" => {
            r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$"
        }
        "url" => {
            r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"
        }
        "date" => r"^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])$",
        "password" => {
            r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[!@#$%^&*()_+\-=\[\]{};':,.<>/?])[A-Za-z\d!@#$%^&*()_+\-=\[\]{};':,.<>/?]{8,}$"
        }
        "hex_color" => r"^#?([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$",
        "chinese" => r"\p{Han}+",
        "html_tag" => r"</?[a-z][a-z0-9]*[^<>]*>",
        _ => "",
    }
}

// --- 6. UUID ---
pub struct UuidConfig {
    pub count: usize,
    pub hyphens: bool,
    pub uppercase: bool,
}
pub fn generate_uuids(config: UuidConfig) -> Vec<String> {
    let mut res = Vec::new();
    for _ in 0..config.count.min(20) {
        let u = Uuid::new_v4();
        let mut s = if config.hyphens {
            u.to_string()
        } else {
            u.simple().to_string()
        };
        if config.uppercase {
            s = s.to_uppercase();
        }
        res.push(s);
    }
    res
}

// --- 7. Password / Token ---
pub fn generate_token(len: usize, u: bool, l: bool, n: bool, s: bool) -> String {
    let mut charset = String::new();
    if u {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if l {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if n {
        charset.push_str("0123456789");
    }
    if s {
        charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
    }
    if charset.is_empty() {
        return "请选择至少一种字符类型".to_string();
    }

    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn generate_password_strong(len: usize, u: bool, l: bool, n: bool, s: bool) -> String {
    let mut charset = String::new();
    let mut required_chars = Vec::new();
    let mut rng = rand::thread_rng();

    let mut add_set = |chars: &str| {
        charset.push_str(chars);
        if !chars.is_empty() {
            let idx = rng.gen_range(0..chars.len());
            if let Some(c) = chars.chars().nth(idx) {
                required_chars.push(c);
            }
        }
    };

    if u {
        add_set("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if l {
        add_set("abcdefghijklmnopqrstuvwxyz");
    }
    if n {
        add_set("0123456789");
    }
    if s {
        add_set("!@#$%^&*()_+-=[]{}|;:,.<>?");
    }

    if charset.is_empty() {
        return "请选择至少一种字符类型".to_string();
    }

    if len < required_chars.len() {
        return generate_token(len, u, l, n, s);
    }

    let mut result = required_chars;
    while result.len() < len {
        let idx = rng.gen_range(0..charset.len());
        result.push(charset.chars().nth(idx).unwrap());
    }

    result.shuffle(&mut rng);
    result.into_iter().collect()
}

// --- 8. MD5 ---
#[derive(Serialize)]
pub struct Md5Response {
    pub md5_32_lower: String,
    pub md5_32_upper: String,
    pub md5_16_lower: String,
    pub md5_16_upper: String,
}
pub fn calculate_md5(text: &str) -> Md5Response {
    let digest = md5::compute(text);
    let m32 = format!("{:x}", digest);
    let m32_u = m32.to_uppercase();
    let m16 = m32[8..24].to_string();
    let m16_u = m16.to_uppercase();
    Md5Response {
        md5_32_lower: m32,
        md5_32_upper: m32_u,
        md5_16_lower: m16,
        md5_16_upper: m16_u,
    }
}

// --- 9. Base64 ---
pub fn process_base64(text: &str, action: &str) -> String {
    if action == "encode" {
        general_purpose::STANDARD.encode(text)
    } else {
        match general_purpose::STANDARD.decode(text) {
            Ok(b) => String::from_utf8_lossy(&b).to_string(),
            Err(_) => "无效的 Base64 内容".to_string(),
        }
    }
}

// --- 10. URL ---
pub fn process_url(
    input: &str,
) -> (
    String,
    String,
    String,
    String,
    String,
    Vec<(String, String)>,
) {
    let decoded = percent_decode_str(input).decode_utf8_lossy().to_string();
    let encoded = utf8_percent_encode(input, NON_ALPHANUMERIC).to_string();

    // Default to decoding behavior for analysis
    let mut protocol = "-".to_string();
    let mut host = "-".to_string();
    let mut path = "-".to_string();
    let mut params = vec![];

    // Try to parse as URL to get details
    let url_to_parse = if input.starts_with("http") {
        input.to_string()
    } else {
        format!("http://{}", input)
    };
    if let Ok(u) = Url::parse(&url_to_parse) {
        protocol = u.scheme().to_string();
        if let Some(h) = u.host_str() {
            host = h.to_string();
        }
        path = u.path().to_string();
        for (k, v) in u.query_pairs() {
            params.push((k.to_string(), v.to_string()));
        }
    }
    (encoded, decoded, protocol, host, path, params)
}

// --- 11. JSON ---
pub fn process_json(input: &str) -> (String, String, Option<String>) {
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(v) => {
            let pretty = serde_json::to_string_pretty(&v).unwrap_or_default();
            let min = serde_json::to_string(&v).unwrap_or_default();
            (pretty, min, None)
        }
        Err(e) => (String::new(), String::new(), Some(e.to_string())),
    }
}

// --- 12. Escape ---
pub fn process_escape(text: &str, mode: &str) -> String {
    match mode {
        "html_enc" => encode_text(text).to_string(),
        "html_dec" => decode_html_entities(text).to_string(),
        "json_enc" => serde_json::to_string(text).unwrap_or_default(),
        "json_dec" => {
            if let Ok(serde_json::Value::String(s)) = serde_json::from_str(text) {
                s
            } else if let Ok(serde_json::Value::String(s)) =
                serde_json::from_str(&format!("\"{}\"", text))
            {
                s
            } else {
                "Invalid JSON String".to_string()
            }
        }
        _ => String::new(),
    }
}

// --- 13. Date ---
#[derive(Serialize)]
pub struct DateResponse {
    pub unix_sec: i64,
    pub unix_milli: i64,
    pub iso_8601: String,
    pub human_utc: String,
}
pub fn parse_date(input: &str) -> DateResponse {
    // 修复时间戳解析逻辑
    let dt = if let Ok(ts) = input.parse::<i64>() {
        // 大于 30000000000 视为毫秒，否则为秒
        if ts > 30000000000 {
            DateTime::from_timestamp_millis(ts).unwrap_or(Utc::now())
        } else {
            DateTime::from_timestamp(ts, 0).unwrap_or(Utc::now())
        }
    } else {
        Utc::now()
    };

    DateResponse {
        unix_sec: dt.timestamp(),
        unix_milli: dt.timestamp_millis(),
        iso_8601: dt.to_rfc3339(),
        human_utc: dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
    }
}

// --- 14. Color ---
#[derive(Serialize)]
pub struct ColorResponse {
    pub valid: bool,
    pub hex: String,
    pub rgb: String,
    pub hsl: String,
    pub cmyk: String,
}
pub fn convert_color(input: &str) -> ColorResponse {
    match input.parse::<Color>() {
        Ok(c) => {
            let rgba = c.to_rgba8();
            let (r, g, b) = (rgba[0], rgba[1], rgba[2]);

            // HSL
            let (h, s, l, _a) = c.to_hsla();
            let hsl = format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0);

            // CMYK
            let r_f = r as f32 / 255.0;
            let g_f = g as f32 / 255.0;
            let b_f = b as f32 / 255.0;
            let k = 1.0 - r_f.max(g_f).max(b_f);
            let (c_v, m_v, y_v) = if k < 1.0 {
                (
                    (1.0 - r_f - k) / (1.0 - k),
                    (1.0 - g_f - k) / (1.0 - k),
                    (1.0 - b_f - k) / (1.0 - k),
                )
            } else {
                (0.0, 0.0, 0.0)
            };

            let cmyk = format!(
                "cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)",
                c_v * 100.0,
                m_v * 100.0,
                y_v * 100.0,
                k * 100.0
            );

            ColorResponse {
                valid: true,
                hex: c.to_hex_string(),
                rgb: format!("rgb({}, {}, {})", r, g, b),
                hsl,
                cmyk,
            }
        }
        Err(_) => ColorResponse {
            valid: false,
            hex: "".into(),
            rgb: "".into(),
            hsl: "".into(),
            cmyk: "".into(),
        },
    }
}

// --- 15. QR Code ---
pub fn generate_qr(text: &str) -> String {
    let code = QrCode::new(text).unwrap_or_else(|_| QrCode::new("error").unwrap());
    code.render::<svg::Color>().min_dimensions(200, 200).build()
}

// --- 16. JS Enc ---
pub fn obfuscate_js(source: &str) -> String {
    let b64 = general_purpose::STANDARD.encode(source);
    format!("eval(atob('{}'))", b64)
}

// --- 17. YAML / TOML ---
pub fn yaml_to_toml(yaml_str: &str) -> (String, String) {
    match serde_yaml::from_str::<serde_yaml::Value>(yaml_str) {
        Ok(v) => match toml::to_string(&v) {
            Ok(t) => (t, "".to_string()),
            Err(e) => ("".to_string(), e.to_string()),
        },
        Err(e) => ("".to_string(), e.to_string()),
    }
}
pub fn toml_to_yaml(toml_str: &str) -> (String, String) {
    match toml::from_str::<serde_yaml::Value>(toml_str) {
        Ok(v) => match serde_yaml::to_string(&v) {
            Ok(y) => (y, "".to_string()),
            Err(e) => ("".to_string(), e.to_string()),
        },
        Err(e) => ("".to_string(), e.to_string()),
    }
}

// --- 18. Chmod ---
#[derive(Serialize)]
pub struct ChmodResponse {
    pub valid: bool,
    pub command: String,
}
pub fn calculate_chmod(octal: &str, file: &str) -> ChmodResponse {
    if octal.len() != 3 || octal.chars().any(|c| !c.is_digit(8)) {
        return ChmodResponse {
            valid: false,
            command: "Invalid".into(),
        };
    }
    let f = if file.trim().is_empty() {
        "filename"
    } else {
        file.trim()
    };
    ChmodResponse {
        valid: true,
        command: format!("chmod {} {}", octal, f),
    }
}

// --- 19. JWT ---
#[derive(Serialize)]
pub struct JwtResponse {
    pub error: Option<String>,
    pub header: String,
    pub payload: String,
}
pub fn parse_jwt(token: &str) -> JwtResponse {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return JwtResponse {
            error: Some("Invalid Token Format".into()),
            header: "".into(),
            payload: "".into(),
        };
    }
    let decode = |s: &str| -> String {
        match general_purpose::URL_SAFE_NO_PAD.decode(s) {
            Ok(b) => String::from_utf8_lossy(&b).to_string(),
            Err(_) => "Decode Error".into(),
        }
    };
    JwtResponse {
        error: None,
        header: decode(parts[0]),
        payload: decode(parts[1]),
    }
}

// --- 20. Case Converter ---
pub fn convert_case(text: &str, mode: &str) -> String {
    // 1. 预处理：将下划线、中划线替换为空格
    let s = text.replace('_', " ").replace('-', " ");

    // 2. 处理驼峰：在小写字母和大写字母之间插入空格 (例如 camelCase -> camel Case)
    // 使用 lazy_static 或在函数内编译 regex (对于 worker 这种短生命周期，直接编译也可接受，或者用 lazy_static 优化)
    // 这里为了简单直接编译，性能影响微乎其微
    let re = Regex::new(r"(?P<lower>[a-z])(?P<upper>[A-Z])").unwrap();
    let s = re.replace_all(&s, "$lower $upper");

    let words: Vec<&str> = s.split_whitespace().collect();

    match mode {
        "upper" => words.join(" ").to_uppercase(),
        "lower" => words.join(" ").to_lowercase(),
        "snake" => words.join("_").to_lowercase(),
        "kebab" => words.join("-").to_lowercase(),
        "constant" => words.join("_").to_uppercase(),
        "camel" => words
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let w = w.to_lowercase();
                if i == 0 {
                    w
                } else {
                    let mut c = w.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                }
            })
            .collect(),
        "pascal" => words
            .iter()
            .map(|w| {
                let w = w.to_lowercase();
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect(),
        _ => text.to_string(),
    }
}

// --- 21. Tar Command ---
#[derive(Serialize)]
pub struct TarResponse {
    pub command: String,
}
pub fn generate_tar(
    op: &str,
    comp: &str,
    verbose: bool,
    archive: &str,
    files: &str,
) -> TarResponse {
    let mut cmd = String::from("tar -");
    match op {
        "extract" => cmd.push('x'),
        "list" => cmd.push('t'),
        _ => cmd.push('c'),
    }
    match comp {
        "gzip" => cmd.push('z'),
        "bzip2" => cmd.push('j'),
        "xz" => cmd.push('J'),
        _ => {}
    }
    if verbose {
        cmd.push('v');
    }
    cmd.push('f');
    cmd.push(' ');

    let arch = if archive.trim().is_empty() {
        match comp {
            "gzip" => "archive.tar.gz",
            "bzip2" => "archive.tar.bz2",
            "xz" => "archive.tar.xz",
            _ => "archive.tar",
        }
    } else {
        archive.trim()
    };
    cmd.push_str(arch);

    if !files.trim().is_empty() {
        if op == "extract" {
            cmd.push_str(" -C ");
        } else {
            cmd.push(' ');
        }
        cmd.push_str(files.trim());
    }

    TarResponse { command: cmd }
}

// --- 22. Ps Command ---
#[derive(Serialize)]
pub struct PsResponse {
    pub command: String,
}
pub fn generate_ps(
    format: &str,
    sort: &str,
    tree: bool,
    filter: &str,
    wide: bool,
    threads: bool,
    user: &str,
    pid: &str,
) -> PsResponse {
    let mut cmd = String::from("ps");

    match format {
        "ef" => {
            cmd.push_str(" -ef");
            if tree {
                cmd.push_str(" --forest");
            }
            if wide {
                cmd.push_str("ww"); // -efww works on Linux
            }
            if threads {
                cmd.push_str("L"); // -efL
            }
        }
        _ => {
            // Default to aux
            cmd.push_str(" aux");
            if wide {
                cmd.push_str("ww");
            }
            if threads {
                cmd.push('L'); // auxL (BSD style often supports this mixed)
            }
            if tree {
                cmd.push('f');
            }
        }
    }

    if !user.trim().is_empty() {
        cmd.push_str(" -u ");
        cmd.push_str(user.trim());
    }

    if !pid.trim().is_empty() {
        cmd.push_str(" -p ");
        cmd.push_str(pid.trim());
    }

    if !sort.is_empty() && sort != "none" {
        cmd.push_str(" --sort=");
        cmd.push_str(sort);
    }

    if !filter.trim().is_empty() {
        cmd.push_str(" | grep ");
        cmd.push_str(filter.trim());
    }

    PsResponse { command: cmd }
}

// --- 23. Tcpdump Command ---
#[derive(Serialize)]
pub struct TcpdumpResponse {
    pub command: String,
}
pub fn generate_tcpdump(
    interface: &str,
    protocol: &str,
    host: &str,
    port: &str,
    verbose: bool,
    ascii: bool,
    hex: bool,
    write_file: &str,
    count: &str,
) -> TcpdumpResponse {
    let mut cmd = String::from("tcpdump");

    // Interface
    if !interface.trim().is_empty() {
        cmd.push_str(" -i ");
        cmd.push_str(interface.trim());
    } else {
        cmd.push_str(" -i any");
    }

    // Options
    // If writing to file, display options might not be needed, but we keep them if user wants
    if !write_file.trim().is_empty() {
        cmd.push_str(" -w ");
        cmd.push_str(write_file.trim());
    } else {
        if verbose {
            cmd.push_str(" -v");
        }
        if ascii {
            cmd.push_str(" -A");
        }
        if hex {
            cmd.push_str(" -X");
        }
    }

    if !count.trim().is_empty() {
        cmd.push_str(" -c ");
        cmd.push_str(count.trim());
    }

    // Filters
    let mut filters = Vec::new();
    if !protocol.trim().is_empty() && protocol != "all" {
        filters.push(protocol.trim().to_string());
    }
    if !host.trim().is_empty() {
        filters.push(format!("host {}", host.trim()));
    }
    if !port.trim().is_empty() {
        filters.push(format!("port {}", port.trim()));
    }

    if !filters.is_empty() {
        cmd.push_str(" \"");
        cmd.push_str(&filters.join(" and "));
        cmd.push('"');
    }

    TcpdumpResponse { command: cmd }
}

// --- 24. Git Command ---
#[derive(Serialize)]
pub struct GitResponse {
    pub command: String,
}
pub fn generate_git(
    cmd: &str,
    target: &str,
    msg: &str,
    remote: &str,
    branch: &str,
    opt_force: bool,
    opt_rebase: bool,
    opt_all: bool,
    opt_amend: bool,
    opt_hard: bool,
    opt_new_branch: bool,
    opt_tags: bool,
    opt_oneline: bool,
    opt_graph: bool,
) -> GitResponse {
    let mut c = String::from("git");
    c.push(' ');
    c.push_str(cmd);

    match cmd {
        "init" => {
            if !target.trim().is_empty() {
                c.push(' ');
                c.push_str(target.trim());
            }
        }
        "clone" => {
            if !target.trim().is_empty() {
                c.push(' ');
                c.push_str(target.trim());
            }
        }
        "add" => {
            if opt_all {
                c.push_str(" -A");
            } else if !target.trim().is_empty() {
                c.push(' ');
                c.push_str(target.trim());
            } else {
                c.push_str(" .");
            }
        }
        "commit" => {
            if opt_all {
                c.push_str(" -a");
            }
            if opt_amend {
                c.push_str(" --amend");
            }
            if !msg.trim().is_empty() {
                c.push_str(" -m \"");
                c.push_str(&msg.replace('"', "\\\""));
                c.push('"');
            }
        }
        "push" => {
            if opt_force {
                c.push_str(" --force");
            }
            if opt_tags {
                c.push_str(" --tags");
            }
            if !remote.trim().is_empty() {
                c.push(' ');
                c.push_str(remote.trim());
            }
            if !branch.trim().is_empty() {
                c.push(' ');
                c.push_str(branch.trim());
            }
        }
        "pull" => {
            if opt_rebase {
                c.push_str(" --rebase");
            }
            if !remote.trim().is_empty() {
                c.push(' ');
                c.push_str(remote.trim());
            }
            if !branch.trim().is_empty() {
                c.push(' ');
                c.push_str(branch.trim());
            }
        }
        "checkout" => {
            if opt_new_branch {
                c.push_str(" -b");
            }
            if !target.trim().is_empty() {
                c.push(' ');
                c.push_str(target.trim());
            }
        }
        "merge" => {
            if !target.trim().is_empty() {
                c.push(' ');
                c.push_str(target.trim());
            }
        }
        "log" => {
            if opt_oneline {
                c.push_str(" --oneline");
            }
            if opt_graph {
                c.push_str(" --graph");
            }
            c.push_str(" --decorate --all");
        }
        "reset" => {
            if opt_hard {
                c.push_str(" --hard");
            }
            if !target.trim().is_empty() {
                c.push(' ');
                c.push_str(target.trim());
            }
        }
        "remote" => {
            c.push_str(" add ");
            if !remote.trim().is_empty() {
                c.push_str(remote.trim());
            } else {
                c.push_str("origin");
            }
            if !target.trim().is_empty() {
                c.push(' ');
                c.push_str(target.trim());
            }
        }
        "status" => {} // No args usually
        _ => {}
    }

    GitResponse { command: c }
}

// --- 24.5 Git Cheat Sheet ---
#[derive(Serialize)]
pub struct GitCmdResponse {
    pub command: String,
    pub description: String,
}
pub fn generate_git_cmd(action: &str, tag: &str, msg: &str, branch: &str) -> GitCmdResponse {
    let (cmd, desc) = match action {
        "undo_commit" => (
            "git reset --soft HEAD~1".to_string(),
            "撤销最近一次提交，但保留文件修改（Soft Reset）".to_string(),
        ),
        "undo_changes" => (
            "git checkout .".to_string(),
            "撤销工作区所有修改（危险：会丢失未提交的改动）".to_string(),
        ),
        "log_graph" => (
            "git log --graph --oneline --decorate --all".to_string(),
            "以图形化方式查看提交历史".to_string(),
        ),
        "tag" => (
            format!(
                "git tag -a {} -m \"{}\" && git push origin {}",
                tag, msg, tag
            ),
            "创建并推送带注释的标签".to_string(),
        ),
        "branch_delete" => (
            format!(
                "git branch -d {} && git push origin --delete {}",
                branch, branch
            ),
            "删除本地和远程分支".to_string(),
        ),
        "stash" => (
            "git stash && git pull && git stash pop".to_string(),
            "暂存修改，拉取代码，然后恢复修改".to_string(),
        ),
        _ => ("git help".to_string(), "".to_string()),
    };
    GitCmdResponse {
        command: cmd,
        description: desc,
    }
}

// --- 25. Strace Command ---
#[derive(Serialize)]
pub struct StraceResponse {
    pub command: String,
}
pub fn generate_strace(
    target: &str,
    is_pid: bool,
    follow: bool,
    summary: bool,
    output_file: &str,
    filter: &str,
    string_limit: &str,
    timestamp: bool,
) -> StraceResponse {
    let mut cmd = String::from("strace");

    if follow {
        cmd.push_str(" -f");
    }
    if summary {
        cmd.push_str(" -c");
    }
    if timestamp {
        cmd.push_str(" -tt");
    }
    if !string_limit.trim().is_empty() {
        cmd.push_str(" -s ");
        cmd.push_str(string_limit.trim());
    }
    if !output_file.trim().is_empty() {
        cmd.push_str(" -o ");
        cmd.push_str(output_file.trim());
    }
    if !filter.trim().is_empty() {
        cmd.push_str(" -e ");
        cmd.push_str(filter.trim());
    }

    if !target.trim().is_empty() {
        cmd.push(' ');
        if is_pid {
            cmd.push_str("-p ");
        }
        cmd.push_str(target.trim());
    }

    StraceResponse { command: cmd }
}

// --- 26. Iostat Command ---
#[derive(Serialize)]
pub struct IostatResponse {
    pub command: String,
}
pub fn generate_iostat(
    interval: &str,
    count: &str,
    human: bool,
    extended: bool,
    unit: &str,
    partitions: bool,
    timestamp: bool,
    device: &str,
) -> IostatResponse {
    let mut cmd = String::from("iostat");

    if human {
        cmd.push_str(" -h");
    }
    if extended {
        cmd.push_str(" -x");
    }
    if timestamp {
        cmd.push_str(" -t");
    }

    match unit {
        "k" => cmd.push_str(" -k"),
        "m" => cmd.push_str(" -m"),
        _ => {}
    }

    if partitions {
        cmd.push_str(" -p");
    }

    if !device.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(device.trim());
    }

    if !interval.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(interval.trim());
        if !count.trim().is_empty() {
            cmd.push(' ');
            cmd.push_str(count.trim());
        }
    }

    IostatResponse { command: cmd }
}

// --- 27. Nice/Renice Command ---
#[derive(Serialize)]
pub struct NiceResponse {
    pub command: String,
}
pub fn generate_nice(
    mode: &str,
    priority: i32,
    command: &str,
    target_type: &str,
    target: &str,
) -> NiceResponse {
    let mut cmd = String::new();
    // Priority range -20 to 19
    let prio = priority.max(-20).min(19);

    if mode == "renice" {
        // renice priority [[-p] pid ...] [[-g] pgrp ...] [[-u] user ...]
        cmd.push_str("renice -n ");
        cmd.push_str(&prio.to_string());

        match target_type {
            "group" => cmd.push_str(" -g"),
            "user" => cmd.push_str(" -u"),
            _ => cmd.push_str(" -p"), // default pid
        }

        if !target.trim().is_empty() {
            cmd.push(' ');
            cmd.push_str(target.trim());
        }
    } else {
        // nice -n adjustment [command [arg...]]
        cmd.push_str("nice -n ");
        cmd.push_str(&prio.to_string());

        if !command.trim().is_empty() {
            cmd.push(' ');
            cmd.push_str(command.trim());
        }
    }

    NiceResponse { command: cmd }
}

// --- 28. Ls Command ---
#[derive(Serialize)]
pub struct LsResponse {
    pub command: String,
}
pub fn generate_ls(
    path: &str,
    all: bool,
    long: bool,
    human: bool,
    time: bool,
    reverse: bool,
    recursive: bool,
    inode: bool,
    directory: bool,
    color: bool,
) -> LsResponse {
    let mut cmd = String::from("ls");

    if color {
        cmd.push_str(" --color=auto");
    }

    let mut shorts = String::new();
    if all {
        shorts.push('a');
    }
    if long {
        shorts.push('l');
    }
    if human {
        shorts.push('h');
    }
    if time {
        shorts.push('t');
    }
    if reverse {
        shorts.push('r');
    }
    if recursive {
        shorts.push('R');
    }
    if inode {
        shorts.push('i');
    }
    if directory {
        shorts.push('d');
    }

    if !shorts.is_empty() {
        cmd.push_str(" -");
        cmd.push_str(&shorts);
    }
    if !path.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(path.trim());
    }
    LsResponse { command: cmd }
}

// --- 29. Firewall Command ---
#[derive(Serialize)]
pub struct FirewallResponse {
    pub command: String,
}
pub fn generate_firewall(
    op: &str,
    zone: &str,
    target_type: &str,
    target: &str,
    permanent: bool,
) -> FirewallResponse {
    let mut cmd = String::from("firewall-cmd");

    if op == "reload" {
        cmd.push_str(" --reload");
        return FirewallResponse { command: cmd };
    }

    if permanent {
        cmd.push_str(" --permanent");
    }

    let is_all = zone == "all";

    if !is_all && !zone.trim().is_empty() && zone != "default" {
        cmd.push_str(" --zone=");
        cmd.push_str(zone.trim());
    }

    match op {
        "list" => {
            if is_all {
                cmd.push_str(" --list-all-zones");
            } else {
                cmd.push_str(" --list-all");
            }
        }
        "add" => {
            if target_type == "port" {
                cmd.push_str(" --add-port=");
            } else {
                cmd.push_str(" --add-service=");
            }
            if !target.trim().is_empty() {
                cmd.push_str(target.trim());
            }
        }
        "remove" => {
            if target_type == "port" {
                cmd.push_str(" --remove-port=");
            } else {
                cmd.push_str(" --remove-service=");
            }
            if !target.trim().is_empty() {
                cmd.push_str(target.trim());
            }
        }
        _ => {}
    }

    FirewallResponse { command: cmd }
}

// --- 30. Systemctl Command ---
#[derive(Serialize)]
pub struct SystemctlResponse {
    pub command: String,
}
pub fn generate_systemctl(
    operation: &str,
    service: &str,
    user_mode: bool,
    now: bool,
    force: bool,
    global: bool,
) -> SystemctlResponse {
    let mut cmd = String::from("systemctl");

    if user_mode {
        cmd.push_str(" --user");
    } else if global {
        cmd.push_str(" --global");
    }

    if !operation.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(operation);
    }

    if force {
        cmd.push_str(" --force");
    }
    if now && (operation == "enable" || operation == "disable" || operation == "mask") {
        cmd.push_str(" --now");
    }

    if !service.trim().is_empty() && operation != "daemon-reload" {
        cmd.push(' ');
        cmd.push_str(service.trim());
    }

    SystemctlResponse { command: cmd }
}

// --- 31. Find Command ---
#[derive(Serialize)]
pub struct FindResponse {
    pub command: String,
}
pub fn generate_find(
    path: &str,
    name: &str,
    iname: bool,
    target_type: &str,
    size: &str,
    mtime: &str,
    empty: bool,
    exec: &str,
) -> FindResponse {
    let mut cmd = String::from("find");

    // Path defaults to . if empty, but usually find [path] [options]
    if !path.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(path.trim());
    } else {
        cmd.push_str(" .");
    }

    if !name.trim().is_empty() {
        cmd.push(' ');
        if iname {
            cmd.push_str("-iname");
        } else {
            cmd.push_str("-name");
        }
        cmd.push(' ');
        // Quote the name pattern to prevent shell expansion
        cmd.push('"');
        cmd.push_str(&name.trim().replace('"', "\\\""));
        cmd.push('"');
    }

    if !target_type.trim().is_empty() && target_type != "all" {
        cmd.push_str(" -type ");
        cmd.push_str(target_type.trim());
    }

    if empty {
        cmd.push_str(" -empty");
    } else if !size.trim().is_empty() {
        cmd.push_str(" -size ");
        cmd.push_str(size.trim());
    }

    if !mtime.trim().is_empty() {
        cmd.push_str(" -mtime ");
        cmd.push_str(mtime.trim());
    }

    if !exec.trim().is_empty() {
        cmd.push_str(" -exec ");
        cmd.push_str(exec.trim());
        cmd.push_str(" {} \\;");
    }

    FindResponse { command: cmd }
}

// --- 32. Dockerfile Generator ---
#[derive(Deserialize, Serialize)]
pub struct DockerfileStage {
    #[serde(default)]
    pub image: String,
    #[serde(default, rename = "as")]
    pub as_: String,
    #[serde(default)]
    pub workdir: String,
    #[serde(default)]
    pub copy: String,
    #[serde(default)]
    pub run: String,
    #[serde(default)]
    pub env: String,
    #[serde(default)]
    pub expose: String,
    #[serde(default)]
    pub cmd: String,
    #[serde(default)]
    pub entrypoint: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub volume: String,
    #[serde(default)]
    pub arg: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub healthcheck: String,
}

pub fn generate_dockerfile(stages: &[DockerfileStage]) -> String {
    let mut df = String::new();

    for (index, stage) in stages.iter().enumerate() {
        if index > 0 {
            df.push_str(&format!("\n# Stage {}\n", index + 1));
        }

        // FROM
        if !stage.image.trim().is_empty() {
            df.push_str(&format!("FROM {}", stage.image.trim()));
            if !stage.as_.trim().is_empty() {
                df.push_str(&format!(" AS {}", stage.as_.trim()));
            }
            df.push('\n');
        } else {
            df.push_str("FROM scratch\n");
        }

        // ARG
        for line in stage.arg.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("ARG {}\n", line.trim()));
            }
        }

        // LABEL
        for line in stage.label.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("LABEL {}\n", line.trim()));
            }
        }

        // WORKDIR
        if !stage.workdir.trim().is_empty() {
            df.push_str(&format!("WORKDIR {}\n", stage.workdir.trim()));
        }

        // ENV
        for line in stage.env.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("ENV {}\n", line.trim()));
            }
        }

        // COPY
        for line in stage.copy.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("COPY {}\n", line.trim()));
            }
        }

        // RUN
        for line in stage.run.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("RUN {}\n", line.trim()));
            }
        }

        // EXPOSE
        if !stage.expose.trim().is_empty() {
            for port in stage.expose.split(&[',', ' '][..]) {
                if !port.trim().is_empty() {
                    df.push_str(&format!("EXPOSE {}\n", port.trim()));
                }
            }
        }

        // USER
        if !stage.user.trim().is_empty() {
            df.push_str(&format!("USER {}\n", stage.user.trim()));
        }

        // VOLUME
        if !stage.volume.trim().is_empty() {
            for vol in stage.volume.split(&[',', ' '][..]) {
                if !vol.trim().is_empty() {
                    df.push_str(&format!("VOLUME {}\n", vol.trim()));
                }
            }
        }

        // HEALTHCHECK
        if !stage.healthcheck.trim().is_empty() {
            df.push_str(&format!("HEALTHCHECK {}\n", stage.healthcheck.trim()));
        }

        // ENTRYPOINT
        if !stage.entrypoint.trim().is_empty() {
            df.push_str(&format!("ENTRYPOINT {}\n", stage.entrypoint.trim()));
        }

        // CMD
        if !stage.cmd.trim().is_empty() {
            df.push_str(&format!("CMD {}\n", stage.cmd.trim()));
        }
    }

    df
}

// --- 33. Nginx Config Generator ---
pub fn generate_nginx_config(
    domain: &str,
    port: u16,
    root: &str,
    path: &str,
    proxy: &str,
    upstream: &str,
    https: bool,
    force_https: bool,
    ssl_cert: &str,
    ssl_key: &str,
    spa: bool,
    gzip: bool,
    client_max_body_size: &str,
    keepalive_timeout: &str,
    proxy_connect_timeout: &str,
    proxy_read_timeout: &str,
    proxy_send_timeout: &str,
) -> String {
    let mut conf = String::new();
    let domain = if domain.trim().is_empty() {
        "example.com"
    } else {
        domain.trim()
    };

    let location_path = if path.trim().is_empty() {
        "/"
    } else {
        path.trim()
    };

    // Upstream Block
    let has_upstream = !upstream.trim().is_empty();
    let upstream_name = format!("{}_backend", domain.replace('.', "_"));

    if has_upstream {
        conf.push_str(&format!("upstream {} {{\n", upstream_name));
        for line in upstream.lines() {
            if !line.trim().is_empty() {
                conf.push_str(&format!("    server {};\n", line.trim()));
            }
        }
        conf.push_str("}\n\n");
    }

    // HTTP Redirect Block
    if https && force_https {
        conf.push_str("server {\n");
        conf.push_str("    listen 80;\n");
        conf.push_str(&format!("    server_name {};\n", domain));
        conf.push_str("    return 301 https://$host$request_uri;\n");
        conf.push_str("}\n\n");
    }

    // Main Server Block
    conf.push_str("server {\n");

    if https {
        conf.push_str("    listen 443 ssl http2;\n");
        conf.push_str(&format!("    server_name {};\n\n", domain));

        let cert_path = if ssl_cert.trim().is_empty() {
            format!("/etc/nginx/ssl/{}.crt", domain)
        } else {
            ssl_cert.trim().to_string()
        };
        let key_path = if ssl_key.trim().is_empty() {
            format!("/etc/nginx/ssl/{}.key", domain)
        } else {
            ssl_key.trim().to_string()
        };

        conf.push_str(&format!("    ssl_certificate {};\n", cert_path));
        conf.push_str(&format!("    ssl_certificate_key {};\n", key_path));
        conf.push_str("    ssl_protocols TLSv1.2 TLSv1.3;\n");
        conf.push_str("    ssl_ciphers HIGH:!aNULL:!MD5;\n\n");
    } else {
        conf.push_str(&format!("    listen {};\n", port));
        conf.push_str(&format!("    server_name {};\n\n", domain));
    }

    // Access Logs
    conf.push_str(&format!(
        "    access_log /var/log/nginx/{}.access.log;\n",
        domain
    ));
    conf.push_str(&format!(
        "    error_log /var/log/nginx/{}.error.log;\n\n",
        domain
    ));

    // Gzip
    if gzip {
        conf.push_str("    gzip on;\n");
        conf.push_str("    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;\n\n");
    }

    // Limits & Timeouts
    if !client_max_body_size.trim().is_empty() {
        conf.push_str(&format!(
            "    client_max_body_size {};\n",
            client_max_body_size.trim()
        ));
    }
    if !keepalive_timeout.trim().is_empty() {
        conf.push_str(&format!(
            "    keepalive_timeout {};\n\n",
            keepalive_timeout.trim()
        ));
    }

    // Proxy or Static
    if has_upstream {
        conf.push_str(&format!("    location {} {{\n", location_path));
        conf.push_str(&format!("        proxy_pass http://{};\n", upstream_name));
        conf.push_str("        proxy_set_header Host $host;\n");
        conf.push_str("        proxy_set_header X-Real-IP $remote_addr;\n");
        conf.push_str("        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n");
        conf.push_str("        proxy_set_header X-Forwarded-Proto $scheme;\n");
        if !proxy_connect_timeout.trim().is_empty() {
            conf.push_str(&format!(
                "        proxy_connect_timeout {};\n",
                proxy_connect_timeout.trim()
            ));
        }
        if !proxy_read_timeout.trim().is_empty() {
            conf.push_str(&format!(
                "        proxy_read_timeout {};\n",
                proxy_read_timeout.trim()
            ));
        }
        if !proxy_send_timeout.trim().is_empty() {
            conf.push_str(&format!(
                "        proxy_send_timeout {};\n",
                proxy_send_timeout.trim()
            ));
        }
        conf.push_str("    }\n");
    } else if !proxy.trim().is_empty() {
        conf.push_str(&format!("    location {} {{\n", location_path));
        conf.push_str(&format!("        proxy_pass {};\n", proxy.trim()));
        conf.push_str("        proxy_set_header Host $host;\n");
        conf.push_str("        proxy_set_header X-Real-IP $remote_addr;\n");
        conf.push_str("        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n");
        conf.push_str("        proxy_set_header X-Forwarded-Proto $scheme;\n");
        if !proxy_connect_timeout.trim().is_empty() {
            conf.push_str(&format!(
                "        proxy_connect_timeout {};\n",
                proxy_connect_timeout.trim()
            ));
        }
        if !proxy_read_timeout.trim().is_empty() {
            conf.push_str(&format!(
                "        proxy_read_timeout {};\n",
                proxy_read_timeout.trim()
            ));
        }
        if !proxy_send_timeout.trim().is_empty() {
            conf.push_str(&format!(
                "        proxy_send_timeout {};\n",
                proxy_send_timeout.trim()
            ));
        }
        conf.push_str("    }\n");
    } else {
        let root_path = if root.trim().is_empty() {
            "/var/www/html"
        } else {
            root.trim()
        };
        conf.push_str(&format!("    root {};\n", root_path));
        conf.push_str("    index index.html index.htm;\n\n");

        conf.push_str(&format!("    location {} {{\n", location_path));
        if spa {
            conf.push_str("        try_files $uri $uri/ /index.html;\n");
        } else {
            conf.push_str("        try_files $uri $uri/ =404;\n");
        }
        conf.push_str("    }\n");
    }

    conf.push_str("}\n");
    conf
}

// --- 37. Unit Converter ---
#[derive(Serialize)]
pub struct UnitResponse {
    pub result: f64,
    pub value: f64,
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub type_: String,
}
pub fn convert_unit(value: f64, type_: &str, from: &str, to: &str) -> UnitResponse {
    let mut result = 0.0;

    if type_ == "storage" {
        let units = vec!["B", "KB", "MB", "GB", "TB", "PB"];
        let from_idx = units
            .iter()
            .position(|&u| u == from.to_uppercase())
            .unwrap_or(0) as i32;
        let to_idx = units
            .iter()
            .position(|&u| u == to.to_uppercase())
            .unwrap_or(2) as i32; // Default MB

        // Convert to Bytes first
        let bytes = value * 1024f64.powi(from_idx);
        // Convert to target
        result = bytes / 1024f64.powi(to_idx);
    } else if type_ == "time" {
        // Base unit: ms
        let get_ms = |u: &str| match u.to_lowercase().as_str() {
            "s" => 1000.0,
            "m" => 60000.0,
            "h" => 3600000.0,
            "d" => 86400000.0,
            _ => 1.0, // ms
        };
        let from_val = get_ms(from);
        let to_val = get_ms(to);
        result = (value * from_val) / to_val;
    }

    UnitResponse {
        result,
        value,
        from: from.to_string(),
        to: to.to_string(),
        type_: type_.to_string(),
    }
}

// --- 38. cURL Generator ---
#[derive(Serialize)]
pub struct CurlResponse {
    pub command: String,
    pub python: String,
}
pub fn generate_curl(method: &str, url: &str, headers: &str, body: &str) -> CurlResponse {
    let method_upper = method.to_uppercase();
    let url_val = if url.is_empty() {
        "http://localhost:8080".to_string()
    } else {
        url.replace('\'', "'\\''")
    };

    // cURL Command
    let mut cmd = format!("curl -X {} '{}'", method_upper, url_val);

    // Python Code
    let mut py = String::from("import requests\n\n");
    py.push_str(&format!("url = \"{}\"\n", url_val));

    let mut has_headers = false;

    // Headers (Simple parsing, assuming JSON string or just lines)
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(headers) {
        if let Some(obj) = json.as_object() {
            if !obj.is_empty() {
                has_headers = true;
                py.push_str("\nheaders = {\n");
                for (k, v) in obj {
                    let val = v.as_str().unwrap_or("");
                    cmd.push_str(&format!(" \\\n  -H '{}: {}'", k, val));
                    py.push_str(&format!("  '{}': '{}',\n", k, val));
                }
                py.push_str("}\n");
            }
        }
    }

    // Body
    let mut has_payload = false;
    if ["POST", "PUT", "PATCH"].contains(&method_upper.as_str()) && !body.is_empty() {
        // Check if body looks like JSON
        if body.trim().starts_with('{') || body.trim().starts_with('[') {
            cmd.push_str(" \\\n  -H 'Content-Type: application/json'");
        }
        // Escape single quotes for shell
        let escaped_body = body.replace('\'', "'\\''");
        cmd.push_str(&format!(" \\\n  -d '{}'", escaped_body));

        // Python Payload
        has_payload = true;
        // Simple escaping for Python string
        let py_body = body.replace('\\', "\\\\").replace('"', "\\\"");
        py.push_str(&format!("\npayload = \"{}\"\n", py_body));
    }

    py.push_str(&format!(
        "\nresponse = requests.request(\"{}\", url",
        method_upper
    ));
    if has_headers {
        py.push_str(", headers=headers");
    }
    if has_payload {
        py.push_str(", data=payload");
    }
    py.push_str(")\n\nprint(response.text)");

    CurlResponse {
        command: cmd,
        python: py,
    }
}

// --- 34. Lorem Ipsum ---
pub fn generate_lorem(count: usize, mode: &str) -> String {
    let words = vec![
        "lorem",
        "ipsum",
        "dolor",
        "sit",
        "amet",
        "consectetur",
        "adipiscing",
        "elit",
        "sed",
        "do",
        "eiusmod",
        "tempor",
        "incididunt",
        "ut",
        "labore",
        "et",
        "dolore",
        "magna",
        "aliqua",
        "ut",
        "enim",
        "ad",
        "minim",
        "veniam",
        "quis",
        "nostrud",
        "exercitation",
        "ullamco",
        "laboris",
        "nisi",
        "ut",
        "aliquip",
        "ex",
        "ea",
        "commodo",
        "consequat",
        "duis",
        "aute",
        "irure",
        "dolor",
        "in",
        "reprehenderit",
        "in",
        "voluptate",
        "velit",
        "esse",
        "cillum",
        "dolore",
        "eu",
        "fugiat",
        "nulla",
        "pariatur",
        "excepteur",
        "sint",
        "occaecat",
        "cupidatat",
        "non",
        "proident",
        "sunt",
        "in",
        "culpa",
        "qui",
        "officia",
        "deserunt",
        "mollit",
        "anim",
        "id",
        "est",
        "laborum",
    ];

    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    let count = count.max(1).min(100); // Limit count

    match mode {
        "words" => {
            for _ in 0..count {
                result.push(words.choose(&mut rng).unwrap().to_string());
            }
            result.join(" ")
        }
        "sentences" => {
            for _ in 0..count {
                let len = rng.gen_range(5..15);
                let mut sentence = Vec::new();
                for i in 0..len {
                    let w = words.choose(&mut rng).unwrap();
                    if i == 0 {
                        let mut c = w.chars();
                        match c.next() {
                            None => sentence.push(w.to_string()),
                            Some(f) => {
                                sentence.push(f.to_uppercase().collect::<String>() + c.as_str())
                            }
                        }
                    } else {
                        sentence.push(w.to_string());
                    }
                }
                result.push(sentence.join(" ") + ".");
            }
            result.join(" ")
        }
        _ => {
            // paragraphs
            for _ in 0..count {
                let s_len = rng.gen_range(3..8);
                let mut para = Vec::new();
                for _ in 0..s_len {
                    let len = rng.gen_range(5..15);
                    let mut sentence = Vec::new();
                    for i in 0..len {
                        let w = words.choose(&mut rng).unwrap();
                        if i == 0 {
                            let mut c = w.chars();
                            match c.next() {
                                None => sentence.push(w.to_string()),
                                Some(f) => {
                                    sentence.push(f.to_uppercase().collect::<String>() + c.as_str())
                                }
                            }
                        } else {
                            sentence.push(w.to_string());
                        }
                    }
                    para.push(sentence.join(" ") + ".");
                }
                result.push(para.join(" "));
            }
            result.join("\n\n")
        }
    }
}

// --- 35. Rsync Command ---
#[derive(Serialize)]
pub struct RsyncResponse {
    pub command: String,
    pub ssh_config: String,
}
pub fn generate_rsync(
    source: &str,
    user: &str,
    host: &str,
    port: &str,
    remote_path: &str,
    archive: bool,
    compress: bool,
    verbose: bool,
    delete: bool,
    dry_run: bool,
    progress: bool,
    ssh: bool,
    exclude: &str,
) -> RsyncResponse {
    let mut cmd = String::from("rsync");

    // Short options
    let mut shorts = String::new();
    if archive {
        shorts.push('a');
    }
    if compress {
        shorts.push('z');
    }
    if verbose {
        shorts.push('v');
    }
    if dry_run {
        shorts.push('n');
    }
    if progress {
        shorts.push('P');
    }

    if !shorts.is_empty() {
        cmd.push_str(" -");
        cmd.push_str(&shorts);
    }

    // Long options
    if delete {
        cmd.push_str(" --delete");
    }
    if !port.trim().is_empty() && port.trim() != "22" {
        cmd.push_str(&format!(" -e 'ssh -p {}'", port.trim()));
    } else if ssh {
        cmd.push_str(" -e ssh");
    }
    if !exclude.trim().is_empty() {
        cmd.push_str(" --exclude='");
        cmd.push_str(exclude.trim());
        cmd.push('\'');
    }

    // Source
    if !source.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(source.trim());
    } else {
        cmd.push_str(" /source/path");
    }

    // Dest
    cmd.push(' ');
    if !host.trim().is_empty() {
        // Remote destination
        if !user.trim().is_empty() {
            cmd.push_str(user.trim());
            cmd.push('@');
        }
        cmd.push_str(host.trim());
        cmd.push(':');
        if !remote_path.trim().is_empty() {
            cmd.push_str(remote_path.trim());
        }
    } else {
        // Local destination or missing host
        if !remote_path.trim().is_empty() {
            cmd.push_str(remote_path.trim());
        } else {
            cmd.push_str("/dest/path");
        }
    }

    // Generate SSH Config
    let mut ssh_config = String::new();
    if !host.trim().is_empty() {
        ssh_config.push_str(&format!("Host {}\n", host.trim()));
        ssh_config.push_str(&format!("    HostName {}\n", host.trim()));
        if !user.trim().is_empty() {
            ssh_config.push_str(&format!("    User {}\n", user.trim()));
        }
        if !port.trim().is_empty() && port.trim() != "22" {
            ssh_config.push_str(&format!("    Port {}\n", port.trim()));
        }
    }

    RsyncResponse {
        command: cmd,
        ssh_config,
    }
}

// --- 36. Fake User Generator ---
#[derive(Serialize)]
pub struct FakeUser {
    pub name: String,
    pub email: String,
    pub address: String,
    pub phone: String,
}

pub fn generate_fake_users(count: usize, locale: &str) -> Vec<FakeUser> {
    let mut rng = rand::thread_rng();
    let mut users = Vec::new();
    let count = count.max(1).min(50);

    let (first_names, last_names, domains, streets, cities) = if locale == "cn" {
        (
            vec![
                "伟", "芳", "娜", "敏", "静", "秀英", "丽", "强", "磊", "军", "洋", "勇", "艳",
                "杰", "娟", "涛", "明", "超", "秀兰", "霞",
            ],
            vec![
                "王", "李", "张", "刘", "陈", "杨", "黄", "赵", "吴", "周", "徐", "孙", "马", "朱",
                "胡", "郭", "何", "高", "林", "罗",
            ],
            vec!["qq.com", "163.com", "sina.com", "gmail.com", "outlook.com"],
            vec![
                "人民路",
                "建设路",
                "解放路",
                "和平路",
                "中山路",
                "文化路",
                "南京路",
                "北京路",
            ],
            vec![
                "北京", "上海", "广州", "深圳", "杭州", "成都", "武汉", "西安", "南京", "重庆",
            ],
        )
    } else {
        (
            vec![
                "James",
                "Mary",
                "John",
                "Patricia",
                "Robert",
                "Jennifer",
                "Michael",
                "Linda",
                "William",
                "Elizabeth",
                "David",
                "Barbara",
                "Richard",
                "Susan",
                "Joseph",
                "Jessica",
            ],
            vec![
                "Smith",
                "Johnson",
                "Williams",
                "Brown",
                "Jones",
                "Garcia",
                "Miller",
                "Davis",
                "Rodriguez",
                "Martinez",
                "Hernandez",
                "Lopez",
                "Gonzalez",
                "Wilson",
            ],
            vec![
                "gmail.com",
                "yahoo.com",
                "hotmail.com",
                "outlook.com",
                "example.com",
            ],
            vec![
                "Main St",
                "High St",
                "Broadway",
                "Market St",
                "Park Ave",
                "Oak St",
                "Washington St",
            ],
            vec![
                "New York",
                "Los Angeles",
                "Chicago",
                "Houston",
                "Phoenix",
                "Philadelphia",
                "San Antonio",
                "San Diego",
            ],
        )
    };

    for _ in 0..count {
        let first = first_names.choose(&mut rng).unwrap();
        let last = last_names.choose(&mut rng).unwrap();
        let domain = domains.choose(&mut rng).unwrap();

        let name = if locale == "cn" {
            format!("{}{}", last, first)
        } else {
            format!("{} {}", first, last)
        };

        let email = if locale == "cn" {
            let r: u32 = rng.gen_range(10000..99999);
            format!("user{}@{}", r, domain)
        } else {
            format!(
                "{}.{}@{}",
                first.to_lowercase(),
                last.to_lowercase(),
                domain
            )
        };

        let address = if locale == "cn" {
            let city = cities.choose(&mut rng).unwrap();
            let street = streets.choose(&mut rng).unwrap();
            let no = rng.gen_range(1..999);
            format!("{}市{} {}号", city, street, no)
        } else {
            let city = cities.choose(&mut rng).unwrap();
            let street = streets.choose(&mut rng).unwrap();
            let no = rng.gen_range(1..9999);
            format!("{} {}, {}", no, street, city)
        };

        let phone = if locale == "cn" {
            format!(
                "1{}{}",
                rng.gen_range(3..=9),
                rng.gen_range(100000000..=999999999)
            )
        } else {
            format!(
                "+1-{}-{}-{}",
                rng.gen_range(200..999),
                rng.gen_range(200..999),
                rng.gen_range(1000..9999)
            )
        };

        users.push(FakeUser {
            name,
            email,
            address,
            phone,
        });
    }
    users
}

// --- 39. Credit Card Generator ---
#[derive(Serialize)]
pub struct CreditCard {
    pub number: String,
    pub issuer: String,
    pub expiry: String,
    pub cvv: String,
}

pub fn generate_credit_cards(count: usize, issuer: &str) -> Vec<CreditCard> {
    let mut rng = rand::thread_rng();
    let mut cards = Vec::new();
    let count = count.max(1).min(50);

    for _ in 0..count {
        let (len, mut prefix) = match issuer {
            "visa" => (16, vec![4]),
            "mastercard" => (16, vec![5, 1 + rng.gen_range(0..5)]), // 51-55
            "amex" => (15, vec![3, if rng.gen_bool(0.5) { 4 } else { 7 }]), // 34, 37
            "discover" => (16, vec![6, 0, 1, 1]),
            _ => (16, vec![4]),
        };

        while prefix.len() < len - 1 {
            prefix.push(rng.gen_range(0..10));
        }

        // Luhn algorithm
        let mut sum = 0;
        for (i, &d) in prefix.iter().rev().enumerate() {
            let mut val = d;
            if i % 2 == 0 {
                val *= 2;
                if val > 9 {
                    val -= 9;
                }
            }
            sum += val;
        }
        let check_digit = (10 - (sum % 10)) % 10;
        prefix.push(check_digit);

        let number = prefix.iter().map(|d| d.to_string()).collect::<String>();

        // Expiry
        let month = rng.gen_range(1..=12);
        let year = rng.gen_range(25..30); // 2025-2030
        let expiry = format!("{:02}/{:02}", month, year);

        // CVV
        let cvv_len = if issuer == "amex" { 4 } else { 3 };
        let cvv = (0..cvv_len)
            .map(|_| rng.gen_range(0..10).to_string())
            .collect::<String>();

        cards.push(CreditCard {
            number,
            issuer: issuer.to_string(),
            expiry,
            cvv,
        });
    }
    cards
}

// --- 40. Awk Command ---
#[derive(Serialize)]
pub struct AwkResponse {
    pub command: String,
}
pub fn generate_awk(separator: &str, variable: &str, code: &str, file: &str) -> AwkResponse {
    let mut cmd = String::from("awk");

    if !separator.is_empty() && separator != "space" {
        cmd.push_str(" -F '");
        cmd.push_str(&separator.replace('\'', "'\\''"));
        cmd.push('\'');
    }

    if !variable.trim().is_empty() {
        cmd.push_str(" -v ");
        cmd.push_str(variable.trim());
    }

    cmd.push_str(" '");
    if !code.trim().is_empty() {
        cmd.push_str(code.trim());
    } else {
        cmd.push_str("{print $0}");
    }
    cmd.push('\'');

    if !file.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(file.trim());
    }

    AwkResponse { command: cmd }
}

// --- 41. Sed Command ---
#[derive(Serialize)]
pub struct SedResponse {
    pub command: String,
}
pub fn generate_sed(
    operation: &str,
    pattern: &str,
    replacement: &str,
    flags: &str,
    inplace: bool,
    file: &str,
) -> SedResponse {
    let mut cmd = String::from("sed");

    if inplace {
        cmd.push_str(" -i");
    }

    cmd.push_str(" '");

    match operation {
        "substitute" => {
            cmd.push('s');
            cmd.push('/');
            cmd.push_str(&pattern.replace('/', "\\/"));
            cmd.push('/');
            cmd.push_str(&replacement.replace('/', "\\/"));
            cmd.push('/');
            if !flags.is_empty() {
                cmd.push_str(flags);
            }
        }
        "delete" => {
            cmd.push_str(pattern);
            cmd.push('d');
        }
        "insert" => {
            cmd.push_str(pattern);
            cmd.push_str("i\\");
            cmd.push(' ');
            cmd.push_str(replacement);
        }
        "append" => {
            cmd.push_str(pattern);
            cmd.push_str("a\\");
            cmd.push(' ');
            cmd.push_str(replacement);
        }
        _ => {}
    }
    cmd.push('\'');

    if !file.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(file.trim());
    }

    SedResponse { command: cmd }
}
