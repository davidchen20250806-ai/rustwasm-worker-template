use serde::{Deserialize, Serialize};

// --- Request Structs (from lib.rs) ---

#[derive(Deserialize)]
pub struct SqlRequest {
    pub sql: String,
}
#[derive(Deserialize)]
pub struct DiffRequest {
    pub old: String,
    pub new: String,
}
#[derive(Deserialize)]
pub struct CronRequest {
    pub cron: String,
}
#[derive(Deserialize)]
pub struct SubnetRequest {
    pub ip: String,
    pub cidr: u8,
}
#[derive(Deserialize)]
pub struct RegexGenRequest {
    pub key: String,
}
#[derive(Deserialize)]
pub struct RegexRequest {
    pub pattern: String,
    pub text: String,
}
#[derive(Deserialize)]
pub struct UuidRequest {
    pub count: usize,
    pub hyphens: bool,
    pub uppercase: bool,
}
#[derive(Deserialize)]
pub struct JwtRequest {
    pub token: String,
}
#[derive(Deserialize)]
pub struct PasswordRequest {
    pub length: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}
#[derive(Deserialize)]
pub struct TokenRequest {
    pub length: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}
#[derive(Deserialize)]
pub struct Base64Request {
    pub text: String,
    pub action: String,
}
#[derive(Deserialize)]
pub struct JsonRequest {
    pub input: String,
}
#[derive(Deserialize)]
pub struct EscapeRequest {
    pub text: String,
    pub mode: String,
}
#[derive(Deserialize)]
pub struct DateRequest {
    pub input: String,
}
#[derive(Deserialize)]
pub struct ColorRequest {
    pub input: String,
}
#[derive(Deserialize)]
pub struct QrRequest {
    pub text: String,
}
#[derive(Deserialize)]
pub struct JsEncRequest {
    pub js: String,
}
#[derive(Deserialize)]
pub struct YamlRequest {
    pub yaml: String,
}
#[derive(Deserialize)]
pub struct TomlRequest {
    pub toml: String,
}
#[derive(Deserialize)]
pub struct Md5Request {
    pub text: String,
}
#[derive(Deserialize)]
pub struct ChmodRequest {
    pub octal: String,
    pub file: String,
}
#[derive(Deserialize)]
pub struct UrlRequest {
    pub input: String,
}
#[derive(Deserialize)]
pub struct CaseRequest {
    pub text: String,
    pub mode: String,
}
#[derive(Deserialize)]
pub struct TarRequest {
    pub op: String,
    pub comp: String,
    pub verbose: bool,
    pub archive: String,
    pub files: String,
}
#[derive(Deserialize)]
pub struct PsRequest {
    pub format: String,
    pub sort: String,
    pub tree: bool,
    pub filter: String,
    pub wide: bool,
    pub threads: bool,
    pub user: String,
    pub pid: String,
}
#[derive(Deserialize)]
pub struct TcpdumpRequest {
    pub interface: String,
    pub protocol: String,
    pub host: String,
    pub port: String,
    pub verbose: bool,
    pub ascii: bool,
    pub hex: bool,
    pub write_file: String,
    pub count: String,
}
#[derive(Deserialize)]
pub struct GitRequest {
    pub cmd: String,
    pub target: String,
    pub msg: String,
    pub remote: String,
    pub branch: String,
    pub opt_force: bool,
    pub opt_rebase: bool,
    pub opt_all: bool,
    pub opt_amend: bool,
    pub opt_hard: bool,
    pub opt_new_branch: bool,
    pub opt_tags: bool,
    pub opt_oneline: bool,
    pub opt_graph: bool,
}
#[derive(Deserialize)]
pub struct GitCmdRequest {
    pub action: String,
    pub tag: String,
    pub msg: String,
    pub branch: String,
}
#[derive(Deserialize)]
pub struct StraceRequest {
    pub target: String,
    pub is_pid: bool,
    pub follow: bool,
    pub summary: bool,
    pub output_file: String,
    pub filter: String,
    pub string_limit: String,
    pub timestamp: bool,
}
#[derive(Deserialize)]
pub struct IostatRequest {
    pub interval: String,
    pub count: String,
    pub human: bool,
    pub extended: bool,
    pub unit: String,
    pub partitions: bool,
    pub timestamp: bool,
    pub device: String,
}
#[derive(Deserialize)]
pub struct NiceRequest {
    pub mode: String,
    pub priority: i32,
    pub command: String,
    pub target_type: String,
    pub target: String,
}
#[derive(Deserialize)]
pub struct LsRequest {
    pub path: String,
    pub all: bool,
    pub long: bool,
    pub human: bool,
    pub time: bool,
    pub reverse: bool,
    pub recursive: bool,
    pub inode: bool,
    pub directory: bool,
    pub color: bool,
}
#[derive(Deserialize)]
pub struct FirewallRequest {
    pub op: String,
    pub zone: String,
    pub target_type: String,
    pub target: String,
    pub permanent: bool,
}
#[derive(Deserialize)]
pub struct SystemctlRequest {
    pub operation: String,
    pub service: String,
    pub user_mode: bool,
    pub now: bool,
    pub force: bool,
    pub global: bool,
}
#[derive(Deserialize)]
pub struct FindRequest {
    pub path: String,
    pub name: String,
    pub iname: bool,
    pub target_type: String,
    pub size: String,
    pub mtime: String,
    pub empty: bool,
    pub exec: String,
}
#[derive(Deserialize)]
pub struct DockerfileRequest {
    pub stages: Vec<DockerfileStage>,
}
#[derive(Deserialize)]
pub struct NginxRequest {
    pub domain: String,
    pub port: u16,
    pub root: String,
    pub locations: Vec<NginxLocation>,
    pub upstream: String,
    pub https: bool,
    pub force_https: bool,
    pub ssl_cert: String,
    pub ssl_key: String,
    pub gzip: bool,
    pub client_max_body_size: String,
    pub keepalive_timeout: String,
    pub proxy_connect_timeout: String,
    pub proxy_read_timeout: String,
    pub proxy_send_timeout: String,
}
#[derive(Deserialize)]
pub struct LoremRequest {
    pub count: usize,
    pub mode: String,
}
#[derive(Deserialize)]
pub struct RsyncRequest {
    pub source: String,
    pub user: String,
    pub host: String,
    pub port: String,
    pub remote_path: String,
    pub archive: bool,
    pub compress: bool,
    pub verbose: bool,
    pub delete: bool,
    pub dry_run: bool,
    pub progress: bool,
    pub ssh: bool,
    pub exclude: String,
}
#[derive(Deserialize)]
pub struct FakeUserRequest {
    pub count: usize,
    pub locale: String,
}
#[derive(Deserialize)]
pub struct UnitRequest {
    pub value: String, // Frontend sends as string
    #[serde(rename = "type")]
    pub type_: String,
    pub from: String,
    pub to: String,
}
#[derive(Deserialize)]
pub struct CurlRequest {
    pub method: String,
    pub url: String,
    pub headers: String,
    pub body: String,
}
#[derive(Deserialize)]
pub struct CreditCardRequest {
    pub count: usize,
    pub issuer: String,
}
#[derive(Deserialize)]
pub struct AwkRequest {
    pub separator: String,
    pub variable: String,
    pub code: String,
    pub file: String,
}
#[derive(Deserialize)]
pub struct SedRequest {
    pub operation: String,
    pub pattern: String,
    pub replacement: String,
    pub flags: String,
    pub inplace: bool,
    pub file: String,
}
#[derive(Deserialize)]
pub struct RegexBuildRequest {
    pub starts_with: String,
    pub not_starts_with: String,
    pub ends_with: String,
    pub not_ends_with: String,
    pub contains: String,
    pub not_contains: String,
}

