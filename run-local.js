// Simple local runner for the Cloudflare Worker
const { Worker } = require('worker_threads');
const fs = require('fs');
const path = require('path');

// Read the built worker files
const workerPath = path.join(__dirname, 'build', 'worker', 'shim.mjs');

if (!fs.existsSync(workerPath)) {
  console.error('Worker not built. Please run: cargo install -q worker-build && worker-build --release');
  process.exit(1);
}

console.log('Starting local worker...');
console.log('Worker path:', workerPath);

// For Cloudflare Workers, we can use miniflare as a local emulator
// Let's check if miniflare is available
const { execSync } = require('child_process');
try {
  execSync('npx miniflare --version', { stdio: 'ignore' });
  console.log('Using miniflare to run worker...');
  
  // Run miniflare with the built worker
  const { spawn } = require('child_process');
  const miniflare = spawn('npx', ['miniflare', 'build/worker/shim.mjs', '--watch', '--debug']);
  
  miniflare.stdout.on('data', (data) => {
    console.log(data.toString());
  });
  
  miniflare.stderr.on('data', (data) => {
    console.error(data.toString());
  });
  
  miniflare.on('close', (code) => {
    console.log(`Miniflare exited with code ${code}`);
  });
  
} catch (error) {
  console.log('Miniflare not available, showing build info...');
  console.log('Build completed successfully!');
  console.log('Worker files:');
  
  // List built files
  const buildFiles = fs.readdirSync(path.join(__dirname, 'build'));
  buildFiles.forEach(file => {
    console.log(`- ${file}`);
  });
  
  console.log('\nTo run this worker:');
  console.log('1. Deploy to Cloudflare: npx wrangler publish');
  console.log('2. Or use miniflare locally: npm install -g miniflare && npx miniflare build/worker/shim.mjs');
}
