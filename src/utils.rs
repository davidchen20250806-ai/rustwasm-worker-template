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
use serde::Serialize;
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
        "password" => r"^[a-zA-Z0-9!@#$%^&*()_+\-=\[\]{};':,.<>/?]{8,}$",
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
pub fn process_url(input: &str) -> (String, String, String, String, Vec<(String, String)>) {
    let decoded = percent_decode_str(input).decode_utf8_lossy().to_string();
    let encoded = utf8_percent_encode(input, NON_ALPHANUMERIC).to_string();

    // Default to decoding behavior for analysis
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
        if let Some(h) = u.host_str() {
            host = h.to_string();
        }
        path = u.path().to_string();
        for (k, v) in u.query_pairs() {
            params.push((k.to_string(), v.to_string()));
        }
    }
    (encoded, decoded, host, path, params)
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
pub fn calculate_chmod(octal: &str) -> ChmodResponse {
    if octal.len() != 3 || octal.chars().any(|c| !c.is_digit(8)) {
        return ChmodResponse {
            valid: false,
            command: "Invalid".into(),
        };
    }
    ChmodResponse {
        valid: true,
        command: format!("chmod {} filename", octal),
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
