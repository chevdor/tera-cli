class Tera < Formula
  desc "A command line utility written in Rust to render templates using the tera templating engine"
  homepage "https://github.com/chevdor/tera-cli"
  url "https://github.com/chevdor/tera-cli/releases/download/v1.2.3/{{ name }}-mac-v1.2.3.tar.gz.tar.gz "
  sha256 "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b"
  version "1.2.3"

  def install
    bin.install "tera"
  end
end
