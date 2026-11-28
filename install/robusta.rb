# Installs the `robusta` executable of one release. The release fills in its
# download URL, version and checksums, and pushes the result to the
# robusta-rs/homebrew-java tap.
class Robusta < Formula
  desc "Fast, Rust-native Java formatter and linter"
  homepage "https://robusta.rs"
  version "@VERSION@"
  license "MIT"

  on_macos do
    on_arm do
      url "@RELEASE_URL@/robusta-aarch64-apple-darwin"
      sha256 "@SHA256_aarch64-apple-darwin@"
    end
    on_intel do
      url "@RELEASE_URL@/robusta-x86_64-apple-darwin"
      sha256 "@SHA256_x86_64-apple-darwin@"
    end
  end

  on_linux do
    on_arm do
      url "@RELEASE_URL@/robusta-aarch64-unknown-linux-musl"
      sha256 "@SHA256_aarch64-unknown-linux-musl@"
    end
    on_intel do
      url "@RELEASE_URL@/robusta-x86_64-unknown-linux-musl"
      sha256 "@SHA256_x86_64-unknown-linux-musl@"
    end
  end

  def install
    bin.install Dir["robusta-*"].first => "robusta"
  end

  test do
    assert_equal "Version: #{version}", shell_output("#{bin}/robusta --version").strip
  end
end
