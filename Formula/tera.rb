class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v0.2.2/tera-macos-v0.2.2.tar.gz"
  sha256 "e4e7ce044d04d9fbc7a9a3559b1e833555709ed75be5ebf6ef9f4ab0f5458b4e"
  version "0.2.2"

  def install
    bin.install "tera"
  end
end
