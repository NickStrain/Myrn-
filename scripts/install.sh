#!/bin/sh
# Install myrn on Linux or macOS:
#
#   curl -fsSL https://github.com/NickStrain/Myrn-/releases/latest/download/install.sh | sh
#
# Downloads the prebuilt binary for this machine from the GitHub release,
# checks it against the release's sha256sums.txt, and puts it in ~/.local/bin.
#
# Environment:
#   MYRN_VERSION        a version such as 0.1.0 (default: the latest release)
#   MYRN_INSTALL_DIR    where to put the binary (default: ~/.local/bin)
#   MYRN_DOWNLOAD_BASE  download from here instead of the GitHub release
#   MYRN_TARGET         override the detected target (e.g. x86_64-unknown-linux-musl)
set -eu

REPO="NickStrain/Myrn-"
INSTALL_DIR="${MYRN_INSTALL_DIR:-$HOME/.local/bin}"

say() { printf 'myrn: %s\n' "$*" >&2; }
die() { say "$*"; exit 1; }

detect_target() {
    os=$(uname -s)
    arch=$(uname -m)
    case "$arch" in
        x86_64 | amd64) arch=x86_64 ;;
        aarch64 | arm64) arch=aarch64 ;;
        *) die "no prebuilt binary for CPU '$arch'; build from source: cargo install --git https://github.com/$REPO myrn" ;;
    esac
    case "$os" in
        Linux) echo "$arch-unknown-linux-musl" ;;
        Darwin)
            # A shell running under Rosetta reports x86_64 on Apple silicon.
            if [ "$arch" = x86_64 ] && [ "$(sysctl -n sysctl.proc_translated 2>/dev/null || echo 0)" = 1 ]; then
                arch=aarch64
            fi
            echo "$arch-apple-darwin"
            ;;
        *) die "no prebuilt binary for '$os'; on Windows use install.ps1" ;;
    esac
}

fetch() { # url dest
    if command -v curl > /dev/null 2>&1; then
        curl -fsSL --retry 3 -o "$2" "$1"
    elif command -v wget > /dev/null 2>&1; then
        wget -q -O "$2" "$1"
    else
        die "need curl or wget"
    fi
}

sha256() {
    if command -v sha256sum > /dev/null 2>&1; then
        sha256sum "$1" | cut -d' ' -f1
    elif command -v shasum > /dev/null 2>&1; then
        shasum -a 256 "$1" | cut -d' ' -f1
    else
        die "need sha256sum or shasum to check the download"
    fi
}

target="${MYRN_TARGET:-$(detect_target)}"
archive="myrn-$target.tar.gz"
if [ -n "${MYRN_DOWNLOAD_BASE:-}" ]; then
    base="${MYRN_DOWNLOAD_BASE%/}"
elif [ -n "${MYRN_VERSION:-}" ]; then
    base="https://github.com/$REPO/releases/download/v${MYRN_VERSION#v}"
else
    base="https://github.com/$REPO/releases/latest/download"
fi

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT INT TERM

say "downloading $archive"
fetch "$base/$archive" "$tmp/$archive"
fetch "$base/sha256sums.txt" "$tmp/sha256sums.txt"

want=$(awk -v f="$archive" '{ sub(/^\*/, "", $2) } $2 == f { print $1 }' "$tmp/sha256sums.txt")
[ -n "$want" ] || die "$archive is not listed in sha256sums.txt"
got=$(sha256 "$tmp/$archive")
[ "$got" = "$want" ] || die "checksum mismatch for $archive (got $got, expected $want)"

tar -xzf "$tmp/$archive" -C "$tmp"
[ -f "$tmp/myrn" ] || die "$archive does not contain a myrn binary"
mkdir -p "$INSTALL_DIR"
# Replace atomically, so a running myrn (a hook, the MCP server) is not cut off.
cp "$tmp/myrn" "$INSTALL_DIR/.myrn.new"
chmod 755 "$INSTALL_DIR/.myrn.new"
mv -f "$INSTALL_DIR/.myrn.new" "$INSTALL_DIR/myrn"

say "installed $("$INSTALL_DIR/myrn" --version) to $INSTALL_DIR/myrn"
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *) say "add $INSTALL_DIR to your PATH, e.g.: echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.profile" ;;
esac
say "next, in a Git repository: myrn init && myrn connect --all"
