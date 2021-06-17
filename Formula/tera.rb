class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v1.2.3/tera-macos-v1.2.3.tar.gz"
  sha256 "8839a924582932905213185610e5a3d2086e5461d98e218b58fcf973abea10ed"
  version "1.2.3"

  def install
    bin.install "tera"
  end
end
