class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v0.2.1/tera-macos-v0.2.1.tar.gz"
  sha256 "80a10353b9775803ad738e012b5789a07b083d7545d7acf89a3c1e5ddb2e0217"
  version "0.2.1"

  def install
    bin.install "tera"
  end
end
