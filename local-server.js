// Local server for running the Cloudflare Worker
const express = require('express');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = 8787;

// Middleware for parsing JSON
app.use(express.json());

// Serve the homepage
app.get('/', (req, res) => {
  try {
    const htmlContent = fs.readFileSync(path.join(__dirname, 'src', 'html.rs'), 'utf8');
    console.log('HTML file read successfully, length:', htmlContent.length);

    // Try a simpler approach: find the HTML content between the quotes
    // Look for the pattern: r####"
    // Then capture everything until the closing "####
    const startMarker = 'r####"';
    const endMarker = '"####';

    const startIndex = htmlContent.indexOf(startMarker);
    if (startIndex !== -1) {
      const contentStart = startIndex + startMarker.length;
      const endIndex = htmlContent.indexOf(endMarker, contentStart);
      if (endIndex !== -1) {
        const htmlContentExtracted = htmlContent.substring(contentStart, endIndex);
        console.log('HTML content extracted successfully using simple method');
        res.send(htmlContentExtracted);
        return;
      }
    }

    // If that fails, try with r#"
    const startMarkerAlt = 'r#"';
    const endMarkerAlt = '"#';

    const startIndexAlt = htmlContent.indexOf(startMarkerAlt);
    if (startIndexAlt !== -1) {
      const contentStartAlt = startIndexAlt + startMarkerAlt.length;
      const endIndexAlt = htmlContent.indexOf(endMarkerAlt, contentStartAlt);
      if (endIndexAlt !== -1) {
        const htmlContentExtracted = htmlContent.substring(contentStartAlt, endIndexAlt);
        console.log('HTML content extracted successfully using alternative method');
        res.send(htmlContentExtracted);
        return;
      }
    }

    // If no pattern matches, send the entire file as plain text for debugging
    console.log('No HTML pattern matched, sending plain text for debugging');
    res.send('<pre>' + htmlContent.substring(0, 500) + '...</pre>');
  } catch (error) {
    console.error('Error reading HTML file:', error);
    res.send('<h1>Worker Tools</h1><p>Local server running successfully!</p>');
  }
});

// Serve the Dockerfile test page
app.get('/dockerfile', (req, res) => {
  res.sendFile(path.join(__dirname, 'dockerfile_test.html'));
});

// Worker module loading is disabled since we've implemented all necessary API endpoints locally
// This avoids the "Cannot access 'require' before initialization" error
console.log('Starting basic server with local API implementations...');

// API routes (simulating the worker's API)
// 实现所有前端可能调用的API端点

// 基础工具API
app.post('/api/sql', (req, res) => {
  const sql = req.body.sql || '';

  if (!sql) {
    res.json({ result: '-- 请输入 SQL 语句进行格式化' });
    return;
  }

  // 简单的 SQL 格式化逻辑
  let formattedSql = sql
    // 关键字大写
    .replace(/\b(SELECT|FROM|WHERE|JOIN|LEFT|RIGHT|INNER|OUTER|GROUP|BY|ORDER|LIMIT|OFFSET|INSERT|UPDATE|DELETE|CREATE|ALTER|DROP|TABLE|DATABASE|INDEX|VIEW|PROCEDURE|FUNCTION|TRIGGER)\b/gi, match => match.toUpperCase())
    // 在关键字前添加换行
    .replace(/\b(SELECT|FROM|WHERE|JOIN|LEFT|RIGHT|INNER|OUTER|GROUP|BY|ORDER|LIMIT|OFFSET|INSERT|UPDATE|DELETE|CREATE|ALTER|DROP|TABLE|DATABASE|INDEX|VIEW|PROCEDURE|FUNCTION|TRIGGER)\b/gi, '\n$1')
    // 在逗号后添加空格
    .replace(/,/g, ', ')
    // 在等号前后添加空格
    .replace(/=/g, ' = ')
    // 在操作符前后添加空格
    .replace(/([<>!]=?|LIKE|IN|NOT|AND|OR|IS|NULL)/gi, ' $1 ')
    // 移除多余的空格
    .replace(/\s+/g, ' ')
    // 移除行首的空格
    .replace(/^\s+/gm, '')
    // 保留空行
    .replace(/\n\s*\n/g, '\n\n');

  res.json({ result: formattedSql });
});

app.post('/api/diff', (req, res) => {
  const oldText = req.body.old || '';
  const newText = req.body.new || '';
  const chunks = [];

  if (oldText !== newText) {
    // 简单的文本对比逻辑
    const oldLines = oldText.split('\n');
    const newLines = newText.split('\n');
    const maxLines = Math.max(oldLines.length, newLines.length);

    for (let i = 0; i < maxLines; i++) {
      const oldLine = oldLines[i] || '';
      const newLine = newLines[i] || '';

      if (oldLine !== newLine) {
        if (oldLine && !newLine) {
          // 删除的行
          chunks.push({ tag: 'delete', text: oldLine + '\n' });
        } else if (!oldLine && newLine) {
          // 新增的行
          chunks.push({ tag: 'insert', text: newLine + '\n' });
        } else {
          // 修改的行
          chunks.push({ tag: 'delete', text: oldLine + '\n' });
          chunks.push({ tag: 'insert', text: newLine + '\n' });
        }
      } else {
        // 相同的行
        chunks.push({ tag: 'equal', text: oldLine + '\n' });
      }
    }
  }

  res.json({ chunks });
});

app.post('/api/cron', (req, res) => {
  const cron = req.body.cron || '';

  if (!cron) {
    res.json({ valid: false, next_runs: [], error: '请输入 Cron 表达式' });
    return;
  }

  // 简单的 Cron 表达式验证
  const cronRegex = /^\s*([^\s]+)\s+([^\s]+)\s+([^\s]+)\s+([^\s]+)\s+([^\s]+)\s*$/;
  if (!cronRegex.test(cron)) {
    res.json({ valid: false, next_runs: [], error: 'Cron 表达式格式错误' });
    return;
  }

  // 更详细的 Cron 表达式验证
  const parts = cron.trim().split(/\s+/);
  if (parts.length !== 5) {
    res.json({ valid: false, next_runs: [], error: 'Cron 表达式必须包含 5 个字段' });
    return;
  }

  const [minute, hour, day, month, weekday] = parts;

  // 验证分钟字段
  if (!validateCronField(minute, 0, 59)) {
    res.json({ valid: false, next_runs: [], error: '分钟字段格式错误' });
    return;
  }

  // 验证小时字段
  if (!validateCronField(hour, 0, 23)) {
    res.json({ valid: false, next_runs: [], error: '小时字段格式错误' });
    return;
  }

  // 验证日期字段
  if (!validateCronField(day, 1, 31)) {
    res.json({ valid: false, next_runs: [], error: '日期字段格式错误' });
    return;
  }

  // 验证月份字段
  if (!validateCronField(month, 1, 12)) {
    res.json({ valid: false, next_runs: [], error: '月份字段格式错误' });
    return;
  }

  // 验证星期字段
  if (!validateCronField(weekday, 0, 7)) {
    res.json({ valid: false, next_runs: [], error: '星期字段格式错误' });
    return;
  }

  // 生成下次执行时间（基于 Cron 表达式的简单模拟）
  const nextRuns = [];
  const now = new Date();

  // 简单的模拟：根据 Cron 表达式的分钟和小时字段生成执行时间
  for (let i = 0; i < 5; i++) {
    const nextRun = new Date(now);
    nextRun.setHours(nextRun.getHours() + i + 1);

    // 如果指定了小时，使用指定的小时
    if (hour !== '*') {
      nextRun.setHours(parseInt(hour) || 0);
    }

    // 如果指定了分钟，使用指定的分钟
    if (minute !== '*') {
      nextRun.setMinutes(parseInt(minute) || 0);
    } else {
      nextRun.setMinutes(0);
    }

    nextRun.setSeconds(0);
    nextRuns.push(nextRun.toISOString().replace('T', ' ').substring(0, 19) + ' UTC');
  }

  res.json({ valid: true, next_runs: nextRuns, error: '' });
});

