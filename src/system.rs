use crate::models::*;

pub fn calculate_chmod(octal: &str, file: &str) -> ChmodResponse {
    if octal.len() != 3 || octal.chars().any(|c| !c.is_digit(8)) {
        return ChmodResponse {
            valid: false,
            command: "Invalid".into(),
        };
    }
    let f = if file.trim().is_empty() {
        "filename"
    } else {
        file.trim()
    };
    ChmodResponse {
        valid: true,
        command: format!("chmod {} {}", octal, f),
    }
}

pub fn generate_tar(
    op: &str,
    comp: &str,
    verbose: bool,
    archive: &str,
    files: &str,
) -> TarResponse {
    let mut cmd = String::from("tar -");
    match op {
        "extract" => cmd.push('x'),
        "list" => cmd.push('t'),
        _ => cmd.push('c'),
    }
    match comp {
        "gzip" => cmd.push('z'),
        "bzip2" => cmd.push('j'),
        "xz" => cmd.push('J'),
        _ => {}
    }
    if verbose {
        cmd.push('v');
    }
    cmd.push('f');
    cmd.push(' ');

    let arch_name = if archive.trim().is_empty() {
        match comp {
            "gzip" => "archive.tar.gz",
            "bzip2" => "archive.tar.bz2",
            "xz" => "archive.tar.xz",
            _ => "archive.tar",
        }
    } else {
        archive.trim()
    };
    cmd.push('"');
    cmd.push_str(&arch_name.replace('"', "\\\""));
    cmd.push('"');

    if !files.trim().is_empty() {
        if op == "extract" {
            cmd.push_str(" -C \"");
            cmd.push_str(&files.trim().replace('"', "\\\""));
            cmd.push('"');
        } else {
            cmd.push(' ');
            cmd.push_str(files.trim());
        }
    }

    TarResponse { command: cmd }
}

pub fn generate_ps(
    format: &str,
    sort: &str,
    tree: bool,
    filter: &str,
    wide: bool,
    threads: bool,
    user: &str,
    pid: &str,
) -> PsResponse {
    let mut cmd = String::from("ps");

    match format {
        "ef" => {
            cmd.push_str(" -ef");
            if tree {
                cmd.push_str(" --forest");
            }
            if wide {
                cmd.push_str("ww");
            }
            if threads {
                cmd.push_str("L");
            }
        }
        _ => {
            cmd.push_str(" aux");
            if wide {
                cmd.push_str("ww");
            }
            if threads {
                cmd.push('L');
            }
            if tree {
                cmd.push('f');
            }
        }
    }

    if !user.trim().is_empty() {
        cmd.push_str(" -u ");
        cmd.push_str(user.trim());
    }

    if !pid.trim().is_empty() {
        cmd.push_str(" -p ");
        cmd.push_str(pid.trim());
    }

    if !sort.is_empty() && sort != "none" {
        cmd.push_str(" --sort=");
        cmd.push_str(sort);
    }

    if !filter.trim().is_empty() {
        cmd.push_str(" | grep \"");
        cmd.push_str(&filter.trim().replace('"', "\\\""));
        cmd.push('"');
    }

    PsResponse { command: cmd }
}

pub fn generate_strace(
    target: &str,
    is_pid: bool,
    follow: bool,
    summary: bool,
    output_file: &str,
    filter: &str,
    string_limit: &str,
    timestamp: bool,
) -> StraceResponse {
    let mut cmd = String::from("strace");

    if follow {
        cmd.push_str(" -f");
    }
    if summary {
        cmd.push_str(" -c");
    }
    if timestamp {
        cmd.push_str(" -tt");
    }
    if !string_limit.trim().is_empty() {
        cmd.push_str(" -s ");
        cmd.push_str(string_limit.trim());
    }
    if !output_file.trim().is_empty() {
        cmd.push_str(" -o \"");
        cmd.push_str(&output_file.trim().replace('"', "\\\""));
        cmd.push('"');
    }
    if !filter.trim().is_empty() {
        cmd.push_str(" -e \"");
        cmd.push_str(&filter.trim().replace('"', "\\\""));
        cmd.push('"');
    }

    if !target.trim().is_empty() {
        cmd.push(' ');
        if is_pid {
            cmd.push_str("-p ");
        }
        cmd.push_str(target.trim());
    }

    StraceResponse { command: cmd }
}

