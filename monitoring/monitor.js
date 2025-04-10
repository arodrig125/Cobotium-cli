/**
 * Cobotium Program Monitoring Script
 * 
 * This script monitors the Cobotium token program on the Solana blockchain.
 * It tracks transactions, accounts, and errors, and sends alerts when necessary.
 * 
 * Usage:
 * 1. Install dependencies: npm install @solana/web3.js axios discord.js
 * 2. Update the configuration section below
 * 3. Run the script: node monitor.js
 * 
 * For production use, run with PM2: pm2 start monitor.js --name cobotium-monitor
 */

const { Connection, PublicKey } = require('@solana/web3.js');
const axios = require('axios');
const fs = require('fs');
const path = require('path');
const { WebhookClient } = require('discord.js');

// ======== CONFIGURATION ========
// Update these values with your own
const CONFIG = {
  // Program ID of your Cobotium token program
  PROGRAM_ID: 'YOUR_PROGRAM_ID_HERE',
  
  // RPC endpoint (use a dedicated endpoint for production)
  RPC_URL: 'https://api.mainnet-beta.solana.com',
  
  // Discord webhook URL for alerts
  DISCORD_WEBHOOK: 'YOUR_DISCORD_WEBHOOK_URL',
  
  // Email for alerts (requires mailgun or similar)
  ALERT_EMAIL: 'your-email@example.com',
  MAILGUN_API_KEY: 'YOUR_MAILGUN_API_KEY',
  MAILGUN_DOMAIN: 'YOUR_MAILGUN_DOMAIN',
  
  // Logging
  LOG_DIR: path.join(__dirname, 'logs'),
  TRANSACTION_LOG: 'transactions.log',
  ERROR_LOG: 'errors.log',
  ALERT_LOG: 'alerts.log',
  
  // Alert thresholds
  TRANSACTION_THRESHOLD: 100, // Alert if transactions per minute exceeds this
  ERROR_THRESHOLD: 5, // Alert if errors per minute exceeds this
  LARGE_TRANSFER_THRESHOLD: 1000000000, // 1000 tokens with 9 decimals
  
  // Monitoring intervals (in milliseconds)
  ACCOUNT_SCAN_INTERVAL: 3600000, // 1 hour
  METRICS_UPDATE_INTERVAL: 60000, // 1 minute
};

// ======== INITIALIZATION ========

// Create log directory if it doesn't exist
if (!fs.existsSync(CONFIG.LOG_DIR)) {
  fs.mkdirSync(CONFIG.LOG_DIR, { recursive: true });
}

// Initialize connection
const connection = new Connection(CONFIG.RPC_URL);
const programId = new PublicKey(CONFIG.PROGRAM_ID);

// Initialize Discord webhook
const webhook = CONFIG.DISCORD_WEBHOOK ? new WebhookClient({ url: CONFIG.DISCORD_WEBHOOK }) : null;

// Initialize metrics
const metrics = {
  accountCount: 0,
  transactionCount: 0,
  errorCount: 0,
  lastTransactionTime: 0,
  transactionsPerMinute: 0,
  errorsPerMinute: 0,
  startTime: Date.now(),
  
  // Transaction counters (reset every minute)
  minuteTransactions: 0,
  minuteErrors: 0,
  
  // Instruction type counters
  mintCount: 0,
  transferCount: 0,
  burnCount: 0,
  freezeCount: 0,
  thawCount: 0,
};

// ======== LOGGING FUNCTIONS ========

/**
 * Log a message to a file and console
 */
function log(message, logFile) {
  const timestamp = new Date().toISOString();
  const logMessage = `[${timestamp}] ${message}`;
  
  console.log(logMessage);
  
  const logPath = path.join(CONFIG.LOG_DIR, logFile);
  fs.appendFileSync(logPath, logMessage + '\n');
}

/**
 * Log a transaction
 */
function logTransaction(message) {
  log(message, CONFIG.TRANSACTION_LOG);
}

/**
 * Log an error
 */
function logError(message) {
  log(`ERROR: ${message}`, CONFIG.ERROR_LOG);
}

/**
 * Log and send an alert
 */
