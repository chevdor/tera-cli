# Description

You can find the changelogs below.

# Downloads

Download the binary for your OS from below:
- **Linux**
    - [x86_64 gnu]({{ X86_64_GNU_TGZ_URL }})
    - [x86_64 musl]({{ X86_64_MUSL_TGZ_URL }})
    - [aarch64 gnu]({{ AARCH64_GNU_TGZ_URL }})
    - [aarch64 musl]({{ AARCH64_MUSL_TGZ_URL }})
    - [Debian package]({{ DEBIAN_URL }})
- **MacOS**
    - [Archive]({{ MACOS_TGZ_URL }})
- **Windows**
    - [Archive]({{ WINDOWS_ZIP_URL }})

# Install

## From source

```
cargo install --git https://github.com/chevdor/tera-cli
```

## Linux

### Binaries

```
wget {{ X86_64_GNU_TGZ_URL }}
tar xf tera-cli-x86_64-unknown-linux-gnu.tar.gz
./tera --help
```

### Debian

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
