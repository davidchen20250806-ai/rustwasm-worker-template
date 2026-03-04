pub fn get_homepage() -> &'static str {
    r####"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="icon" type="image/svg+xml" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Crect width='100' height='100' rx='20' fill='%230ea5e9'/%3E%3Ctext x='50' y='50' dy='.35em' text-anchor='middle' font-size='70'%3E🦀%3C/text%3E%3C/svg%3E">
    <title>Rust 极客工具箱 - 免费在线开发者工具集合 (SQL/Cron/JSON/正则)</title>
    <meta name="description" content="基于 Rust 构建的高性能在线开发者工具箱。提供 SQL 格式化、Cron 表达式生成、子网掩码计算、文本对比、正则测试、JSON 格式化、Base64 编解码、UUID 生成等多种实用工具，无需下载，即开即用。">
    <meta name="keywords" content="Rust, 开发者工具, 在线工具, SQL格式化, Cron表达式, 子网计算, 正则表达式生成, JSON格式化, Base64, UUID生成, 程序员工具箱">
    <meta name="robots" content="index, follow">
    
    <!-- Open Graph / Social Sharing -->
    <meta property="og:type" content="website">
    <meta property="og:title" content="Rust 极客工具箱 - 免费在线开发者工具集合">
    <meta property="og:description" content="基于 Rust 构建的高性能在线开发者工具箱。提供 SQL 格式化、Cron 表达式生成、子网掩码计算、文本对比、正则测试、JSON 格式化等多种实用工具。">
    <meta property="twitter:card" content="summary">

    <style>
        :root {
            --primary: #0ea5e9; --primary-hover: #0284c7;
            --bg: #f8fafc; --sidebar-bg: #1e293b; --card-bg: #ffffff;
            --text: #334155; --text-muted: #64748b; --border: #e2e8f0;
            --input-bg: #ffffff; --res-bg: #f1f5f9;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; outline: none; }
        body { font-family: -apple-system, system-ui, sans-serif; display: flex; height: 100vh; background: var(--bg); color: var(--text); }
        
        /* 布局 */
        .sidebar { width: 260px; background: var(--sidebar-bg); color: #e2e8f0; display: flex; flex-direction: column; border-right: 1px solid #334155; flex-shrink: 0; }
        .logo { padding: 20px; font-weight: bold; font-size: 18px; color: var(--primary); border-bottom: 1px solid rgba(255,255,255,0.1); }
        .menu { flex: 1; overflow-y: auto; padding: 10px; scrollbar-width: thin; scrollbar-color: #475569 transparent; }
        .menu::-webkit-scrollbar { width: 6px; }
        .menu::-webkit-scrollbar-track { background: transparent; }
        .menu::-webkit-scrollbar-thumb { background-color: #475569; border-radius: 3px; }
        
        .menu-group { margin-bottom: 5px; }
        .menu-cat { 
            font-size: 12px; color: #94a3b8; padding: 10px 12px; font-weight: bold; 
            text-transform: uppercase; cursor: pointer; display: flex; justify-content: space-between; 
            user-select: none; border-radius: 4px; transition: background 0.2s;
        }
        .menu-cat:hover { background: rgba(255,255,255,0.05); color: #fff; }
        .menu-arrow { font-size: 10px; transition: transform 0.2s; }
        .menu-group.collapsed .menu-list { display: none; }
        .menu-group.collapsed .menu-arrow { transform: rotate(-90deg); }
        
        .menu-list { list-style: none; padding: 0; margin-top: 2px; }
        .link { display: flex; align-items: center; padding: 8px 12px; color: #cbd5e1; text-decoration: none; border-radius: 6px; cursor: pointer; margin-bottom: 1px; font-size: 13px; transition: all 0.15s; }
        .link:hover { background: rgba(255,255,255,0.05); color: white; }
        .link.active { background: var(--primary); color: white; font-weight: 500; }
        .icon { margin-right: 10px; width: 16px; text-align: center; }

        /* 全局样式改进 */
        .main { flex: 1; padding: 40px; overflow-y: auto; background: linear-gradient(135deg, #f5f7fa 0%, #e4e7eb 100%); }
        .panel { display: none; max-width: 1200px; margin: 0 auto; background: white; padding: 30px; border-radius: 16px; box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1); border: 1px solid #e5e7eb; }
        .panel.active { display: block; animation: fade 0.3s ease-out; }
        @keyframes fade { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }

        /* 标题样式 */
        h2 { margin-bottom: 25px; padding-bottom: 15px; border-bottom: 2px solid #f0f0f0; font-size: 24px; color: #1f2937; font-weight: 600; font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif; }

        /* 组件样式 */
        .row { display: flex; gap: 20px; margin-bottom: 20px; align-items: center; flex-wrap: wrap; }
        input, select, textarea { 
            width: 100%; 
            padding: 14px 16px; 
            border: 2px solid #e5e7eb; 
            border-radius: 10px; 
            font-size: 14px; 
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif; 
            background: white; 
            color: #1f2937; 
            transition: all 0.3s ease; 
            box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.05); 
        }
        input:focus, select:focus, textarea:focus { 
            border-color: #3b82f6; 
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1); 
            outline: none; 
        }
        
        /* 按钮样式 */
        .btn { 
            background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%); 
            color: white; 
            border: none; 
            padding: 12px 24px; 
            border-radius: 10px; 
            cursor: pointer; 
            font-size: 14px; 
            font-weight: 600; 
            white-space: nowrap; 
            flex-shrink: 0; 
            display: inline-flex; 
            align-items: center; 
            justify-content: center; 
            gap: 8px; 
            text-decoration: none; 
            transition: all 0.3s ease; 
            box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3); 
        }
        .btn:hover { 
            background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%); 
            box-shadow: 0 6px 16px rgba(59, 130, 246, 0.4); 
            transform: translateY(-2px); 
        }
        .btn.secondary { 
            background: linear-gradient(135deg, #64748b 0%, #475569 100%); 
            box-shadow: 0 4px 12px rgba(100, 116, 139, 0.3); 
        }
        .btn.secondary:hover { 
            background: linear-gradient(135deg, #475569 0%, #334155 100%); 
            box-shadow: 0 6px 16px rgba(100, 116, 139, 0.4); 
        }
        .btn.success { 
            background: linear-gradient(135deg, #10b981 0%, #059669 100%); 
            box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3); 
        }
        .btn.success:hover { 
            background: linear-gradient(135deg, #059669 0%, #047857 100%); 
            box-shadow: 0 6px 16px rgba(16, 185, 129, 0.4); 
        }
        
        /* 图标按钮样式 */
        .icon-btn { 
            background: transparent; 
            border: 2px solid #e5e7eb; 
            border-radius: 8px; 
            padding: 8px; 
            cursor: pointer; 
            color: #6b7280; 
            display: flex; 
            align-items: center; 
            justify-content: center; 
            transition: all 0.3s ease;
        }
        .icon-btn:hover { 
            background: #3b82f6; 
            color: white; 
            border-color: #3b82f6; 
            transform: scale(1.1); 
        }
        .icon-btn svg { 
            width: 16px; 
            height: 16px; 
            fill: none; 
            stroke: currentColor; 
            stroke-width: 2; 
            stroke-linecap: round; 
            stroke-linejoin: round; 
        }

        /* 布局网格 */
        .info-grid-2 { 
            display: grid; 
            grid-template-columns: 1fr 1fr; 
            gap: 15px; 
            margin-bottom: 15px; 
        }
        .info-item { 
            display: flex; 
            align-items: center; 
            background: #f9fafb; 
            padding: 15px 20px; 
            border-radius: 10px; 
            border: 1px solid #e5e7eb; 
            position: relative; 
            min-height: 50px; 
            transition: all 0.3s ease; 
        }
        .info-item:hover { 
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1); 
            transform: translateY(-2px); 
        }
        .info-label { 
            width: 100px; 
            font-size: 13px; 
            font-weight: 600; 
            color: #6b7280; 
            flex-shrink: 0; 
        }
        .info-val { 
            flex: 1; 
            font-family: 'Fira Code', 'Courier New', monospace; 
            font-size: 14px; 
            color: #1f2937; 
            padding-right: 30px; 
            word-break: break-all; 
        }
        .info-item .icon-btn { 
            position: absolute; 
            right: 10px; 
            top: 50%; 
            transform: translateY(-50%); 
            border: none; 
            background: transparent; 
        }
        
        /* 网格布局 */
        .grid-4 { 
            display: grid; 
            grid-template-columns: 1fr 1fr; 
            gap: 20px; 
        }
        .grid-5 { 
            display: grid; 
            grid-template-columns: repeat(5, 1fr); 
            gap: 15px; 
            margin-bottom: 20px; 
        }
        .cron-label { 
            font-size: 13px; 
            color: #6b7280; 
            margin-bottom: 8px; 
            font-weight: 600; 
            white-space: nowrap; 
        }

        /* 编辑器样式 */
        .editor-container { 
            display: grid; 
            grid-template-columns: 1fr 1fr; 
            gap: 25px; 
            height: 400px; 
            margin-bottom: 20px; 
        }
        .editor-box { 
            display: flex; 
            flex-direction: column; 
            border: 2px solid #e5e7eb; 
            border-radius: 12px; 
            overflow: hidden; 
            background: white; 
            margin-bottom: 20px; 
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05); 
            transition: all 0.3s ease; 
        }
        .editor-box:hover { 
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1); 
        }
        .editor-header { 
            background: linear-gradient(135deg, #f9fafb 0%, #f3f4f6 100%); 
            padding: 12px 16px; 
            border-bottom: 1px solid #e5e7eb; 
            font-size: 14px; 
            font-weight: 600; 
            color: #374151; 
            display: flex; 
            justify-content: space-between; 
            align-items: center; 
            height: 48px; 
        }
        .editor-content { 
            flex: 1; 
            border: none; 
            padding: 16px; 
            resize: none; 
            outline: none; 
            margin: 0; 
            border-radius: 0; 
            font-family: 'Fira Code', 'Courier New', monospace; 
            font-size: 14px; 
            line-height: 1.5; 
            color: #1f2937; 
            background: white; 
        }
        
        /* 结果卡片 */
        .result-card { 
            background: #f9fafb; 
            border: 1px solid #e5e7eb; 
            border-radius: 10px; 
            padding: 15px; 
            position: relative; 
            transition: all 0.3s ease; 
        }
        .result-card:hover { 
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1); 
            transform: translateY(-2px); 
        }
        .result-label { 
            font-size: 12px; 
            color: #6b7280; 
            font-weight: 600; 
            margin-bottom: 8px; 
            text-transform: uppercase; 
            letter-spacing: 0.5px; 
        }
        .result-val { 
            font-family: 'Fira Code', 'Courier New', monospace; 
            font-size: 14px; 
            color: #1f2937; 
            word-break: break-all; 
            padding-right: 30px; 
            min-height: 20px; 
        }
        .result-card .icon-btn { 
            position: absolute; 
            right: 10px; 
            top: 50%; 
            transform: translateY(-50%); 
            border: none; 
        }

        /* 提示框样式 */
        #toast { 
            position: fixed; 
            top: 30px; 
            left: 50%; 
            transform: translateX(-50%); 
            background: linear-gradient(135deg, #1f2937 0%, #111827 100%); 
            color: white; 
            padding: 12px 24px; 
            border-radius: 25px; 
            font-size: 14px; 
            font-weight: 500; 
            opacity: 0; 
            pointer-events: none; 
            transition: all 0.3s ease; 
            z-index: 9999; 
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3); 
        }

        /* 响应式设计 */
        @media (max-width: 768px) {
            .main { padding: 20px; }
            .panel { padding: 20px; }
            .editor-container { 
                grid-template-columns: 1fr; 
                height: 600px; 
            }
            .info-grid-2 { 
                grid-template-columns: 1fr; 
            }
            .grid-4 { 
                grid-template-columns: 1fr; 
            }
            .grid-5 { 
                grid-template-columns: repeat(2, 1fr); 
            }
        }

        /* Dockerfile Stage Styles */
        .stage { border: 2px solid var(--border); padding: 20px; margin-bottom: 20px; border-radius: 12px; background: white; position: relative; transition: all 0.3s ease; }
        .stage:hover { border-color: #cbd5e1; box-shadow: 0 4px 12px rgba(0,0,0,0.05); }
        .stage-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; padding-bottom: 15px; border-bottom: 1px solid var(--res-bg); }
        .stage-num { font-weight: 700; color: var(--primary); font-size: 16px; display: flex; align-items: center; gap: 8px; }
        .stage-num::before { content: ''; display: block; width: 4px; height: 16px; background: var(--primary); border-radius: 2px; }
    </style>
    <svg style="display:none">
        <symbol id="i-copy" viewBox="0 0 24 24"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></symbol>
        <symbol id="i-trash" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></symbol>
        <symbol id="i-upload" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></symbol>
        <symbol id="i-download" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></symbol>
    </svg>
</head>
<body>
    <div id="toast">已复制</div>
    <aside class="sidebar">
        <div class="logo">🦀 Rust 工具箱</div>
        <div style="padding: 15px 10px 5px;">
            <input type="text" id="menu-search" placeholder="🔍 搜索工具..." oninput="filterMenu()" style="background:rgba(255,255,255,0.1); border:1px solid rgba(255,255,255,0.1); color:white; padding:10px 12px; font-size:13px; width:100%; box-shadow:none;">
        </div>
        <div class="menu">
            <div class="menu-group">
                <div class="menu-cat" onclick="toggleGroup(this)"><span>开发 & 运维</span><span class="menu-arrow">▼</span></div>
                <ul class="menu-list">
                    <li><a class="link" onclick="nav('whoami', this)"><span class="icon">🆔</span>IP & 请求信息</a></li>
                    <li><a class="link active" onclick="nav('sql', this)"><span class="icon">🗄️</span>SQL 格式化</a></li>
                    <li><a class="link" onclick="nav('cron', this)"><span class="icon">⏰</span>Cron 生成</a></li>
                    <li><a class="link" onclick="nav('subnet', this)"><span class="icon">🌐</span>网络子网计算</a></li>
                    <li><a class="link" onclick="nav('diff', this)"><span class="icon">⚖️</span>文本对比</a></li>
                    <li><a class="link" onclick="nav('regex', this)"><span class="icon">🔍</span>正则表达式生成</a></li>
                    <li><a class="link" onclick="nav('dockerfile', this)"><span class="icon">🐳</span>Dockerfile 生成</a></li>
                    <li><a class="link" onclick="nav('k8s', this)"><span class="icon">☸️</span>K8s YAML 生成</a></li>
                    <li><a class="link" onclick="nav('ansible', this)"><span class="icon">🅰️</span>Ansible YAML 生成</a></li>
                    <li><a class="link" onclick="nav('nginx', this)"><span class="icon">🔧</span>Nginx 配置</a></li>
                    <li><a class="link" onclick="nav('curl', this)"><span class="icon">🔌</span>cURL 生成器</a></li>
                </ul>
            </div>
            <div class="menu-group">
                <div class="menu-cat" onclick="toggleGroup(this)"><span>Linux 命令</span><span class="menu-arrow">▼</span></div>
                <ul class="menu-list">
                    <li><a class="link" onclick="nav('k8s-cmd', this)"><span class="icon">⚓</span>K8s 常用命令</a></li>
                    <li><a class="link" onclick="nav('ls', this)"><span class="icon">📂</span>列出文件 (Ls)</a></li>
                    <li><a class="link" onclick="nav('rsync', this)"><span class="icon">🔄</span>文件同步 (Rsync)</a></li>
                    <li><a class="link" onclick="nav('git', this)"><span class="icon">🎋</span>Git 命令</a></li>
                    <li><a class="link" onclick="nav('chmod', this)"><span class="icon">🐧</span>权限 (Chmod)</a></li>
                    <li><a class="link" onclick="nav('tar', this)"><span class="icon">📦</span>压缩 (Tar)</a></li>
                    <li><a class="link" onclick="nav('ps', this)"><span class="icon">📊</span>进程 (Ps)</a></li>
                    <li><a class="link" onclick="nav('tcpdump', this)"><span class="icon">📡</span>抓包 (Tcpdump)</a></li>
                    <li><a class="link" onclick="nav('strace', this)"><span class="icon">🔬</span>系统调用 (Strace)</a></li>
                    <li><a class="link" onclick="nav('iostat', this)"><span class="icon">📈</span>磁盘 I/O (Iostat)</a></li>
                    <li><a class="link" onclick="nav('nice', this)"><span class="icon">⚡</span>进程优先级 (Nice)</a></li>
                    <li><a class="link" onclick="nav('firewall', this)"><span class="icon">🔥</span>防火墙 (Firewall)</a></li>
                    <li><a class="link" onclick="nav('systemctl', this)"><span class="icon">⚙️</span>服务管理 (Systemctl)</a></li>
                    <li><a class="link" onclick="nav('find', this)"><span class="icon">🔍</span>查找文件 (Find)</a></li>
                    <li><a class="link" onclick="nav('awk', this)"><span class="icon">🦅</span>文本处理 (Awk)</a></li>
                    <li><a class="link" onclick="nav('sed', this)"><span class="icon">✂️</span>流编辑 (Sed)</a></li>
                </ul>
            </div>
            <div class="menu-group">
                <div class="menu-cat" onclick="toggleGroup(this)"><span>文本处理</span><span class="menu-arrow">▼</span></div>
                <ul class="menu-list">
                    <li><a class="link" onclick="nav('fake-user', this)"><span class="icon">👤</span>虚拟身份生成</a></li>
                    <li><a class="link" onclick="nav('lorem', this)"><span class="icon">📝</span>Lorem Ipsum</a></li>
                    <li><a class="link" onclick="nav('cc', this)"><span class="icon">💳</span>信用卡生成</a></li>
                    <li><a class="link" onclick="nav('case', this)"><span class="icon">Aa</span>变量命名转换</a></li>
                    <li><a class="link" onclick="nav('escape', this)"><span class="icon">🔣</span>文本转义</a></li>
                    <li><a class="link" onclick="nav('json', this)"><span class="icon">📋</span>JSON 工具</a></li>
                    <li><a class="link" onclick="nav('base64', this)"><span class="icon">📦</span>Base64 转换</a></li>
                    <li><a class="link" onclick="nav('url', this)"><span class="icon">🔗</span>URL 编解码</a></li>
                    <li><a class="link" onclick="nav('url-parser', this)"><span class="icon">🧩</span>URL 解析器</a></li>
                    <li><a class="link" onclick="nav('yaml', this)"><span class="icon">⚙️</span>YAML 转 TOML</a></li>
                    <li><a class="link" onclick="nav('toml2yaml', this)"><span class="icon">⚙️</span>TOML 转 YAML</a></li>
                </ul>
            </div>
            <div class="menu-group">
                <div class="menu-cat" onclick="toggleGroup(this)"><span>安全 & 加密</span><span class="menu-arrow">▼</span></div>
                <ul class="menu-list">
                    <li><a class="link" onclick="nav('hash', this)"><span class="icon">#️⃣</span>哈希 (MD5)</a></li>
                    <li><a class="link" onclick="nav('jwt', this)"><span class="icon">🛡️</span>JWT 解析</a></li>
                    <li><a class="link" onclick="nav('password', this)"><span class="icon">🔑</span>密码生成</a></li>
                    <li><a class="link" onclick="nav('token', this)"><span class="icon">🎟️</span>Token 生成</a></li>
                    <li><a class="link" onclick="nav('uuid', this)"><span class="icon">🆔</span>UUID 生成</a></li>
                    <li><a class="link" onclick="nav('ssh-key', this)"><span class="icon">🗝️</span>SSH 密钥生成</a></li>
                    <li><a class="link" onclick="nav('jsenc', this)"><span class="icon">🔒</span>JS 混淆</a></li>
                </ul>
            </div>
            <div class="menu-group">
                <div class="menu-cat" onclick="toggleGroup(this)"><span>视觉 & 其他</span><span class="menu-arrow">▼</span></div>
                <ul class="menu-list">
                    <li><a class="link" onclick="nav('date', this)"><span class="icon">📅</span>时间转换</a></li>
                    <li><a class="link" onclick="nav('color', this)"><span class="icon">🎨</span>颜色转换</a></li>
                    <li><a class="link" onclick="nav('qr', this)"><span class="icon">📱</span>二维码生成</a></li>
                    <li><a class="link" onclick="nav('unit', this)"><span class="icon">⚖️</span>单位换算</a></li>
                </ul>
            </div>
            <div class="menu-group">
                <div class="menu-cat" onclick="toggleGroup(this)"><span>关于</span><span class="menu-arrow">▼</span></div>
                <ul class="menu-list">
                    <li><a class="link" onclick="nav('disclaimer', this)"><span class="icon">📜</span>免责声明</a></li>
                </ul>
            </div>
        </div>
    </aside>

    <main class="main">
        <div id="whoami" class="panel">
            <h2>IP & 请求信息 (Whoami)</h2>
            <div class="info-grid-2">
                <div class="info-item"><span class="info-label">IP 地址</span><span id="wa-ip" class="info-val">-</span><button class="icon-btn" onclick="copy('wa-ip')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">国家/地区</span><span id="wa-country" class="info-val">-</span></div>
                <div class="info-item"><span class="info-label">城市</span><span id="wa-city" class="info-val">-</span></div>
                <div class="info-item"><span class="info-label">Ray ID</span><span id="wa-asn" class="info-val">-</span></div>
                <div class="info-item" style="grid-column: span 2"><span class="info-label">User Agent</span><span id="wa-ua" class="info-val" style="font-size:12px">-</span><button class="icon-btn" onclick="copy('wa-ua')"><svg><use href="#i-copy"></use></svg></button></div>
            </div>
            <div style="font-size:12px; color:#64748b; margin-bottom:8px; font-weight:bold; margin-top:20px;">所有请求头 (Headers)</div>
            <div class="editor-box" style="height:300px">
                <textarea id="wa-headers" class="editor-content" readonly style="font-size:12px"></textarea>
            </div>
            <button class="btn" style="width:100%" onclick="doWhoami()">🔄 刷新信息</button>
        </div>

        <div id="lorem" class="panel">
            <h2>Lorem Ipsum 生成</h2>
            <div class="row">
                <input type="number" id="li-count" value="3" style="width:80px" min="1" max="100">
                <select id="li-mode" style="flex:1"><option value="paragraphs">段落 (Paragraphs)</option><option value="sentences">句子 (Sentences)</option><option value="words">单词 (Words)</option></select>
                <button class="btn" onclick="doLorem()">生成</button>
            </div>
            <div class="editor-box" style="height:400px"><div class="editor-header"><span>结果</span><button class="icon-btn" onclick="copy('li-res')"><svg><use href="#i-copy"></use></svg></button></div><textarea id="li-res" class="editor-content" readonly></textarea></div>
        </div>

        <div id="fake-user" class="panel">
            <h2>虚拟身份生成 (Fake User)</h2>
            <div class="row">
                <span>生成数量:</span>
                <input type="number" id="fu-count" value="5" style="width:80px" min="1" max="50">
                <select id="fu-locale" style="width:120px">
                    <option value="en">English (US)</option>
                    <option value="cn">中文 (China)</option>
                </select>
                <button class="btn" onclick="doFakeUser()">🎲 生成数据</button>
            </div>
            <div class="editor-box" style="height:400px"><div class="editor-header"><span>结果 (JSON)</span><button class="icon-btn" onclick="copy('fu-res')"><svg><use href="#i-copy"></use></svg></button></div><textarea id="fu-res" class="editor-content" readonly></textarea></div>
        </div>

        <div id="cc" class="panel">
            <h2>信用卡生成 (测试用)</h2>
            <div class="row">
                <span>生成数量:</span>
                <input type="number" id="cc-count" value="5" style="width:80px" min="1" max="50">
                <select id="cc-issuer" style="width:120px">
                    <option value="visa">Visa</option>
                    <option value="mastercard">Mastercard</option>
                    <option value="amex">Amex</option>
                    <option value="discover">Discover</option>
                </select>
                <button class="btn" onclick="doCc()">🎲 生成</button>
            </div>
            <div class="editor-box" style="height:400px"><div class="editor-header"><span>结果 (JSON)</span><button class="icon-btn" onclick="copy('cc-res')"><svg><use href="#i-copy"></use></svg></button></div><textarea id="cc-res" class="editor-content" readonly></textarea></div>
        </div>

        <div id="regex" class="panel">
            <h2>正则表达式生成</h2>
            <div class="row">
                <select id="reg-key" style="flex:1">
                    <option value="email">电子邮箱 (Email)</option>
                    <option value="phone_cn">中国手机号 (11位)</option>
                    <option value="id_cn">中国身份证 (18位)</option>
                    <option value="ipv4">IPv4 地址</option>
                    <option value="url">网址 (URL)</option>
                    <option value="date">日期 (YYYY-MM-DD)</option>
                    <option value="password">强密码 (字母+数字)</option>
                    <option value="hex_color">Hex 颜色代码</option>
                    <option value="chinese">中文字符</option>
                    <option value="html_tag">HTML 标签</option>
                </select>
                <button class="btn" onclick="doRegGen()">生成模板</button>
                <button class="btn secondary" onclick="toggleRegBuilder()">🛠️ 自定义构建</button>
            </div>
            <div id="reg-builder" style="display:none; background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0; margin-bottom:15px;">
                <div class="grid-4" style="margin-bottom:10px">
                    <div><div class="cron-label">开头是 (Starts)</div><input id="rb-start" placeholder="abc" oninput="doRegBuild()"></div>
                    <div><div class="cron-label">开头不是 (Not Starts)</div><input id="rb-not-start" placeholder="xyz" oninput="doRegBuild()"></div>
                    <div><div class="cron-label">结尾是 (Ends)</div><input id="rb-end" placeholder="123" oninput="doRegBuild()"></div>
                    <div><div class="cron-label">结尾不是 (Not Ends)</div><input id="rb-not-end" placeholder="tmp" oninput="doRegBuild()"></div>
                </div>
                <div class="grid-4">
                    <div style="grid-column: span 2"><div class="cron-label">包含 (Contains)</div><input id="rb-has" placeholder="必须包含的内容" oninput="doRegBuild()"></div>
                    <div style="grid-column: span 2"><div class="cron-label">不包含 (Not Contains)</div><input id="rb-not-has" placeholder="不能包含的内容" oninput="doRegBuild()"></div>
                </div>
            </div>
            <div style="margin-bottom:15px">
                <div class="cron-label">正则表达式 (Pattern)</div>
                <div style="display:flex; gap:10px;">
                    <input type="text" id="reg-p" placeholder="例如: ^\d+$" style="font-family:monospace; font-weight:bold; color:var(--primary); flex:1;">
                    <button class="icon-btn" onclick="copy('reg-p')" title="复制正则"><svg><use href="#i-copy"></use></svg></button>
                </div>
            </div>
            <div class="editor-container" style="height:300px">
                <div class="editor-box">
                    <div class="editor-header"><span>测试文本</span><button class="icon-btn" onclick="setVal('reg-t','')" title="清空"><svg><use href="#i-trash"></use></svg></button></div>
                    <textarea id="reg-t" class="editor-content" placeholder="在此输入待匹配的文本..."></textarea>
                </div>
                <div class="editor-box">
                    <div class="editor-header"><span>匹配结果</span><span id="reg-count" style="color:var(--primary); font-size:12px"></span></div>
                    <textarea id="reg-r" class="editor-content" readonly></textarea>
                </div>
            </div>
            <button class="btn" style="width:100%" onclick="doReg()">🧪 开始测试</button>
        </div>

        <div id="qr" class="panel">
            <h2>二维码生成</h2>
            <div class="row">
                <input type="text" id="qr-text" placeholder="输入链接或文本..." style="flex:1">
                <label class="btn secondary" style="cursor:pointer;" id="qr-upload-btn">
                    <svg style="width:14px;height:14px;fill:none;stroke:currentColor;stroke-width:2;"><use href="#i-upload"></use></svg> <span id="qr-upload-text">上传 Logo</span>
                    <input type="file" id="qr-logo" accept="image/*" style="display:none" onchange="handleLogo(this)">
                </label>
                <button class="btn" onclick="doQr()">生成</button>
                <a id="qr-dl" class="btn success" style="display:none; text-decoration:none; color:white;">
                    <svg style="width:14px;height:14px;fill:none;stroke:currentColor;stroke-width:2;"><use href="#i-download"></use></svg> 下载图片
                </a>
            </div>
            <div style="display:flex; justify-content:center; padding:20px; margin-top:20px; background:white; border-radius:8px; border:1px dashed #e2e8f0; min-height:200px; align-items:center;">
                <canvas id="qr-canvas"></canvas>
            </div>
        </div>

        <div id="subnet" class="panel">
            <h2>网络子网计算</h2>
            <div class="row">
                <input id="sn-ip" value="192.168.1.1" style="flex:2">
                <select id="sn-cidr" style="flex:1">
                    <option value="32">/32 (1 IP)</option><option value="30">/30 (4 IPs)</option><option value="29">/29 (8 IPs)</option>
                    <option value="28">/28 (16 IPs)</option><option value="27">/27 (32 IPs)</option><option value="26">/26 (64 IPs)</option>
                    <option value="25">/25 (128 IPs)</option><option value="24" selected>/24 (256 IPs)</option><option value="23">/23 (512 IPs)</option>
                    <option value="22">/22 (1k IPs)</option><option value="20">/20 (4k IPs)</option><option value="16">/16 (65k IPs)</option>
                    <option value="8">/8 (16M IPs)</option>
                </select>
                <button class="btn" onclick="doSubnet()">计算</button>
            </div>
            <div style="font-size:12px; color:#64748b; margin-bottom:8px; font-weight:bold;">基础信息</div>
            <div class="info-grid-2">
                <div class="info-item"><span class="info-label">CIDR</span><span id="sn-cidr-res" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-cidr-res')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">子网掩码</span><span id="sn-mask" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-mask')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">网络地址</span><span id="sn-net" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-net')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">广播地址</span><span id="sn-broad" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-broad')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">主机总数</span><span id="sn-total" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-total')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">可用主机</span><span id="sn-usable" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-usable')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">IP 类别</span><span id="sn-class" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-class')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">IP 类型</span><span id="sn-type" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-type')"><svg><use href="#i-copy"></use></svg></button></div>
            </div>
            <div style="font-size:12px; color:#64748b; margin-bottom:8px; font-weight:bold; margin-top:10px;">详细信息</div>
            <div class="info-grid-2">
                <div class="info-item"><span class="info-label">通配符</span><span id="sn-wild" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-wild')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">起始 IP</span><span id="sn-first" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-first')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">结束 IP</span><span id="sn-last" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-last')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">二进制掩码</span><span id="sn-bin-mask" class="info-val" style="font-size:11px">-</span><button class="icon-btn" onclick="copy('sn-bin-mask')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item" style="grid-column: span 2"><span class="info-label">二进制 IP</span><span id="sn-bin-ip" class="info-val">-</span><button class="icon-btn" onclick="copy('sn-bin-ip')"><svg><use href="#i-copy"></use></svg></button></div>
            </div>
        </div>

        <div id="cron" class="panel">
            <h2>Cron 生成</h2>
            <div class="grid-5">
                <div><div class="cron-label">分钟 (0-59)</div><input id="c-m" value="*" oninput="upCron()"></div>
                <div><div class="cron-label">小时 (0-23)</div><input id="c-h" value="*" oninput="upCron()"></div>
                <div><div class="cron-label">日期 (1-31)</div><input id="c-d" value="*" oninput="upCron()"></div>
                <div><div class="cron-label">月份 (1-12)</div><input id="c-mo" value="*" oninput="upCron()"></div>
                <div><div class="cron-label">星期 (0-6)</div><input id="c-w" value="*" oninput="upCron()"></div>
            </div>
            <div class="row">
                <input id="cron-res" style="color:var(--primary); font-weight:bold; font-size:16px;" readonly>
                <button class="btn" onclick="doCron()">验证 & 预览</button>
            </div>
            <div class="result-card"><div class="result-label">未来执行时间 (UTC)</div><div class="result-val" id="cron-out" style="white-space:pre-line;">...</div></div>
        </div>

        <div id="git" class="panel">
            <h2>Git 命令生成</h2>
            <div class="row">
                <select id="git-cmd" onchange="updateGitUI(); doGit()" style="flex:1; font-weight:bold; color:var(--primary)">
                    <option value="init">初始化 (init)</option>
                    <option value="clone">克隆 (clone)</option>
                    <option value="status">状态 (status)</option>
                    <option value="add">添加文件 (add)</option>
                    <option value="commit" selected>提交 (commit)</option>
                    <option value="push">推送 (push)</option>
                    <option value="pull">拉取 (pull)</option>
                    <option value="checkout">切换/检出 (checkout)</option>
                    <option value="merge">合并 (merge)</option>
                    <option value="log">日志 (log)</option>
                    <option value="reset">重置 (reset)</option>
                    <option value="remote">远程仓库 (remote)</option>
                    <option disabled>--- 速查 (Cheat Sheet) ---</option>
                    <option value="undo_commit">撤销最近提交 (Undo Commit)</option>
                    <option value="undo_changes">撤销工作区修改 (Undo Changes)</option>
                    <option value="log_graph">图形化日志 (Log Graph)</option>
                    <option value="tag">打标签并推送 (Tag & Push)</option>
                    <option value="branch_delete">删除分支 (Delete Branch)</option>
                    <option value="stash">暂存并拉取 (Stash & Pull)</option>
                </select>
            </div>

            <!-- Dynamic Inputs -->
            <div class="grid-4" style="margin-bottom:15px">
                <div id="g-target-box"><div class="cron-label" id="g-target-lbl">目标文件</div><input id="g-target" oninput="doGit()"></div>
                <div id="g-tag-box" style="display:none"><div class="cron-label">标签名 (Tag)</div><input id="g-tag" value="v1.0.0" oninput="doGit()"></div>
                <div id="g-msg-box"><div class="cron-label">提交信息 (Message)</div><input id="g-msg" placeholder="feat: add new feature" oninput="doGit()"></div>
                <div id="g-remote-box"><div class="cron-label">远程仓库 (Remote)</div><input id="g-remote" value="origin" oninput="doGit()"></div>
                <div id="g-branch-box"><div class="cron-label">分支 (Branch)</div><input id="g-branch" value="main" oninput="doGit()"></div>
            </div>

            <!-- Options Grid -->
            <div id="git-opts" style="margin-bottom:20px; display:grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap:15px; padding:15px; background:#f8fafc; border-radius:8px; border:1px solid #e2e8f0;">
                <label id="opt-all" style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="go-all" onchange="doGit()" style="width:18px;height:18px;accent-color:var(--primary)"> 全部 (-A/-a)</label>
                <label id="opt-force" style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="go-force" onchange="doGit()" style="width:18px;height:18px;accent-color:var(--primary)"> 强制 (--force)</label>
                <label id="opt-rebase" style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="go-rebase" onchange="doGit()" style="width:18px;height:18px;accent-color:var(--primary)"> 变基 (--rebase)</label>
                <label id="opt-amend" style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="go-amend" onchange="doGit()" style="width:18px;height:18px;accent-color:var(--primary)"> 追加 (--amend)</label>
                <label id="opt-hard" style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="go-hard" onchange="doGit()" style="width:18px;height:18px;accent-color:var(--primary)"> 强制重置 (--hard)</label>
                <label id="opt-new" style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="go-new" onchange="doGit()" style="width:18px;height:18px;accent-color:var(--primary)"> 新分支 (-b)</label>
                <label id="opt-tags" style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="go-tags" onchange="doGit()" style="width:18px;height:18px;accent-color:var(--primary)"> 推送标签 (--tags)</label>
                <label id="opt-oneline" style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="go-oneline" checked onchange="doGit()" style="width:18px;height:18px;accent-color:var(--primary)"> 单行 (--oneline)</label>
                <label id="opt-graph" style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="go-graph" checked onchange="doGit()" style="width:18px;height:18px;accent-color:var(--primary)"> 图形化 (--graph)</label>
            </div>

            <div class="result-card">
                <div class="result-label">Git Command</div>
                <div id="git-cmd-res" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px; color:var(--primary); font-weight:bold;">git commit -m "..."</div>
                <button class="icon-btn" onclick="copy('git-cmd-res')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="sql" class="panel active">
            <h2>SQL 格式化</h2>
            <div class="editor-container">
                <div class="editor-box">
                    <div class="editor-header"><span>输入</span><button class="icon-btn" onclick="setVal('sql-in','')" title="清空"><svg><use href="#i-trash"></use></svg></button></div>
                    <textarea id="sql-in" class="editor-content" placeholder="SELECT * FROM table..."></textarea>
                </div>
                <div class="editor-box">
                    <div class="editor-header"><span>结果</span><button class="icon-btn" onclick="copy('sql-out')" title="复制"><svg><use href="#i-copy"></use></svg></button></div>
                    <textarea id="sql-out" class="editor-content" readonly></textarea>
                </div>
            </div>
            <button class="btn" style="width:100%" onclick="doSql()">✨ 立即美化</button>
        </div>

        <div id="hash" class="panel">
            <h2>哈希计算 (MD5)</h2>
            <textarea id="md5-in" style="height:80px; margin-bottom:15px;" placeholder="示例文本：Hello, World!\n或输入任何你想要计算MD5的文本"></textarea>
            <button class="btn" style="width:100%; margin-bottom:20px;" onclick="doMd5()">计算</button>
            <div class="grid-4">
                <div class="result-card"><div class="result-label">32位 (小)</div><div class="result-val" id="m32l"></div><button class="icon-btn" onclick="copy('m32l')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="result-card"><div class="result-label">32位 (大)</div><div class="result-val" id="m32u"></div><button class="icon-btn" onclick="copy('m32u')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="result-card"><div class="result-label">16位 (小)</div><div class="result-val" id="m16l"></div><button class="icon-btn" onclick="copy('m16l')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="result-card"><div class="result-label">16位 (大)</div><div class="result-val" id="m16u"></div><button class="icon-btn" onclick="copy('m16u')"><svg><use href="#i-copy"></use></svg></button></div>
            </div>
        </div>

        <div id="color" class="panel">
            <h2>颜色转换</h2>
            <div class="row">
                <input type="color" id="col-p" oninput="document.getElementById('col-i').value=this.value; doCol()" style="width:60px;height:40px;padding:0;border:none;cursor:pointer">
                <input id="col-i" placeholder="#FFFFFF" oninput="doCol()">
                <button class="btn" onclick="doCol()">转换</button>
            </div>
            <div id="color-preview" style="height:60px; border-radius:6px; border:1px solid var(--border); margin-bottom:15px; display:flex; align-items:center; justify-content:center; font-weight:bold; color:#ccc;">PREVIEW</div>
            <div class="info-grid-2">
                <div class="info-item"><span class="info-label">HEX</span><span id="c-hex" class="info-val"></span><button class="icon-btn" onclick="copy('c-hex')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">RGB</span><span id="c-rgb" class="info-val"></span><button class="icon-btn" onclick="copy('c-rgb')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">HSL</span><span id="c-hsl" class="info-val"></span><button class="icon-btn" onclick="copy('c-hsl')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="info-item"><span class="info-label">CMYK</span><span id="c-cmyk" class="info-val"></span><button class="icon-btn" onclick="copy('c-cmyk')"><svg><use href="#i-copy"></use></svg></button></div>
            </div>
        </div>

        <div id="date" class="panel">
            <h2>时间转换</h2>
            <div class="row"><input id="ts-in" placeholder="Timestamp or YYYY-MM-DD..."><button class="btn" style="background:#64748b" onclick="fillTime()">当前</button><button class="btn" onclick="doDate()">转换</button></div>
            <div class="grid-4">
                <div class="result-card"><div class="result-label">Unix (s)</div><div class="result-val" id="ts-s"></div><button class="icon-btn" onclick="copy('ts-s')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="result-card"><div class="result-label">Unix (ms)</div><div class="result-val" id="ts-ms"></div><button class="icon-btn" onclick="copy('ts-ms')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="result-card"><div class="result-label">ISO 8601</div><div class="result-val" id="ts-iso"></div><button class="icon-btn" onclick="copy('ts-iso')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="result-card"><div class="result-label">UTC</div><div class="result-val" id="ts-utc"></div><button class="icon-btn" onclick="copy('ts-utc')"><svg><use href="#i-copy"></use></svg></button></div>
                <div class="result-card" style="grid-column: span 2"><div class="result-label">本地时间</div><div class="result-val" id="ts-loc" style="color:var(--primary);font-weight:bold"></div><button class="icon-btn" onclick="copy('ts-loc')"><svg><use href="#i-copy"></use></svg></button></div>
            </div>
        </div>

        <div id="diff" class="panel">
            <h2>文本对比</h2>
            <div class="editor-container" style="height:300px">
                <div class="editor-box"><div class="editor-header">旧文本</div><textarea id="diff-a" class="editor-content"></textarea></div>
                <div class="editor-box"><div class="editor-header">新文本</div><textarea id="diff-b" class="editor-content"></textarea></div>
            </div>
            <button class="btn" style="width:100%" onclick="doDiff()">🔍 开始对比</button>
            <div id="diff-res" class="result-card" style="margin-top:20px; display:block; min-height:100px; white-space:pre-wrap; font-family:monospace;"></div>
        </div>

        <div id="uuid" class="panel">
            <h2>UUID 生成器</h2>
            <div class="row">
                <span>生成数量:</span>
                <input type="number" id="uid-n" value="5" style="width:80px">
                <button class="btn" onclick="doUuid()">🎲 随机生成</button>
            </div>
            <div class="editor-box" style="height:300px">
                <div class="editor-header"><span>结果</span><button class="icon-btn" onclick="copy('uid-res')" title="复制"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="uid-res" class="editor-content" readonly></textarea>
            </div>
        </div>

        <div id="ssh-key" class="panel">
            <h2>SSH 密钥生成 (RSA)</h2>
            <div class="row">
                <select id="ssh-algo" style="flex:1"><option value="2048">RSA 2048-bit</option><option value="4096">RSA 4096-bit</option></select>
                <input id="ssh-comment" placeholder="user@host (注释)" value="user@example.com" style="flex:1">
                <button class="btn" onclick="doSshKey()">⚙️ 生成密钥对</button>
            </div>
            <div class="editor-container" style="height:450px">
                <div class="editor-box">
                    <div class="editor-header"><span>私钥 (Private Key - PEM)</span><button class="icon-btn" onclick="copy('ssh-priv')"><svg><use href="#i-copy"></use></svg></button></div>
                    <textarea id="ssh-priv" class="editor-content" readonly style="font-size:12px"></textarea>
                </div>
                <div class="editor-box">
                    <div class="editor-header"><span>公钥 (Public Key - OpenSSH)</span><button class="icon-btn" onclick="copy('ssh-pub')"><svg><use href="#i-copy"></use></svg></button></div>
                    <textarea id="ssh-pub" class="editor-content" readonly style="font-size:12px"></textarea>
                </div>
            </div>
            <div class="row" style="justify-content: center;">
                <button class="btn secondary" onclick="downloadSshKeys()">💾 下载密钥文件</button>
            </div>
        </div>

        <div id="password" class="panel">
            <h2>密码生成</h2>
            <div class="row">
                <span>长度:</span>
                <input type="number" id="pwd-len" value="16" style="width:80px">
                <button class="btn" onclick="doPwd()">🎲 生成密码</button>
            </div>
            <div class="editor-box" style="height:100px">
                <div class="editor-header"><span>结果</span><button class="icon-btn" onclick="copy('pwd-res')" title="复制"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="pwd-res" class="editor-content" readonly style="font-size:18px;color:var(--primary);text-align:center"></textarea>
            </div>
        </div>

        <div id="token" class="panel">
            <h2>Token 生成</h2>
            <div class="row">
                <span>长度:</span>
                <input type="number" id="tok-len" value="32" style="width:80px">
                <button class="btn" onclick="doToken()">🎲 生成 Token</button>
            </div>
            <div class="editor-box" style="height:100px">
                <div class="editor-header"><span>结果</span><button class="icon-btn" onclick="copy('tok-res')" title="复制"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="tok-res" class="editor-content" readonly></textarea>
            </div>
        </div>

        <div id="jwt" class="panel">
            <h2>JWT 解析</h2>
            <div class="editor-box">
                <div class="editor-header"><span>Token 输入</span><button class="icon-btn" onclick="setVal('jwt-in','')"><svg><use href="#i-trash"></use></svg></button></div>
                <textarea id="jwt-in" class="editor-content" style="height:80px" placeholder="eyJ..."></textarea>
            </div>
            <button class="btn" style="width:100%;margin-bottom:15px;margin-top:15px" onclick="doJwt()">🔍 解析 Token</button>
            <div class="editor-container">
                <div class="editor-box">
                    <div class="editor-header"><span>Header</span><button class="icon-btn" onclick="copy('jwt-h')"><svg><use href="#i-copy"></use></svg></button></div>
                    <textarea id="jwt-h" class="editor-content" readonly></textarea>
                </div>
                <div class="editor-box">
                    <div class="editor-header"><span>Payload</span><button class="icon-btn" onclick="copy('jwt-p')"><svg><use href="#i-copy"></use></svg></button></div>
                    <textarea id="jwt-p" class="editor-content" readonly></textarea>
                </div>
            </div>
        </div>

        <div id="base64" class="panel">
            <h2>Base64 转换</h2>
            <div class="editor-box">
                <div class="editor-header"><span>输入内容</span><button class="icon-btn" onclick="setVal('b64-in','')"><svg><use href="#i-trash"></use></svg></button></div>
                <textarea id="b64-in" class="editor-content" style="height:120px" placeholder="示例文本：Hello, World!\n或输入Base64编码：SGVsbG8sIFdvcmxkIQ=="></textarea>
            </div>
            <div class="row" style="margin-top:15px">
                <button class="btn" onclick="doB64('encode')">🔒 编码</button>
                <button class="btn" style="background:#64748b" onclick="doB64('decode')">🔓 解码</button>
            </div>
            <div class="editor-box">
                <div class="editor-header"><span>转换结果</span><button class="icon-btn" onclick="copy('b64-out')"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="b64-out" class="editor-content" style="height:120px" readonly></textarea>
            </div>
        </div>

        <div id="url" class="panel">
            <h2>URL 编解码</h2>
            <div class="editor-box">
                <div class="editor-header"><span>输入内容</span><button class="icon-btn" onclick="setVal('url-in','')"><svg><use href="#i-trash"></use></svg></button></div>
                <textarea id="url-in" class="editor-content" style="height:120px" placeholder="示例URL：https://example.com/path?query=hello world\n或已编码URL：https://example.com/path?query=hello%20world"></textarea>
            </div>
            <div class="row" style="margin-top:15px">
                <button class="btn" onclick="doUrl('enc')">🔗 编码</button>
                <button class="btn" style="background:#64748b" onclick="doUrl('dec')">🔗 解码</button>
            </div>
            <div class="editor-box">
                <div class="editor-header"><span>转换结果</span><button class="icon-btn" onclick="copy('url-out')"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="url-out" class="editor-content" style="height:120px" readonly></textarea>
            </div>
        </div>

        <div id="url-parser" class="panel">
            <h2>URL 解析器</h2>
            <div class="row">
                <input id="url-parse-in" oninput="doUrlParse()" placeholder="粘贴完整的 URL, 例如: https://example.com/path?id=123&user=test">
                <button class="btn" onclick="doUrlParse()">🔍 解析</button>
            </div>
            <div id="url-parse-results" style="display:none;">
                <div style="font-size:12px; color:#64748b; margin-bottom:8px; font-weight:bold; margin-top:20px;">基本组件</div>
                <div class="info-grid-2">
                    <div class="info-item"><span class="info-label">协议</span><span id="url-p-protocol" class="info-val">-</span></div>
                    <div class="info-item"><span class="info-label">主机名</span><span id="url-p-host" class="info-val">-</span></div>
                    <div class="info-item" style="grid-column: span 2"><span class="info-label">路径</span><span id="url-p-path" class="info-val">-</span></div>
                </div>
                <div style="font-size:12px; color:#64748b; margin-bottom:8px; font-weight:bold; margin-top:20px;">查询参数</div>
                <div class="result-card" style="display:block;">
                    <table id="url-params-table" style="width:100%; border-collapse: collapse;">
                        <thead>
                            <tr style="text-align:left; border-bottom: 2px solid var(--border);">
                                <th style="padding:10px; font-size:13px; color:#6b7280;">参数名 (Key)</th>
                                <th style="padding:10px; font-size:13px; color:#6b7280;">参数值 (Value)</th>
                            </tr>
                        </thead>
                        <tbody></tbody>
                    </table>
                    <div id="url-no-params" style="text-align:center; color:#94a3b8; padding: 20px; display:none;">没有查询参数</div>
                </div>
            </div>
        </div>

        <div id="json" class="panel">
            <h2>JSON 工具</h2>
            <div class="editor-container">
                <div class="editor-box">
                    <div class="editor-header"><span>输入 JSON</span><button class="icon-btn" onclick="setVal('json-in','')"><svg><use href="#i-trash"></use></svg></button></div>
                    <textarea id="json-in" class="editor-content" placeholder='输入或粘贴 JSON 数据

