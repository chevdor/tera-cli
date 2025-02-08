class Tera < Formula
  arch arm: "aarch64", intel: 'x86_64'

  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v0.5.0/tera-cli-#{arch}-apple-darwin.tar.gz"
  sha256 arm:   "cde318afb9b41c7e4a0cfa7041b3355b49f33c4ebbe96e966017dc6eb93afaf4",
         intel: "5da9dae6b48ed395804fb804a7a7f2901a6e10d620d40bf7ae67ed4abadbcb29"

  version "0.5.0"

  def install
    bin.install "tera"
  end
end