pub fn generate_iostat(
    interval: &str,
    count: &str,
    human: bool,
    extended: bool,
    unit: &str,
    partitions: bool,
    timestamp: bool,
    device: &str,
) -> IostatResponse {
    let mut cmd = String::from("iostat");

    if human {
        cmd.push_str(" -h");
    }
    if extended {
        cmd.push_str(" -x");
    }
    if timestamp {
        cmd.push_str(" -t");
    }

    match unit {
        "k" => cmd.push_str(" -k"),
        "m" => cmd.push_str(" -m"),
        _ => {}
    }

    if partitions {
        cmd.push_str(" -p");
    }

    if !device.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(device.trim());
    }

    if !interval.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(interval.trim());
        if !count.trim().is_empty() {
            cmd.push(' ');
            cmd.push_str(count.trim());
        }
    }

    IostatResponse { command: cmd }
}

pub fn generate_nice(
    mode: &str,
    priority: i32,
    command: &str,
    target_type: &str,
    target: &str,
) -> NiceResponse {
    let mut cmd = String::new();
    let prio = priority.max(-20).min(19);

    if mode == "renice" {
        cmd.push_str("renice -n ");
        cmd.push_str(&prio.to_string());

        match target_type {
            "group" => cmd.push_str(" -g"),
            "user" => cmd.push_str(" -u"),
            _ => cmd.push_str(" -p"),
        }

        if !target.trim().is_empty() {
            cmd.push(' ');
            cmd.push_str(target.trim());
        }
    } else {
        cmd.push_str("nice -n ");
        cmd.push_str(&prio.to_string());

        if !command.trim().is_empty() {
            cmd.push(' ');
            cmd.push_str(command.trim());
        }
    }

    NiceResponse { command: cmd }
}

pub fn generate_ls(
    path: &str,
    all: bool,
    long: bool,
    human: bool,
    time: bool,
    reverse: bool,
    recursive: bool,
    inode: bool,
    directory: bool,
    color: bool,
) -> LsResponse {
    let mut cmd = String::from("ls");

    if color {
        cmd.push_str(" --color=auto");
    }

    let mut shorts = String::new();
    if all {
        shorts.push('a');
    }
    if long {
        shorts.push('l');
    }
    if human {
        shorts.push('h');
    }
    if time {
        shorts.push('t');
    }
    if reverse {
        shorts.push('r');
    }
    if recursive {
        shorts.push('R');
    }
    if inode {
        shorts.push('i');
    }
    if directory {
        shorts.push('d');
    }

    if !shorts.is_empty() {
        cmd.push_str(" -");
        cmd.push_str(&shorts);
    }
    if !path.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(path.trim());
    }
    LsResponse { command: cmd }
}

pub fn generate_systemctl(
    operation: &str,
    service: &str,
    user_mode: bool,
    now: bool,
    force: bool,
    global: bool,
) -> SystemctlResponse {
    let mut cmd = String::from("systemctl");

    if user_mode {
        cmd.push_str(" --user");
    } else if global {
        cmd.push_str(" --global");
    }

    if !operation.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(operation);
    }

    if force {
        cmd.push_str(" --force");
    }
    if now && (operation == "enable" || operation == "disable" || operation == "mask") {
        cmd.push_str(" --now");
    }

    if !service.trim().is_empty() && operation != "daemon-reload" {
        cmd.push(' ');
        cmd.push_str(service.trim());
    }

    SystemctlResponse { command: cmd }
}

