```bash

adduser username

usermod -aG sudo username

# /etc/sudoer

username ALL=(ALL) NOPASSWD:ALL

# openvpn installation
# https://github.com/angristan/openvpn-install
# First, get the script and make it executable:

curl -O https://raw.githubusercontent.com/angristan/openvpn-install/master/openvpn-install.sh
chmod +x openvpn-install.sh

# Then run it as root:

sudo ./openvpn-install.sh

# Install local .deb package
# So if you have a .deb file, you can install it by:
# 
# Using:

sudo dpkg -i /path/to/deb/file
sudo apt-get install -f

# Using:

sudo apt install ./name.deb

# Or

sudo apt install /path/to/package/name.deb

# With old apt-get versions you must first move your deb file to /var/cache/apt/archives/ directory. For both, after executing this command, it will automatically download its dependencies.

# First installing gdebi and then opening your .deb file using it (Right-click -> Open with). It will install your .deb package with all its dependencies.

```

