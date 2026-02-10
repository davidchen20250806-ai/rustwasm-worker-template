use serde::Serialize;
use md5;
use rand::Rng;
use uuid::Uuid;
use chrono::{DateTime, Utc, TimeZone, NaiveDateTime};
use csscolorparser::Color;
use serde_yaml;
use toml;
use base64::{Engine as _, engine::general_purpose};
use rand::seq::SliceRandom;
use qrcode::{QrCode, EcLevel}; 
use qrcode::render::svg;
use std::net::Ipv4Addr;
use ipnetwork::Ipv4Network;
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use url::Url;

// 1. MD5
#[derive(Serialize)]
pub struct Md5Response {
    pub md5_32_lower: String,
    pub md5_32_upper: String,
    pub md5_16_lower: String,
    pub md5_16_upper: String,
}

pub fn calculate_md5(input: &str) -> Md5Response {
    let digest = md5::compute(input);
    let md5_32_lower = format!("{:x}", digest);
    let md5_32_upper = format!("{:X}", digest);
    let md5_16_lower = md5_32_lower[8..24].to_string();
    let md5_16_upper = md5_32_upper[8..24].to_string();
    Md5Response { md5_32_lower, md5_32_upper, md5_16_lower, md5_16_upper }
}

// 2. Token
pub fn generate_token(length: usize, upper: bool, lower: bool, number: bool, symbol: bool) -> String {
    let mut charset = String::new();
    if upper { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if lower { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if number { charset.push_str("0123456789"); }
    if symbol { charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?"); }
    if charset.is_empty() { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = charset.chars().collect();
    (0..length).map(|_| { let idx = rng.gen_range(0..chars.len()); chars[idx] }).collect()
}

// 3. UUID
pub struct UuidConfig {
    pub count: usize,
    pub uppercase: bool,
    pub hyphens: bool,
}
pub fn generate_uuids(config: UuidConfig) -> Vec<String> {
    let mut results = Vec::new();
    for _ in 0..config.count {
        let id = Uuid::new_v4();
        let mut id_str = if config.hyphens { id.to_string() } else { id.simple().to_string() };
        if config.uppercase { id_str = id_str.to_uppercase(); }
        results.push(id_str);
    }
    results
}

// 4. Date
#[derive(Serialize)]
pub struct DateResponse {
    pub unix_sec: i64,
    pub unix_milli: i64,
    pub iso_8601: String,
    pub rfc_2822: String,
    pub human_utc: String,
}
pub fn parse_date(input: &str) -> Result<DateResponse, String> {
    let input = input.trim();
    let dt: DateTime<Utc>;
    
    // 尝试解析为时间戳
    if let Ok(ts) = input.parse::<i64>() {
        // 如果数字很大，当做毫秒处理
        if ts > 30_000_000_000 { 
            dt = Utc.timestamp_millis_opt(ts).unwrap(); 
        } else { 
            dt = Utc.timestamp_opt(ts, 0).unwrap(); 
        }
    } else {
        // 尝试解析字符串格式
        let formats = vec!["%Y-%m-%d %H:%M:%S", "%Y-%m-%dT%H:%M:%S", "%Y-%m-%d"];
        let mut parsed = None;
        for fmt in formats {
            // 尝试带时间的格式
            if let Ok(naive) = NaiveDateTime::parse_from_str(input, fmt) {
                parsed = Some(DateTime::from_naive_utc_and_offset(naive, Utc));
                break;
            }
            // 尝试只带日期的格式
            if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(input, fmt) {
                if let Some(naive_dt) = naive_date.and_hms_opt(0, 0, 0) {
                    parsed = Some(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
                    break;
                }
            }
        }
        
        // 如果都失败，尝试 RFC3339
        if parsed.is_none() {
            if let Ok(rfc_dt) = DateTime::parse_from_rfc3339(input) { 
                parsed = Some(rfc_dt.with_timezone(&Utc)); 
            }
        }
        
        match parsed {
            Some(d) => dt = d,
            None => return Err("无法识别的时间格式".to_string()),
        }
    }
    
    Ok(DateResponse {
        unix_sec: dt.timestamp(),
        unix_milli: dt.timestamp_millis(),
        iso_8601: dt.to_rfc3339(),
        rfc_2822: dt.to_rfc2822(),
        human_utc: dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
    })
}

// 5. Color
#[derive(Serialize)]
pub struct ColorResponse {
    pub valid: bool,
    pub hex: String,
    pub rgb: String,
    pub hsl: String,
    pub cmyk: String,
    pub name: String,
}
pub fn convert_color(input: &str) -> ColorResponse {
    match input.parse::<Color>() {
        Ok(c) => {
            let [r, g, b, a] = c.to_array();
            let r255 = (r * 255.0).round() as u8;
            let g255 = (g * 255.0).round() as u8;
            let b255 = (b * 255.0).round() as u8;
            let hex = if a < 0.999 { c.to_hex_string() } else { format!("#{:02x}{:02x}{:02x}", r255, g255, b255) };
            let rgb = if a < 0.999 { format!("rgba({}, {}, {}, {:.2})", r255, g255, b255, a) } else { format!("rgb({}, {}, {})", r255, g255, b255) };
            let (h, s, l, a_val) = c.to_hsla();
            let hsl = if a_val < 0.999 { format!("hsla({:.0}, {:.0}%, {:.0}%, {:.2})", h, s * 100.0, l * 100.0, a_val) } else { format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0) };
            let c_k = 1.0 - r.max(g).max(b);
            let cmyk = if c_k < 0.999 {
                let c_c = (1.0 - r - c_k) / (1.0 - c_k);
                let c_m = (1.0 - g - c_k) / (1.0 - c_k);
                let c_y = (1.0 - b - c_k) / (1.0 - c_k);
                format!("cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)", c_c*100.0, c_m*100.0, c_y*100.0, c_k*100.0)
            } else { "cmyk(0%, 0%, 0%, 100%)".to_string() };
            ColorResponse { valid: true, hex, rgb, hsl, cmyk, name: "Color".to_string() }
        },
        Err(_) => ColorResponse { valid: false, hex: "".into(), rgb: "".into(), hsl: "".into(), cmyk: "".into(), name: "".into() }
    }
}

// 6. YAML/TOML
#[derive(Serialize)]
pub struct ConvertResponse { pub result: String, pub error: Option<String> }
pub fn yaml_to_toml(input: &str) -> ConvertResponse {
    let value: serde_yaml::Value = match serde_yaml::from_str(input) { Ok(v) => v, Err(e) => return ConvertResponse { result: "".to_string(), error: Some(format!("YAML Err: {}", e)) } };
    match toml::to_string(&value) { Ok(s) => ConvertResponse { result: s, error: None }, Err(e) => ConvertResponse { result: "".to_string(), error: Some(format!("TOML Err: {}", e)) } }
}
pub fn toml_to_yaml(input: &str) -> ConvertResponse {
    let value: toml::Value = match toml::from_str(input) { Ok(v) => v, Err(e) => return ConvertResponse { result: "".to_string(), error: Some(format!("TOML Err: {}", e)) } };
    match serde_yaml::to_string(&value) { Ok(s) => ConvertResponse { result: s, error: None }, Err(e) => ConvertResponse { result: "".to_string(), error: Some(format!("YAML Err: {}", e)) } }
}

// 7. JS Enc
#[derive(Serialize)]
pub struct JsEncResponse { pub result: String }
pub fn obfuscate_js(input: &str) -> JsEncResponse {
    let encoded = general_purpose::STANDARD.encode(input);
    let packed_code = format!("eval(decodeURIComponent(escape(window.atob('{}'))));", encoded);
    JsEncResponse { result: packed_code }
}

// 8. JSON & URL
#[derive(Serialize)]
pub struct JsonResponse { pub pretty: String, pub minified: String, pub error: Option<String> }
pub fn process_json(input: &str) -> JsonResponse {
    let parsed: serde_json::Value = match serde_json::from_str(input) { Ok(v) => v, Err(e) => return JsonResponse { pretty: "".into(), minified: "".into(), error: Some(format!("JSON Error: {}", e)) } };
    JsonResponse { pretty: serde_json::to_string_pretty(&parsed).unwrap_or_default(), minified: serde_json::to_string(&parsed).unwrap_or_default(), error: None }
}

#[derive(Serialize)]
pub struct UrlResponse { 
    pub encoded: String, 
    pub decoded: String, 
    pub protocol: String, 
    pub host: String, 
    pub path: String, 
    pub params: Vec<(String, String)>, 
    pub error: Option<String> 
}

pub fn process_url(input: &str) -> UrlResponse {
    let input = input.trim();
    
    // 编解码
    let encoded = utf8_percent_encode(input, NON_ALPHANUMERIC).to_string();
    let decoded = percent_decode_str(input).decode_utf8_lossy().to_string();

    match Url::parse(input) {
        Ok(u) => {
            // 修复类型推断错误：显式转换 Cow<str> 到 String
            let params: Vec<(String, String)> = u.query_pairs()
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect();
                
            UrlResponse { 
                encoded, decoded, 
                protocol: u.scheme().to_string(), 
                host: u.host_str().unwrap_or("").to_string(), 
                path: u.path().to_string(), 
                params, 
                error: None 
            }
        },
        Err(_) => UrlResponse { 
            encoded, decoded, 
            protocol: "".into(), host: "".into(), path: "".into(), params: vec![], 
            error: Some("非完整 URL，仅显示编解码结果".into()) 
        }
    }
}

// 9. QR Code
#[derive(Serialize)]
pub struct QrResponse { pub svg: String }
pub fn generate_qr_svg(text: &str) -> Result<QrResponse, String> {
    let code = QrCode::with_error_correction_level(text, EcLevel::H).map_err(|e| format!("QR Error: {}", e))?;
    let svg = code.render().min_dimensions(200, 200).dark_color(svg::Color("#000000")).light_color(svg::Color("#ffffff")).quiet_zone(false).build();
    Ok(QrResponse { svg })
}

// 10. Strong Password
#[derive(Serialize)]
pub struct PasswordResponse { pub password: String }
pub fn generate_password_strong(length: usize, upper: bool, lower: bool, number: bool, symbol: bool) -> PasswordResponse {
    let mut rng = rand::thread_rng();
    let mut password_chars: Vec<char> = Vec::new();
    let mut charset: Vec<char> = Vec::new();
    let upper_chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let lower_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let number_chars: Vec<char> = "0123456789".chars().collect();
    let symbol_chars: Vec<char> = "!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect();
    if upper { password_chars.push(upper_chars[rng.gen_range(0..upper_chars.len())]); charset.extend(&upper_chars); }
    if lower { password_chars.push(lower_chars[rng.gen_range(0..lower_chars.len())]); charset.extend(&lower_chars); }
    if number { password_chars.push(number_chars[rng.gen_range(0..number_chars.len())]); charset.extend(&number_chars); }
    if symbol { password_chars.push(symbol_chars[rng.gen_range(0..symbol_chars.len())]); charset.extend(&symbol_chars); }
    if charset.is_empty() { charset.extend(&lower_chars); }
    let current_len = password_chars.len();
    if length > current_len { for _ in 0..(length - current_len) { let idx = rng.gen_range(0..charset.len()); password_chars.push(charset[idx]); } }
    password_chars.shuffle(&mut rng);
    PasswordResponse { password: password_chars.into_iter().take(length).collect() }
}

// 11. Chmod
#[derive(Serialize)]
pub struct ChmodResponse { pub valid: bool, pub octal: String, pub symbolic: String, pub command: String }
pub fn calculate_chmod(input: &str) -> ChmodResponse {
    let clean_input = input.trim();
    if clean_input.len() != 3 || !clean_input.chars().all(|c| c >= '0' && c <= '7') {
        return ChmodResponse { valid: false, octal: clean_input.to_string(), symbolic: "Invalid".to_string(), command: "".to_string() };
    }
    let mut symbolic = String::from("-");
    let map = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];
    for c in clean_input.chars() { let digit = c.to_digit(10).unwrap() as usize; symbolic.push_str(map[digit]); }
    ChmodResponse { valid: true, octal: clean_input.to_string(), symbolic, command: format!("chmod {} filename", clean_input) }
}

// 12. Subnet Calculator
#[derive(Serialize)]
pub struct SubnetResponse {
    pub valid: bool,
    pub ip: String,
    pub mask: String,
    pub network: String,
    pub broadcast: String,
    pub first_ip: String,
    pub last_ip: String,
    pub total_hosts: u64,
    pub usable_hosts: u64,
    pub cidr: String,
    pub class: String,
    pub binary_ip: String,
    pub binary_mask: String,
}

pub fn calculate_subnet(ip_str: &str, prefix: u8) -> SubnetResponse {
    let ip: Ipv4Addr = match ip_str.parse() {
        Ok(i) => i,
        Err(_) => return SubnetResponse { 
            valid: false, ip: "".into(), mask: "".into(), network: "".into(), broadcast: "".into(),
            first_ip: "".into(), last_ip: "".into(), total_hosts: 0, usable_hosts: 0, cidr: "".into(),
            class: "".into(), binary_ip: "".into(), binary_mask: "".into()
        },
    };

    let net = match Ipv4Network::new(ip, prefix) {
        Ok(n) => n,
        Err(_) => return calculate_subnet("0.0.0.0", 0),
    };

    let mask = net.mask();
    let network_addr = net.network();
    let broadcast_addr = net.broadcast();
    
    // 显式转换 u32 -> u64，解决编译器报错
    let total = net.size() as u64; 
    let usable = if total > 2 { total - 2 } else { 0 };

    let first = if total > 2 {
        let n: u32 = network_addr.into();
        Ipv4Addr::from(n + 1)
    } else { network_addr };

    let last = if total > 2 {
        let b: u32 = broadcast_addr.into();
        Ipv4Addr::from(b - 1)
    } else { broadcast_addr };

    let first_octet = ip.octets()[0];
    let class = if first_octet < 128 { "A" } else if first_octet < 192 { "B" } else if first_octet < 224 { "C" } else if first_octet < 240 { "D (Multicast)" } else { "E (Reserved)" };
    let to_binary = |addr: Ipv4Addr| -> String { addr.octets().iter().map(|&o| format!("{:08b}", o)).collect::<Vec<String>>().join(".") };

    SubnetResponse {
        valid: true,
        ip: ip.to_string(),
        mask: mask.to_string(),
        network: network_addr.to_string(),
        broadcast: broadcast_addr.to_string(),
        first_ip: first.to_string(),
        last_ip: last.to_string(),
        total_hosts: total,
        usable_hosts: usable,
        cidr: net.to_string(),
        class: class.to_string(),
        binary_ip: to_binary(ip),
        binary_mask: to_binary(mask),
    }
}

// 13. JWT Parser
#[derive(Serialize)]
pub struct JwtResponse {
    pub header: String,
    pub payload: String,
    pub error: Option<String>,
}

pub fn decode_jwt_parts(token: &str) -> JwtResponse {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return JwtResponse {
            header: "".into(),
            payload: "".into(),
            error: Some("Token 格式无效，找不到 Header 或 Payload".into()),
        };
    }

    let decode_part = |part: &str| -> String {
        // 使用 URL_SAFE_NO_PAD 配置来解码 JWT
        match general_purpose::URL_SAFE_NO_PAD.decode(part) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => match serde_json::from_str::<serde_json::Value>(&s) {
                    Ok(v) => serde_json::to_string_pretty(&v).unwrap_or(s),
                    Err(_) => s, // 解析 JSON 失败，直接返回原始字符串
                },
                Err(_) => "内容非 UTF-8 格式".to_string(),
            },
            Err(_) => "Base64 解码失败".to_string(),
        }
    };

    JwtResponse {
        header: decode_part(parts[0]),
        payload: decode_part(parts[1]),
        error: None,
    }
}