async function sendAlert(message, level = 'info') {
  // Log the alert
  log(`ALERT (${level}): ${message}`, CONFIG.ALERT_LOG);
  
  // Send to Discord
  if (webhook) {
    try {
      const emoji = level === 'critical' ? '🚨' : level === 'warning' ? '⚠️' : 'ℹ️';
      await webhook.send({
        content: `${emoji} **${level.toUpperCase()}**: ${message}`,
        username: 'Cobotium Monitor',
      });
    } catch (error) {
      logError(`Failed to send Discord alert: ${error.message}`);
    }
  }
  
  // Send email for critical alerts
  if (level === 'critical' && CONFIG.MAILGUN_API_KEY) {
    try {
      await axios.post(
        `https://api.mailgun.net/v3/${CONFIG.MAILGUN_DOMAIN}/messages`,
        new URLSearchParams({
          from: `Cobotium Monitor <monitor@${CONFIG.MAILGUN_DOMAIN}>`,
          to: CONFIG.ALERT_EMAIL,
          subject: `🚨 CRITICAL ALERT: Cobotium Program`,
          text: message,
        }),
        {
          auth: {
            username: 'api',
            password: CONFIG.MAILGUN_API_KEY,
          },
        }
      );
    } catch (error) {
      logError(`Failed to send email alert: ${error.message}`);
    }
  }
}

// ======== MONITORING FUNCTIONS ========

/**
 * Scan program accounts
 */
async function scanAccounts() {
  try {
    logTransaction('Scanning program accounts...');
    
    const accounts = await connection.getProgramAccounts(programId);
    const oldAccountCount = metrics.accountCount;
    metrics.accountCount = accounts.length;
    
    logTransaction(`Found ${accounts.length} accounts for program ${CONFIG.PROGRAM_ID}`);
    
    // Alert on significant account growth
    if (oldAccountCount > 0 && metrics.accountCount > oldAccountCount * 1.1) {
      await sendAlert(
        `Account count increased by ${metrics.accountCount - oldAccountCount} (${((metrics.accountCount - oldAccountCount) / oldAccountCount * 100).toFixed(2)}%)`,
        'warning'
      );
    }
    
    // Analyze accounts (add more analysis as needed)
    // ...
    
  } catch (error) {
    logError(`Error scanning accounts: ${error.message}`);
    metrics.errorCount++;
    metrics.minuteErrors++;
  }
}

/**
 * Update metrics (called every minute)
 */
function updateMetrics() {
  // Calculate transactions per minute
  metrics.transactionsPerMinute = metrics.minuteTransactions;
  metrics.errorsPerMinute = metrics.minuteErrors;
  
  // Check alert thresholds
  if (metrics.transactionsPerMinute > CONFIG.TRANSACTION_THRESHOLD) {
    sendAlert(
      `High transaction volume: ${metrics.transactionsPerMinute} transactions in the last minute`,
      'warning'
    );
  }
  
  if (metrics.errorsPerMinute > CONFIG.ERROR_THRESHOLD) {
    sendAlert(
      `High error rate: ${metrics.errorsPerMinute} errors in the last minute`,
      'critical'
    );
  }
  
  // Log current metrics
  logTransaction(`METRICS: accounts=${metrics.accountCount}, txs=${metrics.transactionCount}, errors=${metrics.errorCount}, txs/min=${metrics.transactionsPerMinute}, errors/min=${metrics.errorsPerMinute}`);
  
  // Reset minute counters
  metrics.minuteTransactions = 0;
  metrics.minuteErrors = 0;
}

/**
 * Parse instruction type from logs
 */
function parseInstructionType(logs) {
  if (!logs || !Array.isArray(logs)) return 'unknown';
  
  for (const log of logs) {
    if (log.includes('Instruction: InitializeMint')) return 'init_mint';
    if (log.includes('Instruction: InitializeAccount')) return 'init_account';
    if (log.includes('Instruction: MintTo')) return 'mint';
    if (log.includes('Instruction: Transfer')) return 'transfer';
    if (log.includes('Instruction: Burn')) return 'burn';
    if (log.includes('Instruction: FreezeAccount')) return 'freeze';
    if (log.includes('Instruction: ThawAccount')) return 'thaw';
  }
  
  return 'unknown';
}

