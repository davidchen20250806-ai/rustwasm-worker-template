use crate::models::*;

pub fn convert_case(text: &str, mode: &str) -> String {
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }

    let mut words = Vec::new();
    let mut current_word = String::new();

    for (i, c) in text.chars().enumerate() {
        if c == ' ' || c == '_' || c == '-' {
            if !current_word.is_empty() {
                words.push(current_word);
                current_word = String::new();
            }
        } else if c.is_uppercase() {
            if !current_word.is_empty() {
                let prev = text.chars().nth(i - 1).unwrap_or(' ');
                if !prev.is_uppercase() {
                    words.push(current_word);
                    current_word = String::new();
                }
            }
            current_word.push(c);
        } else {
            current_word.push(c);
        }
    }
    if !current_word.is_empty() {
        words.push(current_word);
    }

    let words: Vec<String> = words.into_iter().map(|w| w.to_lowercase()).collect();

    match mode {
        "camel" => {
            let mut res = String::new();
            for (i, w) in words.iter().enumerate() {
                if i == 0 {
                    res.push_str(w);
                } else {
                    res.push_str(&capitalize(w));
                }
            }
            res
        }
        "pascal" => words.iter().map(|w| capitalize(w)).collect(),
        "snake" => words.join("_"),
        "kebab" => words.join("-"),
        "constant" => words.join("_").to_uppercase(),
        "upper" => text.to_uppercase(),
        "lower" => text.to_lowercase(),
        _ => text.to_string(),
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn convert_unit(value: f64, type_: &str, from: &str, to: &str) -> UnitResponse {
    let mut result = value;

    if type_ == "storage" {
        let to_bytes = |v: f64, u: &str| match u {
            "B" => v,
            "KB" => v * 1024.0,
            "MB" => v * 1024.0 * 1024.0,
            "GB" => v * 1024.0 * 1024.0 * 1024.0,
            "TB" => v * 1024.0 * 1024.0 * 1024.0 * 1024.0,
            "PB" => v * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
            _ => v,
        };
        let from_bytes = |v: f64, u: &str| match u {
            "B" => v,
            "KB" => v / 1024.0,
            "MB" => v / (1024.0 * 1024.0),
            "GB" => v / (1024.0 * 1024.0 * 1024.0),
            "TB" => v / (1024.0 * 1024.0 * 1024.0 * 1024.0),
            "PB" => v / (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0),
            _ => v,
        };

        let bytes = to_bytes(value, from);
        result = from_bytes(bytes, to);
    } else if type_ == "time" {
        let to_sec = |v: f64, u: &str| match u {
            "ms" => v / 1000.0,
            "s" => v,
            "m" => v * 60.0,
            "h" => v * 3600.0,
            "d" => v * 86400.0,
            _ => v,
        };
        let from_sec = |v: f64, u: &str| match u {
            "ms" => v * 1000.0,
            "s" => v,
            "m" => v / 60.0,
            "h" => v / 3600.0,
            "d" => v / 86400.0,
            _ => v,
        };

        let secs = to_sec(value, from);
        result = from_sec(secs, to);
    }

    UnitResponse {
        result,
        value,
        from: from.to_string(),
        to: to.to_string(),
        type_: type_.to_string(),
    }
}

pub fn obfuscate_js(js: &str) -> String {
    let mut res = String::from("eval(\"");
    for b in js.bytes() {
        res.push_str(&format!("\\x{:02x}", b));
    }
    res.push_str("\");");
    res
}
