# Cobotium Monitoring System

This directory contains the monitoring system for the Cobotium token program on the Solana blockchain.

## Overview

The monitoring system tracks:

- Transactions involving your Cobotium token program
- Account creation and usage
- Errors and exceptions
- Specific operations (minting, transfers, burns, freezes, thaws)
- Performance metrics

## Installation

### Automatic Installation

1. Make the setup script executable:

```bash
chmod +x setup.sh
```

2. Run the setup script:

```bash
./setup.sh
```

3. Follow the prompts to configure the monitoring system.

### Manual Installation

1. Install Node.js and npm if not already installed:

```bash
curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
sudo apt-get install -y nodejs
```

2. Install PM2 globally:

```bash
npm install -g pm2
```

3. Install dependencies:

```bash
npm install
```

4. Edit `monitor.js` and update the configuration section with your:
   - Program ID
   - Discord webhook URL (for alerts)
   - Email settings (for critical alerts)

5. Create the logs directory:

```bash
mkdir -p logs
```

6. Start the monitoring service:

```bash
pm2 start monitor.js --name cobotium-monitor
```

7. Set up PM2 to start on boot:

```bash
pm2 save
pm2 startup
```

## Usage

### Viewing Logs

To view the monitoring logs:

```bash
pm2 logs cobotium-monitor
```

You can also view the log files directly in the `logs` directory:

- `transactions.log`: All transactions and general information
- `errors.log`: Errors and exceptions
- `alerts.log`: Alerts that were sent

### Managing the Service

- Stop the monitoring service:

```bash
pm2 stop cobotium-monitor
```

- Restart the monitoring service:

```bash
pm2 restart cobotium-monitor
```

- View service status:

```bash
pm2 status
```

## Alerts

The monitoring system sends alerts for:

- High transaction volume
- Errors and exceptions
- Large token transfers
- Freeze and thaw operations
- Significant account growth

Alerts are sent to:

1. Discord (via webhook)
2. Email (for critical alerts)
3. Log files (all alerts)

## Customization

You can customize the monitoring system by editing the `CONFIG` object in `monitor.js`:

- `TRANSACTION_THRESHOLD`: Alert threshold for transactions per minute
- `ERROR_THRESHOLD`: Alert threshold for errors per minute
- `LARGE_TRANSFER_THRESHOLD`: Threshold for large transfer alerts
- `ACCOUNT_SCAN_INTERVAL`: How often to scan program accounts
- `METRICS_UPDATE_INTERVAL`: How often to update and check metrics

## Integration with Other Services

### Prometheus Integration

To expose metrics for Prometheus, you can add an HTTP server to `monitor.js`:

```javascript
const express = require('express');
const app = express();
const port = 9090;

app.get('/metrics', (req, res) => {
  res.set('Content-Type', 'text/plain');
  res.send(`
# HELP cobotium_account_count Total number of accounts
# TYPE cobotium_account_count gauge
cobotium_account_count ${metrics.accountCount}

# HELP cobotium_transaction_count Total number of transactions
# TYPE cobotium_transaction_count counter
cobotium_transaction_count ${metrics.transactionCount}

# HELP cobotium_error_count Total number of errors
# TYPE cobotium_error_count counter
cobotium_error_count ${metrics.errorCount}
  `);
});

app.listen(port, () => {
  console.log(`Metrics server listening at http://localhost:${port}`);
});
```

Don't forget to install Express:

```bash
npm install express
```

### Grafana Dashboard

A sample Grafana dashboard configuration is available in the `grafana` directory.

## Troubleshooting

### Common Issues

#### "Error: Connection Closed by Remote"

This usually indicates an issue with the RPC endpoint. Try:

1. Using a different RPC endpoint
2. Checking if you've exceeded rate limits
3. Restarting the monitoring service

#### "Error: Cannot Find Module"

If you see this error, a dependency is missing. Run:

```bash
npm install
```

#### High CPU Usage

If the monitoring service is using too much CPU:

1. Increase the `ACCOUNT_SCAN_INTERVAL` and `METRICS_UPDATE_INTERVAL`
2. Use a more efficient RPC endpoint
3. Filter the transactions you're monitoring

## Support

For support with the monitoring system, please:

1. Check the [Cobotium documentation](https://cobotium.io/docs)
2. Open an issue on the [GitHub repository](https://github.com/arodrig125/Cobotium-cli)
3. Contact the Cobotium team through the website
