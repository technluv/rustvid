# Comprehensive Test Runner for Rust Video Editor (Windows)
# PowerShell script for running all tests on Windows

[CmdletBinding()]
param(
    [Parameter()]
    [ValidateSet("all", "unit", "integration", "platform", "performance", "accessibility", "e2e", "memory", "stress")]
    [string]$TestCategory = "all",
    
    [Parameter()]
    [switch]$GenerateReport,
    
    [Parameter()]
    [switch]$OpenReport
)

$ErrorActionPreference = "Continue"

# Script configuration
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
$ReportDir = Join-Path $ScriptDir "reports"
$Timestamp = Get-Date -Format "yyyyMMdd_HHmmss"

# Colors for output
$Colors = @{
    Success = "Green"
    Error = "Red"
    Warning = "Yellow"
    Info = "Cyan"
}

# Test categories
$TestCategories = @(
    "unit",
    "integration",
    "platform",
    "performance",
    "accessibility",
    "e2e",
    "smoke",
    "stress"
)

function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

function Test-Prerequisites {
    Write-ColorOutput "Checking prerequisites..." $Colors.Info
    
    $prerequisites = @{
        "Rust/Cargo" = { cargo --version }
        "FFmpeg" = { ffmpeg -version }
        "Python" = { python --version }
        "Node.js" = { node --version }
    }
    
    $missingPrereqs = @()
    
    foreach ($prereq in $prerequisites.GetEnumerator()) {
        try {
            & $prereq.Value | Out-Null
            Write-ColorOutput "  ✓ $($prereq.Key) found" $Colors.Success
        }
        catch {
            Write-ColorOutput "  ✗ $($prereq.Key) not found" $Colors.Warning
            $missingPrereqs += $prereq.Key
        }
    }
    
    if ($missingPrereqs.Count -gt 0) {
        Write-ColorOutput "Warning: Some prerequisites are missing. Some tests may be skipped." $Colors.Warning
    }
    
    return $missingPrereqs
}

