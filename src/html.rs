pub fn get_homepage() -> &'static str {
    r####"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust 极客工具箱 - 在线开发者实用工具集</title>
    <meta name="description" content="免费在线开发者工具箱，提供 JWT 解析, UUID 生成, Token 生成, 时间戳转换, 颜色提取, Base64, MD5, JSON 格式化, YAML 转 TOML, 二维码生成等 Rust 高性能工具。">
    <meta name="google-site-verification" content="udcr95cFCSGcY_tu2UQoPK-7ygIkH_ARtI_Vi4DguhM" />
    <meta name="msvalidate.01" content="28230D5CF5CF4D57C17D65DB9628DDBA" />
    <style>
        /* --- 全局样式 --- */
        * { box-sizing: border-box; margin: 0; padding: 0; outline: none; }
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; display: flex; height: 100vh; overflow: hidden; color: #333; background: #f8fafc; }
        
        /* 侧边栏 */
        .sidebar { width: 260px; background-color: #1e293b; color: #e2e8f0; display: flex; flex-direction: column; border-right: 1px solid #334155; flex-shrink: 0; user-select: none; }
        .logo { padding: 20px; font-size: 20px; font-weight: bold; color: #38bdf8; border-bottom: 1px solid #334155; display: flex; align-items: center; gap: 8px; }
        .menu-scroll { flex: 1; overflow-y: auto; padding: 10px; }
        
        /* 菜单 */
        .menu-group { margin-bottom: 5px; }
        .menu-category { font-size: 12px; font-weight: bold; color: #94a3b8; padding: 10px 8px; letter-spacing: 0.5px; cursor: pointer; display: flex; justify-content: space-between; align-items: center; border-radius: 4px; transition: background 0.2s; }
        .menu-category:hover { background-color: #334155; color: #e2e8f0; }
        .menu-arrow { font-size: 10px; transition: transform 0.2s ease; }
        .menu-group.collapsed .menu-list { display: none; }
        .menu-group.collapsed .menu-arrow { transform: rotate(-90deg); }
        .menu-list { list-style: none; padding: 0; margin-bottom: 5px; }
        .menu-link { display: flex; align-items: center; padding: 10px 12px; color: #cbd5e1; text-decoration: none; border-radius: 6px; cursor: pointer; margin-bottom: 2px; transition: all 0.2s; font-size: 14px; }
        .menu-link:hover { background-color: #334155; color: white; }
        .menu-link.active { background-color: #0ea5e9; color: white; font-weight: 500; }
        .menu-icon { margin-right: 8px; width: 16px; text-align: center; }

        /* 内容区 */
        .main-content { flex: 1; background-color: #f8fafc; overflow-y: auto; padding: 40px; }
        .tool-panel { display: none; max-width: 1000px; margin: 0 auto; background: white; padding: 30px; border-radius: 12px; box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1); }
        .tool-panel.active { display: block; animation: fadeIn 0.3s ease; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }

        h2 { margin-bottom: 20px; color: #0f172a; border-bottom: 2px solid #f1f5f9; padding-bottom: 10px; }
        p.tool-desc { margin-bottom: 15px; color: #64748b; font-size: 14px; }

        /* 组件 */
        textarea, input[type="text"], input[type="number"], select { width: 100%; padding: 12px; margin-bottom: 15px; border: 1px solid #cbd5e1; border-radius: 6px; font-family: monospace; font-size: 14px; background: white; transition: border 0.2s; }
        textarea:focus, input:focus, select:focus { outline: 2px solid #0ea5e9; border-color: transparent; }
        textarea { resize: vertical; min-height: 80px; }
        
        button.action-btn { background-color: #0ea5e9; color: white; border: none; padding: 10px 24px; border-radius: 6px; cursor: pointer; margin-right: 10px; font-size: 14px; transition: background 0.2s; white-space: nowrap; flex-shrink: 0; }
        button.action-btn:hover { background-color: #0284c7; }
        button.secondary { background-color: #64748b; }
        button.secondary:hover { background-color: #475569; }

        /* 结果显示容器 */
        .result-box-container { position: relative; background: #f1f5f9; padding: 12px; border-radius: 6px; border: 1px solid #e2e8f0; margin-top: 15px; transition: background 0.2s; }
        .result-text { font-family: monospace; color: #334155; word-break: break-all; padding-right: 40px; min-height: 24px; line-height: 24px; white-space: pre-wrap; }
        
        /* 复制按钮 */
        .copy-btn { position: absolute; top: 8px; right: 8px; background: transparent; border: none; color: #94a3b8; padding: 4px; border-radius: 4px; cursor: pointer; display: flex; align-items: center; justify-content: center; transition: all 0.2s; }
        .copy-btn:hover { background-color: #cbd5e1; color: #334155; }
        .copy-btn svg { width: 18px; height: 18px; }

        /* Textarea 包装器 */
        .textarea-wrapper { position: relative; width: 100%; height: 100%; display: flex; flex-direction: column; }
        .textarea-wrapper textarea { flex: 1; margin-bottom: 0; background: #f8fafc; }
        .textarea-wrapper .copy-btn { top: 10px; right: 10px; background: rgba(255,255,255,0.8); border: 1px solid #cbd5e1; }

        /* 布局网格 */
        .grid-2-col { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-top: 20px; }
        
        /* 信息行 */
        .info-row { position: relative; background: #f1f5f9; padding: 12px; border-radius: 6px; border: 1px solid #e2e8f0; margin-top: 10px; display: flex; align-items: center; }
        .info-label { width: 100px; font-size: 12px; font-weight: bold; color: #64748b; flex-shrink: 0; text-transform: uppercase; }
        .info-value { font-family: monospace; color: #333; flex: 1; word-break: break-all; margin-right: 30px; font-size: 14px; }
        .info-row .copy-btn { top: 50%; transform: translateY(-50%); right: 8px; }

        /* 控制面板 */
        .controls-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-bottom: 20px; }
        .control-group { background: #f8fafc; padding: 15px; border-radius: 8px; border: 1px solid #e2e8f0; display: flex; flex-direction: column; justify-content: center; }
        .control-label { font-size: 14px; color: #64748b; margin-bottom: 5px; font-weight: 500; }
        
        .input-group-row { display: flex; gap: 10px; margin-bottom: 20px; align-items: center; }
        .input-group-row input { margin-bottom: 0; }
        
        input[type="range"] { width: 100%; margin: 10px 0; accent-color: #0ea5e9; cursor: pointer; }
        .checkbox-group { flex-direction: row; flex-wrap: wrap; gap: 15px; }
        .checkbox-group label { display: flex; align-items: center; cursor: pointer; font-size: 14px; user-select: none; }
        .checkbox-group input { margin-right: 6px; width: 16px; height: 16px; accent-color: #0ea5e9; }

        /* 颜色工具 */
        .color-preview-box { height: 100px; border-radius: 8px; margin-bottom: 20px; border: 1px solid #e2e8f0; background-color: transparent; display: flex; align-items: center; justify-content: center; color: rgba(0,0,0,0.5); font-weight: bold; text-shadow: 0 1px 0 rgba(255,255,255,0.4); }
        input[type="color"] { -webkit-appearance: none; border: none; width: 60px; height: 42px; cursor: pointer; background: none; padding: 0; border-radius: 6px; overflow: hidden; margin-right: 10px; }

        /* 转换器布局 */
        .converter-container { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; height: 450px; }
        .converter-box { display: flex; flex-direction: column; height: 100%; position: relative; }
        .converter-label { font-weight: bold; margin-bottom: 8px; color: #475569; display: flex; justify-content: space-between; align-items: center; font-size: 14px; }
        .code-editor { font-family: "Menlo", "Monaco", "Courier New", monospace; line-height: 1.5; font-size: 13px; white-space: pre; overflow-wrap: normal; overflow-x: scroll; border: 1px solid #cbd5e1; border-radius: 6px; padding: 10px; background: white; }
        .error-msg { color: #ef4444; font-size: 13px; margin-top: 5px; min-height: 20px; }

        /* URL 表格 */
        .url-params-table { width: 100%; border-collapse: collapse; font-size: 13px; margin-top: 10px; }
        .url-params-table th, .url-params-table td { text-align: left; padding: 8px; border-bottom: 1px solid #e2e8f0; }
        .url-params-table th { color: #64748b; font-weight: 600; width: 100px; }
        .url-params-table td { font-family: monospace; color: #334155; }

        /* 二维码 */
        .qr-container { display: flex; flex-direction: column; align-items: center; justify-content: center; background: white; border: 2px dashed #cbd5e1; padding: 20px; border-radius: 8px; min-height: 300px; margin-top: 15px; }
        canvas#qr-canvas { max-width: 100%; height: auto; box-shadow: 0 4px 6px rgba(0,0,0,0.1); border-radius: 4px; }
        .file-upload-wrapper { position: relative; margin-bottom: 10px; width: 100%; }
        .file-upload-btn { width: 100%; padding: 10px; background: #f1f5f9; border: 1px dashed #94a3b8; border-radius: 6px; text-align: center; cursor: pointer; color: #64748b; font-size: 13px; transition: all 0.2s; display: block; }
        .file-upload-btn:hover { background: #e2e8f0; border-color: #64748b; color: #334155; }
        input[type="file"] { display: none; }
        .preview-logo { width: 30px; height: 30px; object-fit: contain; margin-left: 10px; vertical-align: middle; display: none; border-radius: 4px; border: 1px solid #ddd; }

        /* Linux Chmod */
        .chmod-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; margin-bottom: 20px; background: #fff; padding: 15px; border-radius: 8px; border: 1px solid #e2e8f0; }
        .chmod-header { font-weight: bold; color: #475569; text-align: center; padding-bottom: 10px; border-bottom: 1px solid #f1f5f9; font-size: 13px; }
        .chmod-cell { display: flex; align-items: center; justify-content: center; padding: 5px; }
        .chmod-cell label { cursor: pointer; display: flex; align-items: center; gap: 5px; color: #334155; font-weight: 500; }
        .chmod-cell input[type="checkbox"] { width: 16px; height: 16px; accent-color: #0ea5e9; }
        .chmod-row-label { font-weight: bold; color: #0f172a; display: flex; align-items: center; justify-content: center; font-size: 14px; }
        .chmod-input-large { font-size: 24px; font-weight: bold; text-align: center; color: #0ea5e9; letter-spacing: 2px; border: none !important; background: transparent !important; }

        /* Toast */
        #toast-container { position: fixed; bottom: 30px; right: 30px; z-index: 9999; display: flex; flex-direction: column; gap: 10px; pointer-events: none; }
        .toast { background: white; padding: 12px 20px; border-radius: 8px; box-shadow: 0 10px 15px -3px rgba(0,0,0,0.1), 0 4px 6px -2px rgba(0,0,0,0.05); border-left: 4px solid #0ea5e9; display: flex; align-items: center; gap: 12px; animation: slideIn 0.3s ease; pointer-events: auto; min-width: 250px; font-size: 14px; color: #334155; }
        .toast.error { border-left-color: #ef4444; }
        .toast.success { border-left-color: #10b981; }
        @keyframes slideIn { from { transform: translateX(100%); opacity: 0; } to { transform: translateX(0); opacity: 1; } }

        /* Subnet */
        .subnet-table { width: 100%; border-collapse: collapse; font-size: 14px; margin-top: 15px; }
        .subnet-table th, .subnet-table td { text-align: left; padding: 10px; border-bottom: 1px solid #e2e8f0; }
        .subnet-table th { color: #64748b; font-weight: 600; width: 120px; }
        .subnet-table td { font-family: monospace; color: #333; }
    </style>
    <div style="display:none">
        <svg id="icon-copy-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
    </div>
</head>
<body>
    <div id="toast-container"></div>

    <aside class="sidebar">
        <div class="logo">🦀 Rust 工具箱</div>
        <div class="menu-scroll">
            <div class="menu-group">
                <div class="menu-category" onclick="toggleGroup(this)"><span>💻 开发 & 运维</span><span class="menu-arrow">▼</span></div>
                <ul class="menu-list">
                    <li><a class="menu-link active" onclick="switchTool('subnet', this)"><span class="menu-icon">🌐</span>网络子网计算</a></li>
                    <li><a class="menu-link" onclick="switchTool('chmod', this)"><span class="menu-icon">🐧</span>Linux 权限</a></li>
                </ul>
            </div>
            <div class="menu-group">
                <div class="menu-category" onclick="toggleGroup(this)"><span>🔐 加密 & 安全</span><span class="menu-arrow">▼</span></div>
                <ul class="menu-list">
                    <li><a class="menu-link" onclick="switchTool('jwt', this)"><span class="menu-icon">🛡️</span>JWT 解析</a></li>
                    <li><a class="menu-link" onclick="switchTool('password', this)"><span class="menu-icon">🔑</span>强密码生成</a></li>
                    <li><a class="menu-link" onclick="switchTool('hash', this)"><span class="menu-icon">#️⃣</span>哈希计算 (MD5)</a></li>
                    <li><a class="menu-link" onclick="switchTool('jsenc', this)"><span class="menu-icon">🛡️</span>JS 加密/混淆</a></li>
                </ul>
            </div>
            <div class="menu-group">
                <div class="menu-category" onclick="toggleGroup(this)"><span>🛠️ 转换 & 生成</span><span class="menu-arrow">▼</span></div>
                <ul class="menu-list">
                    <li><a class="menu-link" onclick="switchTool('qr', this)"><span class="menu-icon">📱</span>二维码生成</a></li>
                    <li><a class="menu-link" onclick="switchTool('base64', this)"><span class="menu-icon">📝</span>Base64 转换</a></li>
                    <li><a class="menu-link" onclick="switchTool('json', this)"><span class="menu-icon">📋</span>JSON 工具</a></li>
                    <li><a class="menu-link" onclick="switchTool('url', this)"><span class="menu-icon">🔗</span>URL 工具</a></li>
                    <li><a class="menu-link" onclick="switchTool('date', this)"><span class="menu-icon">📅</span>时间转换</a></li>
                    <li><a class="menu-link" onclick="switchTool('color', this)"><span class="menu-icon">🎨</span>颜色转换</a></li>
                    <li><a class="menu-link" onclick="switchTool('token', this)"><span class="menu-icon">🔑</span>Token 生成器</a></li>
                    <li><a class="menu-link" onclick="switchTool('uuid', this)"><span class="menu-icon">🆔</span>UUID 生成器</a></li>
                    <li><a class="menu-link" onclick="switchTool('yaml', this)"><span class="menu-icon">⚙️</span>YAML 转 TOML</a></li>
                    <li><a class="menu-link" onclick="switchTool('toml2yaml', this)"><span class="menu-icon">⚙️</span>TOML 转 YAML</a></li>
                </ul>
            </div>
        </div>
    </aside>

    <main class="main-content">
        
        <div id="subnet" class="tool-panel active">
            <h2>🌐 网络子网计算器</h2>
            <p class="tool-desc">输入 IP 地址和 CIDR 掩码位，计算网络范围、广播地址及可用主机数。</p>
            <div class="input-group-row">
                <input type="text" id="sn-ip" placeholder="例如 192.168.1.1" value="192.168.1.1" style="flex:2">
                <select id="sn-cidr" style="flex:1">
                    <option value="32">/32 (1 IP)</option><option value="30">/30 (4 IPs)</option><option value="29">/29 (8 IPs)</option>
                    <option value="28">/28 (16 IPs)</option><option value="27">/27 (32 IPs)</option><option value="26">/26 (64 IPs)</option>
                    <option value="25">/25 (128 IPs)</option><option value="24" selected>/24 (256 IPs)</option><option value="23">/23 (512 IPs)</option>
                    <option value="22">/22 (1k IPs)</option><option value="20">/20 (4k IPs)</option><option value="16">/16 (65k IPs)</option>
                    <option value="8">/8 (16M IPs)</option>
                </select>
                <button class="action-btn" onclick="calcSubnet()">计算</button>
            </div>
            <div style="background:#fff; border-radius:8px; border:1px solid #e2e8f0; padding:15px;">
                <table class="subnet-table">
                    <tr><th>网络地址</th><td id="sn-res-net" style="color:#0ea5e9; font-weight:bold;">-</td></tr>
                    <tr><th>广播地址</th><td id="sn-res-broad">-</td></tr>
                    <tr><th>CIDR</th><td id="sn-res-cidr">-</td></tr>
                    <tr><th>子网掩码</th><td id="sn-res-mask">-</td></tr>
                    <tr><th>可用主机数</th><td id="sn-res-hosts" style="font-weight:bold">-</td></tr>
                    <tr><th>IP 范围</th><td><span id="sn-res-first">-</span> <span style="color:#cbd5e1">~</span> <span id="sn-res-last">-</span></td></tr>
                    <tr><th>二进制 IP</th><td id="sn-res-bin-ip" style="font-size:12px; color:#94a3b8">-</td></tr>
                </table>
            </div>
        </div>

        <div id="chmod" class="tool-panel">
            <h2>🐧 Linux 权限计算器</h2>
            <div class="chmod-grid">
                <div class="chmod-header">角色</div><div class="chmod-header">Read (4)</div><div class="chmod-header">Write (2)</div><div class="chmod-header">Execute (1)</div>
                <div class="chmod-row-label">Owner</div>
                <div class="chmod-cell"><input type="checkbox" id="c_ur" onchange="upChmod(true)"><label for="c_ur">r</label></div>
                <div class="chmod-cell"><input type="checkbox" id="c_uw" onchange="upChmod(true)"><label for="c_uw">w</label></div>
                <div class="chmod-cell"><input type="checkbox" id="c_ux" onchange="upChmod(true)"><label for="c_ux">x</label></div>
                <div class="chmod-row-label">Group</div>
                <div class="chmod-cell"><input type="checkbox" id="c_gr" onchange="upChmod(true)"><label for="c_gr">r</label></div>
                <div class="chmod-cell"><input type="checkbox" id="c_gw" onchange="upChmod(true)"><label for="c_gw">w</label></div>
                <div class="chmod-cell"><input type="checkbox" id="c_gx" onchange="upChmod(true)"><label for="c_gx">x</label></div>
                <div class="chmod-row-label">Others</div>
                <div class="chmod-cell"><input type="checkbox" id="c_or" onchange="upChmod(true)"><label for="c_or">r</label></div>
                <div class="chmod-cell"><input type="checkbox" id="c_ow" onchange="upChmod(true)"><label for="c_ow">w</label></div>
                <div class="chmod-cell"><input type="checkbox" id="c_ox" onchange="upChmod(true)"><label for="c_ox">x</label></div>
            </div>
            <div style="display:flex; gap: 20px; align-items:center; margin-bottom: 20px;">
                <div style="flex:1"><label style="font-size:12px;color:#64748b;">Octal</label><input type="text" id="chmod-octal" class="chmod-input-large" value="000" maxlength="3" oninput="upChmod(false)"></div>
                <div style="flex:2"><label style="font-size:12px;color:#64748b;">Symbolic</label><div id="chmod-symbolic" style="font-family:monospace;font-size:24px;padding:10px;background:#f1f5f9;border-radius:6px;">---------</div></div>
            </div>
            <div class="result-box-container"><div class="info-label">Command</div><div class="result-text" id="chmod-command">chmod 000 filename</div><button class="copy-btn" onclick="copyText('chmod-command', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
        </div>

        <div id="password" class="tool-panel">
             <h2>🔑 强密码生成器</h2>
             <div class="controls-grid">
                <div class="control-group">
                    <span class="control-label">长度: <span id="pwd-len-val">16</span></span>
                    <input type="range" id="pwd-len" min="6" max="64" value="16" oninput="document.getElementById('pwd-len-val').innerText=this.value">
                </div>
                <div class="control-group checkbox-group">
                    <label><input type="checkbox" id="pwd-u" checked> A-Z (大写)</label>
                    <label><input type="checkbox" id="pwd-l" checked> a-z (小写)</label>
                    <label><input type="checkbox" id="pwd-n" checked> 0-9 (数字)</label>
                    <label><input type="checkbox" id="pwd-s" checked> !@# (符号)</label>
                </div>
             </div>
             <button class="action-btn" onclick="genPwd()">🎲 生成密码</button>
             <div class="result-box-container"><div class="result-text" id="pwd-result" style="font-size:18px; color:#2563eb; font-weight:bold;">...</div><button class="copy-btn" onclick="copyText('pwd-result', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
        </div>

        <div id="hash" class="tool-panel">
             <h2>#️⃣ 哈希计算 (MD5)</h2>
             <textarea id="hash-input" placeholder="输入内容..."></textarea>
             <button class="action-btn" onclick="calculateMd5()">⚡ 计算 MD5</button>
             <div class="grid-2-col">
                 <div class="result-box-container"><div class="info-label">32 (小)</div><div class="result-text" id="md5-32-lower"></div><button class="copy-btn" onclick="copyText('md5-32-lower', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                 <div class="result-box-container"><div class="info-label">32 (大)</div><div class="result-text" id="md5-32-upper"></div><button class="copy-btn" onclick="copyText('md5-32-upper', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                 <div class="result-box-container"><div class="info-label">16 (小)</div><div class="result-text" id="md5-16-lower"></div><button class="copy-btn" onclick="copyText('md5-16-lower', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                 <div class="result-box-container"><div class="info-label">16 (大)</div><div class="result-text" id="md5-16-upper"></div><button class="copy-btn" onclick="copyText('md5-16-upper', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
             </div>
        </div>

        <div id="jwt" class="tool-panel">
            <h2>🛡️ JWT 解析器</h2>
            <p class="tool-desc">输入 JWT 令牌，解析并查看其 Header 和 Payload 内容。</p>
            <div class="converter-box" style="height: auto; margin-bottom: 20px;">
                <div class="converter-label">Encoded Token</div>
                <div class="textarea-wrapper">
                    <textarea id="jwt-input" class="code-editor" style="height:100px;" placeholder="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."></textarea>
                </div>
            </div>
            <button class="action-btn" style="margin-bottom: 20px; width: 100%;" onclick="parseJwt()">🔍 解析 Token</button>
            <div class="converter-container">
                <div class="converter-box">
                    <div class="converter-label">Header</div>
                    <div class="textarea-wrapper"><textarea id="jwt-header" class="code-editor" readonly></textarea><button class="copy-btn" onclick="copyText('jwt-header', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                </div>
                <div class="converter-box">
                    <div class="converter-label">Payload</div>
                    <div class="textarea-wrapper"><textarea id="jwt-payload" class="code-editor" readonly></textarea><button class="copy-btn" onclick="copyText('jwt-payload', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                </div>
            </div>
        </div>

        <div id="jsenc" class="tool-panel">
            <h2>🛡️ JS 代码混淆</h2>
            <div class="converter-container">
                <div class="converter-box">
                    <div class="converter-label">JavaScript 源代码<button class="copy-btn" style="position:static" onclick="document.getElementById('js-input').value=''">清空</button></div>
                    <div class="textarea-wrapper"><textarea id="js-input" class="code-editor" placeholder="console.log('Hello');"></textarea></div>
                </div>
                <div class="converter-box">
                    <div class="converter-label">混淆结果 (Eval Packer)</div>
                    <div class="textarea-wrapper"><textarea id="js-output" class="code-editor" readonly></textarea><button class="copy-btn" onclick="copyText('js-output', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                </div>
            </div>
            <button class="action-btn" style="margin-top:15px; width:100%" onclick="doJsEnc()">🔒 混淆加密</button>
        </div>

        <div id="qr" class="tool-panel">
            <h2>📱 二维码生成</h2>
            <div class="input-group-row">
                <input type="text" id="qr-text" placeholder="输入链接或文本...">
                <button class="action-btn" style="margin:0; width:auto;" onclick="generateQR()">生成</button>
            </div>
            <div class="file-upload-wrapper"><label for="qr-logo-upload" class="file-upload-btn">📷 上传 Logo (可选) <img id="logo-preview-img" class="preview-logo"></label><input type="file" id="qr-logo-upload" accept="image/*" onchange="handleLogoUpload(this)"></div>
            <div class="qr-container"><canvas id="qr-canvas" width="300" height="300"></canvas><a id="qr-download" style="display:none; margin-top:15px; color:#0ea5e9; cursor:pointer;">⬇️ 下载图片</a></div>
        </div>

        <div id="base64" class="tool-panel">
             <h2>📦 Base64 编码/解码</h2>
             <textarea id="base64-input" placeholder="输入内容..."></textarea>
             <div class="input-group-row">
                 <button class="action-btn" onclick="convertBase64('encode')">🔒 编码</button>
                 <button class="action-btn secondary" onclick="convertBase64('decode')">🔓 解码</button>
             </div>
             <div class="result-box-container"><div class="result-text" id="base64-result">结果...</div><button class="copy-btn" onclick="copyText('base64-result', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
        </div>

        <div id="json" class="tool-panel">
            <h2>📋 JSON 格式化</h2>
            <div class="converter-container">
                <div class="converter-box">
                    <div class="converter-label">输入 JSON<button class="copy-btn" style="position:static" onclick="document.getElementById('json-input').value=''">清空</button></div>
                    <div class="textarea-wrapper"><textarea id="json-input" class="code-editor" placeholder='{"key":"value"}'></textarea></div>
                </div>
                <div class="converter-box">
                    <div class="converter-label">格式化结果</div>
                    <div class="textarea-wrapper"><textarea id="json-output" class="code-editor" readonly></textarea><button class="copy-btn" onclick="copyText('json-output', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                </div>
            </div>
            <div style="margin-top:15px; display:flex; gap:10px;">
                <button class="action-btn" style="flex:1" onclick="processJson('fmt')">✨ 格式化</button>
                <button class="action-btn secondary" style="flex:1" onclick="processJson('min')">📦 压缩</button>
            </div>
            <div id="json-error" class="error-msg"></div>
        </div>

        <div id="url" class="tool-panel">
            <h2>🔗 URL 工具</h2>
            <textarea id="url-input" style="height:80px;" placeholder="输入 URL..."></textarea>
            <div class="input-group-row">
                <button class="action-btn" onclick="processUrl('enc')">编码</button>
                <button class="action-btn secondary" onclick="processUrl('dec')">解码</button>
            </div>
            <div class="grid-2-col">
                <div class="result-box-container"><div class="info-label">Result</div><div class="result-text" id="url-output">...</div><button class="copy-btn" onclick="copyText('url-output', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
            </div>
            <div style="margin-top:20px; background:#fff; padding:15px; border-radius:8px; border:1px solid #e2e8f0;">
                <h3 style="font-size:14px; margin-bottom:10px; color:#475569">URL 详情</h3>
                <div style="font-size:13px;">
                    <div><strong style="color:#64748b; width:50px; display:inline-block">Proto:</strong> <span id="url-proto">-</span></div>
                    <div><strong style="color:#64748b; width:50px; display:inline-block">Host:</strong> <span id="url-host">-</span></div>
                    <div><strong style="color:#64748b; width:50px; display:inline-block">Path:</strong> <span id="url-path">-</span></div>
                </div>
                <table class="url-params-table"><tbody id="url-params-body"></tbody></table>
            </div>
        </div>

        <div id="date" class="tool-panel">
            <h2>📅 时间转换</h2>
            <div class="input-group-row">
                <input type="text" id="date-input" placeholder="输入时间戳或 yyyy-mm-dd HH:mm:ss">
                <button class="action-btn secondary" onclick="fillNow()">当前时间</button>
                <button class="action-btn" onclick="convertDate()">转换</button>
            </div>
            <div style="margin-top:20px">
                <div class="info-row"><div class="info-label">Unix (秒)</div><div class="info-value" id="date-sec"></div><button class="copy-btn" onclick="copyText('date-sec', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                <div class="info-row"><div class="info-label">Unix (毫秒)</div><div class="info-value" id="date-milli"></div><button class="copy-btn" onclick="copyText('date-milli', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                <div class="info-row"><div class="info-label">ISO 8601</div><div class="info-value" id="date-iso"></div><button class="copy-btn" onclick="copyText('date-iso', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                <div class="info-row"><div class="info-label">UTC 时间</div><div class="info-value" id="date-utc"></div><button class="copy-btn" onclick="copyText('date-utc', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                <div class="info-row" style="border-left: 3px solid #0ea5e9;"><div class="info-label" style="color:#0ea5e9">本地时间</div><div class="info-value" id="date-local"></div><button class="copy-btn" onclick="copyText('date-local', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
            </div>
        </div>

        <div id="color" class="tool-panel">
            <h2>🎨 颜色转换</h2>
            <div class="input-group-row">
                <input type="color" id="color-picker" oninput="pickColor()">
                <input type="text" id="color-input" placeholder="#FFFFFF" oninput="syncColorPicker()">
                <button class="action-btn" onclick="convertColor()">转换</button>
            </div>
            <div id="color-preview" class="color-preview-box">PREVIEW</div>
            <div class="grid-2-col">
                <div class="result-box-container"><div class="info-label">HEX</div><div class="result-text" id="color-hex"></div><button class="copy-btn" onclick="copyText('color-hex', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                <div class="result-box-container"><div class="info-label">RGB</div><div class="result-text" id="color-rgb"></div><button class="copy-btn" onclick="copyText('color-rgb', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                <div class="result-box-container"><div class="info-label">HSL</div><div class="result-text" id="color-hsl"></div><button class="copy-btn" onclick="copyText('color-hsl', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
                <div class="result-box-container"><div class="info-label">CMYK</div><div class="result-text" id="color-cmyk"></div><button class="copy-btn" onclick="copyText('color-cmyk', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
            </div>
        </div>

        <div id="token" class="tool-panel">
             <h2>🔑 Token 生成器</h2>
             <div class="controls-grid">
                <div class="control-group">
                    <span class="control-label">长度: <span id="len-val">32</span></span>
                    <input type="range" id="token-len" min="8" max="128" value="32" oninput="document.getElementById('len-val').innerText=this.value">
                </div>
                <div class="control-group checkbox-group">
                    <label><input type="checkbox" id="tok-u" checked> A-Z</label>
                    <label><input type="checkbox" id="tok-l" checked> a-z</label>
                    <label><input type="checkbox" id="tok-n" checked> 0-9</label>
                    <label><input type="checkbox" id="tok-s" checked> #$!</label>
                </div>
             </div>
             <button class="action-btn" onclick="genToken()">🎲 生成 Token</button>
             <div class="result-box-container"><div class="result-text" id="token-result">...</div><button class="copy-btn" onclick="copyText('token-result', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div>
        </div>

        <div id="uuid" class="tool-panel">
            <h2>🆔 UUID 生成器</h2>
            <div class="controls-grid">
                <div class="control-group">
                    <span class="control-label">数量: <span id="uuid-count-val">5</span></span>
                    <input type="range" id="uuid-count" min="1" max="20" value="5" oninput="document.getElementById('uuid-count-val').innerText=this.value">
                </div>
                <div class="control-group checkbox-group">
                    <label><input type="checkbox" id="uuid-hyp" checked> 连字符 (-)</label>
                    <label><input type="checkbox" id="uuid-up"> 大写 (A-Z)</label>
                </div>
            </div>
            <button class="action-btn" onclick="generateUuid()">🎲 生成 UUID</button>
            <div class="textarea-wrapper" style="margin-top:15px; height:200px">
                <textarea id="uuid-result" readonly style="font-family:monospace; resize:none;"></textarea>
                <button class="copy-btn" onclick="copyText('uuid-result', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button>
            </div>
        </div>

        <div id="yaml" class="tool-panel">
            <h2>⚙️ YAML 转 TOML</h2>
            <div class="converter-container">
                <div class="converter-box"><div class="converter-label">YAML 输入</div><div class="textarea-wrapper"><textarea id="yaml-input" class="code-editor" placeholder="name: Demo"></textarea></div></div>
                <div class="converter-box"><div class="converter-label">TOML 输出</div><div class="textarea-wrapper"><textarea id="toml-output" class="code-editor" readonly></textarea><button class="copy-btn" onclick="copyText('toml-output', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div></div>
            </div>
            <button class="action-btn" style="margin-top: 15px; width: 100%;" onclick="convertYaml()">🔄 转换</button>
            <div id="yaml-error" class="error-msg"></div>
        </div>

        <div id="toml2yaml" class="tool-panel">
            <h2>⚙️ TOML 转 YAML</h2>
            <div class="converter-container">
                <div class="converter-box"><div class="converter-label">TOML 输入</div><div class="textarea-wrapper"><textarea id="toml-input" class="code-editor" placeholder="[package]&#10;name='demo'"></textarea></div></div>
                <div class="converter-box"><div class="converter-label">YAML 输出</div><div class="textarea-wrapper"><textarea id="yaml-output" class="code-editor" readonly></textarea><button class="copy-btn" onclick="copyText('yaml-output', this)"><svg class="icon"><use href="#icon-copy-svg"></use></svg></button></div></div>
            </div>
            <button class="action-btn" style="margin-top: 15px; width: 100%;" onclick="convertToml()">🔄 转换</button>
            <div id="toml-error" class="error-msg"></div>
        </div>

    </main>

    <script>
        function toggleGroup(header) { header.parentNode.classList.toggle('collapsed'); }
        function switchTool(id, el) { document.querySelectorAll('.tool-panel').forEach(p => p.classList.remove('active')); document.getElementById(id).classList.add('active'); document.querySelectorAll('.menu-link').forEach(l => l.classList.remove('active')); el.classList.add('active'); }
        
        // --- Toast Logic ---
        function showToast(msg, type = 'success') {
            const c = document.getElementById('toast-container');
            const t = document.createElement('div'); t.className = `toast ${type}`;
            t.innerHTML = `<span style="font-size:18px">${type==='error'?'❌':'✅'}</span><div>${msg}</div>`;
            c.appendChild(t);
            setTimeout(() => { t.style.opacity='0'; t.style.transform='translateX(100%)'; setTimeout(()=>t.remove(), 300); }, 3000);
        }

        // --- Copy Logic (Works for Div and Textarea) ---
        function copyText(elementId, btnElement) { 
            let text = ""; 
            const el = document.getElementById(elementId); 
            if(el.tagName === 'TEXTAREA') text = el.value; else text = el.innerText; 
            if (!text || text.includes('...') || text === '结果...') { showToast('没有内容可复制', 'error'); return; }
            
            navigator.clipboard.writeText(text).then(() => { 
                showToast('已复制到剪贴板');
            }).catch(e => showToast('复制失败: ' + e, 'error')); 
        }

        // --- Fetch Helper ---
        async function post(url, data) {
            try { 
                const r = await fetch(url, { method: 'POST', headers: {'Content-Type':'application/json'}, body: JSON.stringify(data)});
                if(!r.ok) throw await r.text();
                return await r.json();
            } catch(e) {
                showToast(e, 'error');
                throw e;
            }
        }

        // --- Subnet ---
        async function calcSubnet() {
            try { const d = await post('/api/subnet', {ip: document.getElementById('sn-ip').value, cidr: parseInt(document.getElementById('sn-cidr').value)});
            document.getElementById('sn-res-net').innerText = d.network; document.getElementById('sn-res-broad').innerText = d.broadcast;
            document.getElementById('sn-res-cidr').innerText = d.cidr; document.getElementById('sn-res-mask').innerText = d.mask;
            document.getElementById('sn-res-hosts').innerText = d.usable_hosts;
            document.getElementById('sn-res-first').innerText = d.first_ip; document.getElementById('sn-res-last').innerText = d.last_ip;
            document.getElementById('sn-res-bin-ip').innerText = d.binary_ip; showToast('计算成功'); } catch(e){}
        }

        // --- Chmod ---
        function upChmod(isCheck) {
            let u = (document.getElementById('c_ur').checked?4:0)+(document.getElementById('c_uw').checked?2:0)+(document.getElementById('c_ux').checked?1:0);
            let g = (document.getElementById('c_gr').checked?4:0)+(document.getElementById('c_gw').checked?2:0)+(document.getElementById('c_gx').checked?1:0);
            let o = (document.getElementById('c_or').checked?4:0)+(document.getElementById('c_ow').checked?2:0)+(document.getElementById('c_ox').checked?1:0);
            if(isCheck) document.getElementById('chmod-octal').value = ""+u+g+o;
            else {
                let v = document.getElementById('chmod-octal').value;
                if(v.length===3) {
                    let n=v.split('').map(Number);
                    if(n.every(x=>x>=0&&x<=7)){ u=n[0];g=n[1];o=n[2];
                    document.getElementById('c_ur').checked=u&4; document.getElementById('c_uw').checked=u&2; document.getElementById('c_ux').checked=u&1;
                    document.getElementById('c_gr').checked=g&4; document.getElementById('c_gw').checked=g&2; document.getElementById('c_gx').checked=g&1;
                    document.getElementById('c_or').checked=o&4; document.getElementById('c_ow').checked=o&2; document.getElementById('c_ox').checked=o&1;
                    }
                }
            }
            fetchChmod(document.getElementById('chmod-octal').value);
        }
        async function fetchChmod(octal) { try{let d=await post('/api/chmod',{octal}); if(d.valid){document.getElementById('chmod-symbolic').innerText=d.symbolic;document.getElementById('chmod-command').innerText=d.command;}}catch(e){} }

        // --- Others ---
        async function genPwd(){try{const d=await post('/api/password',{length:parseInt(document.getElementById('pwd-len').value),uppercase:document.getElementById('pwd-u').checked,lowercase:document.getElementById('pwd-l').checked,numbers:document.getElementById('pwd-n').checked,symbols:document.getElementById('pwd-s').checked});document.getElementById('pwd-result').innerText=d.password;showToast('密码生成成功');}catch(e){}}
        async function calculateMd5(){let v=document.getElementById('hash-input').value;if(!v)return;try{let d=await post('/api/md5',{text:v});document.getElementById('md5-32-lower').innerText=d.md5_32_lower;document.getElementById('md5-32-upper').innerText=d.md5_32_upper;document.getElementById('md5-16-lower').innerText=d.md5_16_lower;document.getElementById('md5-16-upper').innerText=d.md5_16_upper;showToast('MD5 计算完成');}catch(e){}}
        async function doJsEnc(){let v=document.getElementById('js-input').value;if(!v)return;try{let d=await post('/api/js-enc',{js:v});document.getElementById('js-output').value=d.result;showToast('混淆完成');}catch(e){}}
        async function generateQR(){let v=document.getElementById('qr-text').value;if(!v)return showToast('请输入内容','error');try{let d=await post('/api/qrcode',{text:v});
            const img=new Image();img.src='data:image/svg+xml;base64,'+btoa(unescape(encodeURIComponent(d.svg)));
            img.onload=function(){const c=document.getElementById('qr-canvas');const x=c.getContext('2d');x.clearRect(0,0,300,300);x.drawImage(img,0,0,300,300);if(uploadedLogo){const s=60,p=(300-s)/2;x.fillStyle='#fff';x.fillRect(p-2,p-2,s+4,s+4);x.drawImage(uploadedLogo,p,p,s,s);} 
            const dl=document.getElementById('qr-download');dl.href=c.toDataURL();dl.style.display='block';showToast('二维码生成成功');
        };}catch(e){}}
        
        // --- JWT Parser ---
        async function parseJwt() {
            let v = document.getElementById('jwt-input').value;
            if(!v) return showToast('请输入 JWT', 'error');
            try {
                let d = await post('/api/jwt', {token: v});
                if(d.error) {
                    showToast(d.error, 'error');
                } else {
                    document.getElementById('jwt-header').value = d.header;
                    document.getElementById('jwt-payload').value = d.payload;
                    showToast('解析成功');
                }
            } catch(e) {}
        }

        // --- Base64/JSON/URL/Token/UUID ---
        async function convertBase64(a){let v=document.getElementById('base64-input').value;if(!v)return;try{let d=await post('/api/base64',{text:v,action:a});document.getElementById('base64-result').innerText=d.result;showToast('转换成功');}catch(e){}}
        async function processJson(m){let v=document.getElementById('json-input').value;if(!v)return;try{let d=await post('/api/json',{input:v});document.getElementById('json-output').value=m==='min'?d.minified:d.pretty;if(d.error)showToast(d.error,'error');else showToast('操作成功');}catch(e){}}
        async function processUrl(m){let v=document.getElementById('url-input').value;if(!v)return;try{let d=await post('/api/url',{input:v});document.getElementById('url-output').innerText=m==='enc'?d.encoded:d.decoded;document.getElementById('url-proto').innerText=d.protocol;document.getElementById('url-host').innerText=d.host;document.getElementById('url-path').innerText=d.path;
            let t=document.getElementById('url-params-body');t.innerHTML="";if(d.params.length){d.params.forEach(p=>t.innerHTML+=`<tr><td>${p[0]}</td><td>${p[1]}</td></tr>`)}else{t.innerHTML="<tr><td colspan='2' style='color:#ccc'>无参数</td></tr>"}
            showToast('处理成功');}catch(e){}}
        async function genToken(){try{let d=await post('/api/token',{length:parseInt(document.getElementById('token-len').value),uppercase:document.getElementById('tok-u').checked,lowercase:document.getElementById('tok-l').checked,numbers:document.getElementById('tok-n').checked,symbols:document.getElementById('tok-s').checked});document.getElementById('token-result').innerText=d.token;showToast('生成成功');}catch(e){}}
        async function generateUuid(){try{let d=await post('/api/uuid',{count:parseInt(document.getElementById('uuid-count').value),hyphens:document.getElementById('uuid-hyp').checked,uppercase:document.getElementById('uuid-up').checked});document.getElementById('uuid-result').value=d.uuids.join('\\n');showToast('生成成功');}catch(e){}}
        
        // --- Date/Color/Yaml ---
        function fillNow(){document.getElementById('date-input').value=Math.floor(Date.now()/1000);convertDate()}
        async function convertDate(){let v=document.getElementById('date-input').value;if(!v)return;try{let d=await post('/api/date',{input:v});document.getElementById('date-sec').innerText=d.unix_sec;document.getElementById('date-milli').innerText=d.unix_milli;document.getElementById('date-utc').innerText=d.human_utc;document.getElementById('date-iso').innerText=d.iso_8601;document.getElementById('date-local').innerText=new Date(d.unix_milli).toLocaleString();showToast('转换成功');}catch(e){}}
        function pickColor(){document.getElementById('color-input').value=document.getElementById('color-picker').value;convertColor()}
        function syncColorPicker(){let v=document.getElementById('color-input').value;if(v.length===7&&v.startsWith('#'))document.getElementById('color-picker').value=v;}
        async function convertColor(){let v=document.getElementById('color-input').value;if(!v)return;try{let d=await post('/api/color',{input:v});if(d.valid){document.getElementById('color-hex').innerText=d.hex;document.getElementById('color-rgb').innerText=d.rgb;document.getElementById('color-hsl').innerText=d.hsl;document.getElementById('color-cmyk').innerText=d.cmyk;let p=document.getElementById('color-preview');p.style.backgroundColor=d.hex;p.style.color=d.hex>'#888888'?'#000':'#fff';showToast('转换成功');}}catch(e){}}
        async function convertYaml(){let v=document.getElementById('yaml-input').value;if(!v)return;try{let d=await post('/api/yaml-to-toml',{yaml:v});if(d.error)document.getElementById('yaml-error').innerText=d.error;else{document.getElementById('toml-output').value=d.result;showToast('转换成功');}}catch(e){}}
        async function convertToml(){let v=document.getElementById('toml-input').value;if(!v)return;try{let d=await post('/api/toml-to-yaml',{toml:v});if(d.error)document.getElementById('toml-error').innerText=d.error;else{document.getElementById('yaml-output').value=d.result;showToast('转换成功');}}catch(e){}}

        // --- Init ---
        let uploadedLogo = null;
        function handleLogoUpload(i){if(i.files&&i.files[0]){const r=new FileReader();r.onload=function(e){uploadedLogo=new Image();uploadedLogo.src=e.target.result;document.getElementById('logo-preview-img').src=e.target.result;document.getElementById('logo-preview-img').style.display='inline-block';};r.readAsDataURL(i.files[0]);}}
        window.onload = function() { fillNow(); upChmod(true); };
    </script>
</body>
</html>
    "####
}