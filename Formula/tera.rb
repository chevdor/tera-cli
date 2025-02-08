class Tera < Formula
  desc 'A command line utility written in Rust to render templates using the Tera templating engine'
  homepage 'https://github.com/chevdor/tera-cli'
  version '0.5.0'

  if Hardware::CPU.arm?
    url 'https://github.com/chevdor/tera-cli/releases/download/v0.5.0/tera-cli-aarch64-apple-darwin.tar.gz'
    sha256 'cde318afb9b41c7e4a0cfa7041b3355b49f33c4ebbe96e966017dc6eb93afaf4'
  else
    url 'https://github.com/chevdor/tera-cli/releases/download/v0.5.0/tera-cli-x86_64-apple-darwin.tar.gz'
    sha256 '5da9dae6b48ed395804fb804a7a7f2901a6e10d620d40bf7ae67ed4abadbcb29'
  end

  def install
    bin.install 'tera'
  end
end
