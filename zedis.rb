class Zedis < Formula
  desc "A tiny key value datastore"
  homepage "https://github.com/drbh/zedis"
  url "https://github.com/drbh/zedis/releases/download/v0.0001/zedis"
  sha256 "e66cdf097e9b902ddbd2bd5f1f584dd860c51c8ca6d3f9bb539deeb4633c22a4"
  version "0.0.1"

  bottle :unneeded

  def install
    bin.install "zedis"
  end
end