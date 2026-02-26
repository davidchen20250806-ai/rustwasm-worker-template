use worker::*;

mod converters;
mod generators;
mod html;
mod models;
mod system;
mod utils;

use models::*;

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
            let uuids = utils::generate_uuids(UuidConfig {
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
            let res = converters::obfuscate_js(&data.js);
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
            match utils::generate_qr(&data.text) {
                Ok(svg) => Response::from_json(&QrResponse { svg, error: None }),
                Err(e) => Response::from_json(&QrResponse {
                    svg: String::new(),
                    error: Some(e),
                }),
            }
        })
        .post_async("/api/chmod", |mut req, _| async move {
            let data: ChmodRequest = req.json().await?;
            Response::from_json(&system::calculate_chmod(&data.octal, &data.file))
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
            Response::from_json(&RegexPatternResponse {
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
            let res = converters::convert_case(&data.text, &data.mode);
            Response::from_json(&GenericResponse { result: res })
        })
        .post_async("/api/tar", |mut req, _| async move {
            let data: TarRequest = req.json().await?;
            Response::from_json(&system::generate_tar(
                &data.op,
                &data.comp,
                data.verbose,
                &data.archive,
                &data.files,
            ))
        })
        .post_async("/api/ps", |mut req, _| async move {
            let data: PsRequest = req.json().await?;
            Response::from_json(&system::generate_ps(
                &data.format,
                &data.sort,
                data.tree,
                &data.filter,
                data.wide,
                data.threads,
                &data.user,
                &data.pid,
            ))
        })
        .post_async("/api/tcpdump", |mut req, _| async move {
            let data: TcpdumpRequest = req.json().await?;
            Response::from_json(&system::generate_tcpdump(
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
            Response::from_json(&system::generate_git(
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
        .post_async("/api/git-cmd", |mut req, _| async move {
            let data: GitCmdRequest = req.json().await?;
            Response::from_json(&system::generate_git_cmd(
                &data.action,
                &data.tag,
                &data.msg,
                &data.branch,
            ))
        })
        .post_async("/api/strace", |mut req, _| async move {
            let data: StraceRequest = req.json().await?;
            Response::from_json(&system::generate_strace(
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
            Response::from_json(&system::generate_iostat(
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
            Response::from_json(&system::generate_nice(
                &data.mode,
                data.priority,
                &data.command,
                &data.target_type,
                &data.target,
            ))
        })
        .post_async("/api/ls", |mut req, _| async move {
            let data: LsRequest = req.json().await?;
            Response::from_json(&system::generate_ls(
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
            Response::from_json(&system::generate_firewall(
                &data.op,
                &data.zone,
                &data.target_type,
                &data.target,
                data.permanent,
            ))
        })
        .post_async("/api/systemctl", |mut req, _| async move {
            let data: SystemctlRequest = req.json().await?;
            Response::from_json(&system::generate_systemctl(
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
            Response::from_json(&system::generate_find(
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
                result: system::generate_dockerfile(&data.stages),
            })
        })
        .post_async("/api/nginx", |mut req, _| async move {
            let data: NginxRequest = req.json().await?;
            Response::from_json(&GenericResponse {
                result: utils::generate_nginx_config(
                    &data.domain,
                    data.port,
                    &data.root,
                    &data.locations,
                    &data.upstream,
                    data.https,
                    data.force_https,
                    &data.ssl_cert,
                    &data.ssl_key,
                    data.gzip,
                    &data.client_max_body_size,
                    &data.keepalive_timeout,
                    &data.proxy_connect_timeout,
                    &data.proxy_read_timeout,
                    &data.proxy_send_timeout,
                ),
            })
        })
        .post_async("/api/lorem", |mut req, _| async move {
            let data: LoremRequest = req.json().await?;
            Response::from_json(&GenericResponse {
                result: generators::generate_lorem(data.count, &data.mode),
            })
        })
        .post_async("/api/rsync", |mut req, _| async move {
            let data: RsyncRequest = req.json().await?;
            Response::from_json(&system::generate_rsync(
                &data.source,
                &data.user,
                &data.host,
                &data.port,
                &data.remote_path,
                data.archive,
                data.compress,
                data.verbose,
                data.delete,
                data.dry_run,
                data.progress,
                data.ssh,
                &data.exclude,
            ))
        })
        .post_async("/api/fake-user", |mut req, _| async move {
            let data: FakeUserRequest = req.json().await?;
            Response::from_json(&FakeUserResponse {
                users: generators::generate_fake_users(data.count, &data.locale),
            })
        })
        .post_async("/api/whoami", |req, _| async move {
            let headers = req.headers();
            let ip = headers
                .get("cf-connecting-ip")
                .ok()
                .flatten()
                .unwrap_or_else(|| "Unknown".into());
            let country = headers
                .get("cf-ipcountry")
                .ok()
                .flatten()
                .unwrap_or_else(|| "-".into());
            let city = headers
                .get("cf-ipcity")
                .ok()
                .flatten()
                .unwrap_or_else(|| "-".into());
            let asn = headers
                .get("cf-ray")
                .ok()
                .flatten()
                .unwrap_or_else(|| "-".into());
            let ua = headers
                .get("user-agent")
                .ok()
                .flatten()
                .unwrap_or_else(|| "-".into());

            let mut header_map = std::collections::HashMap::new();
            for (k, v) in headers.entries() {
                header_map.insert(k, v);
            }

            Response::from_json(&WhoamiResponse {
                ip,
                country,
                city,
                asn,
                user_agent: ua,
                headers: header_map,
            })
        })
        .post_async("/api/unit-convert", |mut req, _| async move {
            let data: UnitRequest = req.json().await?;
            let val = data.value.parse::<f64>().unwrap_or(0.0);
            Response::from_json(&converters::convert_unit(
                val,
                &data.type_,
                &data.from,
                &data.to,
            ))
        })
        .post_async("/api/curl", |mut req, _| async move {
            let data: CurlRequest = req.json().await?;
            Response::from_json(&system::generate_curl(
                &data.method,
                &data.url,
                &data.headers,
                &data.body,
            ))
        })
        .post_async("/api/credit-card", |mut req, _| async move {
            let data: CreditCardRequest = req.json().await?;
            Response::from_json(&CreditCardResponse {
                cards: generators::generate_credit_cards(data.count, &data.issuer),
            })
        })
        .post_async("/api/awk", |mut req, _| async move {
            let data: AwkRequest = req.json().await?;
            Response::from_json(&system::generate_awk(
                &data.separator,
                &data.variable,
                &data.code,
                &data.file,
            ))
        })
        .post_async("/api/sed", |mut req, _| async move {
            let data: SedRequest = req.json().await?;
            Response::from_json(&system::generate_sed(
                &data.operation,
                &data.pattern,
                &data.replacement,
                &data.flags,
                data.inplace,
                &data.file,
            ))
        })
        .post_async("/api/regex-build", |mut req, _| async move {
            let data: RegexBuildRequest = req.json().await?;
            Response::from_json(&generators::generate_custom_regex(
                &data.starts_with,
                &data.not_starts_with,
                &data.ends_with,
                &data.not_ends_with,
                &data.contains,
                &data.not_contains,
            ))
        })
        .run(req, env)
        .await
}
