#!/bin/sh
set -eu

k3s_version="${K3S_VERSION:?set K3S_VERSION, e.g. v1.37.1+k3s1}"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

case "$(uname -m)" in
    x86_64) asset=k3s; arch=amd64 ;;
    aarch64) asset=k3s-arm64; arch=arm64 ;;
    *) echo "bootstrap.sh: unsupported architecture $(uname -m)" >&2; exit 2 ;;
esac

if ! command -v curl >/dev/null 2>&1; then
    apt-get update -qq
    DEBIAN_FRONTEND=noninteractive apt-get install -y -qq curl ca-certificates >/dev/null
fi

installed="$(k3s --version 2>/dev/null | head -n 1 | awk '{print $3}' || true)"
if [ "$installed" != "$k3s_version" ]; then
    release="https://github.com/k3s-io/k3s/releases/download/$(printf %s "$k3s_version" | sed 's/+/%2B/')"
    curl -fsSL -o "$work/$asset" "$release/$asset"
    curl -fsSL -o "$work/sha256sum-$arch.txt" "$release/sha256sum-$arch.txt"
    curl -fsSL -o "$work/install.sh" "https://raw.githubusercontent.com/k3s-io/k3s/$k3s_version/install.sh"
    expected="$(awk -v name="$asset" '$2 == name {print $1}' "$work/sha256sum-$arch.txt")"
    actual="$(sha256sum "$work/$asset" | awk '{print $1}')"
    if [ -z "$expected" ] || [ "$expected" != "$actual" ]; then
        echo "bootstrap.sh: sha256 mismatch for $asset (expected '$expected', got '$actual')" >&2
        exit 3
    fi
    install -m 0755 "$work/$asset" /usr/local/bin/k3s
    INSTALL_K3S_SKIP_DOWNLOAD=true INSTALL_K3S_VERSION="$k3s_version" \
        INSTALL_K3S_EXEC="server --write-kubeconfig-mode 0644 --disable traefik --disable servicelb" \
        sh "$work/install.sh"
fi

systemctl enable --now k3s >/dev/null 2>&1 || true
deadline=$(( $(date +%s) + 300 ))
until k3s kubectl get nodes 2>/dev/null | awk 'NR > 1 && $2 == "Ready" {found = 1} END {exit !found}'; do
    if [ "$(date +%s)" -ge "$deadline" ]; then
        echo "bootstrap.sh: node not Ready after 300 s" >&2
        k3s kubectl get nodes >&2 || true
        exit 4
    fi
    sleep 5
done
echo "K3S-READY $(k3s --version | head -n 1)"
