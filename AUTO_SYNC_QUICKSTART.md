# GitHub Auto-Sync Quick Start Guide

## 🚀 Quick Setup (3 Steps)

### Step 1: Configure Git Credentials
```bash
cd /home/ransomeye/rebuild
./setup_git_credentials.sh
```
Choose your preferred authentication method (SSH recommended).

### Step 2: Enable Auto-Sync
```bash
sudo ./setup_auto_sync.sh
```

### Step 3: Verify It's Working
```bash
# Check timer is active
sudo systemctl status ransomeye-github-sync.timer

# View recent sync logs
tail -20 logs/github_sync.log
```

## ✅ That's It!

Your repository will now automatically sync to GitHub every hour. The sync will:
- ✅ Detect any uncommitted changes
- ✅ Commit them automatically
- ✅ Push to GitHub
- ✅ Log everything to `logs/github_sync.log`

## 📊 Monitoring

### Check Sync Status
```bash
# Timer status
sudo systemctl status ransomeye-github-sync.timer

# Service logs (systemd)
sudo journalctl -u ransomeye-github-sync.service -n 50

# Log file
tail -f logs/github_sync.log
```

### Manual Operations
```bash
# Trigger sync immediately
sudo systemctl start ransomeye-github-sync.service

# Or run script directly
./github_auto_sync.sh
```

## ⚙️ Configuration

### Change Sync Frequency

Edit the timer file:
```bash
sudo nano /etc/systemd/system/ransomeye-github-sync.timer
```

Change the `OnCalendar` line:
- Every hour: `OnCalendar=hourly`
- Every 30 minutes: `OnCalendar=*:0/30`
- Every 15 minutes: `OnCalendar=*:0/15`
- Daily at 2 AM: `OnCalendar=02:00`

Then reload:
```bash
sudo systemctl daemon-reload
sudo systemctl restart ransomeye-github-sync.timer
```

### Disable/Enable Auto-Sync
```bash
# Disable
sudo systemctl stop ransomeye-github-sync.timer
sudo systemctl disable ransomeye-github-sync.timer

# Enable
sudo systemctl enable ransomeye-github-sync.timer
sudo systemctl start ransomeye-github-sync.timer
```

## 🔍 Troubleshooting

### Sync Not Working?

1. **Check authentication**:
   ```bash
   git push origin main
   ```
   If this fails, re-run `./setup_git_credentials.sh`

2. **Check logs**:
   ```bash
   tail -50 logs/github_sync.log
   sudo journalctl -u ransomeye-github-sync.service -n 50
   ```

3. **Check timer**:
   ```bash
   sudo systemctl status ransomeye-github-sync.timer
   ```

4. **Test manually**:
   ```bash
   ./github_auto_sync.sh
   ```

### Authentication Issues

If you see authentication errors:
```bash
# Re-setup credentials
./setup_git_credentials.sh

# Or test SSH
ssh -T git@github.com
```

### Lock File Issues

If sync is stuck (lock file exists):
```bash
# Remove lock file (if sync is truly not running)
rm /tmp/ransomeye_github_sync.lock
```

## 📝 What Gets Synced?

- ✅ All code files
- ✅ Configuration files
- ✅ Documentation
- ✅ Trained models (via Git LFS)
- ✅ Datasets (via Git LFS)
- ✅ Any changes in the repository

## 🔒 Security Notes

- Credentials are stored securely (SSH keys or encrypted credential store)
- Lock files prevent concurrent syncs
- All operations are logged
- Only pushes if there are actual changes

---

**© RansomEye.Tech | Support: Gagan@RansomEye.Tech**

