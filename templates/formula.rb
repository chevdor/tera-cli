{% set BIN = BIN | default(value=NAME | lower) %}
{%- set HOMEPAGE = HOMEPAGE | default(value=SITE ~ "/" ~ REPO) -%}

class {{ NAME }} < Formula
  desc "{{ DESCRIPTION }}"
  homepage "{{ HOMEPAGE }}"
  url "{{ REPO_URL }}/releases/download/v{{ VERSION }}/{{ ARCHIVE }}"
  sha256 "{{ SHA256 }}"
  version "{{ VERSION }}"

  def install
    bin.install "{{ BIN }}"
  end
end