// --- Response Structs (from lib.rs) ---

#[derive(Serialize)]
pub struct GenericResponse {
    pub result: String,
}
#[derive(Serialize)]
pub struct UuidResponse {
    pub uuids: Vec<String>,
}
#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}
#[derive(Serialize)]
pub struct PasswordResponse {
    pub password: String,
}
#[derive(Serialize)]
pub struct UrlResponse {
    pub encoded: String,
    pub decoded: String,
    pub protocol: String,
    pub host: String,
    pub path: String,
    pub params: Vec<(String, String)>,
}
#[derive(Serialize)]
pub struct JsonResponse {
    pub pretty: String,
    pub minified: String,
    pub error: Option<String>,
}
#[derive(Serialize)]
pub struct YamlResponse {
    pub result: String,
    pub error: Option<String>,
}
#[derive(Serialize)]
pub struct WhoamiResponse {
    pub ip: String,
    pub country: String,
    pub city: String,
    pub asn: String,
    pub user_agent: String,
    pub headers: std::collections::HashMap<String, String>,
}
#[derive(Serialize)]
pub struct FakeUserResponse {
    pub users: Vec<FakeUser>,
}
#[derive(Serialize)]
pub struct CreditCardResponse {
    pub cards: Vec<CreditCard>,
}
#[derive(Serialize)]
pub struct QrResponse {
    pub svg: String,
    pub error: Option<String>,
}
#[derive(Serialize)]
pub struct RegexPatternResponse {
    pub pattern: String,
}

