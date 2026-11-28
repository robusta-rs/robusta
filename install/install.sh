#!/bin/sh
# Installs the `robusta` executable of the release this script is attached to.
#
#   curl -fsSL https://robusta.rs/install.sh | sh
#
# The executable goes to `$ROBUSTA_INSTALL_DIR`, or `~/.local/bin` by default.

set -eu

# Replaced with the release's download URL when the release is published.
release_url="@RELEASE_URL@"

error() {
  echo "error: $1" >&2
  exit 1
}

target() {
  case "$(uname -m)" in
    x86_64 | amd64) arch=x86_64 ;;
    aarch64 | arm64) arch=aarch64 ;;
    *) error "unsupported architecture: $(uname -m)" ;;
  esac

  case "$(uname -s)" in
    Linux) echo "$arch-unknown-linux-musl" ;;
    Darwin)
      # A shell running under Rosetta reports x86_64 on Apple silicon.
      if [ "$arch" = x86_64 ] && [ "$(sysctl -n hw.optional.arm64 2>/dev/null)" = 1 ]; then
        arch=aarch64
      fi
      echo "$arch-apple-darwin"
      ;;
    MINGW* | MSYS* | CYGWIN*) error "use install.ps1 to install Robusta on Windows" ;;
    *) error "unsupported operating system: $(uname -s)" ;;
  esac
}

sha256() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | cut -d ' ' -f 1
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | cut -d ' ' -f 1
  else
    error "sha256sum or shasum is required to verify the download"
  fi
}

main() {
  command -v curl >/dev/null 2>&1 || error "curl is required to download Robusta"

  asset="robusta-$(target)"
  install_dir="${ROBUSTA_INSTALL_DIR:-$HOME/.local/bin}"

  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT

  echo "Downloading $release_url/$asset"
  curl -fsSL "$release_url/$asset" -o "$tmp/$asset" || error "failed to download $asset"
  curl -fsSL "$release_url/$asset.sha256" -o "$tmp/$asset.sha256" ||
    error "failed to download $asset.sha256"

  expected="$(cut -d ' ' -f 1 <"$tmp/$asset.sha256")"
  actual="$(sha256 "$tmp/$asset")"
  if [ "$expected" != "$actual" ]; then
    error "checksum mismatch for $asset: expected $expected, got $actual"
  fi

  mkdir -p "$install_dir"
  # Moving within the directory replaces a running executable safely.
  cp "$tmp/$asset" "$install_dir/.robusta.tmp"
  chmod +x "$install_dir/.robusta.tmp"
  mv -f "$install_dir/.robusta.tmp" "$install_dir/robusta"
  echo "Installed robusta to $install_dir/robusta"

  case ":$PATH:" in
    *":$install_dir:"*) ;;
    *)
      echo
      echo "$install_dir is not on your PATH. To add it, put this line in your shell profile:"
      echo
      echo "  export PATH=\"$install_dir:\$PATH\""
      ;;
  esac
}

main