// 验证 Cron 字段的函数
function validateCronField(field, min, max) {
  // 支持 *
  if (field === '*') {
    return true;
  }

  // 支持数字
  if (!isNaN(field)) {
    const num = parseInt(field);
    return num >= min && num <= max;
  }

  // 支持逗号分隔的列表
  if (field.includes(',')) {
    return field.split(',').every(item => {
      const trimmed = item.trim();
      if (!isNaN(trimmed)) {
        const num = parseInt(trimmed);
        return num >= min && num <= max;
      }
      return false;
    });
  }

  // 支持范围
  if (field.includes('-')) {
    const [start, end] = field.split('-');
    if (!isNaN(start) && !isNaN(end)) {
      const startNum = parseInt(start);
      const endNum = parseInt(end);
      return startNum >= min && endNum <= max && startNum <= endNum;
    }
  }

  // 支持步长
  if (field.includes('/')) {
    const [base, step] = field.split('/');
    if (!isNaN(step)) {
      const stepNum = parseInt(step);
      if (base === '*') {
        return stepNum > 0;
      }
      if (!isNaN(base)) {
        const baseNum = parseInt(base);
        return baseNum >= min && baseNum <= max && stepNum > 0;
      }
    }
  }

  return false;
}

app.post('/api/subnet', (req, res) => {
  const ip = req.body.ip || '192.168.1.1';
  const cidr = req.body.cidr || '24';

  // 简单的子网计算逻辑
  const ipParts = ip.split('.').map(Number);
  const prefixLength = parseInt(cidr);

  // 计算子网掩码
  const maskParts = [];
  for (let i = 0; i < 4; i++) {
    if (prefixLength >= (i + 1) * 8) {
      maskParts.push(255);
    } else if (prefixLength >= i * 8) {
      maskParts.push(256 - Math.pow(2, 8 - (prefixLength - i * 8)));
    } else {
      maskParts.push(0);
    }
  }
  const mask = maskParts.join('.');

  // 计算网络地址
  const networkParts = ipParts.map((part, index) => part & maskParts[index]);
  const network = networkParts.join('.');

  // 计算广播地址
  const broadcastParts = networkParts.map((part, index) => part | (255 - maskParts[index]));
  const broadcast = broadcastParts.join('.');

  // 计算可用主机数
  const totalHosts = Math.pow(2, 32 - prefixLength);
  const usableHosts = totalHosts - 2;

  // 计算第一个和最后一个可用 IP
  const firstIpParts = [...networkParts];
  firstIpParts[3] += 1;
  const firstIp = firstIpParts.join('.');

  const lastIpParts = [...broadcastParts];
  lastIpParts[3] -= 1;
  const lastIp = lastIpParts.join('.');

  // 计算通配符掩码
  const wildcardParts = maskParts.map(part => 255 - part);
  const wildcard = wildcardParts.join('.');

  // 确定 IP 类别
  let ipClass = 'A 类';
  if (ipParts[0] >= 128 && ipParts[0] < 192) {
    ipClass = 'B 类';
  } else if (ipParts[0] >= 192 && ipParts[0] < 224) {
    ipClass = 'C 类';
  } else if (ipParts[0] >= 224 && ipParts[0] < 240) {
    ipClass = 'D 类 (组播)';
  } else if (ipParts[0] >= 240) {
    ipClass = 'E 类 (保留)';
  }

  // 确定 IP 类型
  let ipType = '公共 (Public)';
  if (
    (ipParts[0] === 10) ||
    (ipParts[0] === 172 && ipParts[1] >= 16 && ipParts[1] <= 31) ||
    (ipParts[0] === 192 && ipParts[1] === 168)
  ) {
    ipType = '私有 (Private)';
  }

  // 计算二进制掩码
  const binaryMask = maskParts.map(part => part.toString(2).padStart(8, '0')).join('.');

  // 计算二进制 IP
  const binaryIp = ipParts.map(part => part.toString(2).padStart(8, '0')).join('.');

  res.json({
    valid: true,
    ip,
    cidr,
    mask,
    network,
    broadcast,
    total_hosts: totalHosts,
    usable_hosts: usableHosts,
    ip_class: ipClass,
    ip_type: ipType,
    wildcard,
    first_ip: firstIp,
    last_ip: lastIp,
    binary_mask: binaryMask,
    binary_ip: binaryIp
  });
});

app.post('/api/regex-gen', (req, res) => {
  const key = req.body.key || '';

  // 根据 key 返回不同的正则表达式模式
  const pattern = {
    'email': '(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$',
    'phone_cn': '^1[3-9]\\d{9}$',
    'id_cn': '^[1-9]\\d{5}(18|19|20)\\d{2}(0[1-9]|1[0-2])(0[1-9]|[1-2]\\d|3[0-1])\\d{3}[\\dXx]$',
    'ipv4': '^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$',
    'url': 'https?:\\/\\/(www\\.)?[-a-zA-Z0-9@:%._\\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b([-a-zA-Z0-9()@:%_\\+.~#?&//=]*)',
    'date': '^\\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\\d|3[01])$',
    'password': '^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)(?=.*[!@#$%^&*()_+\\-=\\[\\]{};\':,.<>/?])[A-Za-z\\d!@#$%^&*()_+\\-=\\[\\]{};\':,.<>/?]{8,}$',
    'hex_color': '^#?([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$',
    'chinese': '\\p{Han}+',
    'html_tag': '</?[a-z][a-z0-9]*[^<>]*>'
  }[key] || '';

  if (!pattern) {
    res.json({ result: '请选择有效的正则表达式类型', pattern: '' });
    return;
  }

  res.json({ result: '正则表达式生成成功', pattern: pattern });
});

app.post('/api/regex', (req, res) => {
  const pattern = req.body.pattern || '';
  const text = req.body.text || '';
  const replace = req.body.replace;

  if (!pattern) {
    res.json({ matches: [], count: 0, error: '请输入正则表达式模式' });
    return;
  }

  if (!text) {
    res.json({ matches: [], count: 0, error: '请输入测试文本' });
    return;
  }

  try {
    // 创建正则表达式对象
    const regex = new RegExp(pattern, 'g');
    const matches = [];
    let match;

    // 找到所有匹配项
    while ((match = regex.exec(text)) !== null) {
      matches.push(match[0]);

      // 防止无限循环（当正则表达式包含零宽度断言时）
      if (match.index === regex.lastIndex) {
        regex.lastIndex++;
      }
    }

    const response = { matches: matches, count: matches.length, error: null };

    if (replace !== undefined) {
      response.replaced = text.replace(regex, replace);
    }

    res.json(response);
  } catch (error) {
    res.json({ matches: [], count: 0, error: '正则表达式语法错误' });
  }
});

