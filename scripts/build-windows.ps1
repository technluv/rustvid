#!/usr/bin/env pwsh
# Windows Build Script for Rust Video Editor
# Creates MSI installer and portable ZIP

param(
    [Parameter()]
    [string]$Version = "",
    [switch]$SignCode = $false,
    [string]$CertPath = "",
    [string]$CertPassword = "",
    [switch]$CreateInstaller = $true,
    [switch]$CreatePortable = $true
)

$ErrorActionPreference = "Stop"

# Configuration
$ProjectName = "RustVideoEditor"
$ExecutableName = "rust-video-editor"
$AppDataName = "Rust Video Editor"
$Publisher = "RustVideoEditor Team"
$ProjectRoot = (Get-Item $PSScriptRoot).Parent.FullName
$BuildDir = "$ProjectRoot\target\release"
$DistDir = "$ProjectRoot\dist\windows"
$AssetsDir = "$ProjectRoot\assets"

# Colors for output
function Write-ColorOutput {
    param([string]$Message, [ConsoleColor]$ForegroundColor = "White")
    Write-Host $Message -ForegroundColor $ForegroundColor
}

Write-ColorOutput "🚀 Windows Build Script for $ProjectName" -ForegroundColor Cyan
Write-ColorOutput "=====================================" -ForegroundColor Cyan

# Get version from Cargo.toml if not provided
if ([string]::IsNullOrEmpty($Version)) {
    $CargoToml = Get-Content "$ProjectRoot\Cargo.toml" -Raw
    if ($CargoToml -match 'version\s*=\s*"([^"]+)"') {
        $Version = $Matches[1]
        Write-ColorOutput "📦 Detected version: $Version" -ForegroundColor Green
    } else {
        Write-ColorOutput "❌ Could not detect version from Cargo.toml" -ForegroundColor Red
        exit 1
    }
}

# Clean and create output directories
Write-ColorOutput "`n🧹 Cleaning output directories..." -ForegroundColor Yellow
if (Test-Path $DistDir) {
    Remove-Item -Path $DistDir -Recurse -Force
}
New-Item -ItemType Directory -Path $DistDir -Force | Out-Null
New-Item -ItemType Directory -Path "$DistDir\bundle" -Force | Out-Null

# Build the application
Write-ColorOutput "`n🔨 Building release version..." -ForegroundColor Yellow
Push-Location $ProjectRoot
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo build failed"
    }
} finally {
    Pop-Location
}

# Check if executable exists
$ExePath = "$BuildDir\$ExecutableName.exe"
if (-not (Test-Path $ExePath)) {
    Write-ColorOutput "❌ Executable not found at: $ExePath" -ForegroundColor Red
    exit 1
}

# Create application bundle directory
$BundleDir = "$DistDir\bundle"
Write-ColorOutput "`n📁 Creating application bundle..." -ForegroundColor Yellow

# Copy executable
Copy-Item -Path $ExePath -Destination $BundleDir

# Copy FFmpeg binaries if they exist
$FFmpegDir = "$ProjectRoot\ffmpeg\windows"
if (Test-Path $FFmpegDir) {
    Write-ColorOutput "📋 Copying FFmpeg binaries..." -ForegroundColor Yellow
    Copy-Item -Path "$FFmpegDir\*" -Destination $BundleDir -Include "*.exe", "*.dll"
}

# Copy assets
if (Test-Path $AssetsDir) {
    Write-ColorOutput "🎨 Copying assets..." -ForegroundColor Yellow
    Copy-Item -Path $AssetsDir -Destination $BundleDir -Recurse
}

# Create config directory
New-Item -ItemType Directory -Path "$BundleDir\config" -Force | Out-Null

# Copy README and LICENSE
$ReadmePath = "$ProjectRoot\README.md"
$LicensePath = "$ProjectRoot\LICENSE"
if (Test-Path $ReadmePath) {
    Copy-Item -Path $ReadmePath -Destination $BundleDir
}
if (Test-Path $LicensePath) {
    Copy-Item -Path $LicensePath -Destination $BundleDir
}

# Sign the executable if requested
if ($SignCode -and -not [string]::IsNullOrEmpty($CertPath)) {
    Write-ColorOutput "`n🔐 Signing executable..." -ForegroundColor Yellow
    $SignToolPath = "${env:ProgramFiles(x86)}\Windows Kits\10\bin\10.0.22621.0\x64\signtool.exe"
    
    if (Test-Path $SignToolPath) {
        & $SignToolPath sign /f $CertPath /p $CertPassword /t http://timestamp.sectigo.com `
            /fd sha256 /v "$BundleDir\$ExecutableName.exe"
        
        if ($LASTEXITCODE -ne 0) {
            Write-ColorOutput "⚠️  Code signing failed, continuing without signature" -ForegroundColor Yellow
        } else {
            Write-ColorOutput "✅ Code signing successful" -ForegroundColor Green
        }
    } else {
        Write-ColorOutput "⚠️  SignTool not found, skipping code signing" -ForegroundColor Yellow
    }
}