例如：
{
  "name": "John",
  "age": 30,
  "email": "john@example.com",
  "address": {
    "city": "New York",
    "zip": "10001"
  },
  "hobbies": ["coding", "reading"]
}'></textarea>
                </div>
                <div class="editor-box">
                    <div class="editor-header"><span>处理结果</span><button class="icon-btn" onclick="copy('json-out')"><svg><use href="#i-copy"></use></svg></button></div>
                    <textarea id="json-out" class="editor-content" readonly placeholder="处理后的 JSON 将显示在这里"></textarea>
                </div>
            </div>
            <div class="row" style="margin-top:20px; justify-content:center; gap:20px">
                <button class="btn" onclick="doJson('fmt')">✨ 格式化</button>
                <button class="btn secondary" onclick="doJson('min')">📦 压缩</button>
            </div>
        </div>

        <div id="escape" class="panel">
            <h2>文本转义</h2>
            <div class="row">
                <select id="esc-m" style="flex:1">
                    <option value="html_enc">HTML 转义 (Encode)</option>
                    <option value="html_dec">HTML 还原 (Decode)</option>
                    <option value="json_enc">JSON 转义</option>
                    <option value="json_dec">JSON 还原</option>
                </select>
                <button class="btn" onclick="doEsc()">执行转换</button>
            </div>
            <div class="editor-container">
                <div class="editor-box">
                    <div class="editor-header"><span>输入</span><button class="icon-btn" onclick="setVal('esc-in','')"><svg><use href="#i-trash"></use></svg></button></div>
                    <textarea id="esc-in" class="editor-content" placeholder="示例文本：<div>Hello & World</div>\n或已转义文本：&lt;div&gt;Hello &amp; World&lt;/div&gt;"></textarea>
                </div>
                <div class="editor-box">
                    <div class="editor-header"><span>结果</span><button class="icon-btn" onclick="copy('esc-out')"><svg><use href="#i-copy"></use></svg></button></div>
                    <textarea id="esc-out" class="editor-content" readonly></textarea>
                </div>
            </div>
        </div>

        <div id="jsenc" class="panel">
            <h2>JS 代码混淆</h2>
            <div class="editor-container">
                <div class="editor-box">
                    <div class="editor-header"><span>源代码</span><button class="icon-btn" onclick="setVal('js-in','')"><svg><use href="#i-trash"></use></svg></button></div>
                    <textarea id="js-in" class="editor-content" placeholder="// 示例 JavaScript 代码\nfunction hello() {\n  console.log('Hello, World!');\n  for (let i = 0; i < 5; i++) {\n    console.log(i);\n  }\n}\nhello();"></textarea>
                </div>
                <div class="editor-box">
                    <div class="editor-header"><span>混淆结果</span><button class="icon-btn" onclick="copy('js-out')"><svg><use href="#i-copy"></use></svg></button></div>
                    <textarea id="js-out" class="editor-content" readonly></textarea>
                </div>
            </div>
            <button class="btn" style="width:100%" onclick="doJsEnc()">🔒 执行混淆</button>
        </div>

        <div id="yaml" class="panel"><h2>YAML 转 TOML</h2><div class="editor-container"><div class="editor-box"><div class="editor-header">YAML 输入</div><textarea id="yaml-input" class="editor-content" placeholder="# 示例 YAML
