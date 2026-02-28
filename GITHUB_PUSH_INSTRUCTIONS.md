# QuickNote — GitHub Push Instructions

**Important**: Your GitHub token is not currently accessible from this environment. Here's how to push the code manually:

---

## Option 1: Use GitHub CLI (Recommended)

```bash
# Install GitHub CLI if needed
brew install gh  # macOS
# or download from https://cli.github.com/

# Authenticate with GitHub
gh auth login

# Clone/create repository and push
cd /Users/nitin/.openclaw/workspace/projects/QuickNote

# Create new repo on GitHub (you'll be prompted)
gh repo create QuickNote --public --description "Portable Knowledge Pocket" --source=. --push

# Or if you already created a repo:
git remote add origin https://github.com/YOUR_USERNAME/QuickNote.git
git push -u origin main
```

---

## Option 2: Use Personal Access Token

1. **Generate a token** at GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Select scopes: `repo` (full control of private repositories)
   - Generate token and copy it

2. **Set up the repository**:
```bash
cd /Users/nitin/.openclaw/workspace/projects/QuickNote

# Create new repo on GitHub first via web UI or API

# Add remote with your token (replace YOUR_TOKEN and YOUR_USERNAME)
git remote add origin https://YOUR_TOKEN@github.com/YOUR_USERNAME/QuickNote.git

# Push to main branch
git push -u origin main
```

---

## Option 3: SSH Keys

If you prefer SSH authentication:

1. **Generate SSH key** (if you don't have one):
```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519
cat ~/.ssh/id_ed25519.pub  # Copy this to GitHub
```

2. **Add SSH key** at GitHub → Settings → SSH and GPG keys → New SSH key

3. **Set up repository**:
```bash
cd /Users/nitin/.openclaw/workspace/projects/QuickNote

git remote add origin git@github.com:YOUR_USERNAME/QuickNote.git
git push -u origin main
```

---

## Repository Name & URL

Once pushed, your repository will be at one of these URLs:

- **HTTPS**: `https://github.com/YOUR_USERNAME/QuickNote`
- **SSH**: `git@github.com:YOUR_USERNAME/QuickNote.git`

---

## After Pushing: Update Documentation

Don't forget to update the README with the actual repository URL!

Example change in `/Users/nitin/.openclaw/workspace/projects/QuickNote/README.md`:

```markdown
## License

MIT License — Free for personal and commercial use.

**Source Code**: [github.com/YOUR_USERNAME/QuickNote](https://github.com/YOUR_USERNAME/QuickNote)

---
```

---

## Troubleshooting

### "Permission denied (publickey)" 
- You're using SSH but haven't added your key to GitHub
- **Fix**: Add SSH key at GitHub settings as described above

### "Authentication failed" with HTTPS token
- Token is expired or has insufficient permissions
- **Fix**: Generate a new token with `repo` scope

### Git remote already exists
```bash
git remote remove origin  # Remove existing remote
git remote add origin https://github.com/YOUR_USERNAME/QuickNote.git  # Add new one
git push -u origin main
```

---

## Quick Commands Summary

```bash
# 1. Check current status
cd /Users/nitin/.openclaw/workspace/projects/QuickNote
git status

# 2. Commit any uncommitted changes (if needed)
git add .
git commit -m "Finalize project before pushing"

# 3. Set up remote (choose one method above)
git remote add origin https://YOUR_TOKEN@github.com/YOUR_USERNAME/QuickNote.git

# 4. Push to GitHub
git push -u origin main

# 5. Verify on GitHub
open https://github.com/YOUR_USERNAME/QuickNote
```

---

**Need help?** Just let me know which method you'd like to use, and I can guide you through the steps! 🚀
