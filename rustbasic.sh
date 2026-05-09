#!/bin/bash

# RustBasic Smart Installer (Mac & Linux)
# ---------------------------------------

set -e

# Warna output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Deteksi OS
OS_TYPE="$(uname)"
if [ "$OS_TYPE" == "Darwin" ]; then
    SED_EXT="''"
    SHELL_CONFIG="$HOME/.zshrc"
    [ ! -f "$SHELL_CONFIG" ] && SHELL_CONFIG="$HOME/.bash_profile"
else
    SED_EXT=""
    SHELL_CONFIG="$HOME/.bashrc"
    [ -f "$HOME/.zshrc" ] && SHELL_CONFIG="$HOME/.zshrc"
fi

echo -e "${CYAN}${BOLD}Menjalankan RustBasic Installer untuk $OS_TYPE...${NC}\n"

# --- FUNGSI UNINSTALL ---
uninstall_rustbasic() {
    echo -e "${RED}Menghapus instalasi RustBasic...${NC}"
    cargo uninstall rustbasic || true
    if [ -f "/usr/local/bin/rustbasic" ]; then
        sudo rm -f /usr/local/bin/rustbasic || true
    fi
    # Hapus alias dari config shell
    if [ "$OS_TYPE" == "Darwin" ]; then
        sed -i '' '/alias rustbasic=/d' "$SHELL_CONFIG" || true
    else
        sed -i '/alias rustbasic=/d' "$SHELL_CONFIG" || true
    fi
    echo -e "${GREEN}✅ Berhasil dihapus.${NC}"
}

# --- LOGIKA MENU ---
if command -v rustbasic-cli &> /dev/null || [ -f "/usr/local/bin/rustbasic" ]; then
    echo -e "${BLUE}RustBasic terdeteksi.${NC} Pilih opsi:"
    echo "1) Reinstall"
    echo "2) Uninstall"
    echo "3) Exit"
    read -p "Pilihan: " choice
    case $choice in
        1) uninstall_rustbasic ;;
        2) uninstall_rustbasic; exit 0 ;;
        *) exit 0 ;;
    esac
fi

# --- INSTALASI ---
echo -e "${BLUE}⏳ Langkah 1/3: Membangun dari GitHub...${NC}"
cargo install --git https://github.com/herisvan321/rustbasic --bin rustbasic-cli --force

echo -e "${BLUE}⏳ Langkah 2/3: Mendaftarkan perintah global...${NC}"
# Symlink untuk akses instan
if [ -w "/usr/local/bin" ]; then
    ln -sf "$HOME/.cargo/bin/rustbasic-cli" /usr/local/bin/rustbasic
else
    sudo ln -sf "$HOME/.cargo/bin/rustbasic-cli" /usr/local/bin/rustbasic
fi

# Alias untuk permanen
if ! grep -q "alias rustbasic=" "$SHELL_CONFIG"; then
    echo "alias rustbasic='rustbasic-cli'" >> "$SHELL_CONFIG"
fi

echo -e "${BLUE}⏳ Langkah 3/3: Cek dependency...${NC}"
command -v cargo-watch &> /dev/null || cargo install cargo-watch

echo -e "\n${GREEN}${BOLD}✨ RUSTBASIC BERHASIL TERPASANG! ✨${NC}"
echo -e "Gunakan perintah: ${BOLD}rustbasic${NC}"
echo -e "Konfigurasi disimpan di: $SHELL_CONFIG"
echo -e "Silakan jalankan 'source $SHELL_CONFIG' jika perintah belum muncul."