function Initialize-TestEnvironment {
    Write-ColorOutput "Initializing test environment..." $Colors.Info
    
    # Create report directories
    $dirs = @(
        $ReportDir,
        (Join-Path $ReportDir "coverage"),
        (Join-Path $ReportDir "benchmarks"),
        (Join-Path $ReportDir "memory")
    )
    
    foreach ($dir in $dirs) {
        if (!(Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
        }
    }
    
    Write-ColorOutput "Test reports will be saved to: $ReportDir" $Colors.Info
}

function Generate-TestFixtures {
    Write-ColorOutput "Generating test fixtures..." $Colors.Info
    
    $fixtureScript = Join-Path $ScriptDir "fixtures\create_test_videos.py"
    if (Test-Path $fixtureScript) {
        try {
            python $fixtureScript
            Write-ColorOutput "  ✓ Test fixtures generated" $Colors.Success
        }
        catch {
            Write-ColorOutput "  ✗ Failed to generate test fixtures: $_" $Colors.Warning
        }
    }
}

function Run-UnitTests {
    Write-ColorOutput "`nRunning Unit Tests..." $Colors.Warning
    
    $env:RUST_LOG = "debug"
    $outputFile = Join-Path $ReportDir "unit_tests_$Timestamp.json"
    
    try {
        cargo test --lib --features test-utils -- `
            --test-threads=4 `
            --nocapture `
            --format json | Out-File $outputFile -Encoding UTF8
        
        Write-ColorOutput "  ✓ Unit tests completed" $Colors.Success
    }
    catch {
        Write-ColorOutput "  ✗ Unit tests failed: $_" $Colors.Error
    }
    
    # Generate coverage if tarpaulin is available
    if (Get-Command cargo-tarpaulin -ErrorAction SilentlyContinue) {
        Write-ColorOutput "  Generating coverage report..." $Colors.Info
        cargo tarpaulin --out Html --output-dir (Join-Path $ReportDir "coverage")
    }
}

function Run-IntegrationTests {
    Write-ColorOutput "`nRunning Integration Tests..." $Colors.Warning
    
    if ($missingPrereqs -notcontains "FFmpeg") {
        $env:RUST_LOG = "debug"
        
        try {
            cargo test --test integration_tests -- `
                --test-threads=2 `
                --nocapture
            
            Write-ColorOutput "  ✓ Integration tests completed" $Colors.Success
        }
        catch {
            Write-ColorOutput "  ✗ Integration tests failed: $_" $Colors.Error
        }
    }
    else {
        Write-ColorOutput "  Skipping FFmpeg-dependent integration tests" $Colors.Warning
    }
}

function Run-PlatformTests {
    Write-ColorOutput "`nRunning Platform-Specific Tests (Windows)..." $Colors.Warning
    
    try {
        cargo test --features windows-specific -- --nocapture
        Write-ColorOutput "  ✓ Platform tests completed" $Colors.Success
    }
    catch {
        Write-ColorOutput "  ✗ Platform tests failed: $_" $Colors.Error
    }
}

function Run-PerformanceTests {
    Write-ColorOutput "`nRunning Performance Benchmarks..." $Colors.Warning
    
    try {
        # Run criterion benchmarks
        cargo bench --bench '*' -- --save-baseline $Timestamp
        
        # Copy benchmark results
        $criterionDir = Join-Path $ProjectRoot "target\criterion"
        if (Test-Path $criterionDir) {
            Copy-Item -Path $criterionDir -Destination (Join-Path $ReportDir "benchmarks") -Recurse -Force
        }
        
        # Run custom benchmarks
        cargo test --test performance_tests --release -- --nocapture
        
        Write-ColorOutput "  ✓ Performance tests completed" $Colors.Success
    }
    catch {
        Write-ColorOutput "  ✗ Performance tests failed: $_" $Colors.Error
    }
}

function Run-AccessibilityTests {
    Write-ColorOutput "`nRunning Accessibility Tests..." $Colors.Warning
    
    # Run Tauri accessibility tests if UI crate exists
    $uiDir = Join-Path $ProjectRoot "crates\ui"
    if (Test-Path $uiDir) {
        Push-Location $uiDir
        
        if ((Test-Path "package.json") -and (Get-Command npm -ErrorAction SilentlyContinue)) {
            try {
                npm test -- --coverage
                Write-ColorOutput "  ✓ Frontend accessibility tests completed" $Colors.Success
            }
            catch {
                Write-ColorOutput "  ✗ Frontend accessibility tests failed: $_" $Colors.Error
            }
        }
        
        Pop-Location
    }
    
    try {
        cargo test --test accessibility_tests -- --nocapture
        Write-ColorOutput "  ✓ Accessibility tests completed" $Colors.Success
    }
    catch {
        Write-ColorOutput "  ✗ Accessibility tests failed: $_" $Colors.Error
    }
}

function Run-E2ETests {
    Write-ColorOutput "`nRunning End-to-End Tests..." $Colors.Warning
    
    # Start test server if needed
    $serveScript = Join-Path $ProjectRoot "demo-site\serve.py"
    $serverProcess = $null
    
    if (Test-Path $serveScript) {
        try {
            $serverProcess = Start-Process python -ArgumentList $serveScript -PassThru -WindowStyle Hidden
            Start-Sleep -Seconds 2
        }
        catch {
            Write-ColorOutput "  Warning: Could not start test server" $Colors.Warning
        }
    }
    
    try {
        cargo test --test e2e_tests -- --nocapture
        Write-ColorOutput "  ✓ E2E tests completed" $Colors.Success
    }
    catch {
        Write-ColorOutput "  ✗ E2E tests failed: $_" $Colors.Error
    }
    finally {
        # Clean up server
        if ($serverProcess -and !$serverProcess.HasExited) {
            Stop-Process -Id $serverProcess.Id -Force
        }
    }
}

function Run-MemoryTests {
    Write-ColorOutput "`nRunning Memory Tests..." $Colors.Warning
    
    try {
        cargo test --test memory_tests -- --nocapture
        Write-ColorOutput "  ✓ Memory tests completed" $Colors.Success
    }
    catch {
        Write-ColorOutput "  ✗ Memory tests failed: $_" $Colors.Error
    }
    
    # Note: Windows doesn't have valgrind, but we can use Application Verifier if available
}

function Run-StressTests {
    Write-ColorOutput "`nRunning Stress Tests..." $Colors.Warning
    
    $env:RUST_LOG = "warn"
    
    try {
        cargo test --test stress_tests --release -- `
            --test-threads=1 `
            --nocapture
        
        Write-ColorOutput "  ✓ Stress tests completed" $Colors.Success
    }
    catch {
        Write-ColorOutput "  ✗ Stress tests failed: $_" $Colors.Error
    }
}

function Generate-TestReport {
    Write-ColorOutput "`nGenerating test reports..." $Colors.Info
    
    $reportScript = Join-Path $ScriptDir "generate_test_report.py"
    if (Test-Path $reportScript) {
        try {
            python $reportScript
            Write-ColorOutput "  ✓ Test reports generated" $Colors.Success
            
            # Find the generated HTML report
            $htmlReport = Get-ChildItem -Path $ReportDir -Filter "test_report_*.html" | 
                         Sort-Object LastWriteTime -Descending | 
                         Select-Object -First 1
            
            if ($htmlReport -and $OpenReport) {
                Start-Process $htmlReport.FullName
            }
        }
        catch {
            Write-ColorOutput "  ✗ Failed to generate test reports: $_" $Colors.Error
        }
    }
}

function Run-AllTests {
    $testFunctions = @{
        "unit" = { Run-UnitTests }
        "integration" = { Run-IntegrationTests }
        "platform" = { Run-PlatformTests }
        "performance" = { Run-PerformanceTests }
        "accessibility" = { Run-AccessibilityTests }
        "e2e" = { Run-E2ETests }
        "memory" = { Run-MemoryTests }
        "stress" = { Run-StressTests }
    }
    
    if ($TestCategory -eq "all") {
        foreach ($category in $TestCategories) {
            if ($testFunctions.ContainsKey($category)) {
                & $testFunctions[$category]
            }
        }
    }
    else {
        if ($testFunctions.ContainsKey($TestCategory)) {
            & $testFunctions[$TestCategory]
        }
        else {
            Write-ColorOutput "Unknown test category: $TestCategory" $Colors.Error
            exit 1
        }
    }
}

# Main execution
Write-ColorOutput "=========================================" $Colors.Info
Write-ColorOutput "Rust Video Editor - Comprehensive Test Suite" $Colors.Info
Write-ColorOutput "=========================================" $Colors.Info
Write-ColorOutput ""

# Check prerequisites
$missingPrereqs = Test-Prerequisites

# Initialize environment
Initialize-TestEnvironment

# Generate fixtures
Generate-TestFixtures

# Run tests
Run-AllTests

# Generate report if requested
if ($GenerateReport) {
    Generate-TestReport
}

Write-ColorOutput "`nAll tests completed!" $Colors.Success
Write-ColorOutput "Test reports saved to: $ReportDir" $Colors.Info