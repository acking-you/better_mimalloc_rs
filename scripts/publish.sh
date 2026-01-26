#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <version>" >&2
  exit 1
fi

version="$1"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ ! -f "$root/libmimalloc-sys/c_src/mimalloc/include/mimalloc.h" ]]; then
  echo "mimalloc submodule not initialized." >&2
  echo "Run: git submodule update --init --recursive" >&2
  exit 1
fi

tmp_dir="$(mktemp -d)"
cargo_home="$(mktemp -d)"
keep_tmp="${PUBLISH_KEEP_TEMP:-0}"
cleanup() {
  if [[ "$keep_tmp" == "1" ]]; then
    echo "Keeping temp dir: $tmp_dir" >&2
  else
    rm -rf "$tmp_dir"
  fi
  rm -rf "$cargo_home"
}
trap cleanup EXIT

if [[ -f "$HOME/.cargo/credentials.toml" ]]; then
  cp "$HOME/.cargo/credentials.toml" "$cargo_home/credentials.toml"
elif [[ -f "$HOME/.cargo/credentials" ]]; then
  cp "$HOME/.cargo/credentials" "$cargo_home/credentials"
fi

cat > "$cargo_home/config.toml" <<'CFG'
[net]
git-fetch-with-cli = true
CFG

rsync -a --delete \
  --exclude '.git' \
  --exclude 'target' \
  --exclude '.DS_Store' \
  "$root/" "$tmp_dir/"

python3 - "$version" "$tmp_dir" <<'PY'
import re
import sys
from pathlib import Path

version = sys.argv[1]
root = Path(sys.argv[2])

root_toml = root / "Cargo.toml"
sys_toml = root / "libmimalloc-sys" / "Cargo.toml"


def update_version(path: Path, version: str) -> None:
    data = path.read_text()
    data, n = re.subn(r"(?m)^version = \".*\"$", f'version = "{version}"', data, count=1)
    if n != 1:
        raise SystemExit(f"Failed to update version in {path}")
    path.write_text(data)


def update_dep_version(path: Path, dep: str, version: str) -> None:
    data = path.read_text()
    pattern = rf'({re.escape(dep)}\s*=\s*\{{[^\n]*\bversion\s*=\s*")([^"]+)("[^\n]*\}})'
    data, n = re.subn(pattern, lambda m: f"{m.group(1)}{version}{m.group(3)}", data, count=1)
    if n != 1:
        raise SystemExit(f"Failed to update dependency {dep} in {path}")
    path.write_text(data)


update_version(root_toml, version)
update_version(sys_toml, version)
update_dep_version(root_toml, "better_mimalloc_sys", version)
PY

publish_crate() {
  local manifest="$1"
  local output
  if ! output=$(CARGO_HOME="$cargo_home" cargo publish --registry crates-io --manifest-path "$manifest" 2>&1); then
    if echo "$output" | grep -qi "already exists"; then
      echo "$output" >&2
      return 0
    fi
    echo "$output" >&2
    return 1
  fi
  echo "$output"
}

(
  cd "$tmp_dir"
  publish_crate libmimalloc-sys/Cargo.toml
)

(
  cd "$tmp_dir"
  for attempt in {1..10}; do
    if CARGO_HOME="$cargo_home" cargo publish --registry crates-io --manifest-path Cargo.toml; then
      exit 0
    fi
    echo "Publish main crate failed; waiting for registry to update (attempt $attempt/10)..." >&2
    sleep 10
  done
  echo "Failed to publish better_mimalloc_rs after retries." >&2
  exit 1
)
