VERSION := `toml get Cargo.toml package.version | jq -r`
TARGET_DIR := "target/release"

# List available commands
default:
  @just --list --unsorted

# Generate the readme as .md
md:
    #!/usr/bin/env bash
    asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

# Generate usage samples
_usage:
	cargo run -q -- --help > doc/usage.adoc

# Generate documentation
doc:_usage
	cargo doc -p tera-cli --all-features --no-deps


# Run rustfmt
_fmt:
	cargo fmt --all

# Run clippy
_clippy:
	cargo clippy

# Run checks such as clippy, rustfmt, etc...
check: _clippy _fmt

brew:
	#!/usr/bin/env bash
	RUST_LOG=info
	cargo build --release
	TARGET_DIR="target/release"
	tar -czf $TARGET_DIR/tera-macos-$VERSION.tar.gz -C $TARGET_DIR tera
	SHA256=$(shasum -a 256 $TARGET_DIR/tera-macos-$VERSION.tar.gz | awk '{ print $1}' | tee $TARGET_DIR/tera-macos-$VERSION.tar.gz.sha256)
	NAME=Tera
	DESCRIPTION="A command line utility written in Rust to render templates using the tera templating engine"
	SITE=https://github.com
	REPO=chevdor/tera-cli
	tera --template templates/formula.rb --env-only > Formula/tera.rb
