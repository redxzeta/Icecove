#!/usr/bin/env bash
set -euo pipefail

# ─────────────────────────────────────────────────────────────────────────────
# alcove — Install from source
# ─────────────────────────────────────────────────────────────────────────────
#
# Usage:
#   ./install.sh            Build & install binary, then optionally run setup
#   ./install.sh --no-setup Build & install binary only
#   ./install.sh uninstall  Remove binary via cargo uninstall
#
# After install, use the binary for all configuration:
#   alcove setup            Interactive setup (docs root, categories, agents)
#   alcove uninstall        Remove skills, config, and legacy files
#
# ─────────────────────────────────────────────────────────────────────────────

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BINARY_NAME="alcove"

info()  { echo -e "  $*"; }
ok()    { echo -e "  ${GREEN}✓${NC} $*"; }
warn()  { echo -e "  ${YELLOW}!${NC} $*"; }
err()   { echo -e "  ${RED}✗${NC} $*" >&2; }

install_binary() {
    if ! command -v cargo &>/dev/null; then
        err "Rust/cargo not found. Install from https://rustup.rs"
        exit 1
    fi

    if [[ "${SKIP_BUILD:-}" == "1" ]] && command -v "$BINARY_NAME" &>/dev/null; then
        ok "Binary already installed → $(command -v "$BINARY_NAME")"
        return
    fi

    info "Building and installing $BINARY_NAME..."
    (cd "$REPO_ROOT" && cargo install --path . 2>&1)

    local bin_path
    bin_path="$(command -v "$BINARY_NAME" 2>/dev/null || echo "$HOME/.cargo/bin/$BINARY_NAME")"
    ok "Binary → $bin_path"

    local cargo_bin="$HOME/.cargo/bin"
    if ! echo "$PATH" | tr ':' '\n' | grep -qx "$cargo_bin"; then
        warn "$cargo_bin is not in PATH. Add to your shell profile:"
        echo "      export PATH=\"$cargo_bin:\$PATH\""
    fi

    # Clean up legacy locations
    for legacy in "$HOME/.local/bin/docs-bridge-mcp" "$HOME/.local/bin/docs-bridge" "$HOME/.local/bin/alcove"; do
        if [[ -L "$legacy" || -f "$legacy" ]]; then
            rm -f "$legacy"
            ok "Removed legacy: $legacy"
        fi
    done
}

do_uninstall() {
    echo ""
    echo -e "${BOLD}Uninstalling alcove...${NC}"
    echo ""

    if command -v cargo &>/dev/null; then
        cargo uninstall "$BINARY_NAME" 2>/dev/null && ok "Removed binary via cargo uninstall" || true
    fi

    echo ""
    info "To also remove skills and config: alcove uninstall"
    echo ""
}

main() {
    case "${1:-}" in
        uninstall)
            do_uninstall
            ;;
        -h|--help)
            echo "Usage:"
            echo "  ./install.sh              Build & install, then optionally run setup"
            echo "  ./install.sh --no-setup   Build & install binary only"
            echo "  ./install.sh uninstall    Remove binary via cargo uninstall"
            echo ""
            echo "After install, use the binary directly:"
            echo "  alcove setup              Interactive setup"
            echo "  alcove uninstall          Remove skills & config"
            ;;
        --no-setup)
            echo ""
            echo -e "${BOLD}── Install Binary ──${NC}"
            install_binary
            echo ""
            ;;
        *)
            echo ""
            echo -e "${BOLD}── Install Binary ──${NC}"
            install_binary
            echo ""

            if [[ "${NO_SETUP:-}" != "1" ]]; then
                read -rp "  Run interactive setup (agents, docs root)? [Y/n] " ans
                if [[ "${ans:-Y}" != "n" && "${ans:-Y}" != "N" ]]; then
                    alcove setup
                else
                    echo ""
                    info "Skipped. Run 'alcove setup' later to configure."
                    echo ""
                fi
            fi
            ;;
    esac
}

main "$@"
