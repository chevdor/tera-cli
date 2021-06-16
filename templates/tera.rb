{% set bin = bin | default(value=name | lower) %}
{%- set homepage = homepage | default(value=site ~ "/" ~ repo) -%}

class {{ name }} < Formula
  desc "{{ description }}"
  homepage "{{ homepage}}"
  url "{{ site }}/{{ repo }}/releases/download/v{{ version}}/{{ archive | default(value= bin ~"-mac-v" ~ version) }}.tar.gz "
  sha256 "{{ sha256 }}"
  version "{{ version }}"

  def install
    bin.install "{{ bin }}"
  end
end
