use serde::{Deserialize, Serialize};
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
        .post_async("/api/k8s-yaml", |mut req, _| async move {
            let data: K8sRequest = req.json().await?;
            let yaml = generate_k8s_yaml(&data);
            Response::from_json(&GenericResponse { result: yaml })
        })
        .post_async("/api/k8s-cmd", |mut req, _| async move {
            let data: K8sCmdRequest = req.json().await?;
            let (cmd, desc) = generate_k8s_cmd(&data);
            Response::from_json(&K8sCmdResponse {
                command: cmd,
                description: desc,
            })
        })
        .post_async("/api/ansible", |mut req, _| async move {
            let data: AnsibleRequest = req.json().await?;
            let yaml = generate_ansible_yaml(&data);
            Response::from_json(&GenericResponse { result: yaml })
        })
        .run(req, env)
        .await
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct K8sRequest {
    #[serde(default = "default_kind")]
    kind: String,
    #[serde(default = "default_name")]
    name: String,
    #[serde(default = "default_namespace")]
    namespace: String,
    #[serde(default = "default_image")]
    image: String,
    #[serde(default = "default_replicas")]
    replicas: i32,
    #[serde(default = "default_port")]
    port: i32,
    #[serde(default = "default_target_port")]
    target_port: i32,
    #[serde(default = "default_service_type")]
    service_type: String,
    #[serde(default = "default_ingress_host")]
    ingress_host: String,
    #[serde(default = "default_ingress_path")]
    ingress_path: String,
    #[serde(default = "default_pull_policy")]
    pull_policy: String,
    cpu_limit: Option<String>,
    memory_limit: Option<String>,
    cpu_request: Option<String>,
    memory_request: Option<String>,
    #[serde(default)]
    env: Vec<K8sEnvVar>,
    #[serde(default = "default_schedule")]
    schedule: String,
    #[serde(default = "default_restart_policy")]
    restart_policy: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct K8sCmdRequest {
    action: String,
    #[serde(default = "default_namespace")]
    namespace: String,
    #[serde(default = "default_resource_type")]
    resource_type: String,
    #[serde(default)]
    resource_name: String,
    #[serde(default)]
    replicas: i32,
    #[serde(default)]
    local_port: i32,
    #[serde(default)]
    remote_port: i32,
    #[serde(default)]
    output_format: String,
}

#[derive(Serialize)]
struct K8sCmdResponse {
    command: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnsibleRequest {
    #[serde(default = "default_play_name")]
    play_name: String,
    #[serde(default = "default_hosts")]
    hosts: String,
    #[serde(default)]
    r#become: bool,
    #[serde(default = "default_true")]
    gather_facts: bool,
    #[serde(default)]
    vars: String,
    #[serde(default)]
    tasks: String,
    #[serde(default)]
    handlers: String,
}

#[derive(Deserialize, Serialize)]
struct K8sEnvVar {
    key: String,
    value: String,
}

fn default_kind() -> String {
    "Deployment".to_string()
}
fn default_name() -> String {
    "app-name".to_string()
}
fn default_namespace() -> String {
    "default".to_string()
}
fn default_image() -> String {
    "nginx:latest".to_string()
}
fn default_replicas() -> i32 {
    1
}
fn default_port() -> i32 {
    80
}
fn default_target_port() -> i32 {
    80
}
fn default_service_type() -> String {
    "ClusterIP".to_string()
}
fn default_ingress_host() -> String {
    "example.com".to_string()
}
fn default_ingress_path() -> String {
    "/".to_string()
}
fn default_pull_policy() -> String {
    "IfNotPresent".to_string()
}
fn default_schedule() -> String {
    "*/1 * * * *".to_string()
}
fn default_restart_policy() -> String {
    "Always".to_string()
}
fn default_play_name() -> String {
    "Ansible Playbook".to_string()
}
fn default_hosts() -> String {
    "all".to_string()
}
fn default_resource_type() -> String {
    "pod".to_string()
}
fn default_true() -> bool {
    true
}

fn generate_k8s_yaml(data: &K8sRequest) -> String {
    let mut yaml = String::new();

    if data.kind == "Deployment" {
        yaml.push_str(&format!(
            r#"apiVersion: apps/v1
kind: Deployment
metadata:
  name: {}
  namespace: {}
  labels:
    app: {}
spec:
  replicas: {}
  selector:
    matchLabels:
      app: {}
  template:
    metadata:
      labels:
        app: {}
    spec:
      containers:
      - name: {}
        image: {}
        imagePullPolicy: {}
        ports:
        - containerPort: {}
"#,
            data.name,
            data.namespace,
            data.name,
            data.replicas,
            data.name,
            data.name,
            data.name,
            data.image,
            data.pull_policy,
            data.port
        ));

        if data.cpu_limit.is_some()
            || data.memory_limit.is_some()
            || data.cpu_request.is_some()
            || data.memory_request.is_some()
        {
            yaml.push_str("        resources:\n");
            if data.cpu_limit.is_some() || data.memory_limit.is_some() {
                yaml.push_str("          limits:\n");
                if let Some(ref cpu) = data.cpu_limit {
                    yaml.push_str(&format!("            cpu: {}\n", cpu));
                }
                if let Some(ref mem) = data.memory_limit {
                    yaml.push_str(&format!("            memory: {}\n", mem));
                }
            }
            if data.cpu_request.is_some() || data.memory_request.is_some() {
                yaml.push_str("          requests:\n");
                if let Some(ref cpu) = data.cpu_request {
                    yaml.push_str(&format!("            cpu: {}\n", cpu));
                }
                if let Some(ref mem) = data.memory_request {
                    yaml.push_str(&format!("            memory: {}\n", mem));
                }
            }
        }

        if !data.env.is_empty() {
            yaml.push_str("        env:\n");
            for e in &data.env {
                if !e.key.is_empty() && !e.value.is_empty() {
                    yaml.push_str(&format!(
                        "        - name: {}\n          value: \"{}\"\n",
                        e.key, e.value
                    ));
                }
            }
        }

        yaml.push_str(&format!("      restartPolicy: {}", data.restart_policy));
    } else if data.kind == "Service" {
        yaml.push_str(&format!(
            r#"apiVersion: v1
kind: Service
metadata:
  name: {}
  namespace: {}
  labels:
    app: {}
spec:
  type: {}
  selector:
    app: {}
  ports:
  - protocol: TCP
    port: {}
    targetPort: {}
"#,
            data.name,
            data.namespace,
            data.name,
            data.service_type,
            data.name,
            data.port,
            data.target_port
        ));
    } else if data.kind == "Ingress" {
        yaml.push_str(&format!(
            r#"apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: {}
  namespace: {}
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  rules:
  - host: {}
    http:
      paths:
      - path: {}
        pathType: Prefix
        backend:
          service:
            name: {}
            port:
              number: {}
"#,
            data.name, data.namespace, data.ingress_host, data.ingress_path, data.name, data.port
        ));
    } else if data.kind == "CronJob" {
        yaml.push_str(&format!(
            r#"apiVersion: batch/v1
kind: CronJob
metadata:
  name: {}
  namespace: {}
spec:
  schedule: "{}"
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: {}
            image: {}
            imagePullPolicy: {}
            command:
            - /bin/sh
            - -c
            - "echo Hello Kubernetes"
          restartPolicy: OnFailure
"#,
            data.name, data.namespace, data.schedule, data.name, data.image, data.pull_policy
        ));
    } else if data.kind == "ConfigMap" {
        yaml.push_str(&format!(
            r#"apiVersion: v1
kind: ConfigMap
metadata:
  name: {}
  namespace: {}
data:
"#,
            data.name, data.namespace
        ));
        if !data.env.is_empty() {
            for e in &data.env {
                if !e.key.is_empty() && !e.value.is_empty() {
                    yaml.push_str(&format!("  {}: \"{}\"\n", e.key, e.value));
                }
            }
        } else {
            yaml.push_str("  config.json: |\n    {\n      \"key\": \"value\"\n    }");
        }
    } else if data.kind == "Secret" {
        yaml.push_str(&format!(
            r#"apiVersion: v1
kind: Secret
metadata:
  name: {}
  namespace: {}
type: Opaque
data:
  # Data should be base64 encoded
"#,
            data.name, data.namespace
        ));
        if !data.env.is_empty() {
            use base64::{engine::general_purpose, Engine as _};
            for e in &data.env {
                if !e.key.is_empty() && !e.value.is_empty() {
                    let b64 = general_purpose::STANDARD.encode(&e.value);
                    yaml.push_str(&format!("  {}: {}\n", e.key, b64));
                }
            }
        } else {
            yaml.push_str("  username: YWRtaW4=");
        }
    }

    yaml
}

fn generate_k8s_cmd(data: &K8sCmdRequest) -> (String, String) {
    let ns = if data.namespace.is_empty() {
        "default"
    } else {
        &data.namespace
    };
    let name = if data.resource_name.is_empty() {
        "app"
    } else {
        &data.resource_name
    };
    let rtype = if data.resource_type.is_empty() {
        "pod"
    } else {
        &data.resource_type
    };

    let output = if data.output_format.is_empty() {
        String::new()
    } else {
        format!(" -o {}", data.output_format)
    };

    match data.action.as_str() {
        "get" => (
            format!("kubectl get {} -n {}{}", rtype, ns, output),
            format!("获取 {} 列表", rtype),
        ),
        "describe" => (
            format!("kubectl describe {} {} -n {}", rtype, name, ns),
            format!("查看 {} {} 的详细信息", rtype, name),
        ),
        "delete" => (
            format!("kubectl delete {} {} -n {}", rtype, name, ns),
            format!("删除 {} {}", rtype, name),
        ),
        "logs" => {
            let target = if rtype == "pod" {
                name.to_string()
            } else {
                format!("{}/{}", rtype, name)
            };
            (
                format!("kubectl logs -f {} -n {}", target, ns),
                format!("查看 {} {} 的日志", rtype, name),
            )
        }
        "exec" => (
            format!("kubectl exec -it {} -n {} -- /bin/sh", name, ns),
            format!("进入 Pod {} 的 Shell (仅适用于 Pod)", name),
        ),
        "scale" => (
            format!(
                "kubectl scale {} {} --replicas={} -n {}",
                rtype, name, data.replicas, ns
            ),
            format!("将 {} {} 伸缩到 {} 个副本", rtype, name, data.replicas),
        ),
        "port_forward" => (
            format!(
                "kubectl port-forward {} {}:{} -n {}",
                if rtype == "pod" {
                    name.to_string()
                } else {
                    format!("{}/{}", rtype, name)
                },
                data.local_port,
                data.remote_port,
                ns
            ),
            format!(
                "端口转发 {} {} -> {}",
                rtype, data.local_port, data.remote_port
            ),
        ),
        "rollout_restart" => (
            format!("kubectl rollout restart {} {} -n {}", rtype, name, ns),
            format!("重启 {} {} (滚动更新)", rtype, name),
        ),
        "rollout_status" => (
            format!("kubectl rollout status {} {} -n {}", rtype, name, ns),
            format!("查看 {} {} 的滚动更新状态", rtype, name),
        ),
        "rollout_history" => (
            format!("kubectl rollout history {} {} -n {}", rtype, name, ns),
            format!("查看 {} {} 的历史版本", rtype, name),
        ),
        "rollout_undo" => (
            format!("kubectl rollout undo {} {} -n {}", rtype, name, ns),
            format!("回滚 {} {} 到上一个版本", rtype, name),
        ),
        _ => ("kubectl --help".to_string(), "显示帮助信息".to_string()),
    }
}

fn generate_ansible_yaml(data: &AnsibleRequest) -> String {
    let mut yaml = String::new();
    yaml.push_str(&format!("- name: {}\n", data.play_name));
    yaml.push_str(&format!("  hosts: {}\n", data.hosts));
    if data.r#become {
        yaml.push_str("  become: yes\n");
    }
    if !data.gather_facts {
        yaml.push_str("  gather_facts: no\n");
    }
    if !data.vars.trim().is_empty() {
        yaml.push_str("  vars:\n");
        for line in data.vars.lines() {
            yaml.push_str(&format!("    {}\n", line));
        }
    }
    yaml.push_str("  tasks:\n");

    if data.tasks.trim().is_empty() {
        yaml.push_str("    - name: Ping hosts\n      ping:\n");
    } else {
        for line in data.tasks.lines() {
            yaml.push_str(&format!("    {}\n", line));
        }
    }

    if !data.handlers.trim().is_empty() {
        yaml.push_str("  handlers:\n");
        for line in data.handlers.lines() {
            yaml.push_str(&format!("    {}\n", line));
        }
    }
    yaml
}
