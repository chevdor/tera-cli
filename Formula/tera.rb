class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v0.2.4/tera-macos-v0.2.4.tar.gz"
  sha256 "1ef7cacb6d88000695eac33be94f90d68aaab4bbb4c7c04388750e6c1d9b9970"
  version "0.2.4"

  def install
    bin.install "tera"
  end
end
