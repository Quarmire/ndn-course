#!/usr/bin/env bash
# ndn-course setup — the "minutes to first packet" contract, step 2 of 3.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

say()  { printf '\033[1m%s\033[0m\n' "$*"; }
warn() { printf '\033[33m! %s\033[0m\n' "$*"; }
fail() { printf '\033[31m✗ %s\033[0m\n' "$*"; exit 1; }

say "ndn-course setup"

# --- 1. toolchain ------------------------------------------------------------
command -v git   >/dev/null 2>&1 || fail "git not found — install git first"
if ! command -v cargo >/dev/null 2>&1; then
  fail "cargo not found — install Rust via https://rustup.rs then re-run setup.sh"
fi
say "✓ git + cargo present ($(cargo --version))"

# --- 2. pinned sibling repos (from pins.toml) --------------------------------
# pins.toml format (order matters within a section: url before tag):
#   [repos.<name>]
#   url = "<git url>"
#   tag = "<tag>"        # "UNPINNED" = skip clone, warn
say "checking pinned sibling repos…"
awk '
  /^\[repos\./ { line=$0; gsub(/\[repos\.|\]/,"",line); name=line }
  /^url[ \t]*=/ { line=$0; sub(/^url[ \t]*=[ \t]*"/,"",line); sub(/".*$/,"",line); url=line }
  /^tag[ \t]*=/ { line=$0; sub(/^tag[ \t]*=[ \t]*"/,"",line); sub(/".*$/,"",line); tag=line;
                  print name" "url" "tag }
' pins.toml | while read -r name url tag; do
  dest="../$name"
  if [ -d "$dest" ]; then
    say "✓ $name found at $dest"
    if [ "$tag" != "UNPINNED" ] && [ -d "$dest/.git" ]; then
      have="$(git -C "$dest" describe --tags --exact-match 2>/dev/null || echo '(no exact tag)')"
      [ "$have" = "$tag" ] || warn "$name is at $have, course pins $tag — exercises may drift"
    fi
  elif [ "$tag" = "UNPINNED" ]; then
    warn "$name missing and pin not yet cut (UNPINNED) — skipping clone; phase-0 labs need it later"
  else
    say "cloning $name @ $tag …"
    git clone --depth 1 --branch "$tag" "$url" "$dest"
  fi
done

# --- 3. build the course CLI --------------------------------------------------
say "building the course CLI…"
chmod +x ./course
cargo build --quiet -p xtask

# --- 4. doctor ----------------------------------------------------------------
./course doctor
say ""
say "next: ./course start"
