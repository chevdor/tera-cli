{% set BIN = BIN | default(value=NAME | lower) %}
{%- set HOMEPAGE = HOMEPAGE | default(value=SITE ~ "/" ~ REPO) -%}

class {{ NAME }} < Formula
  arch arm: "aarch64", intel: 'x86_64'

  desc "{{ DESCRIPTION }}"
  homepage "{{ HOMEPAGE }}"
  url "{{ REPO_URL }}/releases/download/v{{ VERSION }}/tera-cli-#{arch}-apple-darwin.tar.gz"
  sha256 arm:   "{{ AARCH64_SHA256 }}",
         intel: "{{ X86_64_SHA256 }}"

  version "{{ VERSION }}"

  def install
    bin.install "{{ BIN }}"
  end
end