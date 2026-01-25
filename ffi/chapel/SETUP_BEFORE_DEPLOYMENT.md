# Pre-Deployment Checklist & Repository Setup

This guide helps you prepare for Chapel deployment.

---

## Step 1: Verify Prerequisites (1 minute)

```powershell
# Check Git
git --version

# Check source files
Get-ChildItem D:\nuclear-chapel-training -Filter "*.chpl" -Recurse | Measure-Object
# Should show Count: 12 files

# Check Git configuration
git config --global user.name
git config --global user.email
# If empty, run:
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
```

---

## Step 2: Create GitHub Repository (2 minutes)

1. Go to https://github.com/new
2. Fill in:
   - Repository name: `nuclear-chapel-training`
   - Description: `Chapel language training code for nuclear simulation`
   - Visibility: Public (or Private if you prefer)
3. Click "Create repository"
4. Copy the HTTPS URL (should be: `https://github.com/Kimberlyindiva/nuclear-chapel-training.git`)

---

## Step 3: Create Hugging Face Model Repository (2 minutes)

1. Go to https://huggingface.co/new
2. Fill in:
   - Model name: `nuclear-chapel-training`
   - License: MIT (MIT License)
   - Visibility: Public (or Private)
3. Click "Create Model"
4. Copy the repository URL (should be: `https://huggingface.co/Kimberlyindiva/nuclear-chapel-training`)

---

## Step 4: Configure GitHub Authentication (2 minutes)

### Option A: Personal Access Token (Recommended)

```powershell
# 1. Create token: https://github.com/settings/tokens
#    - Select: repo (full control)
#    - Expiration: 90 days or more
# 2. Store securely:
git config --global credential.helper store
# 3. Test:
git ls-remote https://github.com/Kimberlyindiva/nuclear-chapel-training.git
# Enter token when prompted - it will be saved
```

### Option B: SSH Key

```powershell
# 1. Generate key (if needed)
ssh-keygen -t ed25519 -C "your-email@example.com"

# 2. Add to GitHub: https://github.com/settings/keys
# cat ~/.ssh/id_ed25519.pub (copy to GitHub)

# 3. Configure Git for SSH
git config --global url."git@github.com:".insteadOf "https://github.com/"

# 4. Test
git ls-remote git@github.com:Kimberlyindiva/nuclear-chapel-training.git
```

---

## Step 5: Configure Hugging Face Authentication (2 minutes)

```powershell
# Option A: Store credentials in Git
# 1. Create token: https://huggingface.co/settings/tokens
# 2. Configure Git helper
git config --global credential.helper store

# Option B: Use Hugging Face CLI
# (Optional - advanced users)
```

---

## Step 6: Run Deployment (5 minutes)

Once repositories are created and authentication configured:

```powershell
cd D:\nuclear-chapel-training

# Automatic (will ask for confirmation)
.\deploy-launch.ps1

# Or direct deployment
.\deploy-chapel-simple.ps1 -Target all
```

---

## Troubleshooting

### "Repository not found" Error

**Cause**: Repository doesn't exist or credentials wrong

**Fix**:
```powershell
# 1. Verify repository exists on GitHub
# 2. Check credentials
git ls-remote https://github.com/Kimberlyindiva/nuclear-chapel-training.git

# 3. Update credentials with:
git credential approve
# Enter: protocol=https
#        host=github.com
#        username=Kimberlyindiva
#        password=<your-token>
```

### "Permission denied" Error

**Cause**: Authentication failed

**Fix**:
```powershell
# Clear stored credentials and re-enter
git credential reject
protocol=https
host=github.com
hostname=github.com

# Try clone again - will prompt for credentials
git clone https://github.com/Kimberlyindiva/nuclear-chapel-training.git
```

### "fatal: could not create work tree dir" Error

**Cause**: Temporary directory permissions

**Fix**:
```powershell
# Change temp directory
$env:TEMP = "D:\Temp"
mkdir D:\Temp -ErrorAction SilentlyContinue

# Re-run deployment
.\deploy-chapel-simple.ps1 -Target all
```

---

## Deployment Checklist

- [ ] Git installed and configured (`git config --global user.name` shows name)
- [ ] Chapel source files present (12 .chpl files in D:\nuclear-chapel-training)
- [ ] GitHub repository created (https://github.com/Kimberlyindiva/nuclear-chapel-training)
- [ ] Hugging Face repository created (https://huggingface.co/Kimberlyindiva/nuclear-chapel-training)
- [ ] Personal Access Token created and stored
- [ ] Credentials tested (`git ls-remote https://github.com/Kimberlyindiva/nuclear-chapel-training.git` works)
- [ ] Ready to deploy

---

## Next: Run Deployment

```powershell
cd D:\nuclear-chapel-training
.\deploy-chapel-simple.ps1 -Target all
```

This will:
1. Clone both GitHub and HF repositories
2. Copy all 12 Chapel source files
3. Add documentation
4. Commit changes
5. Push to GitHub and Hugging Face
6. Display verification URLs

**Estimated time**: 5 minutes (mostly downloading repositories)

---

**Questions?** Check the deployment logs or individual script files for details.
