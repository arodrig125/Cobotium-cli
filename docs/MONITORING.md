# Cobotium Monitoring Guide

This guide provides detailed instructions for setting up comprehensive monitoring for your Cobotium token program on the Solana blockchain.

**Official Website**: [cobotium.io](https://cobotium.io)

## Table of Contents

1. [Introduction](#introduction)
2. [Basic Monitoring Setup](#basic-monitoring-setup)
3. [Advanced Monitoring](#advanced-monitoring)
4. [Alerting](#alerting)
5. [Dashboard Setup](#dashboard-setup)
6. [Emergency Response](#emergency-response)

## Introduction

Monitoring your Cobotium token program is essential for:

- Ensuring the program is functioning correctly
- Detecting suspicious activity
- Responding quickly to issues
- Gathering usage statistics
- Planning for future upgrades

This guide will help you set up a comprehensive monitoring system for your Cobotium deployment.

## Basic Monitoring Setup

### Transaction Monitoring

The simplest way to monitor your program is using the Solana CLI:

```bash
solana logs --url https://api.mainnet-beta.solana.com <PROGRAM_ID>
```

This will stream all logs related to your program. You can redirect this output to a file for later analysis:

```bash
solana logs --url https://api.mainnet-beta.solana.com <PROGRAM_ID> > cobotium_logs.txt
```

### Creating a Simple Monitoring Script

Here's a simple bash script to monitor your program and send alerts:

```bash
#!/bin/bash

PROGRAM_ID="your_program_id_here"
EMAIL="your_email@example.com"
DISCORD_WEBHOOK="your_discord_webhook_url"

# Monitor logs
solana logs --url https://api.mainnet-beta.solana.com $PROGRAM_ID | while read line; do
  echo "$line" >> cobotium_logs.txt
  
  # Check for important events
  if [[ $line == *"Instruction: FreezeAccount"* ]]; then
    echo "ALERT: Account freeze detected!" | mail -s "Cobotium Alert" $EMAIL
    curl -H "Content-Type: application/json" -d '{"content": "ALERT: Account freeze detected!"}' $DISCORD_WEBHOOK
  fi
  
  if [[ $line == *"Error"* ]]; then
    echo "ALERT: Error detected in program!" | mail -s "Cobotium Alert" $EMAIL
    curl -H "Content-Type: application/json" -d '{"content": "ALERT: Error detected in program!"}' $DISCORD_WEBHOOK
  fi
done
```

Save this as `monitor.sh`, make it executable with `chmod +x monitor.sh`, and run it in the background with `nohup ./monitor.sh &`.

## Advanced Monitoring

For production environments, you'll want a more robust monitoring solution.

### Setting Up a Dedicated Monitoring Server

1. Provision a dedicated server (e.g., AWS EC2, DigitalOcean Droplet)
2. Install necessary dependencies:

```bash
apt-get update
apt-get install -y nodejs npm python3 python3-pip
pip3 install solana
npm install -g pm2
```

3. Create a more advanced monitoring script:

```javascript
// monitor.js
const { Connection, PublicKey } = require('@solana/web3.js');
const axios = require('axios');
const fs = require('fs');

// Configuration
const PROGRAM_ID = 'your_program_id_here';
const RPC_URL = 'https://api.mainnet-beta.solana.com';
const DISCORD_WEBHOOK = 'your_discord_webhook_url';
const LOG_FILE = 'cobotium_transactions.log';

// Initialize connection
const connection = new Connection(RPC_URL);
const programId = new PublicKey(PROGRAM_ID);

// Log and alert function
async function logAndAlert(message, isAlert = false) {
  const timestamp = new Date().toISOString();
  const logMessage = `[${timestamp}] ${message}`;
  
  console.log(logMessage);
  fs.appendFileSync(LOG_FILE, logMessage + '\n');
  
  if (isAlert) {
    try {
      await axios.post(DISCORD_WEBHOOK, {
        content: `🚨 ALERT: ${message}`
      });
    } catch (error) {
      console.error('Failed to send alert:', error);
    }
  }
}

// Monitor program accounts
async function monitorAccounts() {
  try {
    const accounts = await connection.getProgramAccounts(programId);
    logAndAlert(`Found ${accounts.length} accounts for program ${PROGRAM_ID}`);
    
    // Analyze accounts for suspicious activity
    // ...
  } catch (error) {
    logAndAlert(`Error monitoring accounts: ${error.message}`, true);
  }
}

// Monitor transactions
async function monitorTransactions() {
  try {
    // Subscribe to program logs
    connection.onLogs(programId, (logs) => {
      if (logs.err) {
        logAndAlert(`Transaction error: ${JSON.stringify(logs.err)}`, true);
      } else {
        logAndAlert(`New transaction: ${logs.signature}`);
        
        // Check for specific instructions
        if (logs.logs.some(log => log.includes('Instruction: FreezeAccount'))) {
          logAndAlert(`Account freeze detected in tx: ${logs.signature}`, true);
        }
        
        if (logs.logs.some(log => log.includes('Instruction: MintTo'))) {
          logAndAlert(`Token minting detected in tx: ${logs.signature}`);
        }
      }
    });
    
    logAndAlert('Transaction monitoring started');
  } catch (error) {
    logAndAlert(`Error setting up transaction monitoring: ${error.message}`, true);
  }
}

// Run monitoring
async function main() {
  logAndAlert('Starting Cobotium monitoring service');
  
  // Initial account scan
  await monitorAccounts();
  
  // Set up transaction monitoring
  await monitorTransactions();
  
  // Periodic account scans
  setInterval(monitorAccounts, 3600000); // Every hour
}

main().catch(error => {
  logAndAlert(`Fatal error: ${error.message}`, true);
});
```

4. Install dependencies and run with PM2:

```bash
npm init -y
npm install @solana/web3.js axios
pm2 start monitor.js --name cobotium-monitor
pm2 save
pm2 startup
```

### Integration with Monitoring Services

#### Prometheus and Grafana Setup

1. Install Prometheus:

```bash
wget https://github.com/prometheus/prometheus/releases/download/v2.37.0/prometheus-2.37.0.linux-amd64.tar.gz
tar xvfz prometheus-2.37.0.linux-amd64.tar.gz
cd prometheus-2.37.0.linux-amd64/
```

2. Create a Prometheus config file:

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'cobotium'
    static_configs:
      - targets: ['localhost:9090']
  
  - job_name: 'solana'
    static_configs:
      - targets: ['localhost:8899']
```

3. Install Grafana:

```bash
apt-get install -y apt-transport-https software-properties-common
wget -q -O - https://packages.grafana.com/gpg.key | apt-key add -
echo "deb https://packages.grafana.com/oss/deb stable main" | tee -a /etc/apt/sources.list.d/grafana.list
apt-get update
apt-get install -y grafana
systemctl enable grafana-server
systemctl start grafana-server
```

4. Create a custom exporter for your Cobotium metrics:

```javascript
// exporter.js
const express = require('express');
const { Connection, PublicKey } = require('@solana/web3.js');
const app = express();
const port = 9090;

// Configuration
const PROGRAM_ID = 'your_program_id_here';
const RPC_URL = 'https://api.mainnet-beta.solana.com';

// Initialize connection
const connection = new Connection(RPC_URL);
const programId = new PublicKey(PROGRAM_ID);

// Metrics
let accountCount = 0;
let transactionCount = 0;
let errorCount = 0;
let lastTransactionTime = 0;

// Update metrics periodically
async function updateMetrics() {
  try {
    const accounts = await connection.getProgramAccounts(programId);
    accountCount = accounts.length;
    
    // More metrics collection...
  } catch (error) {
    console.error('Error updating metrics:', error);
    errorCount++;
  }
}

// Expose metrics endpoint for Prometheus
app.get('/metrics', (req, res) => {
  res.set('Content-Type', 'text/plain');
  res.send(`
# HELP cobotium_account_count Total number of accounts for the Cobotium program
# TYPE cobotium_account_count gauge
cobotium_account_count ${accountCount}

# HELP cobotium_transaction_count Total number of transactions processed
# TYPE cobotium_transaction_count counter
cobotium_transaction_count ${transactionCount}

# HELP cobotium_error_count Total number of errors encountered
# TYPE cobotium_error_count counter
cobotium_error_count ${errorCount}

# HELP cobotium_last_transaction_time Timestamp of the last transaction
# TYPE cobotium_last_transaction_time gauge
cobotium_last_transaction_time ${lastTransactionTime}
  `);
});

// Start server
app.listen(port, () => {
  console.log(`Cobotium metrics exporter listening at http://localhost:${port}`);
});

// Update metrics every minute
updateMetrics();
setInterval(updateMetrics, 60000);

// Monitor transactions
connection.onLogs(programId, (logs) => {
  transactionCount++;
  lastTransactionTime = Date.now();
  
  if (logs.err) {
    errorCount++;
  }
});
```

5. Install dependencies and run the exporter:

```bash
npm install express @solana/web3.js
node exporter.js
```

## Alerting

### Setting Up Alerts

Configure alerts for various scenarios:

#### High Transaction Volume

Alert when transaction volume exceeds normal levels, which could indicate:
- Increased adoption (positive)
- Potential attack (negative)

#### Error Rate

Alert when error rate exceeds a threshold, which could indicate:
- Program bugs
- Network issues
- Malicious activity

#### Large Transfers

Alert on unusually large token transfers, which could indicate:
- Whale activity
- Potential theft

#### Freeze Events

Always alert on freeze/thaw events, as these are administrative actions.

### Alert Channels

Set up multiple alert channels for redundancy:

1. Email
2. SMS
3. Discord/Slack
4. PagerDuty (for critical alerts)

## Dashboard Setup

Create a monitoring dashboard using Grafana:

1. Log in to Grafana (default: http://your-server-ip:3000)
2. Add Prometheus as a data source
3. Create a new dashboard with panels for:
   - Account count
   - Transaction volume
   - Error rate
   - Token supply
   - Recent freeze events
   - Program health status

## Emergency Response

### Prepare an Emergency Response Plan

1. **Define Severity Levels**:
   - Level 1: Minor issues (e.g., occasional errors)
   - Level 2: Moderate issues (e.g., increased error rate)
   - Level 3: Critical issues (e.g., potential exploit)

2. **Response Team**:
   - Assign roles and responsibilities
   - Create an on-call schedule
   - Establish communication channels

3. **Response Procedures**:
   - For Level 3 incidents, consider using the freeze functionality
   - Document steps for common scenarios
   - Create recovery procedures

4. **Post-Incident Analysis**:
   - Document what happened
   - Identify root causes
   - Implement preventive measures

### Emergency Actions

In case of a critical security incident:

1. Freeze affected accounts using:
```bash
cobotium-cli --program-id <PROGRAM_ID> freeze-account --account <ACCOUNT_ADDRESS> --mint <MINT_ADDRESS> --freeze-authority <FREEZE_AUTH_KEYPAIR>
```

2. Notify users through:
   - Website (cobotium.io)
   - Social media
   - Email

3. Investigate the incident while accounts are frozen

4. Implement fixes and thaw accounts when safe

## Conclusion

By implementing this monitoring setup, you'll have comprehensive visibility into your Cobotium token program's operation on mainnet. This will help ensure the security and reliability of your token platform.

For more information and updates, visit [cobotium.io](https://cobotium.io).
