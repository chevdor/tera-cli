# Description

You can find the changelogs below.

# Downloads

Download the binary for your OS from below:
- **Linux**
    - [Debian package]({{ DEBIAN_URL }})
- **MacOS**
    - [Archive]({{ MACOS_TGZ_URL }})
# Install

## From source

```
cargo install --git https://github.com/chevdor/tera-cli
```

## Linux
```
wget {{ DEBIAN_URL }}
dpkg -i tera-cli_linux_amd64.deb
tera --help
```

## MacOS

```
brew tap chevdor/tera-cli https://github.com/chevdor/tera-cli
brew update
brew install chevdor/tera-cli/tera
```

{{ CHANGELOG }}