# Create portable ZIP
if ($CreatePortable) {
    Write-ColorOutput "`n📦 Creating portable ZIP..." -ForegroundColor Yellow
    $ZipPath = "$DistDir\$ProjectName-$Version-windows-portable.zip"
    
    # Use .NET compression
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [System.IO.Compression.ZipFile]::CreateFromDirectory($BundleDir, $ZipPath)
    
    Write-ColorOutput "✅ Created portable package: $ZipPath" -ForegroundColor Green
    Write-ColorOutput "   Size: $([math]::Round((Get-Item $ZipPath).Length / 1MB, 2)) MB" -ForegroundColor Gray
}

# Create MSI installer
if ($CreateInstaller) {
    Write-ColorOutput "`n📦 Creating MSI installer..." -ForegroundColor Yellow
    
    # Create WiX source file
    $WixSource = @"
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
    <Product Id="*" 
             Name="$AppDataName" 
             Language="1033" 
             Version="$Version" 
             Manufacturer="$Publisher" 
             UpgradeCode="7F2E9A2B-4D5C-4A6E-9F8D-1B2C3D4E5F6A">
        
        <Package InstallerVersion="500" 
                 Compressed="yes" 
                 InstallScope="perMachine" 
                 Platform="x64"
                 Description="Professional video editing software built with Rust" />
        
        <MajorUpgrade DowngradeErrorMessage="A newer version of [ProductName] is already installed." />
        <MediaTemplate EmbedCab="yes" />
        
        <Feature Id="ProductFeature" Title="$AppDataName" Level="1">
            <ComponentGroupRef Id="ProductComponents" />
            <ComponentRef Id="ApplicationShortcut" />
            <ComponentRef Id="DesktopShortcut" />
        </Feature>
        
        <!-- Install directory -->
        <Directory Id="TARGETDIR" Name="SourceDir">
            <Directory Id="ProgramFiles64Folder">
                <Directory Id="INSTALLFOLDER" Name="$AppDataName">
                    <Directory Id="CONFIGFOLDER" Name="config" />
                    <Directory Id="ASSETSFOLDER" Name="assets" />
                </Directory>
            </Directory>
            
            <Directory Id="ProgramMenuFolder">
                <Directory Id="ApplicationProgramsFolder" Name="$AppDataName"/>
            </Directory>
            
            <Directory Id="DesktopFolder" Name="Desktop" />
        </Directory>
        
        <!-- Application shortcut -->
        <DirectoryRef Id="ApplicationProgramsFolder">
            <Component Id="ApplicationShortcut" Guid="B3E4F5C6-7D8A-4E9B-A1C2-3D4E5F6A7B8C">
                <Shortcut Id="ApplicationStartMenuShortcut"
                          Name="$AppDataName"
                          Description="Professional video editing software"
                          Target="[INSTALLFOLDER]$ExecutableName.exe"
                          WorkingDirectory="INSTALLFOLDER"
                          Icon="MainIcon" />
                <RemoveFolder Id="ApplicationProgramsFolder" On="uninstall"/>
                <RegistryValue Root="HKCU" 
                               Key="Software\$Publisher\$AppDataName" 
                               Name="installed" 
                               Type="integer" 
                               Value="1" 
                               KeyPath="yes"/>
            </Component>
        </DirectoryRef>
        
        <!-- Desktop shortcut -->
        <DirectoryRef Id="DesktopFolder">
            <Component Id="DesktopShortcut" Guid="C4D5E6F7-8A9B-4F1C-B2D3-4E5F6A7B8C9D">
                <Shortcut Id="DesktopShortcut"
                          Name="$AppDataName"
                          Description="Professional video editing software"
                          Target="[INSTALLFOLDER]$ExecutableName.exe"
                          WorkingDirectory="INSTALLFOLDER"
                          Icon="MainIcon" />
                <RemoveFolder Id="DesktopFolder" On="uninstall"/>
                <RegistryValue Root="HKCU" 
                               Key="Software\$Publisher\$AppDataName" 
                               Name="desktopShortcut" 
                               Type="integer" 
                               Value="1" 
                               KeyPath="yes"/>
            </Component>
        </DirectoryRef>
        
        <!-- Icon -->
        <Icon Id="MainIcon" SourceFile="$BundleDir\assets\icon.ico" />
        
        <!-- UI -->
        <UIRef Id="WixUI_InstallDir" />
        <Property Id="WIXUI_INSTALLDIR" Value="INSTALLFOLDER" />
        
        <!-- License -->
        <WixVariable Id="WixUILicenseRtf" Value="$ProjectRoot\LICENSE.rtf" />
    </Product>
    
    <!-- Component group -->
    <Fragment>
        <ComponentGroup Id="ProductComponents" Directory="INSTALLFOLDER">
            <Component Id="MainExecutable" Guid="A1B2C3D4-5E6F-7A8B-9C0D-1E2F3A4B5C6D" Win64="yes">
                <File Id="MainExe" 
                      Source="$BundleDir\$ExecutableName.exe" 
                      KeyPath="yes" />
            </Component>
            
            <!-- Add all DLL files -->
            <Component Id="Dependencies" Guid="B2C3D4E5-6F7A-8B9C-0D1E-2F3A4B5C6D7E" Win64="yes">
                <File Source="$BundleDir\*.dll" />
            </Component>
        </ComponentGroup>
    </Fragment>
</Wix>
"@
    
    # Save WiX source
    $WixPath = "$DistDir\installer.wxs"
    $WixSource | Out-File -FilePath $WixPath -Encoding UTF8
    
    # Check if WiX is installed
    $WixBinPath = "${env:ProgramFiles(x86)}\WiX Toolset v3.11\bin"
    if (Test-Path $WixBinPath) {
        $env:Path += ";$WixBinPath"
        
        # Compile WiX source
        Write-ColorOutput "   Compiling WiX source..." -ForegroundColor Gray
        & candle.exe -arch x64 -out "$DistDir\installer.wixobj" $WixPath
        
        if ($LASTEXITCODE -eq 0) {
            # Link to create MSI
            Write-ColorOutput "   Creating MSI..." -ForegroundColor Gray
            & light.exe -ext WixUIExtension -out "$DistDir\$ProjectName-$Version-windows.msi" "$DistDir\installer.wixobj"
            
            if ($LASTEXITCODE -eq 0) {
                $MsiPath = "$DistDir\$ProjectName-$Version-windows.msi"
                Write-ColorOutput "✅ Created MSI installer: $MsiPath" -ForegroundColor Green
                Write-ColorOutput "   Size: $([math]::Round((Get-Item $MsiPath).Length / 1MB, 2)) MB" -ForegroundColor Gray
                
                # Sign MSI if requested
                if ($SignCode -and -not [string]::IsNullOrEmpty($CertPath)) {
                    Write-ColorOutput "🔐 Signing MSI installer..." -ForegroundColor Yellow
                    & $SignToolPath sign /f $CertPath /p $CertPassword /t http://timestamp.sectigo.com `
                        /fd sha256 /v $MsiPath
                }
            } else {
                Write-ColorOutput "⚠️  Failed to create MSI" -ForegroundColor Yellow
            }
        } else {
            Write-ColorOutput "⚠️  Failed to compile WiX source" -ForegroundColor Yellow
        }
    } else {
        Write-ColorOutput "⚠️  WiX Toolset not found, skipping MSI creation" -ForegroundColor Yellow
        Write-ColorOutput "   Install from: https://wixtoolset.org/releases/" -ForegroundColor Gray
    }
}

