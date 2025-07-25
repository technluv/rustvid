#!/bin/bash
# Code Signing Configuration Script
# Handles code signing for all platforms

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🔐 Code Signing Configuration${NC}"
echo -e "${CYAN}============================${NC}"

# Function to setup Windows code signing
setup_windows_signing() {
    echo -e "\n${YELLOW}🪟 Windows Code Signing Setup${NC}"
    echo -e "${YELLOW}----------------------------${NC}"
    
    cat << 'EOF'
Requirements:
1. Code Signing Certificate (.pfx or .p12 file)
2. Certificate password
3. Windows SDK with signtool.exe

Steps to obtain a certificate:
1. Purchase from a Certificate Authority (CA):
   - DigiCert
   - Sectigo (formerly Comodo)
   - GlobalSign
   
2. Or use a self-signed certificate for testing:
   ```powershell
   New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=YourCompany" -KeyExportPolicy Exportable -CertStoreLocation "Cert:\CurrentUser\My"
   ```

Environment variables needed:
- WINDOWS_CERTIFICATE: Base64-encoded certificate file
- WINDOWS_CERTIFICATE_PASSWORD: Certificate password

To encode certificate:
```bash
base64 -i certificate.pfx -o certificate.txt
```
EOF
}

# Function to setup macOS code signing
setup_macos_signing() {
    echo -e "\n${YELLOW}🍎 macOS Code Signing Setup${NC}"
    echo -e "${YELLOW}---------------------------${NC}"
    
    cat << 'EOF'
Requirements:
1. Apple Developer Account ($99/year)
2. Developer ID Application certificate
3. Developer ID Installer certificate (optional)
4. Notarization credentials

Steps:
1. Join Apple Developer Program
2. Create certificates in Apple Developer portal
3. Download and install in Keychain
4. Export as .p12 file with password

Environment variables needed:
- MACOS_CERTIFICATE: Base64-encoded .p12 file
- MACOS_CERTIFICATE_PWD: Certificate password
- MACOS_SIGNING_IDENTITY: "Developer ID Application: Your Name (TEAMID)"
- MACOS_NOTARIZATION_APPLE_ID: Your Apple ID
- MACOS_NOTARIZATION_PWD: App-specific password

To create app-specific password:
1. Go to https://appleid.apple.com
2. Sign in and go to Security
3. Generate app-specific password

To encode certificate:
```bash
base64 -i certificate.p12 -o certificate.txt
```
EOF
}

# Function to setup Linux code signing
setup_linux_signing() {
    echo -e "\n${YELLOW}🐧 Linux Code Signing Setup${NC}"
    echo -e "${YELLOW}---------------------------${NC}"
    
    cat << 'EOF'
Linux typically uses GPG signing for packages:

Requirements:
1. GPG key pair
2. Public key uploaded to keyservers

Steps:
1. Generate GPG key:
   ```bash
   gpg --full-generate-key
   ```

2. Export public key:
   ```bash
   gpg --armor --export your@email.com > public.key
   ```

3. Sign packages:
   - DEB: Use dpkg-sig
   - RPM: Use rpmsign
   - AppImage: Built-in signing support

For AppImage signing:
```bash
# Generate key pair
openssl genrsa -out private.key 4096
openssl rsa -in private.key -pubout -out public.key

# Sign AppImage
./appimagetool --sign --sign-key private.key MyApp.AppImage
```
EOF
}

# Function to create signing configuration file
create_signing_config() {
    echo -e "\n${YELLOW}📝 Creating signing configuration...${NC}"
    
    CONFIG_FILE="signing-config.json"
    
    cat > "$CONFIG_FILE" << 'EOF'
{
  "windows": {
    "certificate_path": "${WINDOWS_CERTIFICATE_PATH}",
    "certificate_password": "${WINDOWS_CERTIFICATE_PASSWORD}",
    "timestamp_server": "http://timestamp.sectigo.com",
    "algorithm": "sha256"
  },
  "macos": {
    "certificate_path": "${MACOS_CERTIFICATE_PATH}",
    "certificate_password": "${MACOS_CERTIFICATE_PWD}",
    "identity": "${MACOS_SIGNING_IDENTITY}",
    "notarization": {
      "apple_id": "${MACOS_NOTARIZATION_APPLE_ID}",
      "password": "${MACOS_NOTARIZATION_PWD}",
      "asc_provider": "${MACOS_ASC_PROVIDER}"
    },
    "entitlements": {
      "com.apple.security.cs.allow-jit": true,
      "com.apple.security.cs.allow-unsigned-executable-memory": true,
      "com.apple.security.device.camera": true,
      "com.apple.security.device.microphone": true
    }
  },
  "linux": {
    "gpg_key": "${LINUX_GPG_KEY}",
    "gpg_passphrase": "${LINUX_GPG_PASSPHRASE}",
    "appimage_key": "${LINUX_APPIMAGE_KEY}"
  }
}
EOF
    
    echo -e "${GREEN}✅ Created $CONFIG_FILE${NC}"
    echo -e "${YELLOW}⚠️  Remember to set environment variables before building!${NC}"
}

# Function to verify signing setup
verify_signing() {
    echo -e "\n${YELLOW}🔍 Verifying signing setup...${NC}"
    
    # Check Windows
    if command -v signtool &> /dev/null; then
        echo -e "${GREEN}✅ Windows: signtool found${NC}"
    else
        echo -e "${YELLOW}⚠️  Windows: signtool not found${NC}"
    fi
    
    # Check macOS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v codesign &> /dev/null; then
            echo -e "${GREEN}✅ macOS: codesign found${NC}"
            
            # List available identities
            echo -e "${CYAN}Available signing identities:${NC}"
            security find-identity -v -p codesigning
        else
            echo -e "${RED}❌ macOS: codesign not found${NC}"
        fi
    fi
    
    # Check Linux
    if command -v gpg &> /dev/null; then
        echo -e "${GREEN}✅ Linux: gpg found${NC}"
        
        # List available keys
        echo -e "${CYAN}Available GPG keys:${NC}"
        gpg --list-secret-keys
    else
        echo -e "${YELLOW}⚠️  Linux: gpg not found${NC}"
    fi
}

# Main menu
while true; do
    echo -e "\n${CYAN}Select an option:${NC}"
    echo "1) Windows signing setup"
    echo "2) macOS signing setup"
    echo "3) Linux signing setup"
    echo "4) Create signing configuration"
    echo "5) Verify signing setup"
    echo "6) Exit"
    
    read -p "Enter choice [1-6]: " choice
    
    case $choice in
        1) setup_windows_signing ;;
        2) setup_macos_signing ;;
        3) setup_linux_signing ;;
        4) create_signing_config ;;
        5) verify_signing ;;
        6) echo -e "${GREEN}Exiting...${NC}"; exit 0 ;;
        *) echo -e "${RED}Invalid choice${NC}" ;;
    esac
done