use worker::*;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

mod html;
mod utils;

// ... 之前的 Base64Request, Md5Request 定义保持不变 ...
#[derive(Deserialize)]
struct Base64Request { text: String, action: String }
#[derive(Serialize)]
struct Base64Response { result: String }
#[derive(Deserialize)]
struct Md5Request { text: String }
#[derive(Deserialize)]
struct UuidRequest {
    count: usize,
    uppercase: bool,
    hyphens: bool,
}
// 👇 新增：Date 请求参数
#[derive(Deserialize)]

struct DateRequest {
    input: String,
}
#[derive(Serialize)]
struct UuidResponse {
    uuids: Vec<String>,
}

// 👇 新增：Token 请求参数
#[derive(Deserialize)]
struct TokenRequest {
    length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
}
#[derive(Deserialize)]
struct YamlRequest {
    yaml: String,
}

// 👇 新增：Token 返回结果
#[derive(Serialize)]
struct TokenResponse {
    token: String,
}
#[derive(Deserialize)]
struct ColorRequest {
    input: String,
}

#[derive(Deserialize)]
struct TomlRequest {
    toml: String,
}

// 👇 新增：JS 加密请求参数
#[derive(Deserialize)]
struct JsEncRequest {
    js: String,
}
#[derive(Deserialize)]
struct CommonRequest { input: String }

// 👇 新增：密码请求结构
#[derive(Deserialize)]
struct PasswordRequest {
    length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
}
// 👇 新增：QR 请求结构
#[derive(Deserialize)]
struct QrRequest {
    text: String,
}
// 👇 新增：Chmod 请求参数
#[derive(Deserialize)]
struct ChmodRequest {
    octal: String, // 接收 "755"
}
// 👇 新增：Subnet 请求结构
#[derive(Deserialize)]
struct SubnetRequest {
    ip: String,
    cidr: u8,
}

// 👇 新增：JWT 请求结构
#[derive(Deserialize)]
struct JwtRequest {
    token: String,
}
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| Response::from_html(html::get_homepage()))
        
        // ... Base64 和 MD5 的路由保持不变 ...
        .post_async("/api/base64", |mut req, _| async move {
             let data: Base64Request = req.json().await?;
             let result = match data.action.as_str() {
                 "encode" => general_purpose::STANDARD.encode(data.text.as_bytes()),
                 "decode" => match general_purpose::STANDARD.decode(data.text.as_bytes()) {
                     Ok(bytes) => String::from_utf8(bytes).unwrap_or("非 UTF-8".into()),
                     Err(_) => "解码失败".into(),
                 },
                 _ => "未知操作".into(),
             };
             Response::from_json(&Base64Response { result })
        })
        .post_async("/api/md5", |mut req, _| async move {
            let data: Md5Request = req.json().await?;
            let response = utils::calculate_md5(&data.text);
            Response::from_json(&response)
        })

        // 👇 新增：Token 生成接口
        .post_async("/api/token", |mut req, _| async move {
            let data: TokenRequest = req.json().await?;
            // 限制一下最大长度，防止恶意请求卡死服务器
            let len = if data.length > 1024 { 1024 } else { data.length };
            
            let token = utils::generate_token(len, data.uppercase, data.lowercase, data.numbers, data.symbols);
            Response::from_json(&TokenResponse { token })
        })
        .post_async("/api/uuid", |mut req, _| async move {
            let data: UuidRequest = req.json().await?;
            // 限制最大生成数量，防止卡死
            let count = if data.count > 100 { 100 } else { data.count };
            
            let uuids = utils::generate_uuids(utils::UuidConfig {
                count,
                uppercase: data.uppercase,
                hyphens: data.hyphens,
            });
            
            
            Response::from_json(&UuidResponse { uuids })
        })
        .post_async("/api/date", |mut req, _| async move {
            let data: DateRequest = req.json().await?;
            
            match utils::parse_date(&data.input) {
                Ok(response) => Response::from_json(&response),
                Err(e) => Response::error(e, 400),
            }
        })
        .post_async("/api/color", |mut req, _| async move {
            let data: ColorRequest = req.json().await?;
            let response = utils::convert_color(&data.input);
            if response.valid {
                Response::from_json(&response)
            } else {
                Response::error("无效的颜色值", 400)
            }
        })

        .post_async("/api/yaml-to-toml", |mut req, _| async move {
            let data: YamlRequest = req.json().await?;
            let response = utils::yaml_to_toml(&data.yaml);
            Response::from_json(&response)
        })
        // 👇 新增：TOML 转 YAML 接口
        .post_async("/api/toml-to-yaml", |mut req, _| async move {
            let data: TomlRequest = req.json().await?;
            let response = utils::toml_to_yaml(&data.toml);
            Response::from_json(&response)
        })
        .post_async("/api/js-enc", |mut req, _| async move {
            let data: JsEncRequest = req.json().await?;
            let response = utils::obfuscate_js(&data.js);
            Response::from_json(&response)
        })
        .post_async("/api/json", |mut req, _| async move {
            let data: CommonRequest = req.json().await?;
            let response = utils::process_json(&data.input);
            Response::from_json(&response)
        })
        
        // 👇 新增：URL 接口
        .post_async("/api/url", |mut req, _| async move {
            let data: CommonRequest = req.json().await?;
            let response = utils::process_url(&data.input);
            Response::from_json(&response)
        })
        // 👇 新增：强密码生成接口
        .post_async("/api/password", |mut req, _| async move {
            let data: PasswordRequest = req.json().await?;
            // 限制长度 4 ~ 128
            let len = if data.length < 4 { 4 } else if data.length > 128 { 128 } else { data.length };
            
            let response = utils::generate_password_strong(len, data.uppercase, data.lowercase, data.numbers, data.symbols);
            Response::from_json(&response)
        })
        .post_async("/api/qrcode", |mut req, _| async move {
            let data: QrRequest = req.json().await?;
            if data.text.is_empty() {
                return Response::error("内容不能为空", 400);
            }
            
            match utils::generate_qr_svg(&data.text) {
                Ok(res) => Response::from_json(&res),
                Err(e) => Response::error(e, 500),
            }
        })
        // 👇 新增：Linux 权限接口
        .post_async("/api/chmod", |mut req, _| async move {
            let data: ChmodRequest = req.json().await?;
            let response = utils::calculate_chmod(&data.octal);
            Response::from_json(&response)
        })
        // 👇 新增：子网计算接口
        .post_async("/api/subnet", |mut req, _| async move {
            let data: SubnetRequest = req.json().await?;
            // 简单的防呆，cidr 必须在 0-32 之间
            let cidr = if data.cidr > 32 { 32 } else { data.cidr };
            
            let response = utils::calculate_subnet(&data.ip, cidr);
            if response.valid {
                Response::from_json(&response)
            } else {
                Response::error("无效的 IP 地址", 400)
            }
        })
        // 👇 新增：JWT 解析接口
        .post_async("/api/jwt", |mut req, _| async move {
            let data: JwtRequest = req.json().await?;
            let response = utils::decode_jwt_parts(&data.token);
            Response::from_json(&response)
        })

        .run(req, env)
        .await
}