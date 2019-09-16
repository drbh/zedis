class Zedis < Formula
  desc "A tiny key value datastore"
  homepage "https://github.com/drbh/zedis"
  url "https://github.com/drbh/zedis/archive/v0.0001.tar.gz"
  sha256 "2f65b56d61dda3408502084e9e1692c26f7dff09fe3a6fed80606a21aeeb5c51"
  version "0.0.1"

  bottle :unneeded

  def install
    bin.install "zedis"
  end
end