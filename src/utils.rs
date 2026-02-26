use crate::models::*;
use base64::{engine::general_purpose, Engine as _};
use chrono::{TimeZone, Utc};
use cron::Schedule;
use ipnetwork::IpNetwork;
use qrcode::render::svg;
use qrcode::QrCode;
use rand::Rng;
use serde_json::Value;
use similar::{ChangeTag, TextDiff};
use std::str::FromStr;

pub fn format_sql(sql: &str) -> String {
    let options = sqlformat::FormatOptions {
        indent: sqlformat::Indent::Spaces(2),
        uppercase: true,
        lines_between_queries: 1,
    };
    sqlformat::format(sql, &sqlformat::QueryParams::None, options)
}

pub fn compute_diff(old: &str, new: &str) -> DiffResponse {
    let diff = TextDiff::from_lines(old, new);
    let mut chunks = Vec::new();

    for change in diff.iter_all_changes() {
        let tag = match change.tag() {
            ChangeTag::Delete => "delete",
            ChangeTag::Insert => "insert",
            ChangeTag::Equal => "equal",
        };
        chunks.push(DiffChunk {
            tag: tag.to_string(),
            text: change.to_string(),
        });
    }
    DiffResponse { chunks }
}

pub fn check_cron(cron: &str) -> CronResponse {
    // The `cron` crate requires 6 or 7 fields (Seconds is the first one).
    // Standard Linux cron has 5 fields. We need to handle this.
    let cron_expr = if cron.trim().split_whitespace().count() == 5 {
        format!("0 {}", cron)
    } else {
        cron.to_string()
    };
    match Schedule::from_str(&cron_expr) {
        Ok(schedule) => {
            let next_runs = schedule
                .upcoming(Utc)
                .take(5)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
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

pub fn calculate_subnet(ip: &str, cidr: u8) -> SubnetResponse {
    let net_str = format!("{}/{}", ip, cidr);
    if let Ok(net) = net_str.parse::<IpNetwork>() {
        let mask = net.mask();
        let network = net.network();

        let (broadcast, first_ip, last_ip, wildcard, binary_ip, binary_mask, ip_class, ip_type) =
            match net {
                IpNetwork::V4(v4_net) => {
                    let broadcast = v4_net.broadcast();
                    let mask_u32 = u32::from(v4_net.mask());
                    let wildcard_u32 = !mask_u32;
                    let wildcard = std::net::Ipv4Addr::from(wildcard_u32).to_string();

                    let first = if v4_net.prefix() >= 31 {
                        v4_net.network()
                    } else {
                        std::net::Ipv4Addr::from(u32::from(v4_net.network()) + 1)
                    };
                    let last = if v4_net.prefix() >= 31 {
                        v4_net.broadcast()
                    } else {
                        std::net::Ipv4Addr::from(u32::from(v4_net.broadcast()) - 1)
                    };

                    let ip_u32 = u32::from(v4_net.ip());
                    let bin_ip = format!(
                        "{:08b}.{:08b}.{:08b}.{:08b}",
                        ip_u32 >> 24,
                        (ip_u32 >> 16) & 0xFF,
                        (ip_u32 >> 8) & 0xFF,
                        ip_u32 & 0xFF
                    );
                    let bin_mask = format!(
                        "{:08b}.{:08b}.{:08b}.{:08b}",
                        mask_u32 >> 24,
                        (mask_u32 >> 16) & 0xFF,
                        (mask_u32 >> 8) & 0xFF,
                        mask_u32 & 0xFF
                    );

                    let first_octet = (ip_u32 >> 24) as u8;
                    let class = if first_octet < 128 {
                        "A"
                    } else if first_octet < 192 {
                        "B"
                    } else if first_octet < 224 {
                        "C"
                    } else if first_octet < 240 {
                        "D (Multicast)"
                    } else {
                        "E (Reserved)"
                    };

                    let type_ = if v4_net.ip().is_private() {
                        "Private"
                    } else {
                        "Public"
                    };

                    (
                        broadcast.to_string(),
                        first.to_string(),
                        last.to_string(),
                        wildcard,
                        bin_ip,
                        bin_mask,
                        class.to_string(),
                        type_.to_string(),
                    )
                }
                IpNetwork::V6(v6_net) => (
                    "".to_string(),
                    v6_net.network().to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "IPv6".to_string(),
                    "Public".to_string(),
                ),
            };

        let total_hosts = if net.prefix() == 32 {
            1
        } else {
            2u64.pow(32 - net.prefix() as u32)
        };
        let usable_hosts = if total_hosts > 2 { total_hosts - 2 } else { 0 };

        SubnetResponse {
            valid: true,
            ip: ip.to_string(),
            cidr: cidr.to_string(),
            mask: mask.to_string(),
            wildcard,
            network: network.to_string(),
            broadcast,
            first_ip,
            last_ip,
            total_hosts,
            usable_hosts,
            ip_class,
            ip_type,
            binary_ip,
            binary_mask,
        }
    } else {
        SubnetResponse {
            valid: false,
            ip: ip.to_string(),
            cidr: cidr.to_string(),
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
}

pub fn calculate_md5(text: &str) -> Md5Response {
    let digest = md5::compute(text);
    let hash = format!("{:x}", digest);
    Md5Response {
        md5_32_lower: hash.clone(),
        md5_32_upper: hash.to_uppercase(),
        md5_16_lower: hash[8..24].to_string(),
        md5_16_upper: hash[8..24].to_uppercase(),
    }
}

pub fn generate_token(
    length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> String {
    let mut charset = String::new();
    if uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if lowercase {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if numbers {
        charset.push_str("0123456789");
    }
    if symbols {
        charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
    }

    if charset.is_empty() {
        return String::new();
    }

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn generate_uuids(config: UuidConfig) -> Vec<String> {
    let mut res = Vec::new();
    for _ in 0..config.count {
        let uuid = uuid::Uuid::new_v4();
        let mut s = if config.hyphens {
            uuid.to_string()
        } else {
            uuid.simple().to_string()
        };
        if config.uppercase {
            s = s.to_uppercase();
        }
        res.push(s);
    }
    res
}

pub fn parse_date(input: &str) -> DateResponse {
    let get_dt = || -> Option<chrono::DateTime<Utc>> {
        let trimmed_input = input.trim();
        if let Ok(ts) = trimmed_input.parse::<i64>() {
            return if ts > 100_000_000_000 {
                // Heuristic for milliseconds
                Utc.timestamp_millis_opt(ts).single()
            } else {
                Utc.timestamp_opt(ts, 0).single()
            };
        }

        if let Ok(d) = chrono::DateTime::parse_from_rfc3339(trimmed_input) {
            return Some(d.with_timezone(&Utc));
        }

        let formats = [
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%d %H:%M",
            "%Y-%m-%d",
            "%Y/%m/%d %H:%M:%S",
            "%Y/%m/%d",
        ];

        for fmt in formats {
            if let Ok(ndt) = chrono::NaiveDateTime::parse_from_str(trimmed_input, fmt) {
                return Some(Utc.from_utc_datetime(&ndt));
            }
            if let Ok(nd) = chrono::NaiveDate::parse_from_str(trimmed_input, fmt) {
                if let Some(ndt) = nd.and_hms_opt(0, 0, 0) {
                    return Some(Utc.from_utc_datetime(&ndt));
                }
            }
        }

        None
    };

    if let Some(dt) = get_dt() {
        DateResponse {
            valid: true,
            unix_sec: dt.timestamp(),
            unix_milli: dt.timestamp_millis(),
            iso_8601: dt.to_rfc3339(),
            human_utc: dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            error: None,
        }
    } else {
        DateResponse {
            valid: false,
            unix_sec: 0,
            unix_milli: 0,
            iso_8601: "".to_string(),
            human_utc: "".to_string(),
            error: Some("无效的日期格式或不存在的日期 (Invalid or non-existent date)".to_string()),
        }
    }
}

pub fn convert_color(input: &str) -> ColorResponse {
    match csscolorparser::parse(input) {
        Ok(c) => {
            let [r, g, b, _a] = c.to_rgba8();
            let hex = c.to_hex_string();
            let rgb = c.to_rgb_string();
            let (h, s, l, _a) = c.to_hsla();
            let hsl = format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0);

            let r_f = r as f64 / 255.0;
            let g_f = g as f64 / 255.0;
            let b_f = b as f64 / 255.0;
            let k = 1.0 - r_f.max(g_f).max(b_f);
            let cmyk = if k == 1.0 {
                "cmyk(0%, 0%, 0%, 100%)".to_string()
            } else {
                let c_v = (1.0 - r_f - k) / (1.0 - k);
                let m_v = (1.0 - g_f - k) / (1.0 - k);
                let y_v = (1.0 - b_f - k) / (1.0 - k);
                format!(
                    "cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)",
                    c_v * 100.0,
                    m_v * 100.0,
                    y_v * 100.0,
                    k * 100.0
                )
            };

            ColorResponse {
                valid: true,
                hex,
                rgb,
                hsl,
                cmyk,
            }
        }
        Err(_) => ColorResponse {
            valid: false,
            hex: "".into(),
            rgb: "".into(),
            hsl: "".to_string(),
            cmyk: "".into(),
        },
    }
}

pub fn process_base64(text: &str, action: &str) -> String {
    if action == "encode" {
        general_purpose::STANDARD.encode(text)
    } else {
        String::from_utf8(general_purpose::STANDARD.decode(text).unwrap_or_default())
            .unwrap_or_default()
    }
}

pub fn process_json(input: &str) -> (String, String, Option<String>) {
    match serde_json::from_str::<Value>(input) {
        Ok(v) => (
            serde_json::to_string_pretty(&v).unwrap_or_default(),
            serde_json::to_string(&v).unwrap_or_default(),
            None,
        ),
        Err(e) => (String::new(), String::new(), Some(e.to_string())),
    }
}

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
    let encoded = ::urlencoding::encode(input).to_string();
    let decoded = urlencoding::decode(input).unwrap_or_default().to_string();

    let mut protocol = String::new();
    let mut host = String::new();
    let mut path = String::new();
    let mut params = Vec::new();

    if let Ok(u) = url::Url::parse(input) {
        protocol = u.scheme().to_string();
        host = u.host_str().unwrap_or("").to_string();
        path = u.path().to_string();
        for (k, v) in u.query_pairs() {
            params.push((k.to_string(), v.to_string()));
        }
    }

    (encoded, decoded, protocol, host, path, params)
}

pub fn generate_password_strong(
    length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> String {
    generate_token(length, uppercase, lowercase, numbers, symbols)
}

pub fn generate_qr(text: &str) -> Result<String, String> {
    let code = QrCode::new(text).map_err(|e| e.to_string())?;
    let svg = code.render::<svg::Color>().build();
    Ok(svg)
}

pub fn yaml_to_toml(yaml: &str) -> (String, String) {
    match serde_yaml::from_str::<Value>(yaml) {
        Ok(v) => match toml::to_string(&v) {
            Ok(t) => (t, String::new()),
            Err(e) => (String::new(), e.to_string()),
        },
        Err(e) => (String::new(), e.to_string()),
    }
}

pub fn toml_to_yaml(toml: &str) -> (String, String) {
    match toml::from_str::<Value>(toml) {
        Ok(v) => match serde_yaml::to_string(&v) {
            Ok(s) => (s, String::new()),
            Err(e) => (String::new(), e.to_string()),
        },
        Err(e) => (String::new(), e.to_string()),
    }
}

pub fn parse_jwt(token: &str) -> JwtResponse {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return JwtResponse {
            error: Some("Invalid JWT format".into()),
            header: "".into(),
            payload: "".into(),
        };
    }
    let decode = |s: &str| -> String {
        let s = s.replace('-', "+").replace('_', "/");
        let padded = match s.len() % 4 {
            2 => format!("{}==", s),
            3 => format!("{}=", s),
            _ => s,
        };
        String::from_utf8(general_purpose::STANDARD.decode(padded).unwrap_or_default())
            .unwrap_or_default()
    };

    JwtResponse {
        error: None,
        header: decode(parts[0]),
        payload: decode(parts[1]),
    }
}

pub fn test_regex(pattern: &str, text: &str) -> RegexResponse {
    match regex::Regex::new(pattern) {
        Ok(re) => {
            let matches: Vec<String> = re.find_iter(text).map(|m| m.as_str().to_string()).collect();
            RegexResponse {
                count: matches.len(),
                matches,
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

pub fn get_common_regex(key: &str) -> &str {
    match key {
        "email" => r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
        "phone_cn" => r"^1[3-9]\d{9}$",
        "ipv4" => r"^((25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(25[0-5]|2[0-4]\d|[01]?\d\d?)$",
        "url" => r"^https?://([\w-]+\.)+[\w-]+(/[\w-./?%&=]*)?$",
        _ => "",
    }
}

pub fn process_escape(text: &str, mode: &str) -> String {
    match mode {
        "html_enc" => html_escape::encode_text(text).to_string(),
        "html_dec" => html_escape::decode_html_entities(text).to_string(),
        "json_enc" => text
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t"),
        "json_dec" => text
            .replace("\\\"", "\"")
            .replace("\\\\", "\\")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t"),
        _ => text.to_string(),
    }
}

pub fn generate_nginx_config(
    domain: &str,
    port: u16,
    root: &str,
    locations: &[NginxLocation],
    upstream: &str,
    https: bool,
    force_https: bool,
    ssl_cert: &str,
    ssl_key: &str,
    gzip: bool,
    client_max_body_size: &str,
    keepalive_timeout: &str,
    proxy_connect_timeout: &str,
    proxy_read_timeout: &str,
    proxy_send_timeout: &str,
) -> String {
    let mut conf = String::new();

    if !upstream.trim().is_empty() {
        conf.push_str("upstream backend {\n");
        for line in upstream.lines() {
            if !line.trim().is_empty() {
                conf.push_str(&format!("    server {};\n", line.trim()));
            }
        }
        conf.push_str("}\n\n");
    }

    conf.push_str("server {\n");
    conf.push_str(&format!("    listen {};\n", port));
    if https {
        conf.push_str("    listen 443 ssl;\n");
    }
    conf.push_str(&format!(
        "    server_name {};\n",
        if domain.is_empty() {
            "localhost"
        } else {
            domain
        }
    ));

    if https {
        conf.push_str(&format!(
            "    ssl_certificate {};\n",
            if ssl_cert.is_empty() {
                "/etc/nginx/ssl/cert.pem"
            } else {
                ssl_cert
            }
        ));
        conf.push_str(&format!(
            "    ssl_certificate_key {};\n",
            if ssl_key.is_empty() {
                "/etc/nginx/ssl/key.pem"
            } else {
                ssl_key
            }
        ));
    }

    if force_https && port != 443 {
        conf.push_str("    if ($scheme != \"https\") {\n");
        conf.push_str("        return 301 https://$host$request_uri;\n");
        conf.push_str("    }\n");
    }

    if !root.is_empty() {
        conf.push_str(&format!("    root {};\n", root));
    }
    conf.push_str("    index index.html index.htm;\n");

    if gzip {
        conf.push_str("    gzip on;\n");
        conf.push_str("    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;\n");
    }

    if !client_max_body_size.is_empty() {
        conf.push_str(&format!(
            "    client_max_body_size {};\n",
            client_max_body_size
        ));
    }
    if !keepalive_timeout.is_empty() {
        conf.push_str(&format!("    keepalive_timeout {};\n", keepalive_timeout));
    }

    for loc in locations {
        conf.push_str(&format!("\n    location {} {{\n", loc.path));
        if !loc.root.is_empty() {
            conf.push_str(&format!("        root {};\n", loc.root));
        }
        if !loc.proxy.is_empty() {
            conf.push_str(&format!("        proxy_pass {};\n", loc.proxy));
            conf.push_str("        proxy_set_header Host $host;\n");
            conf.push_str("        proxy_set_header X-Real-IP $remote_addr;\n");
            if !proxy_connect_timeout.is_empty() {
                conf.push_str(&format!(
                    "        proxy_connect_timeout {};\n",
                    proxy_connect_timeout
                ));
            }
            if !proxy_read_timeout.is_empty() {
                conf.push_str(&format!(
                    "        proxy_read_timeout {};\n",
                    proxy_read_timeout
                ));
            }
            if !proxy_send_timeout.is_empty() {
                conf.push_str(&format!(
                    "        proxy_send_timeout {};\n",
                    proxy_send_timeout
                ));
            }
        } else if loc.spa {
            conf.push_str("        try_files $uri $uri/ /index.html;\n");
        }
        conf.push_str("    }\n");
    }

    conf.push_str("}\n");
    conf
}
