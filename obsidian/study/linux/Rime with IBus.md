https://github.com/rime/home/wiki/RimeWithIBus

```
sudo apt-get install ibus-rime
```

[InputMethodBuster](https://wiki.debian.org/InputMethodBuster)

---

User Guide for the Input Method on Debian Buster.

Since current Debian default Desktop install uses Gnome wayland, this page focus on Gnome wayland.

(Please limit content to the basics. If you have detailed and specific topics, please create other pages and link them from this page.)

## Input Method Frameworks

There are 4 popular input method (IM) frameworks;

|   |   |
|---|---|
|Input method framework|Note|
|[ibus](https://packages.debian.org/ibus "DebianPkg")|Default for Gnome (recommended)|
|[uim](https://packages.debian.org/uim "DebianPkg")|Popular with Japanese (emacs friendly)|
|[fcitx](https://packages.debian.org/fcitx "DebianPkg")|Popular with Chinese|
|[scim](https://packages.debian.org/scim "DebianPkg")|Preferred by KDE(?)|

The im-config package helps to install and to manage all these IMs on one system.

It is usually simpler to use only one IM on a system.

### Input Method Engines for ibus

For ibus, the installation of their associated Input Method Engine (IME) support packages pulls in all the common IME packages.

|   |   |   |
|---|---|---|
|Language|IME support package|Note|
|Chinese (cn)|[ibus-pinyin](https://packages.debian.org/ibus-pinyin "DebianPkg")|Chinese Pinyin input (cn, deprecated)|
|Chinese (cn)|[ibus-libpinyin](https://packages.debian.org/ibus-libpinyin "DebianPkg")|Chinese Pinyin input (cn)|