pub fn generate_find(
    path: &str,
    name: &str,
    iname: bool,
    target_type: &str,
    size: &str,
    mtime: &str,
    empty: bool,
    exec: &str,
) -> FindResponse {
    let mut cmd = String::from("find");

    if !path.trim().is_empty() {
        cmd.push_str(" \"");
        cmd.push_str(&path.trim().replace('"', "\\\""));
        cmd.push('"');
    } else {
        cmd.push_str(" .");
    }

    if !name.trim().is_empty() {
        cmd.push(' ');
        if iname {
            cmd.push_str("-iname");
        } else {
            cmd.push_str("-name");
        }
        cmd.push(' ');
        cmd.push('"');
        cmd.push_str(&name.trim().replace('"', "\\\""));
        cmd.push('"');
    }

    if !target_type.trim().is_empty() && target_type != "all" {
        cmd.push_str(" -type ");
        cmd.push_str(target_type.trim());
    }

    if empty {
        cmd.push_str(" -empty");
    } else if !size.trim().is_empty() {
        cmd.push_str(" -size ");
        cmd.push_str(size.trim());
    }

    if !mtime.trim().is_empty() {
        cmd.push_str(" -mtime ");
        cmd.push_str(mtime.trim());
    }

    if !exec.trim().is_empty() {
        cmd.push_str(" -exec ");
        cmd.push_str(exec.trim());
        cmd.push_str(" {} \\;");
    }

    FindResponse { command: cmd }
}

pub fn generate_dockerfile(stages: &[DockerfileStage]) -> String {
    let mut df = String::new();

    for (index, stage) in stages.iter().enumerate() {
        if index > 0 {
            df.push_str(&format!("\n# Stage {}\n", index + 1));
        }

        if !stage.image.trim().is_empty() {
            df.push_str(&format!("FROM {}", stage.image.trim()));
            if !stage.as_.trim().is_empty() {
                df.push_str(&format!(" AS {}", stage.as_.trim()));
            }
            df.push('\n');
        } else {
            df.push_str("FROM scratch\n");
        }

        for line in stage.arg.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("ARG {}\n", line.trim()));
            }
        }

        for line in stage.label.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("LABEL {}\n", line.trim()));
            }
        }

        if !stage.workdir.trim().is_empty() {
            df.push_str(&format!("WORKDIR {}\n", stage.workdir.trim()));
        }

        for line in stage.env.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("ENV {}\n", line.trim()));
            }
        }

        for line in stage.copy.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("COPY {}\n", line.trim()));
            }
        }

        for line in stage.run.lines() {
            if !line.trim().is_empty() {
                df.push_str(&format!("RUN {}\n", line.trim()));
            }
        }

        if !stage.expose.trim().is_empty() {
            for port in stage.expose.split(&[',', ' '][..]) {
                if !port.trim().is_empty() {
                    df.push_str(&format!("EXPOSE {}\n", port.trim()));
                }
            }
        }

        if !stage.user.trim().is_empty() {
            df.push_str(&format!("USER {}\n", stage.user.trim()));
        }

        if !stage.volume.trim().is_empty() {
            for vol in stage.volume.split(&[',', ' '][..]) {
                if !vol.trim().is_empty() {
                    df.push_str(&format!("VOLUME {}\n", vol.trim()));
                }
            }
        }

        if !stage.healthcheck.trim().is_empty() {
            df.push_str(&format!("HEALTHCHECK {}\n", stage.healthcheck.trim()));
        }

        if !stage.entrypoint.trim().is_empty() {
            df.push_str(&format!("ENTRYPOINT {}\n", stage.entrypoint.trim()));
        }

        if !stage.cmd.trim().is_empty() {
            df.push_str(&format!("CMD {}\n", stage.cmd.trim()));
        }
    }

    df
}