// --- Structs from utils.rs ---

#[derive(Serialize)]
pub struct DiffChunk {
    pub tag: String,
    pub text: String,
}
#[derive(Serialize)]
pub struct DiffResponse {
    pub chunks: Vec<DiffChunk>,
}
#[derive(Serialize)]
pub struct CronResponse {
    pub valid: bool,
    pub next_runs: Vec<String>,
    pub error: String,
}
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
#[derive(Serialize)]
pub struct RegexResponse {
    pub matches: Vec<String>,
    pub count: usize,
    pub error: Option<String>,
}
#[derive(Serialize)]
pub struct RegexBuildResponse {
    pub pattern: String,
    pub error: Option<String>,
}
pub struct UuidConfig {
    pub count: usize,
    pub hyphens: bool,
    pub uppercase: bool,
}
#[derive(Serialize)]
pub struct Md5Response {
    pub md5_32_lower: String,
    pub md5_32_upper: String,
    pub md5_16_lower: String,
    pub md5_16_upper: String,
}
#[derive(Serialize)]
pub struct DateResponse {
    pub valid: bool,
    pub unix_sec: i64,
    pub unix_milli: i64,
    pub iso_8601: String,
    pub human_utc: String,
    pub error: Option<String>,
}
#[derive(Serialize)]
pub struct ColorResponse {
    pub valid: bool,
    pub hex: String,
    pub rgb: String,
    pub hsl: String,
    pub cmyk: String,
}
#[derive(Serialize)]
pub struct ChmodResponse {
    pub valid: bool,
    pub command: String,
}
#[derive(Serialize)]
pub struct JwtResponse {
    pub error: Option<String>,
    pub header: String,
    pub payload: String,
}
#[derive(Serialize)]
pub struct TarResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct PsResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct TcpdumpResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct GitResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct GitCmdResponse {
    pub command: String,
    pub description: String,
}
#[derive(Serialize)]
pub struct StraceResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct IostatResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct NiceResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct LsResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct FirewallResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct SystemctlResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct FindResponse {
    pub command: String,
}
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
#[derive(Deserialize, Serialize, Clone)]
pub struct NginxLocation {
    pub path: String,
    pub proxy: String,
    pub root: String,
    pub spa: bool,
}
#[derive(Serialize)]
pub struct UnitResponse {
    pub result: f64,
    pub value: f64,
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Serialize)]
pub struct CurlResponse {
    pub command: String,
    pub python: String,
}
#[derive(Serialize)]
pub struct RsyncResponse {
    pub command: String,
    pub ssh_config: String,
}
#[derive(Serialize)]
pub struct FakeUser {
    pub name: String,
    pub email: String,
    pub address: String,
    pub phone: String,
}
#[derive(Serialize)]
pub struct CreditCard {
    pub number: String,
    pub issuer: String,
    pub expiry: String,
    pub cvv: String,
}
#[derive(Serialize)]
pub struct AwkResponse {
    pub command: String,
}
#[derive(Serialize)]
pub struct SedResponse {
    pub command: String,
}