/**
 * Estimate transfer amount from logs (simplified)
 */
function estimateTransferAmount(logs) {
  if (!logs || !Array.isArray(logs)) return 0;
  
  for (const log of logs) {
    if (log.includes('Transferred ')) {
      const match = log.match(/Transferred (\d+) tokens/);
      if (match && match[1]) {
        return parseInt(match[1], 10);
      }
    }
    
    if (log.includes('Minted ')) {
      const match = log.match(/Minted (\d+) tokens/);
      if (match && match[1]) {
        return parseInt(match[1], 10);
      }
    }
    
    if (log.includes('Burned ')) {
      const match = log.match(/Burned (\d+) tokens/);
      if (match && match[1]) {
        return parseInt(match[1], 10);
      }
    }
  }
  
  return 0;
}

/**
 * Monitor program transactions
 */
async function monitorTransactions() {
  try {
    logTransaction('Starting transaction monitoring...');
    
    // Subscribe to program logs
    connection.onLogs(programId, (logs) => {
      metrics.transactionCount++;
      metrics.minuteTransactions++;
      metrics.lastTransactionTime = Date.now();
      
      const instructionType = parseInstructionType(logs.logs);
      const amount = estimateTransferAmount(logs.logs);
      
      // Update instruction counters
      if (instructionType === 'mint') metrics.mintCount++;
      if (instructionType === 'transfer') metrics.transferCount++;
      if (instructionType === 'burn') metrics.burnCount++;
      if (instructionType === 'freeze') metrics.freezeCount++;
      if (instructionType === 'thaw') metrics.thawCount++;
      
      // Log transaction
      logTransaction(`Transaction ${logs.signature} - Type: ${instructionType}, Amount: ${amount}`);
      
      // Check for errors
      if (logs.err) {
        logError(`Transaction error in ${logs.signature}: ${JSON.stringify(logs.err)}`);
        metrics.errorCount++;
        metrics.minuteErrors++;
        
        // Alert on errors
        sendAlert(`Transaction error: ${logs.signature}`, 'warning');
      }
      
      // Alert on large transfers
      if ((instructionType === 'transfer' || instructionType === 'mint') && amount > CONFIG.LARGE_TRANSFER_THRESHOLD) {
        sendAlert(`Large ${instructionType}: ${amount} tokens in tx ${logs.signature}`, 'warning');
      }
      
      // Always alert on freeze/thaw
      if (instructionType === 'freeze') {
        sendAlert(`Account frozen in tx ${logs.signature}`, 'critical');
      }
      
      if (instructionType === 'thaw') {
        sendAlert(`Account thawed in tx ${logs.signature}`, 'critical');
      }
    });
    
    logTransaction('Transaction monitoring started successfully');
  } catch (error) {
    logError(`Error setting up transaction monitoring: ${error.message}`);
    metrics.errorCount++;
    
    // Try to reconnect after a delay
    setTimeout(monitorTransactions, 30000);
  }
}

// ======== MAIN FUNCTION ========

/**
 * Main function
 */
async function main() {
  logTransaction('='.repeat(50));
  logTransaction(`Starting Cobotium monitoring service for program ${CONFIG.PROGRAM_ID}`);
  logTransaction(`RPC URL: ${CONFIG.RPC_URL}`);
  logTransaction('='.repeat(50));
  
  try {
    // Initial account scan
    await scanAccounts();
    
    // Set up transaction monitoring
    await monitorTransactions();
    
    // Set up periodic tasks
    setInterval(scanAccounts, CONFIG.ACCOUNT_SCAN_INTERVAL);
    setInterval(updateMetrics, CONFIG.METRICS_UPDATE_INTERVAL);
    
    // Send startup alert
    await sendAlert(`Cobotium monitoring service started for program ${CONFIG.PROGRAM_ID}`, 'info');
    
  } catch (error) {
    logError(`Fatal error: ${error.message}`);
    await sendAlert(`Monitoring service error: ${error.message}`, 'critical');
    process.exit(1);
  }
}

// Start the monitoring service
main().catch(error => {
  console.error(`Unhandled error: ${error.message}`);
  process.exit(1);
});
