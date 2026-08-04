param(
    [string]$InstallRoot = $env:ADL_VECTOR_INSTALL_ROOT
)

$ErrorActionPreference = "Stop"

$Version = "0.56.0"
$Archive = "vector-0.56.0-x86_64-pc-windows-msvc.zip"
$ArchiveSha256 = "67611f6b18c3b267ab26402c0dddc59e59bbccd762c7c0ea5f654f4ec4e6bf42"
$Url = "https://github.com/vectordotdev/vector/releases/download/v$Version/$Archive"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
if ([string]::IsNullOrWhiteSpace($InstallRoot)) {
    $InstallRoot = Join-Path $RepoRoot ".adl"
}

$BinDir = Join-Path $InstallRoot "bin"
$DownloadDir = Join-Path $InstallRoot "downloads\vector"
$ComponentDir = Join-Path $InstallRoot "components\vector"
$ProvenanceDir = Join-Path $BinDir ".provenance"
$LockRoot = Join-Path $InstallRoot "locks"
$LockDir = Join-Path $LockRoot "vector-install.lock"
$ArchivePath = Join-Path $DownloadDir $Archive
$Target = Join-Path $BinDir "vector.exe"
$Provenance = Join-Path $ProvenanceDir "vector.json"

function Get-Sha256Hex([string]$Path) {
    return (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToLowerInvariant()
}

function Test-CurrentInstall {
    if (!(Test-Path $Target -PathType Leaf) -or !(Test-Path $Provenance -PathType Leaf)) {
        return $false
    }
    $VersionText = & $Target --version 2>$null
    if ($LASTEXITCODE -ne 0 -or $VersionText -notmatch "vector $Version") {
        return $false
    }
    $ProvenanceText = Get-Content -Raw -Path $Provenance
    if ($ProvenanceText -notmatch [Regex]::Escape('"archive_sha256":"' + $ArchiveSha256 + '"')) {
        return $false
    }
    $InstalledSha = Get-Sha256Hex $Target
    $RecordedSha = ([Regex]::Match($ProvenanceText, '"binary_sha256":"([^"]+)"')).Groups[1].Value
    return $InstalledSha -eq $RecordedSha
}

if (Test-CurrentInstall) {
    Write-Output "vector component unchanged: $Target"
    exit 0
}

New-Item -ItemType Directory -Force -Path $BinDir, $DownloadDir, $ComponentDir, $ProvenanceDir, $LockRoot | Out-Null
try {
    New-Item -ItemType Directory -Path $LockDir -ErrorAction Stop | Out-Null
} catch {
    Write-Error "install_vector_component: another verified Vector installation is active"
    exit 75
}

$Stage = $null
try {
    if (!(Test-Path $ArchivePath -PathType Leaf) -or (Get-Sha256Hex $ArchivePath) -ne $ArchiveSha256) {
        if (Test-Path $ArchivePath) {
            Remove-Item -Force $ArchivePath
        }
        Invoke-WebRequest -Uri $Url -OutFile $ArchivePath
    }
    if ((Get-Sha256Hex $ArchivePath) -ne $ArchiveSha256) {
        Write-Error "install_vector_component: checksum mismatch for $Archive"
        exit 1
    }

    $Stage = Join-Path $ComponentDir ("install." + [Guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Force -Path $Stage | Out-Null
    Expand-Archive -Path $ArchivePath -DestinationPath $Stage -Force
    $Source = Get-ChildItem -Path $Stage -Recurse -Filter "vector.exe" | Select-Object -First 1
    if ($null -eq $Source) {
        Write-Error "install_vector_component: archive does not contain vector.exe"
        exit 1
    }

    $NewBinary = Join-Path $BinDir (".vector." + [Guid]::NewGuid().ToString("N") + ".exe")
    Copy-Item -Path $Source.FullName -Destination $NewBinary
    $VersionText = & $NewBinary --version
    if ($LASTEXITCODE -ne 0 -or $VersionText -notmatch "vector $Version") {
        Write-Error "install_vector_component: installed vector.exe did not report version $Version"
        exit 1
    }
    $BinarySha256 = Get-Sha256Hex $NewBinary
    Move-Item -Force -Path $NewBinary -Destination $Target

    $LicenseSource = Get-ChildItem -Path $Stage -Recurse -Filter "LICENSE" | Select-Object -First 1
    if ($null -ne $LicenseSource) {
        $ShareDir = Join-Path $InstallRoot "share\vector"
        New-Item -ItemType Directory -Force -Path $ShareDir | Out-Null
        Copy-Item -Force -Path $LicenseSource.FullName -Destination (Join-Path $ShareDir "LICENSE")
    }

    $ProvenanceRecord = [ordered]@{
        schema = "adl.component.provenance.v1"
        component = "vector"
        version = $Version
        platform = "Windows-x86_64"
        archive = $Archive
        archive_sha256 = $ArchiveSha256
        binary_sha256 = $BinarySha256
        source = $Url
        license = "MPL-2.0"
        installed_ref = ".adl/bin/vector.exe"
    }
    $ProvenanceRecord | ConvertTo-Json -Compress | Set-Content -NoNewline -Path $Provenance
    Add-Content -Path $Provenance -Value ""
    Write-Output "vector component installed: $Target"
} finally {
    if ($null -ne $Stage -and (Test-Path $Stage)) {
        Remove-Item -Recurse -Force $Stage
    }
    if (Test-Path $LockDir) {
        Remove-Item -Force -Recurse $LockDir
    }
}
