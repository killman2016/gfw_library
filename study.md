Quick setup - if you've done this kind of thing before

or HTTPS https://github.com/killman2016/gfw_library.git
     SSH git@github.com:killman2016/gfw_library.git

…or create a new repository on the command line

echo "# gfw_library" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin git@github.com:killman2016/gfw_library.git
git push -u origin main

…or push an existing repository from the command line

git remote add origin git@github.com:killman2016/gfw_library.git
git branch -M main
git push -u origin main

…or import code from another repository

You can initialize this repository with code from a Subversion, Mercurial, or TFS project.
