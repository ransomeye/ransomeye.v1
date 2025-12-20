# GitHub Repository Setup Guide

This guide will help you sync your RansomEye project to GitHub, including all trained models and datasets.

## Prerequisites

1. **GitHub Account**: You need a GitHub account. If you don't have one, create it at https://github.com/signup

2. **Git LFS**: The sync script will install Git LFS automatically if needed, or you can install it manually:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install git-lfs
   
   # Or visit: https://git-lfs.github.com/
   ```

3. **GitHub Authentication**: Choose one of these methods:

   **Option A: GitHub CLI (Recommended)**
   ```bash
   # Install GitHub CLI
   sudo apt-get install gh
   
   # Authenticate
   gh auth login
   ```

   **Option B: Personal Access Token**
   - Go to https://github.com/settings/tokens
   - Generate a new token with `repo` scope
   - Save the token securely

## Step-by-Step Setup

### Step 1: Configure Git User

```bash
cd /home/ransomeye/rebuild

# Set your Git identity (replace with your details)
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Or set globally for all repositories
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

### Step 2: Run the Sync Script

```bash
cd /home/ransomeye/rebuild
./sync_to_github.sh
```

The script will:
- ✅ Install Git LFS if needed
- ✅ Configure Git LFS for large files (models, datasets)
- ✅ Stage all your files
- ✅ Create initial commit
- ✅ Help you create GitHub repository
- ✅ Push everything to GitHub

### Step 3: Manual GitHub Repository Creation (if not using GitHub CLI)

If you're not using GitHub CLI, follow these steps:

1. **Create Repository on GitHub**:
   - Go to https://github.com/new
   - Repository name: `ransomeye-rebuild` (or your preferred name)
   - Description: "RansomEye Enterprise Security Platform"
   - Choose Public or Private
   - **DO NOT** initialize with README, .gitignore, or license
   - Click "Create repository"

2. **Get Repository URL**:
   - Copy the repository URL (HTTPS or SSH)
   - Example: `https://github.com/yourusername/ransomeye-rebuild.git`

3. **Run Sync Script**:
   - The script will prompt you for the repository URL
   - Enter the URL when prompted
   - The script will push all files

### Step 4: Verify Upload

After pushing, verify on GitHub:
- Check that all files are present
- Verify large files are tracked via Git LFS (they'll show as "Stored with Git LFS")
- Check that trained models and datasets are included

## Troubleshooting

### Git LFS Issues

If you see errors about Git LFS:
```bash
# Reinstall Git LFS
git lfs install

# Verify Git LFS is working
git lfs version
```

### Authentication Issues

**For HTTPS:**
- Use Personal Access Token as password
- Or use GitHub CLI: `gh auth login`

**For SSH:**
- Generate SSH key: `ssh-keygen -t ed25519 -C "your.email@example.com"`
- Add to GitHub: https://github.com/settings/keys
- Test: `ssh -T git@github.com`

### Large File Issues

If files are too large even for Git LFS:
- GitHub free accounts: 1GB repo limit, 100MB file limit
- Consider GitHub Large File Storage (LFS) or alternative storage
- Contact GitHub support for enterprise limits

### Push Errors

If push fails:
```bash
# Check remote URL
git remote -v

# Update remote if needed
git remote set-url origin https://github.com/yourusername/ransomeye-rebuild.git

# Try pushing again
git push -u origin main
```

## File Size Considerations

- **Git LFS** is configured for:
  - Model files: `.pkl`, `.gguf`, `.h5`, `.pb`, `.onnx`, `.pt`, `.pth`, `.ckpt`
  - Large datasets: `.csv.gz`, `.parquet`, `.feather`
  
- **GitHub Limits**:
  - Free: 1GB repo, 100MB per file
  - Pro: 2GB repo, 100MB per file
  - Enterprise: Custom limits

- **If your repository exceeds limits**:
  - Consider using GitHub Releases for large model files
  - Use external storage (S3, Azure Blob) for very large datasets
  - Split large files into smaller chunks

## Next Steps

After successful sync:

1. **Set up GitHub Actions** (optional):
   - Create `.github/workflows/` directory
   - Add CI/CD pipelines

2. **Configure Branch Protection**:
   - Go to repository Settings → Branches
   - Protect `main` branch

3. **Add Collaborators**:
   - Settings → Collaborators
   - Invite team members

4. **Set up Webhooks** (optional):
   - For integration with other systems

## Support

If you encounter issues:
- Check GitHub status: https://www.githubstatus.com/
- Review Git LFS documentation: https://git-lfs.github.com/
- Contact: Gagan@RansomEye.Tech

---

**© RansomEye.Tech | Support: Gagan@RansomEye.Tech**

