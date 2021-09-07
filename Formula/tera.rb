class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v0.2.0/tera-macos-v0.2.0.tar.gz"
  sha256 "aa2535d54996dcc766aeea749dcad55c3e40cdfd16da66c6db7f720ec34ad6ff"
  version "0.2.0"

  def install
    bin.install "tera"
  end
end