pub fn generate_rsync(
    source: &str,
    user: &str,
    host: &str,
    port: &str,
    remote_path: &str,
    archive: bool,
    compress: bool,
    verbose: bool,
    delete: bool,
    dry_run: bool,
    progress: bool,
    ssh: bool,
    exclude: &str,
) -> RsyncResponse {
    let mut cmd = String::from("rsync");

    let mut shorts = String::new();
    if archive {
        shorts.push('a');
    }
    if compress {
        shorts.push('z');
    }
    if verbose {
        shorts.push('v');
    }
    if dry_run {
        shorts.push('n');
    }
    if progress {
        shorts.push('P');
    }

    if !shorts.is_empty() {
        cmd.push_str(" -");
        cmd.push_str(&shorts);
    }

    if delete {
        cmd.push_str(" --delete");
    }
    if !port.trim().is_empty() && port.trim() != "22" {
        cmd.push_str(&format!(" -e 'ssh -p {}'", port.trim()));
    } else if ssh {
        cmd.push_str(" -e ssh");
    }
    if !exclude.trim().is_empty() {
        cmd.push_str(" --exclude='");
        cmd.push_str(&exclude.trim().replace('\'', "'\\''"));
        cmd.push('\'');
    }

    if !source.trim().is_empty() {
        cmd.push_str(" \"");
        cmd.push_str(&source.trim().replace('"', "\\\""));
        cmd.push('"');
    } else {
        cmd.push_str(" /source/path");
    }

    cmd.push_str(" \"");
    if !host.trim().is_empty() {
        if !user.trim().is_empty() {
            cmd.push_str(user.trim());
            cmd.push('@');
        }
        cmd.push_str(host.trim());
        cmd.push(':');
        if !remote_path.trim().is_empty() {
            cmd.push_str(remote_path.trim());
        }
    } else {
        if !remote_path.trim().is_empty() {
            cmd.push_str(remote_path.trim());
        } else {
            cmd.push_str("/dest/path");
        }
    }
    cmd.push('"');

    let mut ssh_config = String::new();
    if !host.trim().is_empty() {
        ssh_config.push_str(&format!("Host {}\n", host.trim()));
        ssh_config.push_str(&format!("    HostName {}\n", host.trim()));
        if !user.trim().is_empty() {
            ssh_config.push_str(&format!("    User {}\n", user.trim()));
        }
        if !port.trim().is_empty() && port.trim() != "22" {
            ssh_config.push_str(&format!("    Port {}\n", port.trim()));
        }
    }

    RsyncResponse {
        command: cmd,
        ssh_config,
    }
}

pub fn generate_awk(separator: &str, variable: &str, code: &str, file: &str) -> AwkResponse {
    let mut cmd = String::from("awk");

    if !separator.is_empty() && separator != "space" {
        cmd.push_str(" -F '");
        cmd.push_str(&separator.replace('\'', "'\\''"));
        cmd.push('\'');
    }

    if !variable.trim().is_empty() {
        cmd.push_str(" -v ");
        cmd.push_str(variable.trim());
    }

    cmd.push_str(" '");
    if !code.trim().is_empty() {
        cmd.push_str(&code.trim().replace('\'', "'\\''"));
    } else {
        cmd.push_str("{print $0}");
    }
    cmd.push('\'');

    if !file.trim().is_empty() {
        cmd.push_str(" \"");
        cmd.push_str(&file.trim().replace('"', "\\\""));
        cmd.push('"');
    }

    AwkResponse { command: cmd }
}

pub fn generate_sed(
    operation: &str,
    pattern: &str,
    replacement: &str,
    flags: &str,
    inplace: bool,
    file: &str,
) -> SedResponse {
    let mut cmd = String::from("sed");
    if inplace {
        cmd.push_str(" -i");
    }
    cmd.push_str(" '");
    match operation {
        "substitute" => {
            cmd.push_str("s/");
            cmd.push_str(&pattern.replace('/', "\\/"));
            cmd.push('/');
            cmd.push_str(&replacement.replace('/', "\\/"));
            cmd.push('/');
            cmd.push_str(flags);
        }
        "delete" => {
            cmd.push_str(pattern);
            cmd.push('d');
        }
        "insert" => {
            cmd.push_str(pattern);
            cmd.push_str("i\\ ");
            cmd.push_str(replacement);
        }
        "append" => {
            cmd.push_str(pattern);
            cmd.push_str("a\\ ");
            cmd.push_str(replacement);
        }
        _ => {}
    }
    cmd.push('\'');
    if !file.trim().is_empty() {
        cmd.push(' ');
        cmd.push_str(file.trim());
    }
    SedResponse { command: cmd }
}

