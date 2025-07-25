# Installation Verification Script for Rust Video Editor (Windows)
# This script checks if Rust Video Editor is properly installed and all dependencies are met

$ErrorActionPreference = "Continue"
$Errors = 0
$Warnings = 0

# Functions
function Write-Success {
    param([string]$Message)
    Write-Host "✓ " -ForegroundColor Green -NoNewline
    Write-Host $Message
}

function Write-Error {
    param([string]$Message)
    Write-Host "✗ " -ForegroundColor Red -NoNewline
    Write-Host $Message
    $script:Errors++
}

function Write-Warning {
    param([string]$Message)
    Write-Host "⚠ " -ForegroundColor Yellow -NoNewline
    Write-Host $Message
    $script:Warnings++
}

function Write-Info {
    param([string]$Message)
    Write-Host "ℹ " -ForegroundColor Cyan -NoNewline
    Write-Host $Message
}

function Test-Command {
    param(
        [string]$Command,
        [string]$Description
    )
    
    $cmd = Get-Command $Command -ErrorAction SilentlyContinue
    if ($cmd) {
        Write-Success "$Description found: $($cmd.Source)"
        return $true
    } else {
        Write-Error "$Description not found"
        return $false
    }
}

function Test-Registry {
    param(
        [string]$Path,
        [string]$Name
    )
    
    try {
        $value = Get-ItemProperty -Path $Path -Name $Name -ErrorAction SilentlyContinue
        return $value -ne $null
    } catch {
        return $false
    }
}

# Header
Write-Host "================================================" -ForegroundColor Blue
Write-Host "Rust Video Editor Installation Verification" -ForegroundColor Blue
Write-Host "================================================" -ForegroundColor Blue
Write-Host ""

# Check for Rust Video Editor executable
Write-Host "Checking Rust Video Editor installation..."
if (Test-Command "rust-video-editor" "Rust Video Editor") {
    try {
        $version = & rust-video-editor --version 2>$null
        Write-Info "Version: $version"
    } catch {
        Write-Info "Version information unavailable"
    }
} else {
    Write-Info "Checking common installation paths..."
    $commonPaths = @(
        "$env:ProgramFiles\Rust Video Editor\rust-video-editor.exe",
        "${env:ProgramFiles(x86)}\Rust Video Editor\rust-video-editor.exe",
        "$env:LOCALAPPDATA\Programs\Rust Video Editor\rust-video-editor.exe",
        "$env:USERPROFILE\scoop\apps\rust-video-editor\current\rust-video-editor.exe"
    )
    
    $found = $false
    foreach ($path in $commonPaths) {
        if (Test-Path $path) {
            Write-Success "Found at: $path"
            $found = $true
            break
        }
    }
    
    if (-not $found) {
        Write-Error "Rust Video Editor executable not found in common paths"
    }
}

Write-Host ""
Write-Host "Checking system dependencies..."

# Check for Visual C++ Redistributable
Write-Host ""
Write-Host "Visual C++ Runtime:"
$vcRedistKeys = @(
    "HKLM:\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64",
    "HKLM:\SOFTWARE\WOW6432Node\Microsoft\VisualStudio\14.0\VC\Runtimes\x64"
)

$vcRedistFound = $false
foreach ($key in $vcRedistKeys) {
    if (Test-Path $key) {
        try {
            $installed = Get-ItemProperty -Path $key -Name "Installed" -ErrorAction SilentlyContinue
            if ($installed.Installed -eq 1) {
                $version = Get-ItemProperty -Path $key -Name "Version" -ErrorAction SilentlyContinue
                Write-Success "Visual C++ Redistributable found (version: $($version.Version))"
                $vcRedistFound = $true
                break
            }
        } catch {}
    }
}

if (-not $vcRedistFound) {
    Write-Error "Visual C++ Redistributable 2019 or later not found"
    Write-Info "Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe"
}

# Check for Vulkan
Write-Host ""
Write-Host "Graphics/Vulkan:"
$vulkanDll = "$env:WINDIR\System32\vulkan-1.dll"
if (Test-Path $vulkanDll) {
    Write-Success "Vulkan runtime found"
    
    # Try to get Vulkan version
    if (Test-Command "vulkaninfo" "Vulkan Info") {
        try {
            $gpuInfo = & vulkaninfo 2>$null | Select-String "deviceName" | Select-Object -First 1
            if ($gpuInfo) {
                $gpu = $gpuInfo -replace '.*deviceName\s*=\s*', ''
                Write-Info "GPU: $gpu"
            }
        } catch {}
    }
} else {
    Write-Error "Vulkan runtime not found"
    Write-Info "Download from: https://vulkan.lunarg.com/sdk/home#windows"
}

