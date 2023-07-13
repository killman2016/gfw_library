## Quick setup - if you've done this kind of thing before

CLI SSH agent using git:

```bash
## or HTTPS https://github.com/killman2016/gfw_library.git
##      SSH git@github.com:killman2016/gfw_library.git

### …or create a new repository on the command line

# ssh agent ... command line ...
eval `ssh-agent -s`
ssh-add
# test ...
ssh -T git@github.com

# below for git operation
echo "# gfw_library" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin git@github.com:killman2016/gfw_library.git
git push -u origin main

# update again
git add .
git commit -m "new file added"
git push origin
git push -u origin main
```

### …or push an existing repository from the command line

```bash
git remote add origin git@github.com:killman2016/gfw_library.git
git branch -M main
git push -u origin main
```
### …or import code from another repository

You can initialize this repository with code from a Subversion, Mercurial, or TFS project.
