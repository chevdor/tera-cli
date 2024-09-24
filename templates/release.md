# Description

You can find the changelogs below.

# Downloads

Download the binary for your OS from below:
- **Linux**
    - [x86_64 gnu]({{ ARCHIVE_URL_PREFIX ~ X86_64_GNU_TGZ }})
    - [x86_64 musl]({{ ARCHIVE_URL_PREFIX ~ X86_64_MUSL_TGZ }})
    - [aarch64 gnu]({{ ARCHIVE_URL_PREFIX ~ AARCH64_GNU_TGZ }})
    - [aarch64 musl]({{ ARCHIVE_URL_PREFIX ~ AARCH64_MUSL_TGZ }})
    - [Debian package]({{ DEBIAN_URL }})
- **MacOS**
    - [x86_64]({{ ARCHIVE_URL_PREFIX ~ X86_64_MACOS_TGZ }})
    - [aarch64]({{ ARCHIVE_URL_PREFIX ~ AARCH64_MACOS_TGZ }})
- **Windows**
    - [Archive]({{ ARCHIVE_URL_PREFIX ~ WINDOWS_ZIP }})

# Install

## From source

```
cargo install --git https://github.com/chevdor/tera-cli
```

## Linux

### Binaries

```
wget {{ ARCHIVE_URL_PREFIX ~ X86_64_GNU_TGZ }}
tar xf {{ X86_64_GNU_TGZ }}
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
