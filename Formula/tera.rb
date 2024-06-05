class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v0.4.0/tera-macos-v0.4.0.tar.gz"
  sha256 "932c80baaf11b37ebe25a86b80d6ccd4c7f4cd308d188f5e2a29582959e99dc3"
  version "0.4.0"

  def install
    bin.install "tera"
  end
end
