{% set BIN = BIN | default(value=NAME | lower) %}
{%- set HOMEPAGE = HOMEPAGE | default(value=SITE ~ "/" ~ REPO) -%}

class {{ NAME }} < Formula
  desc "{{ DESCRIPTION }}"
  homepage "{{ HOMEPAGE }}"
  version "{{ VERSION }}"

  if Hardware::CPU.arm?
    url '{{ REPO_URL }}/releases/download/v{{ VERSION }}/tera-cli-aarch64-apple-darwin.tar.gz'
    sha256 '{{ AARCH64_SHA256 }}'
  else
    url '{{ REPO_URL }}/releases/download/v{{ VERSION }}/tera-cli-x86_64-apple-darwin.tar.gz'
    sha256 '{{ X86_64_SHA256 }}'
  end

  def install
    bin.install "{{ BIN }}"
  end
end
