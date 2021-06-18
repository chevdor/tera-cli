class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v0.1.3/tera-macos-v0.1.3.tar.gz"
  sha256 "2b9f7a2d4026545ecfb6f4e88a7b7ae13c7899ae82ec11dca7edd6e02dfc110e"
  version "0.1.3"

  def install
    bin.install "tera"
  end
end
