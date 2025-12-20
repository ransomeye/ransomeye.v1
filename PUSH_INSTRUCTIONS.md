# Push Instructions for GitHub

Your changes have been committed locally. To push to GitHub, you need to authenticate.

## Option 1: Use Personal Access Token (Recommended)

1. **Generate a Personal Access Token**:
   - Go to: https://github.com/settings/tokens
   - Click "Generate new token" → "Generate new token (classic)"
   - Name: "RansomEye Sync"
   - Select scopes: `repo` (full control of private repositories)
   - Click "Generate token"
   - **Copy the token immediately** (you won't see it again)

2. **Push using the token**:
   ```bash
   cd /home/ransomeye/rebuild
   git push origin main
   ```
   - Username: `gagan@ransomeye.tech`
   - Password: **Paste your Personal Access Token** (not your GitHub password)

## Option 2: Configure Git Credential Helper

```bash
# Store credentials for future use
git config --global credential.helper store

# Then push (will prompt once, then remember)
git push origin main
```

## Option 3: Use SSH (Most Secure)

1. **Generate SSH key** (if you don't have one):
   ```bash
   ssh-keygen -t ed25519 -C "gagan@ransomeye.tech"
   # Press Enter to accept default location
   # Optionally set a passphrase
   ```

2. **Add SSH key to GitHub**:
   ```bash
   cat ~/.ssh/id_ed25519.pub
   # Copy the output
   ```
   - Go to: https://github.com/settings/keys
   - Click "New SSH key"
   - Paste the key and save

3. **Update remote to use SSH**:
   ```bash
   cd /home/ransomeye/rebuild
   git remote set-url origin git@github.com:ransomeye/ransomeye.v1.git
   git push origin main
   ```

## Verify Push

After pushing, verify on GitHub:
```bash
# Check remote status
git fetch origin
git status

# View on GitHub
echo "Repository: https://github.com/ransomeye/ransomeye.v1"
```

## Adding Trained Datasets

If you have trained datasets or models to add:

1. **Place them in the appropriate directory**:
   ```bash
   # For models
   mkdir -p models/
   # Copy your .pkl, .gguf, etc. files here
   
   # For datasets
   mkdir -p datasets/
   # Copy your .csv, .parquet, etc. files here
   ```

2. **Add and commit**:
   ```bash
   git add models/ datasets/
   git commit -m "Add trained models and datasets"
   git push origin main
   ```

3. **Git LFS will automatically handle large files** (configured in `.gitattributes`)

## Current Status

- ✅ Repository initialized
- ✅ Git LFS configured
- ✅ All code files committed
- ✅ Remote configured: https://github.com/ransomeye/ransomeye.v1.git
- ⏳ Waiting for push (requires authentication)

---

**© RansomEye.Tech | Support: Gagan@RansomEye.Tech**