# Create checksums
Write-ColorOutput "`n🔒 Creating checksums..." -ForegroundColor Yellow
$ChecksumFile = "$DistDir\checksums-windows.txt"
"" | Out-File -FilePath $ChecksumFile

Get-ChildItem -Path $DistDir -Filter "*.zip", "*.msi" | ForEach-Object {
    $Hash = Get-FileHash -Path $_.FullName -Algorithm SHA256
    "$($Hash.Hash)  $($_.Name)" | Out-File -FilePath $ChecksumFile -Append
}

Write-ColorOutput "✅ Created checksums file" -ForegroundColor Green

# Summary
Write-ColorOutput "`n📊 Build Summary" -ForegroundColor Cyan
Write-ColorOutput "================" -ForegroundColor Cyan
Write-ColorOutput "Version: $Version" -ForegroundColor White
Write-ColorOutput "Output Directory: $DistDir" -ForegroundColor White
Write-ColorOutput "`nArtifacts:" -ForegroundColor White

Get-ChildItem -Path $DistDir -Filter "*.zip", "*.msi", "*.txt" | ForEach-Object {
    $Size = [math]::Round($_.Length / 1MB, 2)
    Write-ColorOutput "  - $($_.Name) ($Size MB)" -ForegroundColor Gray
}

Write-ColorOutput "`n✅ Windows build completed successfully!" -ForegroundColor Green