pub fn generate_tcpdump(
    interface: &str,
    protocol: &str,
    host: &str,
    port: &str,
    verbose: bool,
    ascii: bool,
    hex: bool,
    write_file: &str,
    count: &str,
) -> TcpdumpResponse {
    let mut cmd = String::from("tcpdump");
    if !interface.trim().is_empty() {
        cmd.push_str(" -i ");
        cmd.push_str(interface.trim());
    }
    if !protocol.trim().is_empty() && protocol != "all" {
        cmd.push(' ');
        cmd.push_str(protocol.trim());
    }
    if !host.trim().is_empty() {
        cmd.push_str(" host ");
        cmd.push_str(host.trim());
    }
    if !port.trim().is_empty() {
        cmd.push_str(" port ");
        cmd.push_str(port.trim());
    }
    if verbose {
        cmd.push_str(" -v");
    }
    if ascii {
        cmd.push_str(" -A");
    }
    if hex {
        cmd.push_str(" -X");
    }
    if !write_file.trim().is_empty() {
        cmd.push_str(" -w ");
        cmd.push_str(write_file.trim());
    }
    if !count.trim().is_empty() {
        cmd.push_str(" -c ");
        cmd.push_str(count.trim());
    }
    TcpdumpResponse { command: cmd }
}

pub fn generate_git(
    cmd: &str,
    target: &str,
    msg: &str,
    remote: &str,
    branch: &str,
    opt_force: bool,
    opt_rebase: bool,
    opt_all: bool,
    opt_amend: bool,
    opt_hard: bool,
    opt_new_branch: bool,
    opt_tags: bool,
    opt_oneline: bool,
    opt_graph: bool,
) -> GitResponse {
    let mut command = format!("git {}", cmd);
    match cmd {
        "init" | "clone" => {
            if !target.trim().is_empty() {
                command.push(' ');
                command.push_str(target.trim());
            }
        }
        "add" => {
            if opt_all {
                command.push_str(" -A");
            } else if !target.trim().is_empty() {
                command.push(' ');
                command.push_str(target.trim());
            }
        }
        "commit" => {
            if opt_all {
                command.push_str(" -a");
            }
            if opt_amend {
                command.push_str(" --amend");
            }
            if !msg.trim().is_empty() {
                command.push_str(" -m \"");
                command.push_str(&msg.trim().replace('"', "\\\""));
                command.push('"');
            }
        }
        "push" => {
            if opt_force {
                command.push_str(" --force");
            }
            if opt_tags {
                command.push_str(" --tags");
            }
            if !remote.trim().is_empty() {
                command.push(' ');
                command.push_str(remote.trim());
                if !branch.trim().is_empty() {
                    command.push(' ');
                    command.push_str(branch.trim());
                }
            }
        }
        "pull" => {
            if opt_rebase {
                command.push_str(" --rebase");
            }
            if !remote.trim().is_empty() {
                command.push(' ');
                command.push_str(remote.trim());
                if !branch.trim().is_empty() {
                    command.push(' ');
                    command.push_str(branch.trim());
                }
            }
        }
        "checkout" => {
            if opt_new_branch {
                command.push_str(" -b");
            }
            if !target.trim().is_empty() {
                command.push(' ');
                command.push_str(target.trim());
            }
        }
        "merge" => {
            if !target.trim().is_empty() {
                command.push(' ');
                command.push_str(target.trim());
            }
        }
        "log" => {
            if opt_oneline {
                command.push_str(" --oneline");
            }
            if opt_graph {
                command.push_str(" --graph");
            }
        }
        "reset" => {
            if opt_hard {
                command.push_str(" --hard");
            }
            if !target.trim().is_empty() {
                command.push(' ');
                command.push_str(target.trim());
            }
        }
        "remote" => {
            if !remote.trim().is_empty() {
                command.push_str(" add ");
                command.push_str(remote.trim());
                if !target.trim().is_empty() {
                    command.push(' ');
                    command.push_str(target.trim());
                }
            }
        }
        _ => {}
    }
    GitResponse { command }
}