app.post('/api/url', (req, res) => {
  res.json({
    encoded: encodeURIComponent(req.body.input),
    decoded: decodeURIComponent(req.body.input),
    host: '-',
    path: '-',
    params: []
  });
});

app.post('/api/password', (req, res) => {
  const length = req.body.length || 16;
  const uppercase = req.body.uppercase !== false; // 默认启用
  const lowercase = req.body.lowercase !== false; // 默认启用
  const numbers = req.body.numbers !== false; // 默认启用
  const symbols = req.body.symbols !== false; // 默认启用

  // 生成随机密码
  const charset = [];
  if (uppercase) charset.push('ABCDEFGHIJKLMNOPQRSTUVWXYZ');
  if (lowercase) charset.push('abcdefghijklmnopqrstuvwxyz');
  if (numbers) charset.push('0123456789');
  if (symbols) charset.push('!@#$%^&*()_+-=[]{}|;:,.<>?');

  const allChars = charset.join('');
  let password = '';
  for (let i = 0; i < length; i++) {
    const randomIndex = Math.floor(Math.random() * allChars.length);
    password += allChars[randomIndex];
  }

  res.json({ password });
});

app.post('/api/json', (req, res) => {
  const input = req.body.input || '';
  let pretty = input;
  let minified = input;

  try {
    // 格式化 JSON
    const parsed = JSON.parse(input);
    pretty = JSON.stringify(parsed, null, 2);
    minified = JSON.stringify(parsed);
  } catch (error) {
    // 如果输入不是有效的 JSON，保持原样
  }

  res.json({ pretty, minified, error: null });
});

// 补充缺失的API端点
app.post('/api/qrcode', (req, res) => {
  res.json({ svg: '<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200"><rect width="200" height="200" fill="white"/><text x="100" y="100" text-anchor="middle" dominant-baseline="middle">QR Code</text></svg>' });
});

app.post('/api/color', (req, res) => {
  res.json({ valid: true, hex: req.body.input || '#FFFFFF', rgb: 'rgb(255, 255, 255)', hsl: 'hsl(0, 0%, 100%)', cmyk: 'cmyk(0, 0, 0, 0)' });
});

app.post('/api/date', (req, res) => {
  const timestamp = parseInt(req.body.input) || Date.now() / 1000;
  const date = new Date(timestamp * 1000);
  res.json({
    unix_sec: Math.floor(timestamp),
    unix_milli: Math.floor(timestamp * 1000),
    iso_8601: date.toISOString(),
    human_utc: date.toUTCString()
  });
});

app.post('/api/hash', (req, res) => {
  res.json({ md5_32_lower: 'd41d8cd98f00b204e9800998ecf8427e', md5_32_upper: 'D41D8CD98F00B204E9800998ECF8427E', md5_16_lower: '8f00b204e9800998', md5_16_upper: '8F00B204E9800998' });
});

app.post('/api/md5', (req, res) => {
  res.json({ md5_32_lower: 'd41d8cd98f00b204e9800998ecf8427e', md5_32_upper: 'D41D8CD98F00B204E9800998ECF8427E', md5_16_lower: '8f00b204e9800998', md5_16_upper: '8F00B204E9800998' });
});

app.post('/api/token', (req, res) => {
  const length = req.body.length || 32;
  const uppercase = req.body.uppercase !== false; // 默认启用
  const lowercase = req.body.lowercase !== false; // 默认启用
  const numbers = req.body.numbers !== false; // 默认启用
  const symbols = req.body.symbols !== false; // 默认启用

  // 生成随机token
  const charset = [];
  if (uppercase) charset.push('ABCDEFGHIJKLMNOPQRSTUVWXYZ');
  if (lowercase) charset.push('abcdefghijklmnopqrstuvwxyz');
  if (numbers) charset.push('0123456789');
  if (symbols) charset.push('!@#$%^&*()_+-=[]{}|;:,.<>?');

  const allChars = charset.join('');
  let token = '';
  for (let i = 0; i < length; i++) {
    const randomIndex = Math.floor(Math.random() * allChars.length);
    token += allChars[randomIndex];
  }

  res.json({ token });
});

app.post('/api/uuid', (req, res) => {
  const count = req.body.count || 5;
  const uuids = [];
  for (let i = 0; i < count; i++) {
    // 生成标准格式的UUID
    const uuid = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
      const r = Math.random() * 16 | 0;
      const v = c === 'x' ? r : (r & 0x3 | 0x8);
      return v.toString(16);
    });
    uuids.push(uuid);
  }
  res.json({ uuids });
});

app.post('/api/jwt', (req, res) => {
  res.json({ header: '{}', payload: '{}', error: null });
});

app.post('/api/base64', (req, res) => {
  const result = req.body.action === 'encode' ? Buffer.from(req.body.text).toString('base64') : Buffer.from(req.body.text, 'base64').toString('utf8');
  res.json({ result });
});

app.post('/api/escape', (req, res) => {
  res.json({ result: req.body.text });
});

app.post('/api/yaml-to-toml', (req, res) => {
  const yaml = req.body.yaml || '';
  // 简单的 YAML 转 TOML 逻辑
  let toml = '# Converted from YAML\n';

  if (yaml) {
    const lines = yaml.split('\n');
    let indentLevel = 0;
    let currentSection = '';

    lines.forEach(line => {
      line = line.trim();
      if (!line || line.startsWith('#')) return;

      // 检查缩进级别
      const indent = (line.match(/^\s*/)[0] || '').length;
      indentLevel = Math.floor(indent / 2);

      // 处理键值对
      if (line.includes(':')) {
        const [key, value] = line.split(':', 2);
        const cleanKey = key.trim();
        const cleanValue = value.trim();

        if (cleanValue === '') {
          // 处理嵌套对象
          toml += `${cleanKey} = {\n`;
          currentSection = cleanKey;
        } else {
          // 处理普通键值对
          toml += `${cleanKey} = ${cleanValue}\n`;
        }
      }
    });

    // 闭合所有打开的对象
    if (currentSection) {
      toml += `}\n`;
    }
  }

  res.json({ result: toml, error: null });
});

app.post('/api/toml-to-yaml', (req, res) => {
  const toml = req.body.toml || '';
  // 简单的 TOML 转 YAML 逻辑
  let yaml = '# Converted from TOML\n';

  if (toml) {
    const lines = toml.split('\n');
    let indentLevel = 0;

    lines.forEach(line => {
      line = line.trim();
      if (!line || line.startsWith('#')) return;

      // 处理键值对
      if (line.includes('=')) {
        const [key, value] = line.split('=', 2);
        const cleanKey = key.trim();
        const cleanValue = value.trim();

        yaml += `${'  '.repeat(indentLevel)}${cleanKey}: ${cleanValue}\n`;
      }
    });
  }

  res.json({ result: yaml, error: null });
});

app.post('/api/chmod', (req, res) => {
  const file = req.body.file && req.body.file.trim() ? req.body.file.trim() : 'filename';
  res.json({ valid: true, command: `chmod ${req.body.octal} ${file}` });
});

