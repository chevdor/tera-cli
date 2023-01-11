class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v0.2.3/tera-macos-v0.2.3.tar.gz"
  sha256 "a45ee695f5c23472e48c4ded30e7d3c8c54f4e0203cef9908565bb44c792435a"
  version "0.2.3"

  def install
    bin.install "tera"
  end
end
