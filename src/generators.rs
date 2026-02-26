use crate::models::*;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn generate_lorem(count: usize, mode: &str) -> String {
    let words = [
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
    ];
    let mut rng = rand::thread_rng();
    let mut result = String::new();

    if mode == "words" {
        for i in 0..count {
            if i > 0 {
                result.push(' ');
            }
            result.push_str(words.choose(&mut rng).unwrap());
        }
    } else if mode == "sentences" {
        for i in 0..count {
            if i > 0 {
                result.push(' ');
            }
            let len = rng.gen_range(5..15);
            let mut sentence = String::new();
            for j in 0..len {
                if j > 0 {
                    sentence.push(' ');
                }
                sentence.push_str(words.choose(&mut rng).unwrap());
            }
            // Capitalize first letter and add period
            let mut chars = sentence.chars();
            if let Some(first) = chars.next() {
                result.push(first.to_ascii_uppercase());
                result.push_str(chars.as_str());
            }
            result.push('.');
        }
    } else {
        // paragraphs
        for i in 0..count {
            if i > 0 {
                result.push_str("\n\n");
            }
            result.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.");
        }
    }
    result
}

pub fn generate_fake_users(count: usize, locale: &str) -> Vec<FakeUser> {
    let mut users = Vec::new();
    let mut rng = rand::thread_rng();

    let (first_names, last_names, cities) = if locale == "cn" {
        (
            vec!["伟", "芳", "娜", "敏", "静", "秀英", "丽", "强", "磊", "军"],
            vec!["王", "李", "张", "刘", "陈", "杨", "黄", "赵", "吴", "周"],
            vec!["北京", "上海", "广州", "深圳"],
        )
    } else {
        (
            vec![
                "James", "Mary", "John", "Patricia", "Robert", "Jennifer", "Michael", "Linda",
            ],
            vec![
                "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis",
            ],
            vec!["New York", "Los Angeles", "Chicago", "Houston"],
        )
    };
    let domains = [
        "gmail.com",
        "yahoo.com",
        "hotmail.com",
        "outlook.com",
        "example.com",
    ];

    for _ in 0..count {
        let first = first_names.choose(&mut rng).unwrap();
        let last = last_names.choose(&mut rng).unwrap();
        let domain = domains.choose(&mut rng).unwrap();
        let city = cities.choose(&mut rng).unwrap();

        let (name, email, address, phone) = if locale == "cn" {
            (
                format!("{}{}", last, first),
                format!("user{}@{}", rng.gen_range(1000..9999), domain),
                format!("{}市人民路 {}号", city, rng.gen_range(1..1000)),
                format!(
                    "1{}{}",
                    rng.gen_range(30..99),
                    rng.gen_range(10000000..99999999)
                ),
            )
        } else {
            (
                format!("{} {}", first, last),
                format!(
                    "{}.{}@{}",
                    first.to_lowercase(),
                    last.to_lowercase(),
                    domain
                ),
                format!("{} Main St, {}", rng.gen_range(1..9999), city),
                format!(
                    "+1-555-{}-{}",
                    rng.gen_range(100..999),
                    rng.gen_range(1000..9999)
                ),
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

pub fn generate_credit_cards(count: usize, issuer: &str) -> Vec<CreditCard> {
    let mut cards = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        // 1. 确定前缀和长度
        let (len, mut digits) = match issuer {
            "mastercard" => (16, vec![5, rng.gen_range(1..=5)]),
            "amex" => (15, vec![3, if rng.gen_bool(0.5) { 4 } else { 7 }]),
            "discover" => (16, vec![6, 0, 1, 1]),
            _ => (16, vec![4]), // Visa default
        };

        // 2. 填充随机数字直到最后一位之前
        while digits.len() < len - 1 {
            digits.push(rng.gen_range(0..10));
        }

        // 3. 计算 Luhn 校验位
        let mut sum = 0;
        for (i, &d) in digits.iter().rev().enumerate() {
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
        digits.push(check_digit);

        cards.push(CreditCard {
            number: digits.iter().map(|d| d.to_string()).collect(),
            issuer: issuer.to_string(),
            expiry: format!("{:02}/{:02}", rng.gen_range(1..=12), rng.gen_range(25..30)),
            cvv: format!("{:03}", rng.gen_range(100..999)),
        });
    }
    cards
}

pub fn generate_custom_regex(
    starts: &str,
    not_starts: &str,
    ends: &str,
    not_ends: &str,
    contains: &str,
    not_contains: &str,
) -> RegexBuildResponse {
    let esc = |s: &str| regex::escape(s);
    let mut p = String::from("^");

    if !not_starts.is_empty() {
        p.push_str(&format!("(?!{})", esc(not_starts)));
    }
    if !starts.is_empty() {
        p.push_str(&esc(starts));
    }
    if !contains.is_empty() {
        p.push_str(&format!("(?=.*{})", esc(contains)));
    }
    if !not_contains.is_empty() {
        p.push_str(&format!("(?:(?!{}).)*", esc(not_contains)));
    } else {
        p.push_str(".*");
    }
    if !ends.is_empty() {
        p.push_str(&esc(ends));
    }
    if !not_ends.is_empty() {
        p.push_str(&format!("(?<!{})", esc(not_ends)));
    }
    p.push('$');

    RegexBuildResponse {
        pattern: p,
        error: None,
    }
}
