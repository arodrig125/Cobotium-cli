#!/bin/bash

# Cobotium Monitoring Setup Script
# This script sets up the Cobotium monitoring service

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Installing..."
    curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
    sudo apt-get install -y nodejs
fi

# Check if PM2 is installed
if ! command -v pm2 &> /dev/null; then
    echo "PM2 is not installed. Installing..."
    npm install -g pm2
fi

# Install dependencies
echo "Installing dependencies..."
npm install

# Create logs directory
mkdir -p logs

# Update configuration
echo "Please enter your Cobotium program ID:"
read PROGRAM_ID

echo "Please enter your Discord webhook URL (leave empty to skip):"
read DISCORD_WEBHOOK

echo "Please enter your alert email (leave empty to skip):"
read ALERT_EMAIL

# Update the configuration in monitor.js
sed -i "s/YOUR_PROGRAM_ID_HERE/$PROGRAM_ID/g" monitor.js

if [ ! -z "$DISCORD_WEBHOOK" ]; then
    sed -i "s|YOUR_DISCORD_WEBHOOK_URL|$DISCORD_WEBHOOK|g" monitor.js
fi

if [ ! -z "$ALERT_EMAIL" ]; then
    sed -i "s/your-email@example.com/$ALERT_EMAIL/g" monitor.js
fi

# Start the monitoring service
echo "Starting monitoring service..."
pm2 start monitor.js --name cobotium-monitor

# Save PM2 configuration
pm2 save

# Set up PM2 to start on boot
echo "Setting up PM2 to start on boot..."
pm2 startup

echo "Monitoring service setup complete!"
echo "You can view logs with: pm2 logs cobotium-monitor"
echo "You can stop the service with: pm2 stop cobotium-monitor"
echo "You can restart the service with: pm2 restart cobotium-monitor"
