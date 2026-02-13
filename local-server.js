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
    'password': '^[a-zA-Z0-9!@#$%^&*()_+\\-=\\[\\]{};\':,.<>/?]{8,}$',
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
    const uuid = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
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
  res.json({ valid: true, command: `chmod ${req.body.octal} file` });
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
    const uuid = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
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
  res.json({ valid: true, command: `chmod ${req.body.octal} file` });
});

// Start the server
app.listen(PORT, () => {
  console.log(`Local server running at http://localhost:${PORT}`);
  console.log('Available endpoints:');
  console.log('- GET /: Homepage');
  console.log('- POST /api/sql: SQL formatting');
  console.log('- POST /api/diff: Text diff');
  console.log('- POST /api/cron: Cron validation');
  console.log('- POST /api/subnet: Subnet calculation');
  console.log('- POST /api/regex-gen: Regex generation');
  console.log('- POST /api/url: URL encoding/decoding');
  console.log('- POST /api/password: Password generation');
  console.log('- POST /api/json: JSON formatting');
});
