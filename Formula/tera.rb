class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  version "0.5.1"

  if Hardware::CPU.arm?
    url 'https://github.com/chevdor/tera-cli/releases/download/v0.5.1/tera-cli-aarch64-apple-darwin.tar.gz'
    sha256 '0731db549b9a2f982d105812e7f81b36aaa1799494b1208419c650be3527f950'
  else
    url 'https://github.com/chevdor/tera-cli/releases/download/v0.5.1/tera-cli-x86_64-apple-darwin.tar.gz'
    sha256 '13e9bbc28308217af1b430c1222563405057f9e2db1d1efe4c75a1d52c84df82'
  end

  def install
    bin.install "tera"
  end
end