# Check for GPU drivers
Write-Host ""
Write-Host "GPU Drivers:"
$gpuAdapters = Get-WmiObject Win32_VideoController
foreach ($gpu in $gpuAdapters) {
    Write-Success "GPU: $($gpu.Name)"
    Write-Info "Driver Version: $($gpu.DriverVersion)"
    Write-Info "Driver Date: $($gpu.DriverDate)"
    
    # Check for hardware encoding support
    if ($gpu.Name -match "NVIDIA") {
        Write-Info "NVIDIA GPU detected - NVENC encoding may be available"
    } elseif ($gpu.Name -match "AMD|Radeon") {
        Write-Info "AMD GPU detected - AMF encoding may be available"
    } elseif ($gpu.Name -match "Intel") {
        Write-Info "Intel GPU detected - QuickSync encoding may be available"
    }
}

# Check for DirectX
Write-Host ""
Write-Host "DirectX:"
try {
    $dxdiag = Get-WmiObject -Class Win32_VideoController | Select-Object -First 1
    if ($dxdiag) {
        Write-Success "DirectX compatible GPU found"
    }
} catch {
    Write-Warning "Unable to verify DirectX compatibility"
}

# Check audio system
Write-Host ""
Write-Host "Audio System:"
$audioDevices = Get-WmiObject Win32_SoundDevice
if ($audioDevices) {
    Write-Success "Audio devices found:"
    foreach ($device in $audioDevices) {
        Write-Info "  - $($device.Name)"
    }
} else {
    Write-Warning "No audio devices detected"
}

# Check for Windows Media Foundation
Write-Host ""
Write-Host "Media Framework:"
if (Test-Path "$env:WINDIR\System32\mf.dll") {
    Write-Success "Windows Media Foundation found"
} else {
    Write-Error "Windows Media Foundation not found"
}

# Check for optional codecs
$codecDlls = @{
    "H.264" = "$env:WINDIR\System32\msmpeg2vdec.dll"
    "H.265/HEVC" = "$env:WINDIR\System32\hevcdecoder.dll"
}

foreach ($codec in $codecDlls.GetEnumerator()) {
    if (Test-Path $codec.Value) {
        Write-Success "$($codec.Key) codec available"
    } else {
        Write-Warning "$($codec.Key) codec not found - may require Windows Media Feature Pack"
    }
}

# Check configuration
Write-Host ""
Write-Host "Configuration:"
$configPaths = @(
    "$env:APPDATA\Rust Video Editor",
    "$env:LOCALAPPDATA\Rust Video Editor",
    "$env:ProgramData\Rust Video Editor"
)

foreach ($path in $configPaths) {
    if (Test-Path $path) {
        Write-Success "Config directory found: $path"
    }
}

# Check for file associations
Write-Host ""
Write-Host "File Associations:"
$videoExtensions = @(".mp4", ".avi", ".mov", ".mkv")
foreach ($ext in $videoExtensions) {
    try {
        $assoc = Get-ItemProperty -Path "Registry::HKEY_CLASSES_ROOT\$ext" -ErrorAction SilentlyContinue
        if ($assoc) {
            Write-Info "$ext files can be opened"
        }
    } catch {}
}

# Check Windows version
Write-Host ""
Write-Host "System Information:"
$os = Get-WmiObject -Class Win32_OperatingSystem
Write-Info "Windows Version: $($os.Caption) $($os.Version)"
Write-Info "Architecture: $($os.OSArchitecture)"
Write-Info "Total RAM: $([math]::Round($os.TotalVisibleMemorySize / 1MB, 2)) GB"

# Check for Windows features
if ([Environment]::OSVersion.Version.Major -ge 10) {
    $build = [Environment]::OSVersion.Version.Build
    if ($build -ge 17763) {
        Write-Success "Windows version supports all features"
    } else {
        Write-Warning "Some features may require Windows 10 version 1809 or later"
    }
}

# Summary
Write-Host ""
Write-Host "================================================" -ForegroundColor Blue
Write-Host "Verification Summary" -ForegroundColor Blue
Write-Host "================================================" -ForegroundColor Blue

if ($Errors -eq 0) {
    if ($Warnings -eq 0) {
        Write-Host "All checks passed! " -ForegroundColor Green -NoNewline
        Write-Host "Rust Video Editor should work correctly."
    } else {
        Write-Host "Installation verified " -ForegroundColor Green -NoNewline
        Write-Host "with $Warnings warning(s)."
        Write-Host "Some optional features may not be available."
    }
} else {
    Write-Host "Verification failed " -ForegroundColor Red -NoNewline
    Write-Host "with $Errors error(s) and $Warnings warning(s)."
    Write-Host "Please install missing dependencies and try again."
    Write-Host ""
    Write-Host "For detailed installation instructions, visit:"
    Write-Host "https://github.com/your-org/rust-video-editor/wiki/Installation"
}

Write-Host ""
Write-Host "For troubleshooting, run: rust-video-editor.exe --diagnose"

# Pause if running in a new window
if ($Host.Name -eq "ConsoleHost") {
    Write-Host ""
    Write-Host "Press any key to continue..."
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}

exit $Errors