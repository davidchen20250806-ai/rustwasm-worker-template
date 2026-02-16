use serde::{Deserialize, Serialize};
use worker::*;

mod html;
mod utils;

#[derive(Deserialize)]
struct SqlRequest {
    sql: String,
}
#[derive(Deserialize)]
struct DiffRequest {
    old: String,
    new: String,
}
#[derive(Deserialize)]
struct CronRequest {
    cron: String,
}
#[derive(Deserialize)]
struct SubnetRequest {
    ip: String,
    cidr: u8,
}
#[derive(Deserialize)]
struct RegexGenRequest {
    key: String,
}
#[derive(Deserialize)]
struct RegexRequest {
    pattern: String,
    text: String,
}
#[derive(Deserialize)]
struct UuidRequest {
    count: usize,
    hyphens: bool,
    uppercase: bool,
}
#[derive(Deserialize)]
struct JwtRequest {
    token: String,
}
#[derive(Deserialize)]
struct PasswordRequest {
    length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
}
#[derive(Deserialize)]
struct TokenRequest {
    length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
}
#[derive(Deserialize)]
struct Base64Request {
    text: String,
    action: String,
}
#[derive(Deserialize)]
struct JsonRequest {
    input: String,
}
#[derive(Deserialize)]
struct EscapeRequest {
    text: String,
    mode: String,
}
#[derive(Deserialize)]
struct DateRequest {
    input: String,
}
#[derive(Deserialize)]
struct ColorRequest {
    input: String,
}
#[derive(Deserialize)]
struct QrRequest {
    text: String,
}
#[derive(Deserialize)]
struct JsEncRequest {
    js: String,
}
#[derive(Deserialize)]
struct YamlRequest {
    yaml: String,
}
#[derive(Deserialize)]
struct TomlRequest {
    toml: String,
}
#[derive(Deserialize)]
struct Md5Request {
    text: String,
}
#[derive(Deserialize)]
struct ChmodRequest {
    octal: String,
}
#[derive(Deserialize)]
struct UrlRequest {
    input: String,
}
#[derive(Deserialize)]
struct CaseRequest {
    text: String,
    mode: String,
}
#[derive(Deserialize)]
struct TarRequest {
    op: String,
    comp: String,
    verbose: bool,
    archive: String,
    files: String,
}
#[derive(Deserialize)]
struct PsRequest {
    format: String,
    sort: String,
    tree: bool,
    filter: String,
}
#[derive(Deserialize)]
struct TcpdumpRequest {
    interface: String,
    protocol: String,
    host: String,
    port: String,
    verbose: bool,
    ascii: bool,
    hex: bool,
    write_file: String,
    count: String,
}
#[derive(Deserialize)]
struct GitRequest {
    cmd: String,
    target: String,
    msg: String,
    remote: String,
    branch: String,
    opt_force: bool,
    opt_rebase: bool,
    opt_all: bool,
    opt_amend: bool,
    opt_hard: bool,
    opt_new_branch: bool,
    opt_tags: bool,
    opt_oneline: bool,
    opt_graph: bool,
}
#[derive(Deserialize)]
struct StraceRequest {
    target: String,
    is_pid: bool,
    follow: bool,
    summary: bool,
    output_file: String,
    filter: String,
    string_limit: String,
    timestamp: bool,
}
#[derive(Deserialize)]
struct IostatRequest {
    interval: String,
    count: String,
    human: bool,
    extended: bool,
    unit: String,
    partitions: bool,
    timestamp: bool,
    device: String,
}
#[derive(Deserialize)]
struct NiceRequest {
    mode: String,
    priority: i32,
    command: String,
    target_type: String,
    target: String,
}
#[derive(Deserialize)]
struct LsRequest {
    path: String,
    all: bool,
    long: bool,
    human: bool,
    time: bool,
    reverse: bool,
    recursive: bool,
    inode: bool,
    directory: bool,
    color: bool,
}
#[derive(Deserialize)]
struct FirewallRequest {
    op: String,
    zone: String,
    target_type: String,
    target: String,
    permanent: bool,
}
#[derive(Deserialize)]
struct SystemctlRequest {
    operation: String,
    service: String,
    user_mode: bool,
    now: bool,
    force: bool,
    global: bool,
}
#[derive(Deserialize)]
struct FindRequest {
    path: String,
    name: String,
    iname: bool,
    target_type: String,
    size: String,
    mtime: String,
    empty: bool,
    exec: String,
}
#[derive(Deserialize)]
struct DockerfileRequest {
    image: String,
    workdir: String,
    copy: String,
    run: String,
    env: String,
    expose: String,
    cmd: String,
}
#[derive(Deserialize)]
struct NginxRequest {
    domain: String,
    port: u16,
    root: String,
    path: String,
    proxy: String,
    upstream: String,
    https: bool,
    force_https: bool,
    ssl_cert: String,
    ssl_key: String,
    spa: bool,
    gzip: bool,
    client_max_body_size: String,
    keepalive_timeout: String,
    proxy_connect_timeout: String,
    proxy_read_timeout: String,
    proxy_send_timeout: String,
}

