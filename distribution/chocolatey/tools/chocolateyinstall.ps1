$ErrorActionPreference = 'Stop'
$toolsDir   = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/your-org/rust-video-editor/releases/download/v1.0.0/rust-video-editor-1.0.0-x64-setup.exe'

$packageArgs = @{
  packageName   = $env:ChocolateyPackageName
  unzipLocation = $toolsDir
  fileType      = 'exe'
  url64bit      = $url64
  
  softwareName  = 'Rust Video Editor*'
  
  checksum64    = 'PLACEHOLDER_SHA256_CHECKSUM'
  checksumType64= 'sha256'
  
  silentArgs    = "/S"
  validExitCodes= @(0, 3010, 1641)
}

# Check for Vulkan runtime
Write-Host "Checking for Vulkan runtime..."
$vulkanInstalled = Test-Path "$env:WINDIR\System32\vulkan-1.dll"
if (-not $vulkanInstalled) {
  Write-Warning "Vulkan runtime not detected. GPU acceleration may not work properly."
  Write-Warning "Please install Vulkan runtime from: https://vulkan.lunarg.com/sdk/home#windows"
}

# Check for compatible GPU
Write-Host "Checking for GPU compatibility..."
$gpuInfo = Get-WmiObject Win32_VideoController | Select-Object Name, DriverVersion
Write-Host "Detected GPU(s):"
$gpuInfo | ForEach-Object { Write-Host "  - $($_.Name) (Driver: $($_.DriverVersion))" }

# Install the package
Install-ChocolateyPackage @packageArgs

# Create Start Menu shortcuts
$startMenuPath = [Environment]::GetFolderPath("CommonStartMenu")
$shortcutPath = Join-Path $startMenuPath "Programs\Rust Video Editor.lnk"
if (-not (Test-Path $shortcutPath)) {
  $WshShell = New-Object -comObject WScript.Shell
  $Shortcut = $WshShell.CreateShortcut($shortcutPath)
  $Shortcut.TargetPath = "$env:ProgramFiles\Rust Video Editor\rust-video-editor.exe"
  $Shortcut.IconLocation = "$env:ProgramFiles\Rust Video Editor\rust-video-editor.exe,0"
  $Shortcut.Save()
}

Write-Host "Rust Video Editor has been installed successfully!"
Write-Host ""
Write-Host "To get started:"
Write-Host "  1. Launch from Start Menu or run 'rust-video-editor' from command line"
Write-Host "  2. Check out sample projects in Documents\Rust Video Editor\Samples"
Write-Host "  3. Visit https://github.com/your-org/rust-video-editor/wiki for documentation"