pub fn generate_git_cmd(action: &str, tag: &str, msg: &str, branch: &str) -> GitCmdResponse {
    let (command, description) = match action {
        "undo_commit" => (
            "git reset --soft HEAD~1".to_string(),
            "撤销最近一次提交，但保留文件修改（Soft Reset）".to_string(),
        ),
        "undo_changes" => (
            "git checkout .".to_string(),
            "撤销工作区所有修改（危险：会丢失未提交的改动）".to_string(),
        ),
        "log_graph" => (
            "git log --graph --oneline --decorate --all".to_string(),
            "以图形化方式查看提交历史".to_string(),
        ),
        "tag" => {
            let t = if tag.trim().is_empty() {
                "v1.0.0"
            } else {
                tag.trim()
            };
            let m = if msg.trim().is_empty() {
                "Release version"
            } else {
                msg.trim()
            };
            (
                format!("git tag -a {} -m \"{}\" && git push origin {}", t, m, t),
                "创建并推送带注释的标签".to_string(), // Tag and push with annotation
            )
        }
        "branch_delete" => {
            let b = if branch.trim().is_empty() {
                "feature/old"
            } else {
                branch.trim()
            };
            (
                format!("git branch -d {} && git push origin --delete {}", b, b),
                "删除本地和远程分支".to_string(), // Delete local and remote branch
            )
        }
        "stash" => (
            "git stash && git pull && git stash pop".to_string(),
            "暂存修改，拉取代码，然后恢复修改".to_string(),
        ),
        _ => ("git help".to_string(), "".to_string()),
    };
    GitCmdResponse {
        command,
        description,
    }
}

pub fn generate_firewall(
    op: &str,
    zone: &str,
    target_type: &str,
    target: &str,
    permanent: bool,
) -> FirewallResponse {
    let mut cmd = String::from("firewall-cmd");
    if permanent {
        cmd.push_str(" --permanent");
    }
    if !zone.trim().is_empty() {
        cmd.push_str(" --zone=");
        cmd.push_str(zone.trim());
    }
    match op {
        "add" => {
            if target_type == "port" {
                cmd.push_str(" --add-port=");
            } else {
                cmd.push_str(" --add-service=");
            }
            cmd.push_str(target.trim());
        }
        "remove" => {
            if target_type == "port" {
                cmd.push_str(" --remove-port=");
            } else {
                cmd.push_str(" --remove-service=");
            }
            cmd.push_str(target.trim());
        }
        "list" => cmd.push_str(" --list-all"),
        "reload" => {
            return FirewallResponse {
                command: "firewall-cmd --reload".into(),
            }
        }
        _ => {}
    }
    FirewallResponse { command: cmd }
}

pub fn generate_curl(method: &str, url: &str, headers: &str, body: &str) -> CurlResponse {
    let m = if method.trim().is_empty() {
        "GET".to_string()
    } else {
        method.trim().to_uppercase()
    };
    let u = if url.trim().is_empty() {
        "http://localhost:8080"
    } else {
        url.trim()
    };

    let mut cmd = format!("curl -X {} '{}'", m, u.replace('\'', "'\\''"));
    let mut py = format!("import requests\n\nurl = \"{}\"\n", u);

    let mut has_headers = false;
    let headers_map: Option<std::collections::HashMap<String, String>> =
        serde_json::from_str(headers).ok();

    if let Some(h) = &headers_map {
        if !h.is_empty() {
            has_headers = true;
            py.push_str("\nheaders = {\n");
            for (k, v) in h {
                cmd.push_str(" \\\n  -H '");
                cmd.push_str(k);
                cmd.push_str(": ");
                cmd.push_str(v);
                cmd.push('\'');

                py.push_str(&format!("  '{}': '{}',\n", k, v));
            }
            py.push_str("}\n");
        }
    }

    let mut has_payload = false;
    if ["POST", "PUT", "PATCH"].contains(&m.as_str()) {
        if !body.is_empty() {
            has_payload = true;
            let py_body = body.replace('\\', "\\\\").replace('"', "\\\"");
            py.push_str(&format!("\npayload = \"{}\"\n", py_body));

            // Check if body is valid JSON to add content-type header if not present
            if serde_json::from_str::<serde_json::Value>(body).is_ok() {
                cmd.push_str(" \\\n  -H 'Content-Type: application/json'");
            }
            cmd.push_str(" \\\n  -d '");
            cmd.push_str(&body.replace('\'', "'\\''"));
            cmd.push('\'');
        }
    }

    py.push_str(&format!("\nresponse = requests.request(\"{}\", url", m));
    if has_headers {
        py.push_str(", headers=headers");
    }
    if has_payload {
        py.push_str(", data=payload");
    }
    py.push_str(")\n\nprint(response.text)");

    CurlResponse {
        command: cmd,
        python: py,
    }
}