name: John
age: 30
address:
  city: New York
  zip: 10001
hobbies:
  - reading
  - coding
  - hiking

enabled: true"></textarea></div><div class="editor-box"><div class="editor-header">TOML 结果<button class="icon-btn" onclick="copy('toml-output')"><svg><use href="#i-copy"></use></svg></button></div><textarea id="toml-output" class="editor-content" readonly></textarea></div></div><button class="btn" style="width:100%" onclick="convertYaml()">🔄 转换</button></div>
        <div id="toml2yaml" class="panel"><h2>TOML 转 YAML</h2><div class="editor-container"><div class="editor-box"><div class="editor-header">TOML 输入</div><textarea id="toml-input" class="editor-content" placeholder="# 示例 TOML
name = &quot;John&quot;
age = 30

[address]
city = &quot;New York&quot;
zip = 10001

hobbies = [&quot;reading&quot;, &quot;coding&quot;, &quot;hiking&quot;]

enabled = true"></textarea></div><div class="editor-box"><div class="editor-header">YAML 结果<button class="icon-btn" onclick="copy('yaml-output')"><svg><use href="#i-copy"></use></svg></button></div><textarea id="yaml-output" class="editor-content" readonly></textarea></div></div><button class="btn" style="width:100%" onclick="convertToml()">🔄 转换</button></div>
        
        <div id="chmod" class="panel">
            <h2>Linux 权限</h2>
            <div style="background:#f8fafc; border:1px solid #e2e8f0; border-radius:12px; padding:25px; margin-bottom:25px;">
                <div style="display:grid; grid-template-columns: 100px repeat(3, 1fr); gap:20px; align-items:center; text-align:center;">
                    <!-- Header -->
                    <div></div>
                    <div style="font-weight:600; color:#64748b; font-size:14px;">读取 (Read)</div>
                    <div style="font-weight:600; color:#64748b; font-size:14px;">写入 (Write)</div>
                    <div style="font-weight:600; color:#64748b; font-size:14px;">执行 (Execute)</div>
                    
                    <!-- Owner -->
                    <div style="font-weight:600; text-align:left; color:#334155;">所有者<br><span style="font-size:12px;color:#94a3b8;font-weight:normal">Owner</span></div>
                    <div><input type="checkbox" id="c_ur" onchange="upChmod(true)" style="width:24px;height:24px;cursor:pointer;accent-color:var(--primary)"></div>
                    <div><input type="checkbox" id="c_uw" onchange="upChmod(true)" style="width:24px;height:24px;cursor:pointer;accent-color:var(--primary)"></div>
                    <div><input type="checkbox" id="c_ux" onchange="upChmod(true)" style="width:24px;height:24px;cursor:pointer;accent-color:var(--primary)"></div>
                    
                    <!-- Group -->
                    <div style="font-weight:600; text-align:left; color:#334155;">所属组<br><span style="font-size:12px;color:#94a3b8;font-weight:normal">Group</span></div>
                    <div><input type="checkbox" id="c_gr" onchange="upChmod(true)" style="width:24px;height:24px;cursor:pointer;accent-color:var(--primary)"></div>
                    <div><input type="checkbox" id="c_gw" onchange="upChmod(true)" style="width:24px;height:24px;cursor:pointer;accent-color:var(--primary)"></div>
                    <div><input type="checkbox" id="c_gx" onchange="upChmod(true)" style="width:24px;height:24px;cursor:pointer;accent-color:var(--primary)"></div>
                    
                    <!-- Other -->
                    <div style="font-weight:600; text-align:left; color:#334155;">其他用户<br><span style="font-size:12px;color:#94a3b8;font-weight:normal">Other</span></div>
                    <div><input type="checkbox" id="c_or" onchange="upChmod(true)" style="width:24px;height:24px;cursor:pointer;accent-color:var(--primary)"></div>
                    <div><input type="checkbox" id="c_ow" onchange="upChmod(true)" style="width:24px;height:24px;cursor:pointer;accent-color:var(--primary)"></div>
                    <div><input type="checkbox" id="c_ox" onchange="upChmod(true)" style="width:24px;height:24px;cursor:pointer;accent-color:var(--primary)"></div>
                </div>
            </div>

            <div class="grid-4">
                <div class="result-card">
                    <div class="result-label">Octal Code</div>
                    <input id="chmod-octal" value="755" oninput="upChmod(false)" style="border:none; background:transparent; font-family:monospace; font-size:24px; width:100%; color:var(--primary); font-weight:bold; outline:none;">
                </div>
                <div class="result-card" style="grid-column: span 3;">
                    <div class="result-label">文件名 (Filename)</div>
                    <input id="chmod-file" placeholder="filename" oninput="upChmod(false)" style="border:none; background:transparent; font-family:monospace; font-size:18px; width:100%; color:var(--text); outline:none;">
                </div>
            </div>
            <div class="result-card" style="margin-top:15px">
                <div class="result-label">Linux Command</div>
                <div id="chmod-command" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">chmod 755 filename</div>
                <button class="icon-btn" onclick="copy('chmod-command')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="tar" class="panel">
            <h2>Tar 命令生成</h2>
            <div class="row">
                <select id="tar-op" onchange="doTar()" style="flex:1">
                    <option value="create">创建压缩包 (Create)</option>
                    <option value="extract">解压 (Extract)</option>
                    <option value="list">查看内容 (List)</option>
                </select>
                <select id="tar-comp" onchange="doTar()" style="flex:1">
                    <option value="none">无压缩 (None)</option>
                    <option value="gzip" selected>Gzip (.gz)</option>
                    <option value="bzip2">Bzip2 (.bz2)</option>
                    <option value="xz">XZ (.xz)</option>
                </select>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none">
                    <input type="checkbox" id="tar-v" checked onchange="doTar()" style="width:20px;height:20px"> 详细 (Verbose)
                </label>
            </div>
            <div class="grid-4" style="margin-bottom:20px">
                <div style="grid-column: span 2"><div class="cron-label">归档文件名</div><input id="tar-arch" placeholder="archive.tar.gz" oninput="doTar()"></div>
                <div style="grid-column: span 2"><div class="cron-label">源文件 / 目标目录</div><input id="tar-files" placeholder="/path/to/files" oninput="doTar()"></div>
            </div>
            <div class="result-card">
                <div class="result-label">生成的命令</div>
                <div id="tar-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">tar -czvf archive.tar.gz /path/to/files</div>
                <button class="icon-btn" onclick="copy('tar-cmd')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="ps" class="panel">
            <h2>Ps 命令生成</h2>
            <div class="row">
                <select id="ps-fmt" onchange="doPs()" style="flex:1">
                    <option value="aux">常用 (aux)</option>
                    <option value="ef">全格式 (-ef)</option>
                </select>
                <select id="ps-sort" onchange="doPs()" style="flex:1">
                    <option value="none">默认排序</option>
                    <option value="-%cpu">按 CPU 占用 (降序)</option>
                    <option value="-%mem">按内存占用 (降序)</option>
                    <option value="pid">按 PID (升序)</option>
                </select>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none">
                    <input type="checkbox" id="ps-tree" onchange="doPs()" style="width:20px;height:20px"> 树状图
                </label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none">
                    <input type="checkbox" id="ps-wide" onchange="doPs()" style="width:20px;height:20px"> 完整命令 (ww)
                </label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none">
                    <input type="checkbox" id="ps-threads" onchange="doPs()" style="width:20px;height:20px"> 显示线程 (-L)
                </label>
            </div>
            <div class="grid-4" style="margin-bottom:20px">
                <div style="grid-column: span 2"><div class="cron-label">指定用户 (-u)</div><input id="ps-user" placeholder="root" oninput="doPs()"></div>
                <div style="grid-column: span 2"><div class="cron-label">指定 PID (-p)</div><input id="ps-pid" placeholder="1234" oninput="doPs()"></div>
                <div style="grid-column: span 4"><div class="cron-label">过滤进程名 (Grep)</div><input id="ps-filter" placeholder="例如: nginx" oninput="doPs()"></div>
            </div>
            <div class="result-card">
                <div class="result-label">生成的命令</div>
                <div id="ps-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">ps aux</div>
                <button class="icon-btn" onclick="copy('ps-cmd')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="tcpdump" class="panel">
            <h2>Tcpdump 命令生成</h2>
            <div class="grid-4" style="margin-bottom:15px">
                <div><div class="cron-label">网卡接口 (-i)</div><input id="td-if" placeholder="any" oninput="doTcpdump()"></div>
                <div><div class="cron-label">协议</div><select id="td-proto" onchange="doTcpdump()"><option value="all">全部</option><option value="tcp">TCP</option><option value="udp">UDP</option><option value="icmp">ICMP</option><option value="arp">ARP</option></select></div>
                <div><div class="cron-label">主机 (Host)</div><input id="td-host" placeholder="192.168.1.1" oninput="doTcpdump()"></div>
                <div><div class="cron-label">端口 (Port)</div><input id="td-port" placeholder="80" oninput="doTcpdump()"></div>
            </div>
            <div class="row" style="margin-bottom:15px; gap:20px">
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none">
                    <input type="checkbox" id="td-v" onchange="doTcpdump()" style="width:18px;height:18px"> 详细 (-v)
                </label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none">
                    <input type="checkbox" id="td-a" onchange="doTcpdump()" style="width:18px;height:18px"> ASCII (-A)
                </label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none">
                    <input type="checkbox" id="td-x" onchange="doTcpdump()" style="width:18px;height:18px"> Hex (-X)
                </label>
            </div>
            <div class="grid-4" style="margin-bottom:20px">
                <div style="grid-column: span 3">
                    <div class="cron-label">保存到文件 (-w)</div>
                    <input id="td-w" placeholder="capture.pcap (留空则输出到控制台)" oninput="doTcpdump()">
                </div>
                <div>
                    <div class="cron-label">抓包数量 (-c)</div>
                    <input id="td-c" placeholder="例如: 100" oninput="doTcpdump()">
                </div>
            </div>
            <div class="result-card">
                <div class="result-label">生成的命令</div>
                <div id="td-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">tcpdump -i any</div>
                <button class="icon-btn" onclick="copy('td-cmd')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="strace" class="panel">
            <h2>系统调用 (Strace)</h2>
            <div class="row">
                <div style="flex:1">
                    <div class="cron-label">目标 (PID 或 命令)</div>
                    <input id="st-target" placeholder="例如: 1234 或 ls -la" oninput="doStrace()">
                </div>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none;margin-top:20px">
                    <input type="checkbox" id="st-pid" onchange="doStrace()" style="width:18px;height:18px;accent-color:var(--primary)"> 是 PID (-p)
                </label>
            </div>
            <div class="row" style="margin-bottom:15px; gap:20px; background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0;">
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="st-f" onchange="doStrace()" style="width:18px;height:18px;accent-color:var(--primary)"> 跟踪子进程 (-f)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="st-c" onchange="doStrace()" style="width:18px;height:18px;accent-color:var(--primary)"> 统计摘要 (-c)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="st-tt" onchange="doStrace()" style="width:18px;height:18px;accent-color:var(--primary)"> 时间戳 (-tt)</label>
            </div>
            <div class="grid-4" style="margin-bottom:20px">
                <div><div class="cron-label">过滤表达式 (-e)</div><input id="st-e" placeholder="trace=open,read" oninput="doStrace()"></div>
                <div><div class="cron-label">字符串长度 (-s)</div><input id="st-s" placeholder="32" oninput="doStrace()"></div>
                <div style="grid-column: span 2"><div class="cron-label">输出文件 (-o)</div><input id="st-o" placeholder="output.txt" oninput="doStrace()"></div>
            </div>
            <div class="result-card">
                <div class="result-label">生成的命令</div>
                <div id="st-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">strace</div>
                <button class="icon-btn" onclick="copy('st-cmd')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="iostat" class="panel">
            <h2>磁盘 I/O (Iostat)</h2>
            <div style="background:#f1f5f9; padding:15px; border-radius:8px; margin-bottom:20px; font-size:14px; color:#475569; line-height:1.6;">
                <strong>说明：</strong> <code>iostat</code> 命令用于监控系统输入/输出设备和 CPU 的使用情况。它可以汇报磁盘活动的统计数据，帮助识别 I/O 瓶颈。<br>
                通常包含在 <code>sysstat</code> 软件包中。
            </div>
            <div class="grid-4" style="margin-bottom:15px">
                <div><div class="cron-label">刷新间隔 (秒)</div><input id="io-int" placeholder="例如: 1" oninput="doIostat()"></div>
                <div><div class="cron-label">刷新次数</div><input id="io-cnt" placeholder="例如: 10" oninput="doIostat()"></div>
                <div style="grid-column: span 2"><div class="cron-label">指定设备 (可选)</div><input id="io-dev" placeholder="例如: sda" oninput="doIostat()"></div>
            </div>
            <div class="row" style="margin-bottom:15px; gap:20px; background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0;">
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="io-x" onchange="doIostat()" style="width:18px;height:18px;accent-color:var(--primary)"> 扩展信息 (-x)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="io-h" onchange="doIostat()" style="width:18px;height:18px;accent-color:var(--primary)"> 人类可读 (-h)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="io-t" onchange="doIostat()" style="width:18px;height:18px;accent-color:var(--primary)"> 时间戳 (-t)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="io-p" onchange="doIostat()" style="width:18px;height:18px;accent-color:var(--primary)"> 显示分区 (-p)</label>
            </div>
            <div class="row" style="margin-bottom:20px">
                <div class="cron-label" style="margin-right:10px">单位:</div>
                <select id="io-unit" onchange="doIostat()" style="flex:1"><option value="">默认 (Block)</option><option value="k">KB (-k)</option><option value="m">MB (-m)</option></select>
            </div>
            <div class="result-card">
                <div class="result-label">生成的命令</div>
                <div id="io-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">iostat</div>
                <button class="icon-btn" onclick="copy('io-cmd')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="nice" class="panel">
            <h2>进程优先级 (Nice/Renice)</h2>
            <div style="background:#f1f5f9; padding:15px; border-radius:8px; margin-bottom:20px; font-size:14px; color:#475569; line-height:1.6;">
                <strong>说明：</strong> Linux 进程优先级 (Niceness) 范围从 <strong>-20</strong> (最高优先级) 到 <strong>19</strong> (最低优先级)，默认为 0。<br>
                <ul style="margin-left:20px; margin-top:5px;">
                    <li><code>nice</code>：用于以指定的优先级<strong>启动</strong>新进程。</li>
                    <li><code>renice</code>：用于<strong>调整</strong>正在运行的进程的优先级。</li>
                </ul>
                <div style="margin-top:5px; font-size:12px; color:#64748b">* 注意：设置负值（提高优先级）通常需要 root 权限。</div>
            </div>
            
            <div class="row">
                <div class="cron-label" style="margin-right:10px">模式:</div>
                <select id="ni-mode" onchange="updateNiceUI(); doNice()" style="flex:1">
                    <option value="nice">启动新进程 (nice)</option>
                    <option value="renice">调整现有进程 (renice)</option>
                </select>
            </div>

            <div class="row">
                <div style="flex:1">
                    <div class="cron-label">优先级值 (-20 ~ 19)</div>
                    <div style="display:flex; align-items:center; gap:15px">
                        <input type="range" id="ni-prio-r" min="-20" max="19" value="10" oninput="document.getElementById('ni-prio').value=this.value; doNice()" style="flex:1; cursor:pointer">
                        <input type="number" id="ni-prio" value="10" min="-20" max="19" oninput="document.getElementById('ni-prio-r').value=this.value; doNice()" style="width:80px">
                    </div>
                </div>
            </div>

            <div id="box-nice" style="margin-bottom:20px">
                <div class="cron-label">要执行的命令</div>
                <input id="ni-cmd" placeholder="例如: tar -czf backup.tar.gz /home" oninput="doNice()">
            </div>

            <div id="box-renice" class="grid-4" style="margin-bottom:20px; display:none">
                <div><div class="cron-label">目标类型</div><select id="ni-type" onchange="doNice()"><option value="pid">进程 ID (-p)</option><option value="group">进程组 (-g)</option><option value="user">用户 (-u)</option></select></div>
                <div style="grid-column: span 3"><div class="cron-label">目标 (PID / GID / Username)</div><input id="ni-target" placeholder="例如: 1234 或 www-data" oninput="doNice()"></div>
            </div>

            <div class="result-card"><div class="result-label">生成的命令</div><div id="ni-res" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">nice -n 10</div><button class="icon-btn" onclick="copy('ni-res')"><svg><use href="#i-copy"></use></svg></button></div>
        </div>

        <div id="ls" class="panel">
            <h2>列出文件 (Ls)</h2>
            <div class="row">
                <div style="flex:1">
                    <div class="cron-label">路径 (Path)</div>
                    <input id="ls-path" placeholder="例如: /var/log 或 ." oninput="doLs()">
                </div>
            </div>
            <div class="grid-4" style="margin-bottom:20px; background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0;">
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ls-l" onchange="doLs()" style="width:18px;height:18px;accent-color:var(--primary)"> 长格式 (-l)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ls-a" onchange="doLs()" style="width:18px;height:18px;accent-color:var(--primary)"> 显示隐藏 (-a)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ls-h" onchange="doLs()" style="width:18px;height:18px;accent-color:var(--primary)"> 人类可读 (-h)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ls-t" onchange="doLs()" style="width:18px;height:18px;accent-color:var(--primary)"> 按时间排序 (-t)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ls-r" onchange="doLs()" style="width:18px;height:18px;accent-color:var(--primary)"> 反向排序 (-r)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ls-R" onchange="doLs()" style="width:18px;height:18px;accent-color:var(--primary)"> 递归 (-R)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ls-i" onchange="doLs()" style="width:18px;height:18px;accent-color:var(--primary)"> Inode (-i)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ls-d" onchange="doLs()" style="width:18px;height:18px;accent-color:var(--primary)"> 仅目录 (-d)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ls-c" checked onchange="doLs()" style="width:18px;height:18px;accent-color:var(--primary)"> 颜色 (--color)</label>
            </div>
            <div class="result-card"><div class="result-label">生成的命令</div><div id="ls-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">ls --color=auto</div><button class="icon-btn" onclick="copy('ls-cmd')"><svg><use href="#i-copy"></use></svg></button></div>
        </div>

        <div id="rsync" class="panel">
            <h2>文件同步 (Rsync)</h2>
            <div class="grid-4" style="margin-bottom:15px">
                <div style="grid-column: span 2"><div class="cron-label">源路径 (Source)</div><input id="rs-src" placeholder="/local/path/" oninput="doRsync()"></div>
                <div><div class="cron-label">用户名 (User)</div><input id="rs-user" placeholder="root" oninput="doRsync()"></div>
                <div><div class="cron-label">目标主机 (Host)</div><input id="rs-host" placeholder="192.168.1.100" oninput="doRsync()"></div>
                <div><div class="cron-label">端口 (Port)</div><input id="rs-port" placeholder="22" oninput="doRsync()"></div>
                <div><div class="cron-label">远程路径 (Path)</div><input id="rs-path" placeholder="/var/www/" oninput="doRsync()"></div>
            </div>
            <div class="grid-4" style="margin-bottom:20px; background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0;">
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="rs-a" checked onchange="doRsync()" style="width:18px;height:18px;accent-color:var(--primary)"> 归档模式 (-a)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="rs-z" checked onchange="doRsync()" style="width:18px;height:18px;accent-color:var(--primary)"> 压缩传输 (-z)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="rs-v" checked onchange="doRsync()" style="width:18px;height:18px;accent-color:var(--primary)"> 详细输出 (-v)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="rs-P" checked onchange="doRsync()" style="width:18px;height:18px;accent-color:var(--primary)"> 显示进度 (-P)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="rs-del" onchange="doRsync()" style="width:18px;height:18px;accent-color:var(--primary)"> 删除目标多余文件 (--delete)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="rs-n" onchange="doRsync()" style="width:18px;height:18px;accent-color:var(--primary)"> 试运行 (--dry-run)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="rs-ssh" onchange="doRsync()" style="width:18px;height:18px;accent-color:var(--primary)"> 使用 SSH (-e ssh)</label>
            </div>
            <div style="margin-bottom:20px"><div class="cron-label">排除模式 (--exclude)</div><input id="rs-ex" placeholder="*.tmp" oninput="doRsync()"></div>
            <div class="result-card"><div class="result-label">生成的命令</div><div id="rs-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">rsync -azvP /local/path/ user@host:/remote/path/</div><button class="icon-btn" onclick="copy('rs-cmd')"><svg><use href="#i-copy"></use></svg></button></div>
            <div class="result-card" style="margin-top:15px">
                <div class="result-label">SSH Config Entry (~/.ssh/config)</div>
                <textarea id="rs-ssh-conf" class="result-val" style="width:100%; height:90px; background:transparent; border:none; resize:none; outline:none; font-family:monospace; line-height:1.5;" readonly placeholder="# Fill in Host to generate config"></textarea>
                <button class="icon-btn" onclick="copy('rs-ssh-conf')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="firewall" class="panel">
            <h2>防火墙 (Firewall-cmd)</h2>
            <div style="background:#f1f5f9; padding:15px; border-radius:8px; margin-bottom:20px; font-size:14px; color:#475569; line-height:1.6;">
                <strong>说明：</strong> <code>firewall-cmd</code> 是 Linux 上 firewalld 守护进程的命令行客户端。<br>
                它支持动态管理防火墙规则，无需重启服务。使用 <code>--permanent</code> 标志可将改动保存到配置文件中。
            </div>
            
            <div class="row">
                <div class="cron-label" style="margin-right:10px">操作:</div>
                <select id="fw-op" onchange="updateFwUI(); doFirewall()" style="flex:1">
                    <option value="add">添加规则 (Add)</option>
                    <option value="remove">移除规则 (Remove)</option>
                    <option value="list">列出所有 (List)</option>
                    <option value="reload">重载配置 (Reload)</option>
                </select>
            </div>

            <div id="fw-opts">
                <div class="grid-4" style="margin-bottom:15px">
                    <div><div class="cron-label">区域 (Zone)</div><select id="fw-zone" onchange="doFirewall()">
                        <option value="public" selected>public (默认)</option>
                        <option value="all">所有区域 (All Zones)</option>
                        <option value="home">home</option>
                        <option value="work">work</option>
                        <option value="trusted">trusted</option>
                        <option value="block">block</option>
                        <option value="dmz">dmz</option>
                        <option value="external">external</option>
                        <option value="internal">internal</option>
                        <option value="drop">drop</option>
                    </select></div>
                    <div id="fw-perm-box" style="display:flex; align-items:flex-end; padding-bottom:15px">
                        <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="fw-perm" onchange="doFirewall()" style="width:18px;height:18px;accent-color:var(--primary)"> 永久生效 (--permanent)</label>
                    </div>
                </div>

                <div id="fw-rule-box" class="grid-4" style="margin-bottom:20px">
                    <div><div class="cron-label">类型</div><select id="fw-type" onchange="doFirewall()"><option value="port">端口 (Port)</option><option value="service">服务 (Service)</option></select></div>
                    <div style="grid-column: span 3"><div class="cron-label">值 (例如: 80/tcp 或 http)</div><input id="fw-target" placeholder="80/tcp" oninput="doFirewall()"></div>
                </div>
            </div>

            <div class="result-card"><div class="result-label">生成的命令</div><div id="fw-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">firewall-cmd --permanent --zone=public --add-port=80/tcp</div><button class="icon-btn" onclick="copy('fw-cmd')"><svg><use href="#i-copy"></use></svg></button></div>
        </div>

        <div id="systemctl" class="panel">
            <h2>服务管理 (Systemctl)</h2>
            <div style="background:#f1f5f9; padding:15px; border-radius:8px; margin-bottom:20px; font-size:14px; color:#475569; line-height:1.6;">
                <strong>说明：</strong> <code>systemctl</code> 是用于控制 systemd 系统和服务管理器的主要工具。<br>
                它可以启动、停止、重启服务，以及管理系统启动时的服务状态。
            </div>
            
            <div class="row">
                <div class="cron-label" style="margin-right:10px">操作:</div>
                <select id="sys-op" onchange="updateSysUI(); doSystemctl()" style="flex:1">
                    <option value="status">查看状态 (status)</option>
                    <option value="start">启动 (start)</option>
                    <option value="stop">停止 (stop)</option>
                    <option value="restart">重启 (restart)</option>
                    <option value="reload">重载配置 (reload)</option>
                    <option value="enable">开机自启 (enable)</option>
                    <option value="disable">禁用自启 (disable)</option>
                    <option value="mask">屏蔽服务 (mask)</option>
                    <option value="unmask">取消屏蔽 (unmask)</option>
                    <option value="daemon-reload">重载所有配置 (daemon-reload)</option>
                </select>
            </div>

            <div id="sys-svc-box" style="margin-bottom:15px"><div class="cron-label">服务名称 (Service)</div><input id="sys-svc" placeholder="例如: nginx, docker, sshd" oninput="doSystemctl()"></div>

            <div class="grid-4" style="margin-bottom:20px; background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0;">
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="sys-user" onchange="doSystemctl()" style="width:18px;height:18px;accent-color:var(--primary)"> 用户模式 (--user)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="sys-now" onchange="doSystemctl()" style="width:18px;height:18px;accent-color:var(--primary)"> 立即执行 (--now)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="sys-force" onchange="doSystemctl()" style="width:18px;height:18px;accent-color:var(--primary)"> 强制 (--force)</label>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="sys-global" onchange="doSystemctl()" style="width:18px;height:18px;accent-color:var(--primary)"> 全局 (--global)</label>
            </div>

            <div class="result-card"><div class="result-label">生成的命令</div><div id="sys-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">systemctl status</div><button class="icon-btn" onclick="copy('sys-cmd')"><svg><use href="#i-copy"></use></svg></button></div>
        </div>

        <div id="find" class="panel">
            <h2>查找文件 (Find)</h2>
            <div class="row">
                <div style="flex:1">
                    <div class="cron-label">搜索路径 (Path)</div>
                    <input id="fd-path" placeholder="例如: /var/log 或 ." value="." oninput="doFind()">
                </div>
            </div>
            <div class="grid-4" style="margin-bottom:15px">
                <div style="grid-column: span 2">
                    <div class="cron-label">文件名 (Name)</div>
                    <input id="fd-name" placeholder="*.log" oninput="doFind()">
                </div>
                <div>
                    <div class="cron-label">文件类型 (Type)</div>
                    <select id="fd-type" onchange="doFind()">
                        <option value="all">全部 (All)</option>
                        <option value="f">文件 (f)</option>
                        <option value="d">目录 (d)</option>
                        <option value="l">符号链接 (l)</option>
                    </select>
                </div>
                <div style="display:flex; align-items:flex-end; padding-bottom:15px">
                    <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none">
                        <input type="checkbox" id="fd-iname" onchange="doFind()" style="width:18px;height:18px;accent-color:var(--primary)"> 忽略大小写 (-iname)
                    </label>
                </div>
            </div>
            <div class="grid-4" style="margin-bottom:15px">
                <div><div class="cron-label">大小 (Size)</div><input id="fd-size" placeholder="+100M" oninput="doFind()"></div>
                <div><div class="cron-label">修改时间 (Mtime)</div><input id="fd-mtime" placeholder="-7 (7天内)" oninput="doFind()"></div>
                <div style="display:flex; align-items:flex-end; padding-bottom:15px"><label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="fd-empty" onchange="updateFindUI(); doFind()" style="width:18px;height:18px;accent-color:var(--primary)"> 空文件 (-empty)</label></div>
            </div>
            <div style="margin-bottom:20px"><div class="cron-label">执行命令 (-exec ... {} \;)</div><input id="fd-exec" placeholder="例如: rm -rf 或 chmod 644" oninput="doFind()"></div>
            <div class="result-card">
                <div class="result-label">生成的命令</div>
                <div id="fd-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">find .</div>
                <button class="icon-btn" onclick="copy('fd-cmd')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="awk" class="panel">
            <h2>文本处理 (Awk)</h2>
            <div class="grid-4" style="margin-bottom:15px">
                <div><div class="cron-label">分隔符 (-F)</div><input id="awk-sep" placeholder="space (默认)" oninput="doAwk()"></div>
                <div style="grid-column: span 3"><div class="cron-label">变量赋值 (-v)</div><input id="awk-var" placeholder="例如: limit=100" oninput="doAwk()"></div>
            </div>
            <div style="margin-bottom:15px">
                <div class="cron-label">常用代码片段 (Snippets)</div>
                <select id="awk-snippets" onchange="setAwkSnippet()" style="width:100%; padding:10px; border:2px solid #e5e7eb; border-radius:10px; background:white;">
                    <option value="">-- 选择常用操作 --</option>
                    <option value="{print $1, $3}">打印第1和第3列</option>
                    <option value="{print $NF}">打印最后一列</option>
                    <option value="{print NR, $0}">打印行号和内容</option>
                    <option value="/Error/ {print $0}">打印包含 "Error" 的行</option>
                    <option value="length($0) > 80">打印长度超过80的行</option>
                    <option value="{sum+=$1} END {print sum}">计算第1列总和</option>
                    <option value="{sum+=$1} END {print sum/NR}">计算第1列平均值</option>
                    <option value="NR>=10 && NR<=20">打印第10到20行</option>
                    <option value="!seen[$0]++">删除重复行 (去重)</option>
                </select>
            </div>
            <div style="margin-bottom:15px">
                <div class="cron-label">处理代码 (Pattern { Action })</div>
                <textarea id="awk-code" style="height:100px; font-family:monospace;" placeholder="{print $1, $3}" oninput="doAwk()"></textarea>
            </div>
            <div style="margin-bottom:20px">
                <div class="cron-label">输入文件</div>
                <input id="awk-file" placeholder="data.txt" oninput="doAwk()">
            </div>
            <div class="result-card">
                <div class="result-label">生成的命令</div>
                <div id="awk-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">awk '{print $0}'</div>
                <button class="icon-btn" onclick="copy('awk-cmd')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="sed" class="panel">
            <h2>流编辑 (Sed)</h2>
            <div style="margin-bottom:15px">
                <div class="cron-label">常用代码片段 (Snippets)</div>
                <select id="sed-snippets" onchange="setSedSnippet()" style="width:100%; padding:10px; border:2px solid #e5e7eb; border-radius:10px; background:white;">
                    <option value="">-- 选择常用操作 --</option>
                    <option value='{"op":"substitute","pat":"foo","rep":"bar","flags":"g"}'>全局替换 (s/foo/bar/g)</option>
                    <option value='{"op":"delete","pat":"/^$/","rep":"","flags":""}'>删除空行 (/^$/d)</option>
                    <option value='{"op":"delete","pat":"1","rep":"","flags":""}'>删除第一行 (1d)</option>
                    <option value='{"op":"delete","pat":"$","rep":"","flags":""}'>删除最后一行 ($d)</option>
                    <option value='{"op":"delete","pat":"/Error/","rep":"","flags":""}'>删除包含 Error 的行</option>
                    <option value='{"op":"insert","pat":"1","rep":"#!/bin/bash","flags":""}'>在第一行插入 (Shebang)</option>
                    <option value='{"op":"append","pat":"$","rep":"End of file","flags":""}'>在末尾追加文本</option>
                </select>
            </div>
            <div class="row">
                <select id="sed-op" onchange="doSed()" style="flex:1"><option value="substitute">替换 (s)</option><option value="delete">删除 (d)</option><option value="insert">插入 (i)</option><option value="append">追加 (a)</option></select>
                <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="sed-i" onchange="doSed()" style="width:18px;height:18px;accent-color:var(--primary)"> 原地修改 (-i)</label>
            </div>
            <div class="grid-4" style="margin-bottom:15px">
                <div style="grid-column: span 2"><div class="cron-label">匹配模式 / 行号</div><input id="sed-pat" placeholder="例如: ^Error 或 1,5" oninput="doSed()"></div>
                <div style="grid-column: span 2"><div class="cron-label">替换内容 / 新增文本</div><input id="sed-rep" placeholder="例如: Success" oninput="doSed()"></div>
            </div>
            <div class="grid-4" style="margin-bottom:20px">
                <div>
                    <div class="cron-label">标志 (Flags)</div>
                    <div style="display:flex; gap:15px; height:48px; align-items:center; padding-left:2px;">
                        <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="sed-g" onchange="doSed()" style="width:18px;height:18px;accent-color:var(--primary)"> 全局 (g)</label>
                        <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="sed-p" onchange="doSed()" style="width:18px;height:18px;accent-color:var(--primary)"> 打印 (p)</label>
                        <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="sed-ic" onchange="doSed()" style="width:18px;height:18px;accent-color:var(--primary)"> 忽略大小写 (I)</label>
                    </div>
                </div>
                <div style="grid-column: span 3"><div class="cron-label">输入文件</div><input id="sed-file" placeholder="file.txt" oninput="doSed()"></div>
            </div>
            <div class="result-card">
                <div class="result-label">生成的命令</div>
                <div id="sed-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px;">sed 's///'</div>
                <button class="icon-btn" onclick="copy('sed-cmd')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="case" class="panel">
            <h2>变量命名转换</h2>
            <div class="row">
                <select id="case-m" style="flex:1" onchange="doCase()">
                    <option value="camel">小驼峰 (camelCase)</option>
                    <option value="pascal">大驼峰 (PascalCase)</option>
                    <option value="snake">下划线 (snake_case)</option>
                    <option value="kebab">中划线 (kebab-case)</option>
                    <option value="constant">常量 (CONSTANT_CASE)</option>
                    <option value="upper">全大写 (UPPER CASE)</option>
                    <option value="lower">全小写 (lower case)</option>
                </select>
                <button class="btn" onclick="doCase()">转换</button>
            </div>
            <div class="editor-container" style="height:200px">
                <div class="editor-box"><div class="editor-header"><span>输入</span><button class="icon-btn" onclick="setVal('case-in','')"><svg><use href="#i-trash"></use></svg></button></div><textarea id="case-in" class="editor-content" placeholder="输入任意格式，如: user_id, UserInfo, get-data" oninput="doCase()"></textarea></div>
                <div class="editor-box"><div class="editor-header"><span>结果</span><button class="icon-btn" onclick="copy('case-out')"><svg><use href="#i-copy"></use></svg></button></div><textarea id="case-out" class="editor-content" readonly></textarea></div>
            </div>
        </div>

        <div id="dockerfile" class="panel">
            <h2>Dockerfile 生成器 (支持多阶段构建)</h2>
            <div id="df-stages-container"></div>
            
            <div class="row" style="margin-top: 20px; margin-bottom: 20px;">
                <button class="btn success" onclick="addStage()">+ 添加构建阶段</button>
                <button class="btn" onclick="doDockerfile()">🔨 生成 Dockerfile</button>
            </div>

            <div class="editor-box">
                <div class="editor-header"><span>生成结果</span><button class="icon-btn" onclick="copy('df-res')"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="df-res" class="editor-content" style="height:300px" readonly></textarea>
            </div>
        </div>

        <div id="k8s" class="panel">
            <h2>Kubernetes YAML 生成</h2>
            <div class="row">
                <div class="cron-label" style="margin-right:10px">资源类型:</div>
                <select id="k8s-kind" onchange="updateK8sUI()" style="flex:1">
                    <option value="Deployment">Deployment (部署)</option>
                    <option value="Service">Service (服务)</option>
                    <option value="Ingress">Ingress (路由)</option>
                    <option value="CronJob">CronJob (定时任务)</option>
                    <option value="ConfigMap">ConfigMap (配置)</option>
                    <option value="Secret">Secret (机密)</option>
                </select>
            </div>

            <div class="grid-4" style="margin-bottom:15px">
                <div><div class="cron-label">名称 (Name)</div><input id="k8s-name" value="app-name"></div>
                <div><div class="cron-label">命名空间 (Namespace)</div><input id="k8s-ns" value="default"></div>
                <div id="k8s-img-box" style="grid-column: span 2"><div class="cron-label">镜像 (Image)</div><input id="k8s-image" value="nginx:latest"></div>
            </div>

            <!-- Deployment / CronJob Options -->
            <div id="k8s-workload-opts" class="grid-4" style="margin-bottom:15px">
                <div><div class="cron-label">副本数 (Replicas)</div><input type="number" id="k8s-rep" value="1"></div>
                <div><div class="cron-label">端口 (Port)</div><input type="number" id="k8s-port" value="80"></div>
                <div><div class="cron-label">拉取策略</div><select id="k8s-pull"><option value="IfNotPresent">IfNotPresent</option><option value="Always">Always</option></select></div>
                <div><div class="cron-label">重启策略</div><select id="k8s-restart"><option value="Always">Always</option><option value="OnFailure">OnFailure</option></select></div>
            </div>

            <!-- Resources -->
            <div id="k8s-res-opts" style="margin-bottom:15px; display:none; background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0;">
                <div style="font-size:12px; font-weight:bold; margin-bottom:10px; color:#64748b">资源限制 (Resources)</div>
                <div class="grid-4">
                    <div><div class="cron-label">CPU Request</div><input id="k8s-cpu-r" placeholder="100m"></div>
                    <div><div class="cron-label">Mem Request</div><input id="k8s-mem-r" placeholder="128Mi"></div>
                    <div><div class="cron-label">CPU Limit</div><input id="k8s-cpu-l" placeholder="500m"></div>
                    <div><div class="cron-label">Mem Limit</div><input id="k8s-mem-l" placeholder="512Mi"></div>
                </div>
            </div>

            <!-- Service Options -->
            <div id="k8s-svc-opts" class="grid-4" style="margin-bottom:15px; display:none">
                <div><div class="cron-label">类型 (Type)</div><select id="k8s-svc-type"><option value="ClusterIP">ClusterIP</option><option value="NodePort">NodePort</option><option value="LoadBalancer">LoadBalancer</option></select></div>
                <div><div class="cron-label">服务端口</div><input type="number" id="k8s-svc-port" value="80"></div>
                <div><div class="cron-label">目标端口</div><input type="number" id="k8s-target-port" value="80"></div>
            </div>

            <!-- Ingress Options -->
            <div id="k8s-ing-opts" class="grid-4" style="margin-bottom:15px; display:none">
                <div style="grid-column: span 2"><div class="cron-label">域名 (Host)</div><input id="k8s-host" value="example.com"></div>
                <div style="grid-column: span 2"><div class="cron-label">路径 (Path)</div><input id="k8s-path" value="/"></div>
            </div>

            <!-- CronJob Options -->
            <div id="k8s-cron-opts" style="margin-bottom:15px; display:none">
                <div class="cron-label">调度周期 (Schedule)</div><input id="k8s-schedule" value="*/1 * * * *">
            </div>

            <!-- Env / Data -->
            <div id="k8s-env-box" style="margin-bottom:20px">
                <div class="cron-label" id="k8s-env-lbl">环境变量 (Key=Value, 每行一个)</div>
                <textarea id="k8s-env" style="height:100px; font-family:monospace;" placeholder="DB_HOST=localhost&#10;DB_PORT=5432"></textarea>
            </div>

            <button class="btn" style="width:100%; margin-bottom:20px" onclick="doK8s()">⚙️ 生成 YAML</button>
            
            <div class="editor-box">
                <div class="editor-header"><span>生成结果</span><button class="icon-btn" onclick="copy('k8s-res')"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="k8s-res" class="editor-content" style="height:300px" readonly></textarea>
            </div>
        </div>

        <div id="k8s" class="panel">
            <h2>Kubernetes YAML 生成</h2>
            <div class="row">
                <div class="cron-label" style="margin-right:10px">资源类型:</div>
                <select id="k8s-kind" onchange="updateK8sUI()" style="flex:1">
                    <option value="Deployment">Deployment (部署)</option>
                    <option value="Service">Service (服务)</option>
                    <option value="Ingress">Ingress (路由)</option>
                    <option value="CronJob">CronJob (定时任务)</option>
                    <option value="ConfigMap">ConfigMap (配置)</option>
                    <option value="Secret">Secret (机密)</option>
                </select>
            </div>

            <div class="grid-4" style="margin-bottom:15px">
                <div><div class="cron-label">名称 (Name)</div><input id="k8s-name" value="app-name"></div>
                <div><div class="cron-label">命名空间 (Namespace)</div><input id="k8s-ns" value="default"></div>
                <div id="k8s-img-box" style="grid-column: span 2"><div class="cron-label">镜像 (Image)</div><input id="k8s-image" value="nginx:latest"></div>
            </div>

            <!-- Deployment / CronJob Options -->
            <div id="k8s-workload-opts" class="grid-4" style="margin-bottom:15px">
                <div><div class="cron-label">副本数 (Replicas)</div><input type="number" id="k8s-rep" value="1"></div>
                <div><div class="cron-label">端口 (Port)</div><input type="number" id="k8s-port" value="80"></div>
                <div><div class="cron-label">拉取策略</div><select id="k8s-pull"><option value="IfNotPresent">IfNotPresent</option><option value="Always">Always</option></select></div>
                <div><div class="cron-label">重启策略</div><select id="k8s-restart"><option value="Always">Always</option><option value="OnFailure">OnFailure</option></select></div>
            </div>

            <!-- Resources -->
            <div id="k8s-res-opts" style="margin-bottom:15px; display:none; background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0;">
                <div style="font-size:12px; font-weight:bold; margin-bottom:10px; color:#64748b">资源限制 (Resources)</div>
                <div class="grid-4">
                    <div><div class="cron-label">CPU Request</div><input id="k8s-cpu-r" placeholder="100m"></div>
                    <div><div class="cron-label">Mem Request</div><input id="k8s-mem-r" placeholder="128Mi"></div>
                    <div><div class="cron-label">CPU Limit</div><input id="k8s-cpu-l" placeholder="500m"></div>
                    <div><div class="cron-label">Mem Limit</div><input id="k8s-mem-l" placeholder="512Mi"></div>
                </div>
            </div>

            <!-- Service Options -->
            <div id="k8s-svc-opts" class="grid-4" style="margin-bottom:15px; display:none">
                <div><div class="cron-label">类型 (Type)</div><select id="k8s-svc-type"><option value="ClusterIP">ClusterIP</option><option value="NodePort">NodePort</option><option value="LoadBalancer">LoadBalancer</option></select></div>
                <div><div class="cron-label">服务端口</div><input type="number" id="k8s-svc-port" value="80"></div>
                <div><div class="cron-label">目标端口</div><input type="number" id="k8s-target-port" value="80"></div>
            </div>

            <!-- Ingress Options -->
            <div id="k8s-ing-opts" class="grid-4" style="margin-bottom:15px; display:none">
                <div style="grid-column: span 2"><div class="cron-label">域名 (Host)</div><input id="k8s-host" value="example.com"></div>
                <div style="grid-column: span 2"><div class="cron-label">路径 (Path)</div><input id="k8s-path" value="/"></div>
            </div>

            <!-- CronJob Options -->
            <div id="k8s-cron-opts" style="margin-bottom:15px; display:none">
                <div class="cron-label">调度周期 (Schedule)</div><input id="k8s-schedule" value="*/1 * * * *">
            </div>

            <!-- Env / Data -->
            <div id="k8s-env-box" style="margin-bottom:20px">
                <div class="cron-label" id="k8s-env-lbl">环境变量 (Key=Value, 每行一个)</div>
                <textarea id="k8s-env" style="height:100px; font-family:monospace;" placeholder="DB_HOST=localhost&#10;DB_PORT=5432"></textarea>
            </div>

            <button class="btn" style="width:100%; margin-bottom:20px" onclick="doK8s()">⚙️ 生成 YAML</button>
            
            <div class="editor-box">
                <div class="editor-header"><span>生成结果</span><button class="icon-btn" onclick="copy('k8s-res')"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="k8s-res" class="editor-content" style="height:300px" readonly></textarea>
            </div>
        </div>

        <div id="k8s-cmd" class="panel">
            <h2>Kubernetes 常用命令</h2>
            <div class="row">
                <div style="flex:1">
                    <div class="cron-label">操作 (Action)</div>
                    <select id="kc-action" onchange="updateKcUI(); doK8sCmd()" style="width:100%; font-weight:bold; color:var(--primary)">
                        <option value="get">查看列表 (Get)</option>
                        <option value="describe">查看详情 (Describe)</option>
                        <option value="delete">删除资源 (Delete)</option>
                        <option value="logs">查看日志 (Logs)</option>
                        <option value="exec">进入容器 (Exec)</option>
                        <option value="scale">伸缩副本 (Scale)</option>
                        <option value="port_forward">端口转发 (Port Forward)</option>
                        <option value="rollout_restart">滚动重启 (Rollout Restart)</option>
                        <option value="rollout_status">滚动状态 (Rollout Status)</option>
                        <option value="rollout_history">历史版本 (Rollout History)</option>
                        <option value="rollout_undo">回滚版本 (Rollout Undo)</option>
                    </select>
                </div>
                <div style="flex:1">
                    <div class="cron-label">资源类型 (Resource)</div>
                    <select id="kc-type" onchange="doK8sCmd()" style="width:100%">
                        <option value="pod">Pod</option>
                        <option value="deployment">Deployment</option>
                        <option value="service">Service</option>
                        <option value="ingress">Ingress</option>
                        <option value="configmap">ConfigMap</option>
                        <option value="secret">Secret</option>
                        <option value="statefulset">StatefulSet</option>
                        <option value="job">Job</option>
                        <option value="cronjob">CronJob</option>
                        <option value="node">Node</option>
                        <option value="pvc">PVC</option>
                        <option value="ns">Namespace</option>
                    </select>
                </div>
            </div>
            <div class="grid-4" style="margin-bottom:15px">
                <div><div class="cron-label">命名空间 (-n)</div><input id="kc-ns" value="default" oninput="doK8sCmd()"></div>
                <div><div class="cron-label">输出格式 (-o)</div><select id="kc-out" onchange="doK8sCmd()">
                    <option value="wide" selected>详细 (wide)</option>
                    <option value="yaml">YAML</option>
                    <option value="json">JSON</option>
                    <option value="name">仅名称 (name)</option>
                    <option value="">默认 (Default)</option>
                </select></div>
                <div style="grid-column: span 2"><div class="cron-label">资源名称 (Name)</div><input id="kc-name" placeholder="my-resource-name" oninput="doK8sCmd()"></div>
            </div>
            <div id="kc-extra" class="grid-4" style="margin-bottom:15px; display:none">
                <!-- Dynamic inputs -->
            </div>
            <div class="result-card">
                <div class="result-label">Kubectl Command</div>
                <div id="kc-res" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px; color:var(--primary); font-weight:bold;">kubectl get pods</div>
                <button class="icon-btn" onclick="copy('kc-res')"><svg><use href="#i-copy"></use></svg></button>
            </div>
            <div style="margin-top:10px; font-size:13px; color:#64748b;" id="kc-desc"></div>
        </div>

        <div id="ansible" class="panel">
            <h2>Ansible YAML 生成</h2>
            <div class="grid-4" style="margin-bottom:15px">
                <div style="grid-column: span 2"><div class="cron-label">Playbook 名称</div><input id="ans-name" value="Setup Web Server"></div>
                <div><div class="cron-label">目标主机 (Hosts)</div><input id="ans-hosts" value="webservers"></div>
                <div style="display:flex; align-items:flex-end; padding-bottom:15px; gap:15px">
                    <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ans-become" checked style="width:18px;height:18px;accent-color:var(--primary)"> 提权 (Become)</label>
                    <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ans-facts" checked style="width:18px;height:18px;accent-color:var(--primary)"> 收集事实 (Facts)</label>
                </div>
            </div>
            <div style="margin-bottom:15px">
                <div class="cron-label">变量 (Vars) - Key: Value</div>
                <textarea id="ans-vars" style="height:60px; font-family:monospace;" placeholder="http_port: 80&#10;max_clients: 200"></textarea>
            </div>
            
            <div style="background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0; margin-bottom:20px;">
                <div style="font-weight:bold; color:#475569; margin-bottom:10px; font-size:14px;">添加任务 (Task Builder)</div>
                <div class="row">
                    <select id="ans-mod" onchange="updateAnsModUI()" style="flex:1">
                        <option value="apt">安装包 (Apt/Yum)</option>
                        <option value="service">服务管理 (Service)</option>
                        <option value="copy">复制文件 (Copy)</option>
                        <option value="template">模板渲染 (Template)</option>
                        <option value="file">文件权限/目录 (File)</option>
                        <option value="lineinfile">修改文件行 (LineInFile)</option>
                        <option value="unarchive">解压文件 (Unarchive)</option>
                        <option value="shell">执行命令 (Shell)</option>
                        <option value="git">Git 代码拉取</option>
                        <option value="user">用户管理 (User)</option>
                        <option value="debug">调试信息 (Debug)</option>
                        <option value="block">任务块 (Block/Rescue)</option>
                    </select>
                    <button class="btn success" onclick="addAnsTask()">+ 添加到列表</button>
                </div>
                <div id="ans-mod-opts" class="grid-4" style="margin-top:10px">
                    <!-- Dynamic inputs -->
                </div>
                
                <div style="margin-top:15px; padding-top:15px; border-top:1px dashed #e2e8f0;">
                    <div style="font-size:12px; font-weight:bold; color:#64748b; margin-bottom:10px;">通用选项 (Common)</div>
                    <div class="grid-4">
                        <div><div class="cron-label">注册变量 (Register)</div><input id="am-reg" placeholder="result_var"></div>
                        <div><div class="cron-label">条件 (When)</div><input id="am-when" placeholder="result_var.changed"></div>
                        <div><div class="cron-label">通知 (Notify)</div><input id="am-notify" placeholder="Restart Nginx"></div>
                        <div style="display:flex;align-items:flex-end;padding-bottom:15px"><label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="am-ignore" style="width:18px;height:18px;accent-color:var(--primary)"> 忽略错误</label></div>
                        <div style="grid-column: span 4"><div class="cron-label">循环 (Loop) - 输入列表 (每行一个) 或 变量 ({{ items }})</div><textarea id="am-loop" style="height:60px; font-family:monospace;" placeholder="- item1&#10;- item2&#10;或 {{ my_list }}"></textarea></div>
                    </div>
                </div>
            </div>

            <div style="margin-bottom:20px">
                <div class="cron-label">任务列表 (Tasks YAML)</div>
                <textarea id="ans-tasks" style="height:200px; font-family:monospace;" placeholder="# 点击上方添加任务，或直接在此编辑&#10;- name: Example Task&#10;  debug: msg='Hello'"></textarea>
            </div>
            <div style="margin-bottom:20px">
                <div class="cron-label">处理器 (Handlers YAML)</div>
                <textarea id="ans-handlers" style="height:100px; font-family:monospace;" placeholder="- name: Restart Nginx&#10;  service: name=nginx state=restarted"></textarea>
            </div>
            <button class="btn" style="width:100%; margin-bottom:20px" onclick="doAnsible()">⚙️ 生成 YAML</button>
            <div class="editor-box"><div class="editor-header"><span>生成结果</span><button class="icon-btn" onclick="copy('ans-res')"><svg><use href="#i-copy"></use></svg></button></div><textarea id="ans-res" class="editor-content" style="height:300px" readonly></textarea></div>
        </div>

        <div id="nginx" class="panel">
            <h2>Nginx 配置生成</h2>
            <div class="grid-4">
                <div style="grid-column: span 2"><div class="cron-label">域名 (Server Name)</div><input id="ng-domain" placeholder="example.com" value="example.com"></div>
                <div><div class="cron-label">端口 (Port)</div><input id="ng-port" placeholder="80" value="80"></div>
                <div><div class="cron-label">SPA 模式</div>
                    <div style="margin-top:10px">
                        <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ng-spa" style="width:18px;height:18px;accent-color:var(--primary)"> 开启 (React/Vue)</label>
                    </div>
                </div>
            </div>
            <div class="grid-4" style="margin-bottom:15px">
                <div style="grid-column: span 4"><div class="cron-label">全局根目录 (Global Root)</div><input id="ng-root" placeholder="/var/www/html" value="/var/www/html"></div>
            </div>
            
            <div id="ng-locs-container"></div>
            <button class="btn success" onclick="addNginxLocation()" style="margin-bottom:15px; width:100%">+ 添加路径规则 (Location)</button>

            <div style="margin-bottom:15px">
                 <div class="cron-label">负载均衡 (Upstream Servers) - 每行一个 IP:Port</div>
                 <textarea id="ng-upstream" style="height:80px; font-family:monospace; border:2px solid #e5e7eb; border-radius:10px; padding:10px;" placeholder="127.0.0.1:3001&#10;127.0.0.1:3002"></textarea>
            </div>
            <div style="margin-bottom:10px; font-weight:bold; font-size:13px; color:#64748b">高级设置 (超时与限制)</div>
            <div class="grid-5" style="margin-bottom:15px">
                <div><div class="cron-label">最大上传 (Body)</div><input id="ng-size" placeholder="10m" value="10m"></div>
                <div><div class="cron-label">连接保持 (Keepalive)</div><input id="ng-ka" placeholder="65" value="65"></div>
                <div><div class="cron-label">代理连接超时</div><input id="ng-pct" placeholder="60s"></div>
                <div><div class="cron-label">代理读取超时</div><input id="ng-prt" placeholder="60s"></div>
                <div><div class="cron-label">代理发送超时</div><input id="ng-pst" placeholder="60s"></div>
            </div>
            <div class="row" style="background:#f8fafc; padding:15px; border-radius:8px; border:1px solid #e2e8f0; margin-bottom:20px; flex-direction:column; align-items:stretch; gap:15px">
                <div style="display:flex; gap:20px; flex-wrap:wrap">
                    <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ng-ssl" onchange="toggleSslInputs(); doNginx()" style="width:18px;height:18px;accent-color:var(--primary)"> 启用 HTTPS (SSL)</label>
                    <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ng-force" onchange="doNginx()" style="width:18px;height:18px;accent-color:var(--primary)"> 强制跳转 HTTPS</label>
                    <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="ng-gzip" onchange="doNginx()" style="width:18px;height:18px;accent-color:var(--primary)"> 开启 Gzip</label>
                </div>
                <div id="ssl-inputs" class="grid-4" style="display:none; border-top:1px dashed #cbd5e1; padding-top:15px">
                    <div style="grid-column: span 2"><div class="cron-label">证书路径 (.crt/.pem)</div><input id="ng-crt" placeholder="/etc/nginx/ssl/server.crt"></div>
                    <div style="grid-column: span 2"><div class="cron-label">私钥路径 (.key)</div><input id="ng-key" placeholder="/etc/nginx/ssl/server.key"></div>
                </div>
            </div>
            <button class="btn" style="width:100%; margin-bottom:20px" onclick="doNginx()">⚙️ 生成配置</button>
            <div class="editor-box">
                <div class="editor-header"><span>生成结果</span><button class="icon-btn" onclick="copy('ng-res')"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="ng-res" class="editor-content" style="height:300px" readonly></textarea>
            </div>
        </div>

        <div id="curl" class="panel">
            <h2>cURL 命令生成器</h2>
            <div class="row">
                <select id="curl-method" style="width:100px; font-weight:bold;">
                    <option value="GET">GET</option>
                    <option value="POST">POST</option>
                    <option value="PUT">PUT</option>
                    <option value="DELETE">DELETE</option>
                    <option value="PATCH">PATCH</option>
                </select>
                <input id="curl-url" placeholder="https://api.example.com/v1/resource" style="flex:1">
            </div>
            <div class="grid-4" style="margin-bottom:15px">
                <div style="grid-column: span 2">
                    <div class="cron-label">请求头 (Headers) - JSON 格式 <span id="curl-h-status" style="font-size:12px; margin-left:5px"></span></div>
                    <textarea id="curl-headers" oninput="validateCurlHeaders()" style="height:120px; font-family:monospace;" placeholder='{
  "Authorization": "Bearer token",
  "Content-Type": "application/json"
}'></textarea>
                </div>
                <div style="grid-column: span 2">
                    <div class="cron-label">请求体 (Body)</div>
                    <textarea id="curl-body" style="height:120px; font-family:monospace;" placeholder='{"key": "value"}'></textarea>
                </div>
            </div>
            <button class="btn" style="width:100%; margin-bottom:20px" onclick="doCurl()">🔌 生成命令</button>
            <div class="editor-box">
                <div class="editor-header"><span>生成结果 (cURL)</span><button class="icon-btn" onclick="copy('curl-res')"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="curl-res" class="editor-content" style="height:150px" readonly></textarea>
            </div>
            <div class="editor-box" style="margin-top:20px">
                <div class="editor-header"><span>Python Requests</span><button class="icon-btn" onclick="copy('curl-py')"><svg><use href="#i-copy"></use></svg></button></div>
                <textarea id="curl-py" class="editor-content" style="height:150px" readonly></textarea>
            </div>
        </div>

        <div id="unit" class="panel">
            <h2>单位换算</h2>
            <div class="row">
                <div class="cron-label" style="margin-right:10px">类型:</div>
                <select id="unit-type" onchange="updateUnitUI()" style="flex:1">
                    <option value="storage">存储容量 (Storage)</option>
                    <option value="time">时间 (Time)</option>
                </select>
            </div>
            <div class="grid-4" style="align-items: end;">
                <div><div class="cron-label">数值</div><input type="number" id="unit-val" value="1" oninput="doUnit()"></div>
                <div><div class="cron-label">从 (From)</div><select id="unit-from" onchange="doUnit()"></select></div>
                <div style="text-align:center; padding-bottom:10px; font-size:20px;">➔</div>
                <div><div class="cron-label">到 (To)</div><select id="unit-to" onchange="doUnit()"></select></div>
            </div>
            <div class="result-card" style="margin-top:20px">
                <div class="result-label">换算结果</div>
                <div id="unit-res" class="result-val" style="font-size:24px; color:var(--primary); font-weight:bold;">-</div>
                <button class="icon-btn" onclick="copy('unit-res')"><svg><use href="#i-copy"></use></svg></button>
            </div>
        </div>

        <div id="git-cheat" class="panel">
            <h2>Git 常用命令速查</h2>
            <div class="row">
                <select id="gc-action" onchange="updateGcUI(); doGitCheat()" style="flex:1; font-weight:bold; color:var(--primary)">
                    <option value="undo_commit">撤销最近提交 (Undo Commit)</option>
                    <option value="undo_changes">撤销工作区修改 (Undo Changes)</option>
                    <option value="log_graph">图形化日志 (Log Graph)</option>
                    <option value="tag">打标签并推送 (Tag & Push)</option>
                    <option value="branch_delete">删除分支 (Delete Branch)</option>
                    <option value="stash">暂存并拉取 (Stash & Pull)</option>
                </select>
            </div>
            <div id="gc-inputs" class="grid-4" style="margin-bottom:15px">
                <!-- Dynamic inputs -->
            </div>
            <div class="result-card">
                <div class="result-label">Git Command</div>
                <div id="gc-cmd" class="result-val" style="font-size:16px; display:flex; align-items:center; min-height:36px; color:var(--primary); font-weight:bold;"></div>
                <button class="icon-btn" onclick="copy('gc-cmd')"><svg><use href="#i-copy"></use></svg></button>
            </div>
            <div style="margin-top:10px; font-size:13px; color:#64748b;" id="gc-desc"></div>
        </div>

        <div id="disclaimer" class="panel">
            <h2>免责声明</h2>
            <div style="background:#fff; padding:25px; border-radius:12px; border:1px solid #e2e8f0; color:#334155; line-height:1.8">
                <h3 style="margin-bottom:12px; color:#0f172a; font-size:16px;">1. 服务性质</h3>
                <p style="margin-bottom:20px; color:#475569;">本站提供的所有开发者工具（包括但不限于格式化、转换、生成器等）仅供技术交流、学习和辅助开发使用。工具的计算结果仅供参考，不构成任何专业建议。</p>
                
                <h3 style="margin-bottom:12px; color:#0f172a; font-size:16px;">2. 数据隐私与安全</h3>
                <p style="margin-bottom:20px; color:#475569;">本站基于 Cloudflare Workers 构建，部分计算逻辑在云端运行。虽然我们<strong>不会持久化存储</strong>您的任何输入数据，但鉴于网络环境的复杂性，<strong>请勿在工具中输入任何真实的敏感信息</strong>（如生产环境密码、私钥、API Token、个人身份信息等）。对于因用户主动输入敏感信息而导致的泄露风险，本站不承担责任。</p>
                
                <h3 style="margin-bottom:12px; color:#0f172a; font-size:16px;">3. 免责条款</h3>
                <p style="margin-bottom:20px; color:#475569;">本站不对工具的准确性、及时性、完整性或稳定性做任何明示或暗示的保证。对于因使用或无法使用本站服务而导致的任何直接、间接、附带或后果性的损失（包括但不限于业务中断、数据丢失、利润损失），本站概不负责。</p>
                
                <div style="margin-top:30px; padding-top:20px; border-top:1px dashed #cbd5e1; font-size:13px; color:#94a3b8;">
                    <p>继续使用本站服务即表示您已阅读并同意上述条款。</p>
                </div>
            </div>
        </div>

    </main>

    <script>
        // Core
        function toggleGroup(el) { el.parentElement.classList.toggle('collapsed'); }
        function nav(id, el) { 
            document.querySelectorAll('.panel').forEach(p=>p.classList.remove('active')); 
            document.getElementById(id).classList.add('active'); 
            document.querySelectorAll('.link').forEach(l=>l.classList.remove('active')); 
            el.classList.add('active'); 
        }
        function toast(m, t='success') { 
            const el = document.getElementById('toast'); 
            el.innerText = m; 
            el.style.backgroundColor = t === 'error' ? '#ef4444' : '#334155';
            el.style.opacity = 1; 
            setTimeout(() => el.style.opacity = 0, 2000); 
        }
        function copy(id) { const e=document.getElementById(id); const t=e.tagName==='TEXTAREA'||e.tagName==='INPUT'?e.value:e.innerText; if(!t)return toast('无内容', 'error'); navigator.clipboard.writeText(t).then(()=>toast('已复制')); }
        function setVal(id,v) { document.getElementById(id).value=v; }
        async function post(u,d) { 
            try {
                const r = await fetch('/api'+u, {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify(d)
                });
                if(!r.ok) throw await r.text();
                return await r.json();
            } catch(e) {
                let msg = e;
                // 捕获网络错误（如后端未启动或网络中断）
                if (e instanceof TypeError && e.message.includes('fetch')) {
                    msg = '网络请求失败: 请检查服务是否运行或网络连接';
                }
                toast(msg, 'error');
                throw e;
            } 
        }

        // Menu Filter
        function filterMenu() {
            const v = document.getElementById('menu-search').value.toLowerCase();
            document.querySelectorAll('.link').forEach(l => {
                const t = l.innerText.toLowerCase();
                const match = t.includes(v);
                // 隐藏父级 li 元素以保持布局整洁
                l.parentElement.style.display = match ? 'block' : 'none';
            });
            document.querySelectorAll('.menu-group').forEach(g => {
                const visible = Array.from(g.querySelectorAll('.link')).some(l => l.parentElement.style.display !== 'none');
                g.style.display = visible ? 'block' : 'none';
                if(v && visible) g.classList.remove('collapsed');
            });
        }

        // Logic
        let qrLogo = null;
        function handleLogo(input) {
            if (input.files && input.files[0]) {
                const f = input.files[0];
                document.getElementById('qr-upload-text').innerText = f.name.substring(0, 10) + '...';
                const reader = new FileReader();
                reader.onload = function(e) {
                    qrLogo = new Image();
                    qrLogo.src = e.target.result;
                    toast('Logo 已加载');
                };
                reader.readAsDataURL(f);
            }
        }

        async function doQr() {
            try {
                let text = document.getElementById('qr-text').value;
                if(!text) return toast('请输入内容', 'error');
                let d = await post('/qrcode', {text: text});
                if (d.error) {
                    return toast('生成失败: ' + d.error, 'error');
                }

                const img = new Image();
                img.src = 'data:image/svg+xml;base64,' + btoa(unescape(encodeURIComponent(d.svg)));
                img.onload = () => {
                    const canvas = document.getElementById('qr-canvas');
                    const ctx = canvas.getContext('2d');
                    canvas.width = 300;
                    canvas.height = 300;
                    ctx.drawImage(img, 0, 0, 300, 300);
                    
                    if (qrLogo) {
                        const size = 60;
                        const pos = (300 - size) / 2;
                        ctx.fillStyle = '#fff';
                        ctx.fillRect(pos - 2, pos - 2, size + 4, size + 4);
                        ctx.drawImage(qrLogo, pos, pos, size, size);
                    }
                    
                    // Show download button
                    const dl = document.getElementById('qr-dl');
                    dl.href = canvas.toDataURL("image/png");
                    dl.download = "qrcode.png";
                    dl.style.display = "inline-flex";
                    toast('生成成功');
                };
            } catch(e) {}
        }

        // Logic
        function testRegex() {
            const pattern = document.getElementById('reg-p').value;
            const text = document.getElementById('reg-t').value;
            if(!pattern) return toast('请输入正则表达式', 'error');
            
            try {
                const re = new RegExp(pattern, 'gm');
                const matches = text.match(re);
                
                if (matches && matches.length > 0) {
                    document.getElementById('reg-r').value = matches.join('\n');
                    document.getElementById('reg-count').innerText = `(${matches.length})`;
                    toast(`匹配到 ${matches.length} 项`);
                } else {
                    document.getElementById('reg-r').value = '';
                    document.getElementById('reg-count').innerText = '(0)';
                    toast('❌ 未匹配到任何内容', 'error');
                }
            } catch(e) {
                document.getElementById('reg-r').value = e.message;
                toast('正则语法错误', 'error');
            }
        }

        async function doSubnet() {
            const ip = document.getElementById('sn-ip').value.trim();
            const cidr = document.getElementById('sn-cidr').value;
            
            if(!ip) {
                toast('请输入 IP 地址', 'error');
                return;
            }
            
            if(!cidr) {
                toast('请输入 CIDR 前缀', 'error');
                return;
            }
            
            // 验证 IP 地址格式
            const ipRegex = /^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
            if(!ipRegex.test(ip)) {
                toast('IP 地址格式错误', 'error');
                return;
            }
            
            // 验证 CIDR 前缀范围
            const cidrNum = parseInt(cidr);
            if(isNaN(cidrNum) || cidrNum < 0 || cidrNum > 32) {
                toast('CIDR 前缀必须在 0-32 之间', 'error');
                return;
            }
            
            try {
                let d = await post('/subnet', {ip: ip, cidr: cidrNum});
                if (!d.valid) {
                    return toast('计算失败: 无效的 IP 或 CIDR', 'error');
                }

                document.getElementById('sn-cidr-res').innerText = d.cidr;
                document.getElementById('sn-mask').innerText = d.mask;
                document.getElementById('sn-wild').innerText = d.wildcard;
                document.getElementById('sn-net').innerText = d.network;
                document.getElementById('sn-broad').innerText = d.broadcast;
                document.getElementById('sn-total').innerText = d.total_hosts;
                document.getElementById('sn-usable').innerText = d.usable_hosts;
                document.getElementById('sn-class').innerText = d.ip_class;
                document.getElementById('sn-type').innerText = d.ip_type;
                document.getElementById('sn-first').innerText = d.first_ip;
                document.getElementById('sn-last').innerText = d.last_ip;
                document.getElementById('sn-bin-mask').innerText = d.binary_mask;
                document.getElementById('sn-bin-ip').innerText = d.binary_ip;
            } catch(e) {
                toast('子网计算失败', 'error');
            }
        }

        async function doCol() { 
            try{
                let d=await post('/color',{input:document.getElementById('col-i').value}); 
                if(d.valid){
                    document.getElementById('c-hex').innerText=d.hex; 
                    document.getElementById('c-rgb').innerText=d.rgb;
                    document.getElementById('c-hsl').innerText=d.hsl||'-'; 
                    document.getElementById('c-cmyk').innerText=d.cmyk||'-';
                    document.getElementById('col-p').value=d.hex;
                    let p=document.getElementById('color-preview'); p.style.backgroundColor=d.hex; p.style.color=d.hex>'#888888'?'#000':'#fff';
                    p.innerText = d.hex;
                }
            }catch(e){} 
        }

        async function doPwd() { try{let d=await post('/password',{length:parseInt(document.getElementById('pwd-len').value),uppercase:true,lowercase:true,numbers:true,symbols:true});document.getElementById('pwd-res').value=d.password;}catch(e){} }
        async function doToken() { 
          try {
            const len = parseInt(document.getElementById('tok-len').value) || 32;
            let d = await post('/token', {
                length: len,
                uppercase: true,
                lowercase: true,
                numbers: true,
                symbols: true
            });
            document.getElementById('tok-res').value = d.token;
          } catch(e) {
            toast('Token 生成失败', 'error');
          }
        }
        async function doUrl(a) { 
          let v=document.getElementById('url-in').value;
          if(!v) {
            toast('请输入 URL 或文本', 'error');
            document.getElementById('url-out').value='';
            return;
          }
          try {
            let d=await post('/url',{input:v});
            document.getElementById('url-out').value=a=='enc'?d.encoded:d.decoded;
          } catch(e) {
            toast('URL 编解码失败', 'error');
            document.getElementById('url-out').value='';
          }
        }
        async function doUrlParse() {
            const input = document.getElementById('url-parse-in').value;
            const resultsEl = document.getElementById('url-parse-results');
            if (!input) {
                resultsEl.style.display = 'none';
                return;
            }
            try {
                const d = await post('/url', { input: input });
                
                document.getElementById('url-p-protocol').innerText = d.protocol || '-';
                document.getElementById('url-p-host').innerText = d.host || '-';
                document.getElementById('url-p-path').innerText = d.path || '-';

                const paramsTable = document.getElementById('url-params-table');
                const paramsTableBody = paramsTable.querySelector('tbody');
                const noParamsEl = document.getElementById('url-no-params');
                paramsTableBody.innerHTML = '';

                if (d.params && d.params.length > 0) {
                    d.params.forEach(p => {
                        const row = paramsTableBody.insertRow();
                        row.style.borderBottom = "1px solid #f0f0f0";
                        const keyCell = row.insertCell();
                        keyCell.style.padding = "12px 10px";
                        keyCell.style.fontFamily = "monospace";
                        keyCell.style.color = "#334155";
                        keyCell.textContent = p[0];

                        const valCell = row.insertCell();
                        valCell.style.padding = "12px 10px";
                        valCell.style.fontFamily = "monospace";
                        valCell.style.wordBreak = "break-all";
                        valCell.textContent = p[1];
                    });
                    noParamsEl.style.display = 'none';
                    paramsTable.style.display = 'table';
                } else {
                    noParamsEl.style.display = 'block';
                    paramsTable.style.display = 'none';
                }
                resultsEl.style.display = 'block';
            } catch (e) {
                toast('URL 解析失败，请检查格式', 'error');
                resultsEl.style.display = 'none';
            }
        }
        async function doJsEnc() { 
          let v=document.getElementById('js-in').value;
          if(!v) {
            toast('请输入 JavaScript 代码', 'error');
            document.getElementById('js-out').value='';
            return;
          }
          try {
            let d=await post('/js-enc',{js:v});
            document.getElementById('js-out').value=d.result;
          } catch(e) {
            toast('JS 混淆失败', 'error');
            document.getElementById('js-out').value='';
          }
        }

        async function doSql() { 
          let v=document.getElementById('sql-in').value;
          if(!v) {
            toast('请输入 SQL 语句', 'error');
            document.getElementById('sql-out').value='';
            return;
          }
          try {
            let d=await post('/sql',{sql:v});
            document.getElementById('sql-out').value=d.result;
          } catch(e) {
            toast('SQL 格式化失败', 'error');
            document.getElementById('sql-out').value='';
          }
        }
        function upCron() { document.getElementById('cron-res').value=['c-m','c-h','c-d','c-mo','c-w'].map(k=>document.getElementById(k).value||'*').join(' '); }
        async function doCron() { 
          let v=document.getElementById('cron-res').value;
          if(!v) {
            toast('请输入 Cron 表达式', 'error');
            document.getElementById('cron-out').innerText='';
            return;
          }
          try {
            let d=await post('/cron',{cron:v});
            document.getElementById('cron-out').innerText=d.valid?d.next_runs.join('\n'):d.error;
            if(!d.valid) {
              toast('Cron 表达式格式错误', 'error');
            }
          } catch(e) {
            toast('Cron 验证失败', 'error');
            document.getElementById('cron-out').innerText='';
          }
        }
        async function doMd5() { 
          let v=document.getElementById('md5-in').value;
          if(!v) {
            toast('请输入文本', 'error');
            document.getElementById('m32l').innerText='';
            document.getElementById('m32u').innerText='';
            document.getElementById('m16l').innerText='';
            document.getElementById('m16u').innerText='';
            return;
          }
          try {
            let d=await post('/md5',{text:v});
            document.getElementById('m32l').innerText=d.md5_32_lower;
            document.getElementById('m32u').innerText=d.md5_32_upper;
            document.getElementById('m16l').innerText=d.md5_16_lower||d.md5_32_lower.substring(8,24);
            document.getElementById('m16u').innerText=d.md5_16_upper||d.md5_32_upper.substring(8,24);
          } catch(e) {
            toast('MD5 计算失败', 'error');
            document.getElementById('m32l').innerText='';
            document.getElementById('m32u').innerText='';
            document.getElementById('m16l').innerText='';
            document.getElementById('m16u').innerText='';
          }
        }
        function fillTime() { document.getElementById('ts-in').value=Math.floor(Date.now()/1000); doDate(); }
        async function doDate() { 
          let v=document.getElementById('ts-in').value;
          if(!v) {
            toast('请输入时间戳或日期', 'error');
            document.getElementById('ts-s').innerText='';
            document.getElementById('ts-ms').innerText='';
            document.getElementById('ts-iso').innerText='';
            document.getElementById('ts-utc').innerText='';
            document.getElementById('ts-loc').innerText='';
            return;
          }
          try {
            let d = await post('/date', { input: v });
            if (d.valid) {
                document.getElementById('ts-s').innerText = d.unix_sec;
                document.getElementById('ts-ms').innerText = d.unix_milli;
                document.getElementById('ts-iso').innerText = d.iso_8601;
                document.getElementById('ts-utc').innerText = d.human_utc;
                document.getElementById('ts-loc').innerText = new Date(d.unix_milli).toLocaleString();
            } else {
                toast(d.error || '无效的日期', 'error');
                ['ts-s', 'ts-ms', 'ts-iso', 'ts-utc', 'ts-loc'].forEach(id => document.getElementById(id).innerText = '');
            }
          } catch(e) {
            toast('时间转换失败', 'error');
            ['ts-s', 'ts-ms', 'ts-iso', 'ts-utc', 'ts-loc'].forEach(id => document.getElementById(id).innerText = '');
          }
        }
        async function doDiff() { 
          let o=document.getElementById('diff-a').value, 
              n=document.getElementById('diff-b').value; 
          if(o&&n) {
            try {
              let d=await post('/diff',{old:o,new:n});
              let h="";
              if(d.chunks && d.chunks.length > 0) {
                let hasDifferences = false;
                d.chunks.forEach(c=>{
                  let s="";
                  if(c.tag=='insert') {
                    s="background:#dcfce7;color:#166534";
                    hasDifferences = true;
                  } else if(c.tag=='delete') {
                    s="background:#fee2e2;color:#991b1b;text-decoration:line-through";
                    hasDifferences = true;
                  }
                  h+=`<span style="${s};display:block">${c.text.replace(/</g,'&lt;')}</span>`;
                });
                if (!hasDifferences) {
                  h="<span style='color:#64748b'>没有发现差异，两个文本内容相同</span>";
                }
              } else {
                // 当没有差异时显示提示信息
                h="<span style='color:#64748b'>没有发现差异，两个文本内容相同</span>";
              }
              document.getElementById('diff-res').innerHTML=h;
            } catch(e) {
              console.error('Error comparing text:', e);
              document.getElementById('diff-res').innerHTML="<span style='color:#ef4444'>对比过程中发生错误</span>";
            }
          } else {
            // 当输入为空时显示提示信息
            document.getElementById('diff-res').innerHTML="<span style='color:#f59e0b'>请输入要对比的文本</span>";
          }
        }
        async function doRegGen() { 
          let key=document.getElementById('reg-key').value;
          if(!key) {
            toast('请选择正则表达式类型', 'error');
            document.getElementById('reg-p').value='';
            return;
          }
          try {
            let d=await post('/regex-gen',{key:key});
            if(d.pattern) {
              document.getElementById('reg-p').value=d.pattern;
              toast('正则表达式生成成功', 'success');
            } else {
              toast('请选择有效的正则表达式类型', 'error');
              document.getElementById('reg-p').value='';
            }
          } catch(e) {
            toast('正则表达式生成失败', 'error');
            document.getElementById('reg-p').value='';
          }
        }
        function toggleRegBuilder() {
            const el = document.getElementById('reg-builder');
            el.style.display = el.style.display === 'none' ? 'block' : 'none';
        }
        async function doRegBuild() {
            const start = document.getElementById('rb-start').value;
            const notStart = document.getElementById('rb-not-start').value;
            const end = document.getElementById('rb-end').value;
            const notEnd = document.getElementById('rb-not-end').value;
            const has = document.getElementById('rb-has').value;
            const notHas = document.getElementById('rb-not-has').value;

            const conflicts = [];
            // 互斥条件检测
            if (start && notStart && start === notStart) conflicts.push('"开头是"与"开头不是"');
            if (end && notEnd && end === notEnd) conflicts.push('"结尾是"与"结尾不是"');
            if (has && notHas && has === notHas) conflicts.push('"包含"与"不包含"');
            // 逻辑蕴含检测 (如果开头是X，则必然包含X，若同时禁止包含X则冲突)
            if (start && notHas && start === notHas) conflicts.push('"开头是"与"不包含"');
            if (end && notHas && end === notHas) conflicts.push('"结尾是"与"不包含"');

            if (conflicts.length > 0) {
                toast('⚠️ 逻辑冲突: ' + conflicts.join('; ') + ' 内容相同', 'error');
            }

            try {
                let d = await post('/regex-build', {
                    starts_with: start,
                    not_starts_with: notStart,
                    ends_with: end,
                    not_ends_with: notEnd,
                    contains: has,
                    not_contains: notHas
                });
                document.getElementById('reg-p').value = d.pattern;
                if(document.getElementById('reg-t').value) testRegex();
            } catch(e) {
                console.error(e);
            }
        }
        async function doReg() { testRegex(); } // Mapping old call to new logic
        async function doUuid() { try{let d=await post('/uuid',{count:parseInt(document.getElementById('uid-n').value),hyphens:true,uppercase:false});document.getElementById('uid-res').value=d.uuids.join('\n');}catch(e){} }
        
        // SSH Key Generator Logic
        async function doSshKey() {
            const bits = parseInt(document.getElementById('ssh-algo').value);
            const comment = document.getElementById('ssh-comment').value;
            const btn = document.querySelector('#ssh-key .btn');
            const originalText = btn.innerText;
            
            try {
                btn.innerText = "生成中...";
                btn.disabled = true;
                
                // 1. Generate Key Pair
                const keys = await window.crypto.subtle.generateKey(
                    { name: "RSASSA-PKCS1-v1_5", modulusLength: bits, publicExponent: new Uint8Array([1, 0, 1]), hash: "SHA-256" },
                    true, ["sign", "verify"]
                );

                // 2. Export Private Key (PKCS#8 -> PEM)
                const pkcs8 = await window.crypto.subtle.exportKey("pkcs8", keys.privateKey);
                const privBody = btoa(String.fromCharCode(...new Uint8Array(pkcs8))).match(/.{1,64}/g).join('\n');
                const privPem = `-----BEGIN PRIVATE KEY-----\n${privBody}\n-----END PRIVATE KEY-----`;

                // 3. Export Public Key (JWK -> OpenSSH)
                const jwk = await window.crypto.subtle.exportKey("jwk", keys.publicKey);
                const pubSsh = await jwkToOpenSsh(jwk, comment);

                document.getElementById('ssh-priv').value = privPem;
                document.getElementById('ssh-pub').value = pubSsh;
                toast('密钥生成成功');
            } catch(e) {
                console.error(e);
                toast('生成失败: ' + e.message, 'error');
            } finally {
                btn.innerText = originalText;
                btn.disabled = false;
            }
        }

        async function jwkToOpenSsh(jwk, comment) {
            const b64uToBuf = (str) => {
                const bin = atob(str.replace(/-/g, '+').replace(/_/g, '/'));
                const buf = new Uint8Array(bin.length);
                for(let i=0; i<bin.length; i++) buf[i] = bin.charCodeAt(i);
                return buf;
            };
            
            const e = b64uToBuf(jwk.e);
            const n = b64uToBuf(jwk.n);
            const type = "ssh-rsa";
            
            // Helper to write [length][data]
            const parts = [type, e, n];
            let totalLen = 0;
            parts.forEach(p => {
                totalLen += 4; // length field
                if (typeof p === 'string') totalLen += p.length;
                else {
                    totalLen += p.length;
                    if (p[0] & 0x80) totalLen++; // padding for mpint if MSB set
                }
            });

            const buf = new Uint8Array(totalLen);
            let offset = 0;
            const view = new DataView(buf.buffer);

            parts.forEach(p => {
                if (typeof p === 'string') {
                    view.setUint32(offset, p.length);
                    offset += 4;
                    for(let i=0; i<p.length; i++) buf[offset++] = p.charCodeAt(i);
                } else {
                    let pad = (p[0] & 0x80) ? 1 : 0;
                    view.setUint32(offset, p.length + pad);
                    offset += 4;
                    if(pad) buf[offset++] = 0;
                    buf.set(p, offset);
                    offset += p.length;
                }
            });

            const b64 = btoa(String.fromCharCode(...buf));
            return `${type} ${b64} ${comment}`;
        }

        function downloadSshKeys() {
            const priv = document.getElementById('ssh-priv').value;
            const pub = document.getElementById('ssh-pub').value;
            if(!priv || !pub) return toast('请先生成密钥', 'error');
            
            const dl = (content, name) => {
                const a = document.createElement('a');
                a.href = URL.createObjectURL(new Blob([content], {type: 'text/plain'}));
                a.download = name;
                a.click();
            };
            dl(priv, 'id_rsa');
            setTimeout(() => dl(pub, 'id_rsa.pub'), 500);
        }

        async function doJwt() { 
            try {
                let d = await post('/jwt', {token: document.getElementById('jwt-in').value});
                if (d.error) {
                    toast(d.error, 'error');
                    document.getElementById('jwt-h').value = '解析失败: ' + d.error;
                    document.getElementById('jwt-p').value = '';
                } else {
                    document.getElementById('jwt-h').value = d.header;
                    document.getElementById('jwt-p').value = d.payload;
                    toast('解析成功');
                }
            } catch(e) {} 
        }
        async function doB64(a) { 
          let v=document.getElementById('b64-in').value;
          if(!v) {
            toast('请输入文本', 'error');
            document.getElementById('b64-out').value='';
            return;
          }
          try {
            let d=await post('/base64',{text:v,action:a});
            document.getElementById('b64-out').value=d.result;
          } catch(e) {
            toast('Base64 转换失败', 'error');
            document.getElementById('b64-out').value='';
          }
        }
        async function doJson(m) { 
          let v=document.getElementById('json-in').value;
          if(!v) {
            toast('请输入 JSON 数据', 'error');
            document.getElementById('json-out').value='';
            return;
          }
          try {
            // 验证 JSON 格式
            JSON.parse(v);
            let d=await post('/json',{input:v});
            document.getElementById('json-out').value=m=='min'?d.minified:d.pretty;
          } catch(e) {
            toast('JSON 格式错误', 'error');
            document.getElementById('json-out').value='';
          }
        }
        async function doEsc() { try{let d=await post('/escape',{text:document.getElementById('esc-in').value,mode:document.getElementById('esc-m').value});document.getElementById('esc-out').value=d.result;}catch(e){} }
        async function doCase() { let v=document.getElementById('case-in').value; if(!v) return; try{let d=await post('/case',{text:v,mode:document.getElementById('case-m').value});document.getElementById('case-out').value=d.result;}catch(e){} }
        async function convertYaml() { 
          try {
            const yamlInput = document.getElementById('yaml-input').value;
            if (!yamlInput) {
              toast('请输入 YAML 内容', 'error');
              return;
            }
            let d=await post('/yaml-to-toml',{yaml: yamlInput});
            if (d.error) {
              document.getElementById('toml-output').value='转换失败：' + d.error;
              toast('转换失败', 'error');
            } else if (d && d.result) {
              document.getElementById('toml-output').value=d.result;
              toast('转换成功', 'success');
            } else {
              document.getElementById('toml-output').value='转换失败：无结果';
              toast('转换失败：无结果', 'error');
            }
          } catch(e) {
            console.error('YAML 转 TOML 错误:', e);
            toast('转换过程中发生错误', 'error');
            document.getElementById('toml-output').value='转换失败：' + (e.message || e);
          }
        }
        async function convertToml() { 
          try {
            const tomlInput = document.getElementById('toml-input').value;
            if (!tomlInput) {
              toast('请输入 TOML 内容', 'error');
              return;
            }
            let d=await post('/toml-to-yaml',{toml: tomlInput});
            if (d.error) {
              document.getElementById('yaml-output').value='转换失败：' + d.error;
              toast('转换失败', 'error');
            } else if (d && d.result) {
              document.getElementById('yaml-output').value=d.result;
              toast('转换成功', 'success');
            } else {
              document.getElementById('yaml-output').value='转换失败：无结果';
              toast('转换失败：无结果', 'error');
            }
          } catch(e) {
            console.error('TOML 转 YAML 错误:', e);
            toast('转换过程中发生错误', 'error');
            document.getElementById('yaml-output').value='转换失败：' + (e.message || e);
          }
        }
        function upChmod(c){let u=(document.getElementById('c_ur').checked?4:0)+(document.getElementById('c_uw').checked?2:0)+(document.getElementById('c_ux').checked?1:0),g=(document.getElementById('c_gr').checked?4:0)+(document.getElementById('c_gw').checked?2:0)+(document.getElementById('c_gx').checked?1:0),o=(document.getElementById('c_or').checked?4:0)+(document.getElementById('c_ow').checked?2:0)+(document.getElementById('c_ox').checked?1:0);if(c)document.getElementById('chmod-octal').value=""+u+g+o;else{let v=document.getElementById('chmod-octal').value;if(v.length===3){let n=v.split('').map(Number);if(n.every(x=>x>=0&&x<=7)){u=n[0];g=n[1];o=n[2];document.getElementById('c_ur').checked=u&4;document.getElementById('c_uw').checked=u&2;document.getElementById('c_ux').checked=u&1;document.getElementById('c_gr').checked=g&4;document.getElementById('c_gw').checked=g&2;document.getElementById('c_gx').checked=g&1;document.getElementById('c_or').checked=o&4;document.getElementById('c_ow').checked=o&2;document.getElementById('c_ox').checked=o&1}}}fetchChmod(document.getElementById('chmod-octal').value)}
        async function fetchChmod(o){try{let f=document.getElementById('chmod-file').value;let d=await post('/chmod',{octal:o,file:f});if(d.valid)document.getElementById('chmod-command').innerText=d.command;}catch(e){} }
        async function doTar() { try{let d=await post('/tar',{op:document.getElementById('tar-op').value,comp:document.getElementById('tar-comp').value,verbose:document.getElementById('tar-v').checked,archive:document.getElementById('tar-arch').value,files:document.getElementById('tar-files').value});document.getElementById('tar-cmd').innerText=d.command;}catch(e){} }
        async function doPs() { 
            try {
                let d = await post('/ps', {
                    format: document.getElementById('ps-fmt').value,
                    sort: document.getElementById('ps-sort').value,
                    tree: document.getElementById('ps-tree').checked,
                    filter: document.getElementById('ps-filter').value,
                    wide: document.getElementById('ps-wide').checked,
                    threads: document.getElementById('ps-threads').checked,
                    user: document.getElementById('ps-user').value,
                    pid: document.getElementById('ps-pid').value
                });
                document.getElementById('ps-cmd').innerText = d.command;
            } catch(e) {} 
        }
        async function doTcpdump() { try{let d=await post('/tcpdump',{interface:document.getElementById('td-if').value,protocol:document.getElementById('td-proto').value,host:document.getElementById('td-host').value,port:document.getElementById('td-port').value,verbose:document.getElementById('td-v').checked,ascii:document.getElementById('td-a').checked,hex:document.getElementById('td-x').checked,write_file:document.getElementById('td-w').value,count:document.getElementById('td-c').value});document.getElementById('td-cmd').innerText=d.command;}catch(e){} }
        
        function updateGitUI() {
            const c = document.getElementById('git-cmd').value;
            const show = (id, v) => document.getElementById(id).style.display = v ? '' : 'none';
            const lbl = (t) => document.getElementById('g-target-lbl').innerText = t;
            const isCheat = ['undo_commit', 'undo_changes', 'log_graph', 'tag', 'branch_delete', 'stash'].includes(c);
            
            // Defaults
            show('g-target-box', false); show('g-tag-box', false); show('g-msg-box', false); show('g-remote-box', false); show('g-branch-box', false);
            show('git-opts', !isCheat); // Hide options grid for cheat sheet
            if(!isCheat) ['all','force','rebase','amend','hard','new','tags','oneline','graph'].forEach(k => show('opt-'+k, false));

            if(c==='init') { show('g-target-box',true); lbl('目录 (可选)'); }
            if(c==='clone') { show('g-target-box',true); lbl('仓库 URL'); }
            if(c==='add') { show('g-target-box',true); lbl('文件路径'); show('opt-all',true); }
            if(c==='commit') { show('g-msg-box',true); show('opt-all',true); show('opt-amend',true); }
            if(c==='push') { show('g-remote-box',true); show('g-branch-box',true); show('opt-force',true); show('opt-tags',true); }
            if(c==='pull') { show('g-remote-box',true); show('g-branch-box',true); show('opt-rebase',true); }
            if(c==='checkout') { show('g-target-box',true); lbl('分支名 / Commit'); show('opt-new',true); }
            if(c==='merge') { show('g-target-box',true); lbl('要合并的分支'); }
            if(c==='log') { show('opt-oneline',true); show('opt-graph',true); }
            if(c==='reset') { show('g-target-box',true); lbl('Commit Hash'); show('opt-hard',true); }
            if(c==='remote') { show('g-target-box',true); lbl('仓库 URL'); show('g-remote-box',true); }
            
            // Cheat Sheet UI
            if(c==='tag') { show('g-tag-box',true); show('g-msg-box',true); }
            if(c==='branch_delete') { show('g-branch-box',true); }
        }
        async function doGit() {
            const cmd = document.getElementById('git-cmd').value;
            const cheatActions = ['undo_commit', 'undo_changes', 'log_graph', 'tag', 'branch_delete', 'stash'];
            
            try {
                if (cheatActions.includes(cmd)) {
                    let d = await post('/git-cmd', {
                        action: cmd,
                        tag: document.getElementById('g-tag').value,
                        msg: document.getElementById('g-msg').value,
                        branch: document.getElementById('g-branch').value
                    });
                    document.getElementById('git-cmd-res').innerText = d.command;
                    document.getElementById('git-desc').innerText = d.description || '';
                } else {
                    let d = await post('/git', {
                        cmd: cmd,
                        target: document.getElementById('g-target').value,
                        msg: document.getElementById('g-msg').value,
                        remote: document.getElementById('g-remote').value,
                        branch: document.getElementById('g-branch').value,
                        opt_force: document.getElementById('go-force').checked,
                        opt_rebase: document.getElementById('go-rebase').checked,
                        opt_all: document.getElementById('go-all').checked,
                        opt_amend: document.getElementById('go-amend').checked,
                        opt_hard: document.getElementById('go-hard').checked,
                        opt_new_branch: document.getElementById('go-new').checked,
                        opt_tags: document.getElementById('go-tags').checked,
                        opt_oneline: document.getElementById('go-oneline').checked,
                        opt_graph: document.getElementById('go-graph').checked
                    });
                    document.getElementById('git-cmd-res').innerText = d.command;
                    document.getElementById('git-desc').innerText = '';
                }
            } catch(e) {}
        }
        async function doStrace() {
            try {
                let d = await post('/strace', {
                    target: document.getElementById('st-target').value,
                    is_pid: document.getElementById('st-pid').checked,
                    follow: document.getElementById('st-f').checked,
                    summary: document.getElementById('st-c').checked,
                    timestamp: document.getElementById('st-tt').checked,
                    filter: document.getElementById('st-e').value,
                    string_limit: document.getElementById('st-s').value,
                    output_file: document.getElementById('st-o').value
                });
                document.getElementById('st-cmd').innerText = d.command;
            } catch(e) {}
        }
        async function doIostat() {
            try {
                let d = await post('/iostat', {
                    interval: document.getElementById('io-int').value,
                    count: document.getElementById('io-cnt').value,
                    device: document.getElementById('io-dev').value,
                    extended: document.getElementById('io-x').checked,
                    human: document.getElementById('io-h').checked,
                    timestamp: document.getElementById('io-t').checked,
                    partitions: document.getElementById('io-p').checked,
                    unit: document.getElementById('io-unit').value
                });
                document.getElementById('io-cmd').innerText = d.command;
            } catch(e) {}
        }
        function updateNiceUI() {
            const m = document.getElementById('ni-mode').value;
            document.getElementById('box-nice').style.display = m === 'nice' ? 'block' : 'none';
            document.getElementById('box-renice').style.display = m === 'renice' ? 'grid' : 'none';
        }
        async function doNice() {
            try {
                let d = await post('/nice', {
                    mode: document.getElementById('ni-mode').value,
                    priority: parseInt(document.getElementById('ni-prio').value) || 0,
                    command: document.getElementById('ni-cmd').value,
                    target_type: document.getElementById('ni-type').value,
                    target: document.getElementById('ni-target').value
                });
                document.getElementById('ni-res').innerText = d.command;
            } catch(e) {}
        }
        async function doLs() {
            try {
                let d = await post('/ls', {
                    path: document.getElementById('ls-path').value,
                    long: document.getElementById('ls-l').checked,
                    all: document.getElementById('ls-a').checked,
                    human: document.getElementById('ls-h').checked,
                    time: document.getElementById('ls-t').checked,
                    reverse: document.getElementById('ls-r').checked,
                    recursive: document.getElementById('ls-R').checked,
                    inode: document.getElementById('ls-i').checked,
                    directory: document.getElementById('ls-d').checked,
                    color: document.getElementById('ls-c').checked
                });
                document.getElementById('ls-cmd').innerText = d.command;
            } catch(e) {}
        }
        async function doRsync() {
            try {
                let d = await post('/rsync', {
                    source: document.getElementById('rs-src').value,
                    user: document.getElementById('rs-user').value,
                    host: document.getElementById('rs-host').value,
                    port: document.getElementById('rs-port').value,
                    remote_path: document.getElementById('rs-path').value,
                    archive: document.getElementById('rs-a').checked,
                    compress: document.getElementById('rs-z').checked,
                    verbose: document.getElementById('rs-v').checked,
                    progress: document.getElementById('rs-P').checked,
                    delete: document.getElementById('rs-del').checked,
                    dry_run: document.getElementById('rs-n').checked,
                    ssh: document.getElementById('rs-ssh').checked,
                    exclude: document.getElementById('rs-ex').value
                });
                document.getElementById('rs-cmd').innerText = d.command;
                document.getElementById('rs-ssh-conf').value = d.ssh_config;
            } catch(e) {}
        }
        function updateFwUI() {
            const op = document.getElementById('fw-op').value;
            const isRule = op === 'add' || op === 'remove';
            const isReload = op === 'reload';
            const isList = op === 'list';
            
            document.getElementById('fw-opts').style.display = isReload ? 'none' : 'block';
            document.getElementById('fw-rule-box').style.display = isRule ? 'grid' : 'none';
            document.getElementById('fw-perm-box').style.visibility = isList ? 'hidden' : 'visible';
        }
        async function doFirewall() {
            try {
                let d = await post('/firewall', {
                    op: document.getElementById('fw-op').value,
                    zone: document.getElementById('fw-zone').value,
                    permanent: document.getElementById('fw-perm').checked,
                    target_type: document.getElementById('fw-type').value,
                    target: document.getElementById('fw-target').value
                });
                document.getElementById('fw-cmd').innerText = d.command;
            } catch(e) {}
        }
        function updateSysUI() {
            const op = document.getElementById('sys-op').value;
            const show = (id, v) => document.getElementById(id).parentElement.style.display = v ? 'flex' : 'none';
            
            document.getElementById('sys-svc-box').style.display = op === 'daemon-reload' ? 'none' : 'block';
            
            show('sys-now', ['enable', 'disable', 'mask'].includes(op));
            show('sys-global', ['enable', 'disable', 'mask', 'unmask'].includes(op));
            show('sys-force', ['enable', 'mask'].includes(op));
        }
        async function doSystemctl() {
            try {
                let d = await post('/systemctl', {
                    operation: document.getElementById('sys-op').value,
                    service: document.getElementById('sys-svc').value,
                    user_mode: document.getElementById('sys-user').checked,
                    now: document.getElementById('sys-now').checked,
                    force: document.getElementById('sys-force').checked,
                    global: document.getElementById('sys-global').checked
                });
                document.getElementById('sys-cmd').innerText = d.command;
            } catch(e) {}
        }
        function updateFindUI() {
            const isEmpty = document.getElementById('fd-empty').checked;
            const sizeInput = document.getElementById('fd-size');
            sizeInput.disabled = isEmpty;
            sizeInput.style.opacity = isEmpty ? 0.5 : 1;
        }
        async function doFind() {
            try {
                let d = await post('/find', {
                    path: document.getElementById('fd-path').value,
                    name: document.getElementById('fd-name').value,
                    iname: document.getElementById('fd-iname').checked,
                    target_type: document.getElementById('fd-type').value,
                    size: document.getElementById('fd-size').value,
                    mtime: document.getElementById('fd-mtime').value,
                    empty: document.getElementById('fd-empty').checked,
                    exec: document.getElementById('fd-exec').value
                });
                document.getElementById('fd-cmd').innerText = d.command;
            } catch(e) {}
        }
        function setAwkSnippet() {
            const v = document.getElementById('awk-snippets').value;
            if(v) {
                document.getElementById('awk-code').value = v;
                doAwk();
            }
        }
        function setSedSnippet() {
            const v = document.getElementById('sed-snippets').value;
            if(v) {
                try {
                    const obj = JSON.parse(v);
                    document.getElementById('sed-op').value = obj.op;
                    document.getElementById('sed-pat').value = obj.pat;
                    document.getElementById('sed-rep').value = obj.rep;
                    
                    const f = obj.flags || "";
                    document.getElementById('sed-g').checked = f.includes('g');
                    document.getElementById('sed-p').checked = f.includes('p');
                    document.getElementById('sed-ic').checked = f.includes('I') || f.includes('i');
                    
                    doSed();
                } catch(e) {}
            }
        }
        async function doAwk() {
            try {
                let d = await post('/awk', {
                    separator: document.getElementById('awk-sep').value,
                    variable: document.getElementById('awk-var').value,
                    code: document.getElementById('awk-code').value,
                    file: document.getElementById('awk-file').value
                });
                document.getElementById('awk-cmd').innerText = d.command;
            } catch(e) {}
        }
        async function doSed() {
            try {
                let flags = "";
                if(document.getElementById('sed-g').checked) flags += "g";
                if(document.getElementById('sed-p').checked) flags += "p";
                if(document.getElementById('sed-ic').checked) flags += "I";

                let d = await post('/sed', {
                    operation: document.getElementById('sed-op').value,
                    pattern: document.getElementById('sed-pat').value,
                    replacement: document.getElementById('sed-rep').value,
                    flags: flags,
                    inplace: document.getElementById('sed-i').checked,
                    file: document.getElementById('sed-file').value
                });
                document.getElementById('sed-cmd').innerText = d.command;
            } catch(e) {}
        }

        // Dockerfile Logic
        let stageCount = 0;
        function createStageHTML(index) {
            return `
                <div class="stage" id="stage-${index}">
                    <div class="stage-header">
                        <span class="stage-num">阶段 ${index + 1}</span>
                        ${index > 0 ? `<button class="btn secondary" style="padding:6px 12px;font-size:12px;background:#ef4444;box-shadow:none;" onclick="removeStage(${index})">移除此阶段</button>` : ''}
                    </div>
                    <div class="grid-4">
                        <div style="grid-column: span 2"><div class="cron-label">基础镜像 (FROM)</div><input name="image" placeholder="node:18-alpine"></div>
                        <div style="grid-column: span 2"><div class="cron-label">阶段别名 (AS)</div><input name="as" placeholder="builder"></div>
                    </div>
                    <div class="grid-4" style="margin-top:15px">
                        <div style="grid-column: span 2"><div class="cron-label">工作目录 (WORKDIR)</div><input name="workdir" placeholder="/app"></div>
                        <div style="grid-column: span 2"><div class="cron-label">用户 (USER)</div><input name="user" placeholder="node"></div>
                    </div>
                    <div class="grid-4" style="margin-top:15px">
                        <div style="grid-column: span 2"><div class="cron-label">环境变量 (ENV)</div><textarea name="env" style="height:80px" placeholder="NODE_ENV=production"></textarea></div>
                        <div style="grid-column: span 2"><div class="cron-label">构建参数 (ARG)</div><textarea name="arg" style="height:80px" placeholder="VERSION=1.0.0"></textarea></div>
                    </div>
                    <div class="grid-4" style="margin-top:15px">
                        <div style="grid-column: span 2"><div class="cron-label">复制文件 (COPY)</div><textarea name="copy" style="height:80px" placeholder="package.json ."></textarea></div>
                        <div style="grid-column: span 2"><div class="cron-label">运行命令 (RUN)</div><textarea name="run" style="height:80px" placeholder="npm install"></textarea></div>
                    </div>
                    <div class="grid-4" style="margin-top:15px">
                        <div style="grid-column: span 2"><div class="cron-label">暴露端口 (EXPOSE)</div><input name="expose" placeholder="3000"></div>
                        <div style="grid-column: span 2"><div class="cron-label">挂载点 (VOLUME)</div><input name="volume" placeholder="/data"></div>
                    </div>
                    <div class="grid-4" style="margin-top:15px">
                        <div style="grid-column: span 2"><div class="cron-label">入口点 (ENTRYPOINT)</div><input name="entrypoint" placeholder='["/entrypoint.sh"]'></div>
                        <div style="grid-column: span 2"><div class="cron-label">启动命令 (CMD)</div><input name="cmd" placeholder='["npm", "start"]'></div>
                    </div>
                </div>
            `;
        }
        function addStage() {
            const container = document.getElementById('df-stages-container');
            const div = document.createElement('div');
            div.innerHTML = createStageHTML(stageCount);
            container.appendChild(div.firstElementChild);
            stageCount++;
            updateStageNumbers();
        }
        function removeStage(index) {
            document.getElementById(`stage-${index}`).remove();
            updateStageNumbers();
        }
        function updateStageNumbers() {
            const stages = document.querySelectorAll('.stage');
            stages.forEach((stage, idx) => {
                stage.id = `stage-${idx}`;
                stage.querySelector('.stage-num').innerText = `阶段 ${idx + 1}`;
                const removeBtn = stage.querySelector('button');
                if (removeBtn) removeBtn.setAttribute('onclick', `removeStage(${idx})`);
            });
            stageCount = stages.length;
        }
        async function doDockerfile() {
            try {
                const stages = [];
                document.querySelectorAll('.stage').forEach(stageDiv => {
                    const stage = {};
                    ['image', 'as', 'workdir', 'env', 'copy', 'run', 'expose', 'cmd', 'entrypoint', 'user', 'volume', 'arg'].forEach(field => {
                        const el = stageDiv.querySelector(`[name="${field}"]`);
                        if(el) stage[field] = el.value;
                    });
                    stages.push(stage);
                });
                
                let d = await post('/dockerfile', { stages });
                document.getElementById('df-res').value = d.result;
            } catch(e) {}
        }

        function updateK8sUI() {
            const k = document.getElementById('k8s-kind').value;
            const show = (id, v) => document.getElementById(id).style.display = v ? (id.includes('grid')||id.includes('opts') ? 'grid' : 'block') : 'none';
            
            show('k8s-img-box', false); show('k8s-workload-opts', false); show('k8s-res-opts', false);
            show('k8s-svc-opts', false); show('k8s-ing-opts', false); show('k8s-cron-opts', false); show('k8s-env-box', false);

            if(['Deployment', 'CronJob'].includes(k)) {
                show('k8s-img-box', true); show('k8s-workload-opts', true); show('k8s-res-opts', true); show('k8s-env-box', true);
                document.getElementById('k8s-env-lbl').innerText = '环境变量 (Key=Value)';
            }
            if(k === 'CronJob') show('k8s-cron-opts', true);
            if(k === 'Service') show('k8s-svc-opts', true);
            if(k === 'Ingress') {
                show('k8s-ing-opts', true);
                document.getElementById('k8s-svc-opts').style.display = 'grid'; // Reuse for port
                document.getElementById('k8s-svc-type').parentElement.style.display = 'none';
                document.getElementById('k8s-target-port').parentElement.style.display = 'none';
            }
            if(['ConfigMap', 'Secret'].includes(k)) {
                show('k8s-env-box', true);
                document.getElementById('k8s-env-lbl').innerText = '数据键值对 (Key=Value)';
            }
        }

        async function doK8s() {
            try {
                const envLines = document.getElementById('k8s-env').value.split('\n');
                const env = [];
                envLines.forEach(l => {
                    const p = l.indexOf('=');
                    if(p > 0) env.push({key: l.substring(0, p).trim(), value: l.substring(p+1).trim()});
                });

                let d = await post('/k8s-yaml', {
                    kind: document.getElementById('k8s-kind').value,
                    name: document.getElementById('k8s-name').value,
                    namespace: document.getElementById('k8s-ns').value,
                    image: document.getElementById('k8s-image').value,
                    replicas: parseInt(document.getElementById('k8s-rep').value) || 1,
                    port: parseInt(document.getElementById('k8s-port').value) || 80,
                    targetPort: parseInt(document.getElementById('k8s-target-port').value) || 80,
                    serviceType: document.getElementById('k8s-svc-type').value,
                    ingressHost: document.getElementById('k8s-host').value,
                    ingressPath: document.getElementById('k8s-path').value,
                    pullPolicy: document.getElementById('k8s-pull').value,
                    restartPolicy: document.getElementById('k8s-restart').value,
                    schedule: document.getElementById('k8s-schedule').value,
                    cpuLimit: document.getElementById('k8s-cpu-l').value,
                    memoryLimit: document.getElementById('k8s-mem-l').value,
                    cpuRequest: document.getElementById('k8s-cpu-r').value,
                    memoryRequest: document.getElementById('k8s-mem-r').value,
                    env: env
                });
                document.getElementById('k8s-res').value = d.result;
            } catch(e) {}
        }

        function toggleSslInputs() {
            document.getElementById('ssl-inputs').style.display = document.getElementById('ng-ssl').checked ? 'grid' : 'none';
        }

        let ngLocCount = 0;
        function createNgLocHTML(index) {
            return `
            <div class="ng-loc" id="ng-loc-${index}" style="padding:15px; border:1px solid #e2e8f0; border-radius:8px; margin-bottom:10px; background:#f8fafc;">
                <div style="display:flex; justify-content:space-between; margin-bottom:10px;">
                    <span style="font-weight:bold; color:#0ea5e9;">路径规则 ${index+1}</span>
                    ${index > 0 ? `<button class="btn secondary" style="padding:4px 8px;font-size:12px;background:#ef4444;box-shadow:none;" onclick="removeNgLoc(${index})">移除</button>` : ''}
                </div>
                <div class="grid-4">
                    <div><div class="cron-label">路径 (Path)</div><input name="path" value="/" placeholder="/"></div>
                    <div style="grid-column: span 2"><div class="cron-label">代理地址 (Proxy) 或 留空用静态</div><input name="proxy" placeholder="http://localhost:3000"></div>
                    <div>
                        <div class="cron-label">选项</div>
                        <div style="margin-top:10px">
                            <label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" name="spa" style="width:18px;height:18px;accent-color:var(--primary)"> SPA 模式</label>
                        </div>
                    </div>
                </div>
                <div style="margin-top:10px">
                    <div class="cron-label">根目录重写 (Root Override, 可选)</div><input name="root" placeholder="留空则继承全局设置">
                </div>
            </div>
            `;
        }
        function addNginxLocation() {
            const container = document.getElementById('ng-locs-container');
            const div = document.createElement('div');
            div.innerHTML = createNgLocHTML(ngLocCount);
            container.appendChild(div.firstElementChild);
            ngLocCount++;
        }
        function removeNgLoc(index) {
            document.getElementById(`ng-loc-${index}`).remove();
        }

        async function doNginx() {
            try {
                const locations = [];
                document.querySelectorAll('.ng-loc').forEach(locDiv => {
                    locations.push({
                        path: locDiv.querySelector('[name="path"]').value,
                        proxy: locDiv.querySelector('[name="proxy"]').value,
                        root: locDiv.querySelector('[name="root"]').value,
                        spa: locDiv.querySelector('[name="spa"]').checked
                    });
                });

                let d = await post('/nginx', {
                    domain: document.getElementById('ng-domain').value,
                    port: parseInt(document.getElementById('ng-port').value) || 80,
                    root: document.getElementById('ng-root').value,
                    locations: locations,
                    upstream: document.getElementById('ng-upstream').value,
                    https: document.getElementById('ng-ssl').checked,
                    force_https: document.getElementById('ng-force').checked,
                    ssl_cert: document.getElementById('ng-crt').value,
                    ssl_key: document.getElementById('ng-key').value,
                    gzip: document.getElementById('ng-gzip').checked,
                    client_max_body_size: document.getElementById('ng-size').value,
                    keepalive_timeout: document.getElementById('ng-ka').value,
                    proxy_connect_timeout: document.getElementById('ng-pct').value,
                    proxy_read_timeout: document.getElementById('ng-prt').value,
                    proxy_send_timeout: document.getElementById('ng-pst').value
                });
                document.getElementById('ng-res').value = d.result;
            } catch(e) {}
        }
        async function doLorem() {
            try {
                let d = await post('/lorem', {
                    count: parseInt(document.getElementById('li-count').value) || 3,
                    mode: document.getElementById('li-mode').value
                });
                document.getElementById('li-res').value = d.result;
            } catch(e) {}
        }
        async function doFakeUser() {
            try {
                let d = await post('/fake-user', {
                    count: parseInt(document.getElementById('fu-count').value) || 5,
                    locale: document.getElementById('fu-locale').value
                });
                document.getElementById('fu-res').value = JSON.stringify(d.users, null, 2);
            } catch(e) {}
        }
        async function doCc() {
            try {
                let d = await post('/credit-card', {
                    count: parseInt(document.getElementById('cc-count').value) || 5,
                    issuer: document.getElementById('cc-issuer').value
                });
                document.getElementById('cc-res').value = JSON.stringify(d.cards, null, 2);
            } catch(e) {}
        }
        async function doWhoami() {
            try {
                let d = await post('/whoami', {});
                document.getElementById('wa-ip').innerText = d.ip;
                document.getElementById('wa-country').innerText = d.country;
                document.getElementById('wa-city').innerText = d.city;
                document.getElementById('wa-asn').innerText = d.asn;
                document.getElementById('wa-ua').innerText = d.user_agent;
                let h = ""; for(let k in d.headers) h += k + ": " + d.headers[k] + "\n";
                document.getElementById('wa-headers').value = h;
            } catch(e) {}
        }

        // cURL Logic
        async function doCurl() {
            try {
                let d = await post('/curl', {
                    method: document.getElementById('curl-method').value,
                    url: document.getElementById('curl-url').value,
                    headers: document.getElementById('curl-headers').value,
                    body: document.getElementById('curl-body').value
                });
                document.getElementById('curl-res').value = d.command;
                document.getElementById('curl-py').value = d.python;
            } catch(e) {}
        }

        function validateCurlHeaders() {
            const val = document.getElementById('curl-headers').value;
            const status = document.getElementById('curl-h-status');
            const box = document.getElementById('curl-headers');
            
            if (!val.trim()) {
                status.innerText = '';
                box.style.borderColor = '';
                return;
            }
            try {
                JSON.parse(val);
                status.innerText = '✅';
                status.style.color = '#10b981';
                box.style.borderColor = '#10b981';
            } catch (e) {
                status.innerText = '❌ 格式错误';
                status.style.color = '#ef4444';
                box.style.borderColor = '#ef4444';
            }
        }

        // Unit Convert Logic
        const unitOpts = {
            storage: ['B', 'KB', 'MB', 'GB', 'TB', 'PB'],
            time: ['ms', 's', 'm', 'h', 'd']
        };
        function updateUnitUI() {
            const type = document.getElementById('unit-type').value;
            const opts = unitOpts[type];
            const from = document.getElementById('unit-from');
            const to = document.getElementById('unit-to');
            from.innerHTML = ''; to.innerHTML = '';
            opts.forEach(u => {
                from.add(new Option(u, u));
                to.add(new Option(u, u));
            });
            // Defaults
            if(type === 'storage') { from.value = 'MB'; to.value = 'GB'; }
            if(type === 'time') { from.value = 's'; to.value = 'ms'; }
            doUnit();
        }
        async function doUnit() {
            try {
                let d = await post('/unit-convert', {
                    value: document.getElementById('unit-val').value,
                    type: document.getElementById('unit-type').value,
                    from: document.getElementById('unit-from').value,
                    to: document.getElementById('unit-to').value
                });
                document.getElementById('unit-res').innerText = d.result;
            } catch(e) {}
        }

        // Git Cheat Sheet Logic
        function updateGcUI() {
            const act = document.getElementById('gc-action').value;
            const box = document.getElementById('gc-inputs');
            box.innerHTML = '';
            if(act === 'tag') {
                box.innerHTML = `<div><div class="cron-label">标签名 (Tag)</div><input id="gc-tag" value="v1.0.0" oninput="doGitCheat()"></div>
                                 <div><div class="cron-label">注释 (Message)</div><input id="gc-msg" value="Release v1.0.0" oninput="doGitCheat()"></div>`;
            } else if(act === 'branch_delete') {
                box.innerHTML = `<div><div class="cron-label">分支名 (Branch)</div><input id="gc-branch" value="feature/old" oninput="doGitCheat()"></div>`;
            }
        }
        async function doGitCheat() {
            try {
                let d = await post('/git-cmd', {
                    action: document.getElementById('gc-action').value,
                    tag: document.getElementById('gc-tag') ? document.getElementById('gc-tag').value : '',
                    msg: document.getElementById('gc-msg') ? document.getElementById('gc-msg').value : '',
                    branch: document.getElementById('gc-branch') ? document.getElementById('gc-branch').value : ''
                });
                document.getElementById('gc-cmd').innerText = d.command;
                document.getElementById('gc-desc').innerText = d.description;
            } catch(e) {}
        }

        function updateKcUI() {
            const act = document.getElementById('kc-action').value;
            const box = document.getElementById('kc-extra');
            box.innerHTML = '';
            box.style.display = 'none';

            if (act === 'scale') {
                box.style.display = 'grid';
                box.innerHTML = `<div><div class="cron-label">副本数</div><input type="number" id="kc-rep" value="3" oninput="doK8sCmd()"></div>`;
            } else if (act === 'port_forward') {
                box.style.display = 'grid';
                box.innerHTML = `<div><div class="cron-label">本地端口</div><input type="number" id="kc-lp" value="8080" oninput="doK8sCmd()"></div>
                                 <div><div class="cron-label">容器端口</div><input type="number" id="kc-rp" value="80" oninput="doK8sCmd()"></div>`;
            }
        }

        async function doK8sCmd() {
            try {
                let d = await post('/k8s-cmd', {
                    action: document.getElementById('kc-action').value,
                    namespace: document.getElementById('kc-ns').value,
                    resourceType: document.getElementById('kc-type').value,
                    resourceName: document.getElementById('kc-name').value,
                    outputFormat: document.getElementById('kc-out').value,
                    replicas: document.getElementById('kc-rep') ? parseInt(document.getElementById('kc-rep').value) : 0,
                    localPort: document.getElementById('kc-lp') ? parseInt(document.getElementById('kc-lp').value) : 0,
                    remotePort: document.getElementById('kc-rp') ? parseInt(document.getElementById('kc-rp').value) : 0
                });
                document.getElementById('kc-res').innerText = d.command;
                document.getElementById('kc-desc').innerText = d.description;
            } catch(e) {}
        }

        function updateAnsModUI() {
            const m = document.getElementById('ans-mod').value;
            const box = document.getElementById('ans-mod-opts');
            box.innerHTML = '';
            
            if(m === 'apt') {
                box.innerHTML = `<div><div class="cron-label">包名 (Name)</div><input id="am-name" placeholder="nginx"></div>
                                 <div><div class="cron-label">状态 (State)</div><select id="am-state"><option value="present">安装 (present)</option><option value="absent">卸载 (absent)</option><option value="latest">最新 (latest)</option></select></div>
                                 <div style="display:flex;align-items:flex-end;padding-bottom:15px"><label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="am-cache" checked style="width:18px;height:18px;accent-color:var(--primary)"> 更新缓存</label></div>`;
            } else if(m === 'service') {
                box.innerHTML = `<div><div class="cron-label">服务名 (Name)</div><input id="am-name" placeholder="nginx"></div>
                                 <div><div class="cron-label">状态 (State)</div><select id="am-state"><option value="started">启动 (started)</option><option value="stopped">停止 (stopped)</option><option value="restarted">重启 (restarted)</option><option value="reloaded">重载 (reloaded)</option></select></div>
                                 <div><div class="cron-label">开机自启</div><select id="am-en"><option value="yes">是 (yes)</option><option value="no">否 (no)</option></select></div>`;
            } else if(m === 'copy' || m === 'template') {
                box.innerHTML = `<div><div class="cron-label">源文件 (Src)</div><input id="am-src" placeholder="/local/file.conf"></div>
                                 <div><div class="cron-label">目标路径 (Dest)</div><input id="am-dest" placeholder="/etc/file.conf"></div>
                                 <div><div class="cron-label">权限 (Mode)</div><input id="am-mode" placeholder="0644"></div>
                                 <div><div class="cron-label">所有者 (Owner)</div><input id="am-owner" placeholder="root"></div>
                                 <div><div class="cron-label">所属组 (Group)</div><input id="am-group" placeholder="root"></div>
                                 ${m === 'copy' ? '<div style="display:flex;align-items:flex-end;padding-bottom:15px"><label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="am-backup" style="width:18px;height:18px;accent-color:var(--primary)"> 备份原文件</label></div>' : ''}`;
            } else if(m === 'file') {
                box.innerHTML = `<div><div class="cron-label">路径 (Path)</div><input id="am-path" placeholder="/var/www"></div>
                                 <div><div class="cron-label">状态 (State)</div><select id="am-state"><option value="directory">目录 (directory)</option><option value="file">文件 (file)</option><option value="touch">创建 (touch)</option><option value="absent">删除 (absent)</option></select></div>
                                 <div><div class="cron-label">所有者 (Owner)</div><input id="am-owner" placeholder="www-data"></div>
                                 <div><div class="cron-label">权限 (Mode)</div><input id="am-mode" placeholder="0755"></div>
                                 <div style="display:flex;align-items:flex-end;padding-bottom:15px"><label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="am-recurse" style="width:18px;height:18px;accent-color:var(--primary)"> 递归 (Recurse)</label></div>`;
            } else if(m === 'lineinfile') {
                box.innerHTML = `<div><div class="cron-label">文件路径 (Path)</div><input id="am-path" placeholder="/etc/hosts"></div>
                                 <div style="grid-column:span 3"><div class="cron-label">行内容 (Line)</div><input id="am-line" placeholder="127.0.0.1 localhost"></div>
                                 <div style="grid-column:span 2"><div class="cron-label">正则匹配 (Regexp, 可选)</div><input id="am-regexp" placeholder="^127\.0\.0\.1"></div>
                                 <div><div class="cron-label">状态</div><select id="am-state"><option value="present">存在 (present)</option><option value="absent">缺席 (absent)</option></select></div>`;
            } else if(m === 'unarchive') {
                box.innerHTML = `<div><div class="cron-label">源文件 (Src)</div><input id="am-src" placeholder="https://example.com/file.tar.gz"></div>
                                 <div><div class="cron-label">目标目录 (Dest)</div><input id="am-dest" placeholder="/opt/"></div>
                                 <div style="display:flex;align-items:flex-end;padding-bottom:15px"><label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="am-remote" checked style="width:18px;height:18px;accent-color:var(--primary)"> 远程源 (Remote Src)</label></div>`;
            } else if(m === 'shell') {
                box.innerHTML = `<div style="grid-column:span 4"><div class="cron-label">命令 (Command)</div><input id="am-cmd" placeholder="echo 'hello' > /tmp/test"></div>`;
            } else if(m === 'git') {
                box.innerHTML = `<div style="grid-column:span 2"><div class="cron-label">仓库 URL (Repo)</div><input id="am-repo" placeholder="https://github.com/user/repo.git"></div>
                                 <div><div class="cron-label">目标目录 (Dest)</div><input id="am-dest" placeholder="/opt/app"></div>
                                 <div><div class="cron-label">版本 (Version)</div><input id="am-ver" placeholder="main"></div>`;
            } else if(m === 'user') {
                box.innerHTML = `<div><div class="cron-label">用户名 (Name)</div><input id="am-name" placeholder="deploy"></div>
                                 <div><div class="cron-label">Shell</div><input id="am-shell" value="/bin/bash"></div>
                                 <div><div class="cron-label">组 (Groups)</div><input id="am-groups" placeholder="sudo,docker"></div>
                                 <div style="display:flex;align-items:flex-end;padding-bottom:15px"><label style="display:flex;align-items:center;gap:5px;cursor:pointer;user-select:none"><input type="checkbox" id="am-keygen" style="width:18px;height:18px;accent-color:var(--primary)"> 生成 SSH Key</label></div>`;
            } else if(m === 'debug') {
                box.innerHTML = `<div style="grid-column:span 4"><div class="cron-label">消息 (Msg)</div><input id="am-msg" placeholder="System is {{ ansible_os_family }}"></div>`;
            } else if(m === 'block') {
                box.innerHTML = `<div style="grid-column:span 4"><div class="cron-label">块名称 (Name)</div><input id="am-name" placeholder="Error Handling Block"></div>
                                 <div style="grid-column:span 4"><div class="cron-label">主任务列表 (Block Tasks) - YAML格式 (- name: ...)</div><textarea id="am-block-tasks" style="height:80px;font-family:monospace" placeholder="- name: Task 1\n  command: /bin/true"></textarea></div>
                                 <div style="grid-column:span 4"><div class="cron-label">救援任务 (Rescue) - 可选</div><textarea id="am-rescue-tasks" style="height:60px;font-family:monospace" placeholder="- name: Handle Error\n  debug: msg='Failed'"></textarea></div>
                                 <div style="grid-column:span 4"><div class="cron-label">总是执行 (Always) - 可选</div><textarea id="am-always-tasks" style="height:60px;font-family:monospace" placeholder="- name: Cleanup\n  file: path=/tmp/x state=absent"></textarea></div>`;
            }
        }

        function addAnsTask() {
            const m = document.getElementById('ans-mod').value;
            const val = (id) => document.getElementById(id) ? document.getElementById(id).value : '';
            const chk = (id) => document.getElementById(id) ? document.getElementById(id).checked : false;
            
            let t = '';

            if (m === 'block') {
                t = `- name: ${val('am-name') || 'Block Task'}\n  block:`;
                const indent = (str) => str.split('\n').map(l => '    ' + l).join('\n');
                
                const bTasks = val('am-block-tasks');
                if(bTasks && bTasks.trim()) {
                    t += '\n' + indent(bTasks.trim());
                } else {
                    t += '\n    - debug: msg="Block task placeholder"';
                }
                
                const rTasks = val('am-rescue-tasks');
                if(rTasks && rTasks.trim()) {
                    t += `\n  rescue:\n` + indent(rTasks.trim());
                }
                
                const aTasks = val('am-always-tasks');
                if(aTasks && aTasks.trim()) {
                    t += `\n  always:\n` + indent(aTasks.trim());
                }
            } else {
                t = `- name: ${m.charAt(0).toUpperCase() + m.slice(1)} task\n  ${m}:\n`;
                
                if(m === 'apt') {
                    t += `    name: ${val('am-name')}\n    state: ${val('am-state')}`;
                    if(chk('am-cache')) t += `\n    update_cache: yes`;
                }
                else if(m === 'service') t += `    name: ${val('am-name')}\n    state: ${val('am-state')}\n    enabled: ${val('am-en')}`;
                else if(m === 'copy' || m === 'template') {
                    t += `    src: ${val('am-src')}\n    dest: ${val('am-dest')}`;
                    if(val('am-mode')) t += `\n    mode: '${val('am-mode')}'`;
                    if(val('am-owner')) t += `\n    owner: ${val('am-owner')}`;
                    if(val('am-group')) t += `\n    group: ${val('am-group')}`;
                    if(m === 'copy' && chk('am-backup')) t += `\n    backup: yes`;
                }
                else if(m === 'file') {
                    t += `    path: ${val('am-path')}\n    state: ${val('am-state')}`;
                    if(val('am-owner')) t += `\n    owner: ${val('am-owner')}`;
                    if(val('am-mode')) t += `\n    mode: '${val('am-mode')}'`;
                    if(chk('am-recurse')) t += `\n    recurse: yes`;
                }
                else if(m === 'lineinfile') {
                    t += `    path: ${val('am-path')}\n    line: ${val('am-line')}\n    state: ${val('am-state')}`;
                    if(val('am-regexp')) t += `\n    regexp: '${val('am-regexp')}'`;
                }
                else if(m === 'unarchive') {
                    t += `    src: ${val('am-src')}\n    dest: ${val('am-dest')}`;
                    if(chk('am-remote')) t += `\n    remote_src: yes`;
                }
                else if(m === 'shell') t += `    cmd: ${val('am-cmd')}`;
                else if(m === 'git') {
                    t += `    repo: ${val('am-repo')}\n    dest: ${val('am-dest')}`;
                    if(val('am-ver')) t += `\n    version: ${val('am-ver')}`;
                }
                else if(m === 'user') {
                    t += `    name: ${val('am-name')}\n    shell: ${val('am-shell')}\n    groups: ${val('am-groups')}`;
                    if(chk('am-keygen')) t += `\n    generate_ssh_key: yes`;
                }
                else if(m === 'debug') {
                    t += `    msg: "${val('am-msg')}"`;
                }
            }
            
            // Common options
            if(val('am-reg')) t += `\n  register: ${val('am-reg')}`;
            if(val('am-when')) t += `\n  when: ${val('am-when')}`;
            if(val('am-notify')) t += `\n  notify: ${val('am-notify')}`;
            if(chk('am-ignore')) t += `\n  ignore_errors: yes`;
            
            if(val('am-loop')) {
                const l = val('am-loop').trim();
                if(l.startsWith('{{') || l.startsWith('[')) {
                    t += `\n  loop: ${l}`;
                } else {
                    t += `\n  loop:`;
                    l.split('\n').forEach(line => {
                        if(line.trim()) {
                            t += `\n    - ${line.trim().replace(/^- /, '')}`;
                        }
                    });
                }
            }

            const area = document.getElementById('ans-tasks');
            area.value = (area.value ? area.value + '\n\n' : '') + t;
        }

        async function doAnsible() {
            // 如果任务列表为空，自动添加当前构建器中的任务
            const tasksArea = document.getElementById('ans-tasks');
            if (!tasksArea.value.trim()) {
                addAnsTask();
            }

            try {
                let d = await post('/ansible', {
                    play_name: document.getElementById('ans-name').value,
                    hosts: document.getElementById('ans-hosts').value,
                    become: document.getElementById('ans-become').checked,
                    gather_facts: document.getElementById('ans-facts').checked,
                    vars: document.getElementById('ans-vars').value,
                    tasks: document.getElementById('ans-tasks').value,
                    handlers: document.getElementById('ans-handlers').value
                });
                document.getElementById('ans-res').value = d.result;
            } catch(e) {}
        }

        window.onload = () => { fillTime(); upCron(); upChmod(true); doTar(); doPs(); doTcpdump(); updateGitUI(); doGit(); doStrace(); doIostat(); doNice(); doLs(); doFirewall(); updateSysUI(); doSystemctl(); updateFindUI(); doFind(); doWhoami(); doRsync(); addStage(); addNginxLocation(); updateUnitUI(); updateGcUI(); doGitCheat(); doAwk(); doSed(); updateK8sUI(); updateAnsModUI(); };
    </script>
</body>
</html>
    "####
}