app.post('/api/js-enc', (req, res) => {
  const jsCode = req.body.js || '';
  // 简单的 JS 混淆逻辑
  let obfuscated = jsCode
    .replace(/function\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\(/g, 'function _$1(')
    .replace(/var\s+([a-zA-Z_$][a-zA-Z0-9_$]*)/g, 'var _$1')
    .replace(/let\s+([a-zA-Z_$][a-zA-Z0-9_$]*)/g, 'let _$1')
    .replace(/const\s+([a-zA-Z_$][a-zA-Z0-9_$]*)/g, 'const _$1');

  if (!obfuscated) {
    obfuscated = '// 请输入 JavaScript 代码进行混淆';
  }

  res.json({ result: obfuscated });
});

app.post('/api/lorem', (req, res) => {
  const count = req.body.count || 3;
  const mode = req.body.mode || 'paragraphs';
  const words = ["lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit", "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore", "magna", "aliqua"];

  let result = "";
  if (mode === 'words') {
    result = Array(count).fill(0).map(() => words[Math.floor(Math.random() * words.length)]).join(' ');
  } else if (mode === 'sentences') {
    result = Array(count).fill(0).map(() => {
      const len = 5 + Math.floor(Math.random() * 10);
      const s = Array(len).fill(0).map(() => words[Math.floor(Math.random() * words.length)]).join(' ');
      return s.charAt(0).toUpperCase() + s.slice(1) + '.';
    }).join(' ');
  } else {
    result = Array(count).fill(0).map(() => "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.").join('\n\n');
  }
  res.json({ result });
});

app.post('/api/fake-user', (req, res) => {
  const count = req.body.count || 5;
  const locale = req.body.locale || 'en';
  const users = [];

  const firstNames = locale === 'cn'
    ? ["伟", "芳", "娜", "敏", "静", "秀英", "丽", "强", "磊", "军"]
    : ["James", "Mary", "John", "Patricia", "Robert", "Jennifer", "Michael", "Linda"];
  const lastNames = locale === 'cn'
    ? ["王", "李", "张", "刘", "陈", "杨", "黄", "赵", "吴", "周"]
    : ["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis"];
  const domains = ["gmail.com", "yahoo.com", "hotmail.com", "outlook.com", "example.com"];
  const cities = locale === 'cn' ? ["北京", "上海", "广州", "深圳"] : ["New York", "Los Angeles", "Chicago", "Houston"];

  for (let i = 0; i < count; i++) {
    const first = firstNames[Math.floor(Math.random() * firstNames.length)];
    const last = lastNames[Math.floor(Math.random() * lastNames.length)];
    const domain = domains[Math.floor(Math.random() * domains.length)];
    const city = cities[Math.floor(Math.random() * cities.length)];

    let name, email, address, phone;

    if (locale === 'cn') {
      name = last + first;
      email = `user${Math.floor(Math.random() * 10000)}@${domain}`;
      address = `${city}市人民路 ${Math.floor(Math.random() * 1000)}号`;
      phone = `1${Math.floor(Math.random() * 10 + 30)}${Math.floor(Math.random() * 100000000)}`;
    } else {
      name = `${first} ${last}`;
      email = `${first.toLowerCase()}.${last.toLowerCase()}@${domain}`;
      address = `${Math.floor(Math.random() * 9999)} Main St, ${city}`;
      phone = `+1-555-${Math.floor(Math.random() * 900 + 100)}-${Math.floor(Math.random() * 9000 + 1000)}`;
    }

    users.push({ name, email, address, phone });
  }

  res.json({ users });
});

app.post('/api/credit-card', (req, res) => {
  const count = Math.max(1, Math.min(50, req.body.count || 5));
  const issuer = req.body.issuer || 'visa';
  const cards = [];

  for (let i = 0; i < count; i++) {
    let len = 16;
    let prefix = [4];

    if (issuer === 'mastercard') {
      len = 16;
      prefix = [5, 1 + Math.floor(Math.random() * 5)];
    } else if (issuer === 'amex') {
      len = 15;
      prefix = [3, Math.random() < 0.5 ? 4 : 7];
    } else if (issuer === 'discover') {
      len = 16;
      prefix = [6, 0, 1, 1];
    }

    const digits = [...prefix];
    while (digits.length < len - 1) {
      digits.push(Math.floor(Math.random() * 10));
    }

    // Luhn
    let sum = 0;
    for (let j = digits.length - 1; j >= 0; j--) {
      let val = digits[j];
      if ((digits.length - 1 - j) % 2 === 0) {
        val *= 2;
        if (val > 9) val -= 9;
      }
      sum += val;
    }
    const checkDigit = (10 - (sum % 10)) % 10;
    digits.push(checkDigit);

    const number = digits.join('');
    const month = String(Math.floor(Math.random() * 12) + 1).padStart(2, '0');
    const year = Math.floor(Math.random() * 5) + 25;
    const cvv = Array(issuer === 'amex' ? 4 : 3).fill(0).map(() => Math.floor(Math.random() * 10)).join('');

    cards.push({
      number,
      issuer,
      expiry: `${month}/${year}`,
      cvv
    });
  }
  res.json({ cards });
});

app.post('/api/whoami', (req, res) => {
  res.json({
    ip: '127.0.0.1',
    country: 'Localhost',
    city: 'Local City',
    asn: 'DEV-RAY-ID',
    user_agent: req.get('User-Agent') || 'Mozilla/5.0 (Dev)',
    headers: req.headers
  });
});

app.post('/api/rsync', (req, res) => {
  const { source, user, host, port, remote_path, archive, compress, verbose, delete: del, dry_run, progress, ssh, exclude } = req.body;
  let cmd = 'rsync';
  let shorts = '';
  if (archive) shorts += 'a';
  if (compress) shorts += 'z';
  if (verbose) shorts += 'v';
  if (dry_run) shorts += 'n';
  if (progress) shorts += 'P';

  if (shorts) cmd += ' -' + shorts;

  if (del) cmd += ' --delete';

  if (port && port.trim() !== '22') {
    cmd += ` -e 'ssh -p ${port.trim()}'`;
  } else if (ssh) {
    cmd += ' -e ssh';
  }

  if (exclude && exclude.trim()) cmd += ` --exclude='${exclude.trim()}'`;

  cmd += ' ' + (source && source.trim() ? source.trim() : '/source/path');

  let dest = '';
  if (host && host.trim()) {
    if (user && user.trim()) dest += `${user.trim()}@`;
    dest += `${host.trim()}:${remote_path && remote_path.trim() ? remote_path.trim() : ''}`;
  } else {
    dest = remote_path && remote_path.trim() ? remote_path.trim() : '/dest/path';
  }
  cmd += ' ' + dest;

  let ssh_config = '';
  if (host && host.trim()) {
    ssh_config += `Host ${host.trim()}\n`;
    ssh_config += `    HostName ${host.trim()}\n`;
    if (user && user.trim()) ssh_config += `    User ${user.trim()}\n`;
    if (port && port.trim() && port.trim() !== '22') ssh_config += `    Port ${port.trim()}\n`;
  }

  res.json({ command: cmd, ssh_config });
});

// 处理根路径的API请求（无前缀/api的情况）
app.post('/qrcode', (req, res) => {
  res.json({ svg: '<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200"><rect width="200" height="200" fill="white"/><text x="100" y="100" text-anchor="middle" dominant-baseline="middle">QR Code</text></svg>' });
});

app.post('/color', (req, res) => {
  res.json({ valid: true, hex: req.body.input || '#FFFFFF', rgb: 'rgb(255, 255, 255)', hsl: 'hsl(0, 0%, 100%)', cmyk: 'cmyk(0, 0, 0, 0)' });
});

app.post('/date', (req, res) => {
  const timestamp = parseInt(req.body.input) || Date.now() / 1000;
  const date = new Date(timestamp * 1000);
  res.json({
    unix_sec: Math.floor(timestamp),
    unix_milli: Math.floor(timestamp * 1000),
    iso_8601: date.toISOString(),
    human_utc: date.toUTCString()
  });
});

app.post('/diff', (req, res) => {
  const oldText = req.body.old || '';
  const newText = req.body.new || '';
  const chunks = [];

  if (oldText !== newText) {
    // 简单的文本对比逻辑
    const oldLines = oldText.split('\n');
    const newLines = newText.split('\n');
    const maxLines = Math.max(oldLines.length, newLines.length);

    for (let i = 0; i < maxLines; i++) {
      const oldLine = oldLines[i] || '';
      const newLine = newLines[i] || '';

      if (oldLine !== newLine) {
        if (oldLine && !newLine) {
          // 删除的行
          chunks.push({ tag: 'delete', text: oldLine + '\n' });
        } else if (!oldLine && newLine) {
          // 新增的行
          chunks.push({ tag: 'insert', text: newLine + '\n' });
        } else {
          // 修改的行
          chunks.push({ tag: 'delete', text: oldLine + '\n' });
          chunks.push({ tag: 'insert', text: newLine + '\n' });
        }
      } else {
        // 相同的行
        chunks.push({ tag: 'equal', text: oldLine + '\n' });
      }
    }
  }

  res.json({ chunks });
});

app.post('/subnet', (req, res) => {
  const ip = req.body.ip || '192.168.1.1';
  const cidr = req.body.cidr || '24';

  // 简单的子网计算逻辑
  const ipParts = ip.split('.').map(Number);
  const prefixLength = parseInt(cidr);

  // 计算子网掩码
  const maskParts = [];
  for (let i = 0; i < 4; i++) {
    if (prefixLength >= (i + 1) * 8) {
      maskParts.push(255);
    } else if (prefixLength >= i * 8) {
      maskParts.push(256 - Math.pow(2, 8 - (prefixLength - i * 8)));
    } else {
      maskParts.push(0);
    }
  }
  const mask = maskParts.join('.');

  // 计算网络地址
  const networkParts = ipParts.map((part, index) => part & maskParts[index]);
  const network = networkParts.join('.');

  // 计算广播地址
  const broadcastParts = networkParts.map((part, index) => part | (255 - maskParts[index]));
  const broadcast = broadcastParts.join('.');

  // 计算可用主机数
  const totalHosts = Math.pow(2, 32 - prefixLength);
  const usableHosts = totalHosts - 2;

  // 计算第一个和最后一个可用 IP
  const firstIpParts = [...networkParts];
  firstIpParts[3] += 1;
  const firstIp = firstIpParts.join('.');

  const lastIpParts = [...broadcastParts];
  lastIpParts[3] -= 1;
  const lastIp = lastIpParts.join('.');

  // 计算通配符掩码
  const wildcardParts = maskParts.map(part => 255 - part);
  const wildcard = wildcardParts.join('.');

  // 确定 IP 类别
  let ipClass = 'A 类';
  if (ipParts[0] >= 128 && ipParts[0] < 192) {
    ipClass = 'B 类';
  } else if (ipParts[0] >= 192 && ipParts[0] < 224) {
    ipClass = 'C 类';
  } else if (ipParts[0] >= 224 && ipParts[0] < 240) {
    ipClass = 'D 类 (组播)';
  } else if (ipParts[0] >= 240) {
    ipClass = 'E 类 (保留)';
  }

  // 确定 IP 类型
  let ipType = '公共 (Public)';
  if (
    (ipParts[0] === 10) ||
    (ipParts[0] === 172 && ipParts[1] >= 16 && ipParts[1] <= 31) ||
    (ipParts[0] === 192 && ipParts[1] === 168)
  ) {
    ipType = '私有 (Private)';
  }

  // 计算二进制掩码
  const binaryMask = maskParts.map(part => part.toString(2).padStart(8, '0')).join('.');

  // 计算二进制 IP
  const binaryIp = ipParts.map(part => part.toString(2).padStart(8, '0')).join('.');

  res.json({
    valid: true,
    ip,
    cidr,
    mask,
    network,
    broadcast,
    total_hosts: totalHosts,
    usable_hosts: usableHosts,
    ip_class: ipClass,
    ip_type: ipType,
    wildcard,
    first_ip: firstIp,
    last_ip: lastIp,
    binary_mask: binaryMask,
    binary_ip: binaryIp
  });
});

app.post('/cron', (req, res) => {
  const cron = req.body.cron || '';

  if (!cron) {
    res.json({ valid: false, next_runs: [], error: '请输入 Cron 表达式' });
    return;
  }

  // 简单的 Cron 表达式验证
  const cronRegex = /^\s*([^\s]+)\s+([^\s]+)\s+([^\s]+)\s+([^\s]+)\s+([^\s]+)\s*$/;
  if (!cronRegex.test(cron)) {
    res.json({ valid: false, next_runs: [], error: 'Cron 表达式格式错误' });
    return;
  }

  // 更详细的 Cron 表达式验证
  const parts = cron.trim().split(/\s+/);
  if (parts.length !== 5) {
    res.json({ valid: false, next_runs: [], error: 'Cron 表达式必须包含 5 个字段' });
    return;
  }

  const [minute, hour, day, month, weekday] = parts;

  // 验证分钟字段
  if (!validateCronField(minute, 0, 59)) {
    res.json({ valid: false, next_runs: [], error: '分钟字段格式错误' });
    return;
  }

  // 验证小时字段
  if (!validateCronField(hour, 0, 23)) {
    res.json({ valid: false, next_runs: [], error: '小时字段格式错误' });
    return;
  }

  // 验证日期字段
  if (!validateCronField(day, 1, 31)) {
    res.json({ valid: false, next_runs: [], error: '日期字段格式错误' });
    return;
  }

  // 验证月份字段
  if (!validateCronField(month, 1, 12)) {
    res.json({ valid: false, next_runs: [], error: '月份字段格式错误' });
    return;
  }

  // 验证星期字段
  if (!validateCronField(weekday, 0, 7)) {
    res.json({ valid: false, next_runs: [], error: '星期字段格式错误' });
    return;
  }

  // 生成下次执行时间（基于 Cron 表达式的简单模拟）
  const nextRuns = [];
  const now = new Date();

  // 简单的模拟：根据 Cron 表达式的分钟和小时字段生成执行时间
  for (let i = 0; i < 5; i++) {
    const nextRun = new Date(now);
    nextRun.setHours(nextRun.getHours() + i + 1);

    // 如果指定了小时，使用指定的小时
    if (hour !== '*') {
      nextRun.setHours(parseInt(hour) || 0);
    }

    // 如果指定了分钟，使用指定的分钟
    if (minute !== '*') {
      nextRun.setMinutes(parseInt(minute) || 0);
    } else {
      nextRun.setMinutes(0);
    }

    nextRun.setSeconds(0);
    nextRuns.push(nextRun.toISOString().replace('T', ' ').substring(0, 19) + ' UTC');
  }

  res.json({ valid: true, next_runs: nextRuns, error: '' });
});

app.post('/md5', (req, res) => {
  res.json({ md5_32_lower: 'd41d8cd98f00b204e9800998ecf8427e', md5_32_upper: 'D41D8CD98F00B204E9800998ECF8427E', md5_16_lower: '8f00b204e9800998', md5_16_upper: '8F00B204E9800998' });
});

app.post('/uuid', (req, res) => {
  const count = req.body.count || 5;
  const uuids = [];
  for (let i = 0; i < count; i++) {
    // 生成标准格式的UUID
    const uuid = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
      const r = Math.random() * 16 | 0;
      const v = c === 'x' ? r : (r & 0x3 | 0x8);
      return v.toString(16);
    });
    uuids.push(uuid);
  }
  res.json({ uuids });
});