#[derive(Serialize)]
struct GenericResponse {
    result: String,
}
#[derive(Serialize)]
struct UuidResponse {
    uuids: Vec<String>,
}
#[derive(Serialize)]
struct TokenResponse {
    token: String,
}
#[derive(Serialize)]
struct PasswordResponse {
    password: String,
}
#[derive(Serialize)]
struct UrlResponse {
    encoded: String,
    decoded: String,
    protocol: String,
    host: String,
    path: String,
    params: Vec<(String, String)>,
}
#[derive(Serialize)]
struct JsonResponse {
    pretty: String,
    minified: String,
    error: Option<String>,
}
#[derive(Serialize)]
struct YamlResponse {
    result: String,
    error: Option<String>,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| Response::from_html(html::get_homepage()))
        .get("/api/ping", |_, _| Response::ok("Pong"))
        .post_async("/api/sql", |mut req, _| async move {
            let data: SqlRequest = req.json().await?;
            let res = utils::format_sql(&data.sql);
            Response::from_json(&GenericResponse { result: res })
        })
        .post_async("/api/diff", |mut req, _| async move {
            let data: DiffRequest = req.json().await?;
            Response::from_json(&utils::compute_diff(&data.old, &data.new))
        })
        .post_async("/api/cron", |mut req, _| async move {
            let data: CronRequest = req.json().await?;
            Response::from_json(&utils::check_cron(&data.cron))
        })
        .post_async("/api/subnet", |mut req, _| async move {
            let data: SubnetRequest = req.json().await?;
            Response::from_json(&utils::calculate_subnet(&data.ip, data.cidr))
        })
        .post_async("/api/md5", |mut req, _| async move {
            let data: Md5Request = req.json().await?;
            Response::from_json(&utils::calculate_md5(&data.text))
        })
        .post_async("/api/token", |mut req, _| async move {
            let data: TokenRequest = req.json().await?;
            let token = utils::generate_token(
                data.length,
                data.uppercase,
                data.lowercase,
                data.numbers,
                data.symbols,
            );
            Response::from_json(&TokenResponse { token })
        })
        .post_async("/api/uuid", |mut req, _| async move {
            let data: UuidRequest = req.json().await?;
            let uuids = utils::generate_uuids(utils::UuidConfig {
                count: data.count,
                hyphens: data.hyphens,
                uppercase: data.uppercase,
            });
            Response::from_json(&UuidResponse { uuids })
        })
        .post_async("/api/date", |mut req, _| async move {
            let data: DateRequest = req.json().await?;
            Response::from_json(&utils::parse_date(&data.input))
        })
        .post_async("/api/color", |mut req, _| async move {
            let data: ColorRequest = req.json().await?;
            Response::from_json(&utils::convert_color(&data.input))
        })
        .post_async("/api/base64", |mut req, _| async move {
            let data: Base64Request = req.json().await?;
            let res = utils::process_base64(&data.text, &data.action);
            Response::from_json(&GenericResponse { result: res })
        })
        .post_async("/api/js-enc", |mut req, _| async move {
            let data: JsEncRequest = req.json().await?;
            let res = utils::obfuscate_js(&data.js);
            Response::from_json(&GenericResponse { result: res })
        })
        .post_async("/api/json", |mut req, _| async move {
            let data: JsonRequest = req.json().await?;
            let (pretty, min, err) = utils::process_json(&data.input);
            Response::from_json(&JsonResponse {
                pretty,
                minified: min,
                error: err,
            })
        })
        .post_async("/api/url", |mut req, _| async move {
            let data: UrlRequest = req.json().await?;
            let (enc, dec, protocol, host, path, params) = utils::process_url(&data.input);
            Response::from_json(&UrlResponse {
                encoded: enc,
                decoded: dec,
                protocol,
                host,
                path,
                params,
            })
        })
        .post_async("/api/password", |mut req, _| async move {
            let data: PasswordRequest = req.json().await?;
            let res = utils::generate_password_strong(
                data.length,
                data.uppercase,
                data.lowercase,
                data.numbers,
                data.symbols,
            );
            Response::from_json(&PasswordResponse { password: res })
        })
        .post_async("/api/qrcode", |mut req, _| async move {
            let data: QrRequest = req.json().await?;
            let svg = utils::generate_qr(&data.text);
            // Return raw struct matching frontend expectations { svg: ... }
            #[derive(Serialize)]
            struct QrRes {
                svg: String,
            }
            Response::from_json(&QrRes { svg })
        })
        .post_async("/api/chmod", |mut req, _| async move {
            let data: ChmodRequest = req.json().await?;
            Response::from_json(&utils::calculate_chmod(&data.octal))
        })
        .post_async("/api/yaml-to-toml", |mut req, _| async move {
            let data: YamlRequest = req.json().await?;
            let (res, err) = utils::yaml_to_toml(&data.yaml);
            let err_opt = if err.is_empty() { None } else { Some(err) };
            Response::from_json(&YamlResponse {
                result: res,
                error: err_opt,
            })
        })
        .post_async("/api/toml-to-yaml", |mut req, _| async move {
            let data: TomlRequest = req.json().await?;
            let (res, err) = utils::toml_to_yaml(&data.toml);
            let err_opt = if err.is_empty() { None } else { Some(err) };
            Response::from_json(&YamlResponse {
                result: res,
                error: err_opt,
            })
        })
        .post_async("/api/jwt", |mut req, _| async move {
            let data: JwtRequest = req.json().await?;
            Response::from_json(&utils::parse_jwt(&data.token))
        })
        .post_async("/api/regex", |mut req, _| async move {
            let data: RegexRequest = req.json().await?;
            Response::from_json(&utils::test_regex(&data.pattern, &data.text))
        })
        .post_async("/api/regex-gen", |mut req, _| async move {
            let data: RegexGenRequest = req.json().await?;
            let pat = utils::get_common_regex(&data.key);
            #[derive(Serialize)]
            struct PatRes {
                pattern: String,
            }
            Response::from_json(&PatRes {
                pattern: pat.to_string(),
            })
        })
        .post_async("/api/escape", |mut req, _| async move {
            let data: EscapeRequest = req.json().await?;
            let res = utils::process_escape(&data.text, &data.mode);
            Response::from_json(&GenericResponse { result: res })
        })
        .post_async("/api/case", |mut req, _| async move {
            let data: CaseRequest = req.json().await?;
            let res = utils::convert_case(&data.text, &data.mode);
            Response::from_json(&GenericResponse { result: res })
        })
        .post_async("/api/tar", |mut req, _| async move {
            let data: TarRequest = req.json().await?;
            Response::from_json(&utils::generate_tar(
                &data.op,
                &data.comp,
                data.verbose,
                &data.archive,
                &data.files,
            ))
        })
        .post_async("/api/ps", |mut req, _| async move {
            let data: PsRequest = req.json().await?;
            Response::from_json(&utils::generate_ps(
                &data.format,
                &data.sort,
                data.tree,
                &data.filter,
            ))
        })
        .post_async("/api/tcpdump", |mut req, _| async move {
            let data: TcpdumpRequest = req.json().await?;
            Response::from_json(&utils::generate_tcpdump(
                &data.interface,
                &data.protocol,
                &data.host,
                &data.port,
                data.verbose,
                data.ascii,
                data.hex,
                &data.write_file,
                &data.count,
            ))
        })
        .post_async("/api/git", |mut req, _| async move {
            let data: GitRequest = req.json().await?;
            Response::from_json(&utils::generate_git(
                &data.cmd,
                &data.target,
                &data.msg,
                &data.remote,
                &data.branch,
                data.opt_force,
                data.opt_rebase,
                data.opt_all,
                data.opt_amend,
                data.opt_hard,
                data.opt_new_branch,
                data.opt_tags,
                data.opt_oneline,
                data.opt_graph,
            ))
        })
        .post_async("/api/strace", |mut req, _| async move {
            let data: StraceRequest = req.json().await?;
            Response::from_json(&utils::generate_strace(
                &data.target,
                data.is_pid,
                data.follow,
                data.summary,
                &data.output_file,
                &data.filter,
                &data.string_limit,
                data.timestamp,
            ))
        })
        .post_async("/api/iostat", |mut req, _| async move {
            let data: IostatRequest = req.json().await?;
            Response::from_json(&utils::generate_iostat(
                &data.interval,
                &data.count,
                data.human,
                data.extended,
                &data.unit,
                data.partitions,
                data.timestamp,
                &data.device,
            ))
        })
        .post_async("/api/nice", |mut req, _| async move {
            let data: NiceRequest = req.json().await?;
            Response::from_json(&utils::generate_nice(
                &data.mode,
                data.priority,
                &data.command,
                &data.target_type,
                &data.target,
            ))
        })
        .post_async("/api/ls", |mut req, _| async move {
            let data: LsRequest = req.json().await?;
            Response::from_json(&utils::generate_ls(
                &data.path,
                data.all,
                data.long,
                data.human,
                data.time,
                data.reverse,
                data.recursive,
                data.inode,
                data.directory,
                data.color,
            ))
        })
        .post_async("/api/firewall", |mut req, _| async move {
            let data: FirewallRequest = req.json().await?;
            Response::from_json(&utils::generate_firewall(
                &data.op,
                &data.zone,
                &data.target_type,
                &data.target,
                data.permanent,
            ))
        })
        .post_async("/api/systemctl", |mut req, _| async move {
            let data: SystemctlRequest = req.json().await?;
            Response::from_json(&utils::generate_systemctl(
                &data.operation,
                &data.service,
                data.user_mode,
                data.now,
                data.force,
                data.global,
            ))
        })
        .post_async("/api/find", |mut req, _| async move {
            let data: FindRequest = req.json().await?;
            Response::from_json(&utils::generate_find(
                &data.path,
                &data.name,
                data.iname,
                &data.target_type,
                &data.size,
                &data.mtime,
                data.empty,
                &data.exec,
            ))
        })
        .post_async("/api/dockerfile", |mut req, _| async move {
            let data: DockerfileRequest = req.json().await?;
            Response::from_json(&GenericResponse {
                result: utils::generate_dockerfile(
                    &data.image,
                    &data.workdir,
                    &data.copy,
                    &data.run,
                    &data.env,
                    &data.expose,
                    &data.cmd,
                ),
            })
        })
        .post_async("/api/nginx", |mut req, _| async move {
            let data: NginxRequest = req.json().await?;
            Response::from_json(&GenericResponse {
                result: utils::generate_nginx_config(
                    &data.domain,
                    data.port,
                    &data.root,
                    &data.path,
                    &data.proxy,
                    &data.upstream,
                    data.https,
                    data.force_https,
                    &data.ssl_cert,
                    &data.ssl_key,
                    data.spa,
                    data.gzip,
                    &data.client_max_body_size,
                    &data.keepalive_timeout,
                    &data.proxy_connect_timeout,
                    &data.proxy_read_timeout,
                    &data.proxy_send_timeout,
                ),
            })
        })
        .run(req, env)
        .await
}