app.post('/jwt', (req, res) => {
  res.json({ header: '{}', payload: '{}', error: null });
});

app.post('/base64', (req, res) => {
  const result = req.body.action === 'encode' ? Buffer.from(req.body.text).toString('base64') : Buffer.from(req.body.text, 'base64').toString('utf8');
  res.json({ result });
});

app.post('/js-enc', (req, res) => {
  const jsCode = req.body.js || '';
  // 简单的 JS 混淆逻辑
  let obfuscated = jsCode
    .replace(/function\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\(/g, 'function _$1(')
    .replace(/var\s+([a-zA-Z_$][a-zA-Z0-9_$]*)/g, 'var _$1')
    .replace(/let\s+([a-zA-Z_$][a-zA-Z0-9_$]*)/g, 'let _$1')
    .replace(/const\s+([a-zA-Z_$][a-zA-Z0-9_$]*)/g, 'const _$1');

  if (!obfuscated) {
    obfuscated = '// 请输入 JavaScript 代码进行混淆';
  }

  res.json({ result: obfuscated });
});

app.post('/password', (req, res) => {
  const length = req.body.length || 16;
  const uppercase = req.body.uppercase !== false; // 默认启用
  const lowercase = req.body.lowercase !== false; // 默认启用
  const numbers = req.body.numbers !== false; // 默认启用
  const symbols = req.body.symbols !== false; // 默认启用

  // 生成随机密码
  const charset = [];
  if (uppercase) charset.push('ABCDEFGHIJKLMNOPQRSTUVWXYZ');
  if (lowercase) charset.push('abcdefghijklmnopqrstuvwxyz');
  if (numbers) charset.push('0123456789');
  if (symbols) charset.push('!@#$%^&*()_+-=[]{}|;:,.<>?');

  const allChars = charset.join('');
  let password = '';
  for (let i = 0; i < length; i++) {
    const randomIndex = Math.floor(Math.random() * allChars.length);
    password += allChars[randomIndex];
  }

  res.json({ password });
});

app.post('/token', (req, res) => {
  const length = req.body.length || 32;
  const uppercase = req.body.uppercase !== false; // 默认启用
  const lowercase = req.body.lowercase !== false; // 默认启用
  const numbers = req.body.numbers !== false; // 默认启用
  const symbols = req.body.symbols !== false; // 默认启用

  // 生成随机token
  const charset = [];
  if (uppercase) charset.push('ABCDEFGHIJKLMNOPQRSTUVWXYZ');
  if (lowercase) charset.push('abcdefghijklmnopqrstuvwxyz');
  if (numbers) charset.push('0123456789');
  if (symbols) charset.push('!@#$%^&*()_+-=[]{}|;:,.<>?');

  const allChars = charset.join('');
  let token = '';
  for (let i = 0; i < length; i++) {
    const randomIndex = Math.floor(Math.random() * allChars.length);
    token += allChars[randomIndex];
  }

  res.json({ token });
});

app.post('/sql', (req, res) => {
  const sql = req.body.sql || '';

  if (!sql) {
    res.json({ result: '-- 请输入 SQL 语句进行格式化' });
    return;
  }

  // 简单的 SQL 格式化逻辑
  let formattedSql = sql
    // 关键字大写
    .replace(/\b(SELECT|FROM|WHERE|JOIN|LEFT|RIGHT|INNER|OUTER|GROUP|BY|ORDER|LIMIT|OFFSET|INSERT|UPDATE|DELETE|CREATE|ALTER|DROP|TABLE|DATABASE|INDEX|VIEW|PROCEDURE|FUNCTION|TRIGGER)\b/gi, match => match.toUpperCase())
    // 在关键字前添加换行
    .replace(/\b(SELECT|FROM|WHERE|JOIN|LEFT|RIGHT|INNER|OUTER|GROUP|BY|ORDER|LIMIT|OFFSET|INSERT|UPDATE|DELETE|CREATE|ALTER|DROP|TABLE|DATABASE|INDEX|VIEW|PROCEDURE|FUNCTION|TRIGGER)\b/gi, '\n$1')
    // 在逗号后添加空格
    .replace(/,/g, ', ')
    // 在等号前后添加空格
    .replace(/=/g, ' = ')
    // 在操作符前后添加空格
    .replace(/([<>!]=?|LIKE|IN|NOT|AND|OR|IS|NULL)/gi, ' $1 ')
    // 移除多余的空格
    .replace(/\s+/g, ' ')
    // 移除行首的空格
    .replace(/^\s+/gm, '')
    // 保留空行
    .replace(/\n\s*\n/g, '\n\n');

  res.json({ result: formattedSql });
});

app.post('/regex', (req, res) => {
  const pattern = req.body.pattern || '';
  const text = req.body.text || '';

  if (!pattern) {
    res.json({ matches: [], count: 0, error: '请输入正则表达式模式' });
    return;
  }

  if (!text) {
    res.json({ matches: [], count: 0, error: '请输入测试文本' });
    return;
  }

  try {
    // 创建正则表达式对象
    const regex = new RegExp(pattern, 'g');
    const matches = [];
    let match;

    // 找到所有匹配项
    while ((match = regex.exec(text)) !== null) {
      matches.push(match[0]);

      // 防止无限循环（当正则表达式包含零宽度断言时）
      if (match.index === regex.lastIndex) {
        regex.lastIndex++;
      }
    }

    res.json({ matches: matches, count: matches.length, error: null });
  } catch (error) {
    res.json({ matches: [], count: 0, error: '正则表达式语法错误' });
  }
});

app.post('/chmod', (req, res) => {
  const file = req.body.file && req.body.file.trim() ? req.body.file.trim() : 'filename';
  res.json({ valid: true, command: `chmod ${req.body.octal} ${file}` });
});

app.post('/api/dockerfile', (req, res) => {
  let stages = req.body.stages;

  // Backward compatibility: if no stages array, use root properties as a single stage
  if (!stages || !Array.isArray(stages)) {
    stages = [req.body];
  }

  let df = '';

  stages.forEach((stage, index) => {
    // Add separation between stages
    if (index > 0) {
      df += '\n# Stage ' + (index + 1) + '\n';
    }

    const { image, as, workdir, copy, run, env, expose, cmd, entrypoint, user, volume, arg, label, healthcheck } = stage;

    // FROM
    if (image && image.trim()) {
      df += `FROM ${image.trim()}`;
      if (as && as.trim()) {
        df += ` AS ${as.trim()}`;
      }
      df += '\n';
    } else {
      df += 'FROM scratch\n';
    }

    // ARG
    if (arg && typeof arg === 'string') {
      arg.split('\n').forEach(line => {
        if (line.trim()) df += `ARG ${line.trim()}\n`;
      });
    }

    // LABEL
    if (label && typeof label === 'string') {
      label.split('\n').forEach(line => {
        if (line.trim()) df += `LABEL ${line.trim()}\n`;
      });
    }

    // WORKDIR
    if (workdir && workdir.trim()) {
      df += `WORKDIR ${workdir.trim()}\n`;
    }

    // ENV
    if (env && typeof env === 'string') {
      env.split('\n').forEach(line => {
        if (line.trim()) df += `ENV ${line.trim()}\n`;
      });
    }

    // COPY
    if (copy && typeof copy === 'string') {
      copy.split('\n').forEach(line => {
        if (line.trim()) df += `COPY ${line.trim()}\n`;
      });
    }

    // RUN
    if (run && typeof run === 'string') {
      run.split('\n').forEach(line => {
        if (line.trim()) df += `RUN ${line.trim()}\n`;
      });
    }

    // EXPOSE
    if (expose && typeof expose === 'string' && expose.trim()) {
      expose.split(/[, ]+/).forEach(port => {
        if (port.trim()) df += `EXPOSE ${port.trim()}\n`;
      });
    }

    // USER
    if (user && user.trim()) {
      df += `USER ${user.trim()}\n`;
    }

    // VOLUME
    if (volume && typeof volume === 'string' && volume.trim()) {
      volume.split(/[, ]+/).forEach(v => {
        if (v.trim()) df += `VOLUME ${v.trim()}\n`;
      });
    }

    // HEALTHCHECK
    if (healthcheck && typeof healthcheck === 'string' && healthcheck.trim()) {
      df += `HEALTHCHECK ${healthcheck.trim()}\n`;
    }

    // ENTRYPOINT
    if (entrypoint && typeof entrypoint === 'string' && entrypoint.trim()) {
      df += `ENTRYPOINT ${entrypoint.trim()}\n`;
    }

    // CMD
    if (cmd && typeof cmd === 'string' && cmd.trim()) {
      df += `CMD ${cmd.trim()}\n`;
    }
  });

  res.json({ result: df });
});

app.post('/api/nginx', (req, res) => {
  const { domain, port, root, path, proxy, upstream, https, force_https, ssl_cert, ssl_key, spa, gzip, client_max_body_size, keepalive_timeout, proxy_connect_timeout, proxy_read_timeout, proxy_send_timeout, websocket } = req.body;
  let conf = '';
  const d = (domain && domain.trim()) ? domain.trim() : 'example.com';
  const loc = (path && path.trim()) ? path.trim() : '/';

  // Upstream
  const hasUpstream = upstream && upstream.trim().length > 0;
  const upstreamName = `${d.replace(/\./g, '_')}_backend`;

  if (hasUpstream) {
    conf += `upstream ${upstreamName} {\n`;
    upstream.split('\n').forEach(line => {
      if (line.trim()) conf += `    server ${line.trim()};\n`;
    });
    conf += '}\n\n';
  }

  // HTTP Redirect Block
  if (https && force_https) {
    conf += 'server {\n';
    conf += '    listen 80;\n';
    conf += `    server_name ${d};\n`;
    conf += '    return 301 https://$host$request_uri;\n';
    conf += '}\n\n';
  }

  // Main Server Block
  conf += 'server {\n';

  if (https) {
    conf += '    listen 443 ssl http2;\n';
    conf += `    server_name ${d};\n\n`;
    const certPath = (ssl_cert && ssl_cert.trim()) ? ssl_cert.trim() : `/etc/nginx/ssl/${d}.crt`;
    const keyPath = (ssl_key && ssl_key.trim()) ? ssl_key.trim() : `/etc/nginx/ssl/${d}.key`;

    conf += `    ssl_certificate ${certPath};\n`;
    conf += `    ssl_certificate_key ${keyPath};\n`;
    conf += '    ssl_protocols TLSv1.2 TLSv1.3;\n';
    conf += '    ssl_ciphers HIGH:!aNULL:!MD5;\n\n';
  } else {
    conf += `    listen ${port || 80};\n`;
    conf += `    server_name ${d};\n\n`;
  }

  // Access Logs
  conf += `    access_log /var/log/nginx/${d}.access.log;\n`;
  conf += `    error_log /var/log/nginx/${d}.error.log;\n\n`;

  // Gzip
  if (gzip) {
    conf += '    gzip on;\n';
    conf += '    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;\n\n';
  }

  // Limits & Timeouts
  if (client_max_body_size && client_max_body_size.trim()) {
    conf += `    client_max_body_size ${client_max_body_size.trim()};\n`;
  }
  if (keepalive_timeout && keepalive_timeout.trim()) {
    conf += `    keepalive_timeout ${keepalive_timeout.trim()};\n\n`;
  }

  // Proxy or Static
  if (hasUpstream) {
    conf += `    location ${loc} {\n`;
    conf += `        proxy_pass http://${upstreamName};\n`;
    conf += '        proxy_set_header Host $host;\n';
    conf += '        proxy_set_header X-Real-IP $remote_addr;\n';
    conf += '        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n';
    conf += '        proxy_set_header X-Forwarded-Proto $scheme;\n';
    if (websocket) {
      conf += '        proxy_http_version 1.1;\n';
      conf += '        proxy_set_header Upgrade $http_upgrade;\n';
      conf += '        proxy_set_header Connection "upgrade";\n';
    }
    if (proxy_connect_timeout && proxy_connect_timeout.trim()) conf += `        proxy_connect_timeout ${proxy_connect_timeout.trim()};\n`;
    if (proxy_read_timeout && proxy_read_timeout.trim()) conf += `        proxy_read_timeout ${proxy_read_timeout.trim()};\n`;
    if (proxy_send_timeout && proxy_send_timeout.trim()) conf += `        proxy_send_timeout ${proxy_send_timeout.trim()};\n`;
    conf += '    }\n';
  } else if (proxy && proxy.trim()) {
    conf += `    location ${loc} {\n`;
    conf += `        proxy_pass ${proxy.trim()};\n`;
    conf += '        proxy_set_header Host $host;\n';
    conf += '        proxy_set_header X-Real-IP $remote_addr;\n';
    conf += '        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n';
    conf += '        proxy_set_header X-Forwarded-Proto $scheme;\n';
    if (websocket) {
      conf += '        proxy_http_version 1.1;\n';
      conf += '        proxy_set_header Upgrade $http_upgrade;\n';
      conf += '        proxy_set_header Connection "upgrade";\n';
    }
    if (proxy_connect_timeout && proxy_connect_timeout.trim()) conf += `        proxy_connect_timeout ${proxy_connect_timeout.trim()};\n`;
    if (proxy_read_timeout && proxy_read_timeout.trim()) conf += `        proxy_read_timeout ${proxy_read_timeout.trim()};\n`;
    if (proxy_send_timeout && proxy_send_timeout.trim()) conf += `        proxy_send_timeout ${proxy_send_timeout.trim()};\n`;
    conf += '    }\n';
  } else {
    const rootPath = (root && root.trim()) ? root.trim() : '/var/www/html';
    conf += `    root ${rootPath};\n`;
    conf += '    index index.html index.htm;\n\n';

    conf += `    location ${loc} {\n`;
    if (spa) {
      conf += '        try_files $uri $uri/ /index.html;\n';
    } else {
      conf += '        try_files $uri $uri/ =404;\n';
    }
    conf += '    }\n';
  }

  conf += '}\n';
  res.json({ result: conf });
});

app.post('/api/curl', (req, res) => {
  const method = (req.body.method || 'GET').toUpperCase();
  const url = req.body.url || 'http://localhost:8080';
  const headers = req.body.headers || {};
  const body = req.body.body || '';

  let cmd = `curl -X ${method} '${url.replace(/'/g, "'\\''")}'`;

  // Python generation
  let py = "import requests\n\n";
  py += `url = "${url}"\n`;

  let hasHeaders = false;

  // Headers
  if (typeof headers === 'string') {
    try {
      const h = JSON.parse(headers);
      if (Object.keys(h).length > 0) {
        hasHeaders = true;
        py += "\nheaders = {\n";
        Object.keys(h).forEach(key => {
          cmd += ` \\\n  -H '${key}: ${h[key]}'`;
          py += `  '${key}': '${h[key]}',\n`;
        });
        py += "}\n";
      }
    } catch (e) { }
  } else {
    if (Object.keys(headers).length > 0) {
      hasHeaders = true;
      py += "\nheaders = {\n";
      Object.keys(headers).forEach(key => {
        cmd += ` \\\n  -H '${key}: ${headers[key]}'`;
        py += `  '${key}': '${headers[key]}',\n`;
      });
      py += "}\n";
    }
  }

  // Body
  let hasPayload = false;
  if (['POST', 'PUT', 'PATCH'].includes(method)) {
    if (body) {
      hasPayload = true;
      const pyBody = body.replace(/\\/g, "\\\\").replace(/"/g, "\\\"");
      py += `\npayload = "${pyBody}"\n`;

      if (typeof body === 'object') {
        cmd += ` \\\n  -H 'Content-Type: application/json'`;
        cmd += ` \\\n  -d '${JSON.stringify(body)}'`;
      } else {
        cmd += ` \\\n  -d '${body.replace(/'/g, "'\\''")}'`;
      }
    }
  }

  py += `\nresponse = requests.request("${method}", url`;
  if (hasHeaders) py += ", headers=headers";
  if (hasPayload) py += ", data=payload";
  py += ")\n\nprint(response.text)";

  res.json({ command: cmd, python: py });
});

app.post('/api/unit-convert', (req, res) => {
  const value = parseFloat(req.body.value) || 0;
  const type = req.body.type || 'storage'; // storage, time
  const from = req.body.from || 'B';
  const to = req.body.to || 'MB';

  let result = 0;

  if (type === 'storage') {
    const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
    const fromIdx = units.indexOf(from.toUpperCase());
    const toIdx = units.indexOf(to.toUpperCase());

    if (fromIdx !== -1 && toIdx !== -1) {
      const bytes = value * Math.pow(1024, fromIdx);
      result = bytes / Math.pow(1024, toIdx);
    }
  } else if (type === 'time') {
    const units = { 'ms': 1, 's': 1000, 'm': 60000, 'h': 3600000, 'd': 86400000 };
    const fromVal = units[from.toLowerCase()];
    const toVal = units[to.toLowerCase()];

    if (fromVal && toVal) {
      result = (value * fromVal) / toVal;
    }
  }

  res.json({ result, value, from, to, type });
});

app.post('/api/git-cmd', (req, res) => {
  const action = req.body.action || 'log';
  let command = 'git help';
  let description = '';

  switch (action) {
    case 'undo_commit':
      command = 'git reset --soft HEAD~1';
      description = '撤销最近一次提交，但保留文件修改（Soft Reset）';
      break;
    case 'undo_changes':
      command = 'git checkout .';
      description = '撤销工作区所有修改（危险：会丢失未提交的改动）';
      break;
    case 'log_graph':
      command = 'git log --graph --oneline --decorate --all';
      description = '以图形化方式查看提交历史';
      break;
    case 'tag':
      const tag = req.body.tag || 'v1.0.0';
      const msg = req.body.msg || 'Release version';
      command = `git tag -a ${tag} -m "${msg}" && git push origin ${tag}`;
      description = '创建并推送带注释的标签';
      break;
    case 'branch_delete':
      const branch = req.body.branch || 'feature/old';
      command = `git branch -d ${branch} && git push origin --delete ${branch}`;
      description = '删除本地和远程分支';
      break;
    case 'stash':
      command = 'git stash && git pull && git stash pop';
      description = '暂存修改，拉取代码，然后恢复修改';
      break;
  }

  res.json({ command, description });
});

app.post('/api/awk', (req, res) => {
  const { separator, variable, code, file } = req.body;
  let cmd = 'awk';

  if (separator && separator !== 'space') {
    cmd += ` -F '${separator.replace(/'/g, "'\\''")}'`;
  }

  if (variable && variable.trim()) {
    cmd += ` -v ${variable.trim()}`;
  }

  cmd += ` '${code && code.trim() ? code.trim() : '{print $0}'}'`;

  if (file && file.trim()) {
    cmd += ` ${file.trim()}`;
  }

  res.json({ command: cmd });
});

app.post('/api/sed', (req, res) => {
  const { operation, pattern, replacement, flags, inplace, file } = req.body;
  let cmd = 'sed';

  if (inplace) cmd += ' -i';

  cmd += " '";

  if (operation === 'substitute') {
    cmd += `s/${(pattern || '').replace(/\//g, '\\/')}/${(replacement || '').replace(/\//g, '\\/')}/${flags || ''}`;
  } else if (operation === 'delete') {
    cmd += `${pattern || ''}d`;
  } else if (operation === 'insert') {
    cmd += `${pattern || ''}i\\ ${replacement || ''}`;
  } else if (operation === 'append') {
    cmd += `${pattern || ''}a\\ ${replacement || ''}`;
  }

  cmd += "'";

  if (file && file.trim()) {
    cmd += ` ${file.trim()}`;
  }

  res.json({ command: cmd });
});

// Start the server
app.listen(PORT, () => {
  console.log(`Local server running at http://localhost:${PORT}`);
  console.log('Available endpoints:');
  console.log('- GET /: Homepage');
  console.log('- GET /dockerfile: Dockerfile Generator UI');
  console.log('- POST /api/sql: SQL formatting');
  console.log('- POST /api/diff: Text diff');
  console.log('- POST /api/cron: Cron validation');
  console.log('- POST /api/subnet: Subnet calculation');
  console.log('- POST /api/regex-gen: Regex generation');
  console.log('- POST /api/url: URL encoding/decoding');
  console.log('- POST /api/password: Password generation');
  console.log('- POST /api/json: JSON formatting');
  console.log('- POST /api/curl: Curl command builder');
  console.log('- POST /api/unit-convert: Unit converter');
  console.log('- POST /api/git-cmd: Git command helper');
});
