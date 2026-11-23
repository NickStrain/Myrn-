# Install myrn on Windows (x64 or ARM64):
#
#   irm https://github.com/NickStrain/Myrn-/releases/latest/download/install.ps1 | iex
#
# Downloads the prebuilt binary from the GitHub release, checks it against the
# release's sha256sums.txt, puts it in %LOCALAPPDATA%\Programs\myrn and adds
# that folder to your user PATH. No administrator rights needed.
#
# Environment:
#   MYRN_VERSION        a version such as 0.1.0 (default: the latest release)
#   MYRN_INSTALL_DIR    where to put myrn.exe
#   MYRN_DOWNLOAD_BASE  download from here instead of the GitHub release

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
$Repo = 'NickStrain/Myrn-'

function Say($msg) { Write-Host "myrn: $msg" }

$arch = $env:PROCESSOR_ARCHITEW6432
if (-not $arch) { $arch = $env:PROCESSOR_ARCHITECTURE }
switch ($arch) {
    'AMD64' { $target = 'x86_64-pc-windows-msvc' }
    'ARM64' { $target = 'aarch64-pc-windows-msvc' }
    default { throw "myrn: no prebuilt binary for '$arch'; build from source: cargo install --git https://github.com/$Repo myrn" }
}
$archive = "myrn-$target.zip"

if ($env:MYRN_DOWNLOAD_BASE) {
    $base = $env:MYRN_DOWNLOAD_BASE.TrimEnd('/')
} elseif ($env:MYRN_VERSION) {
    $base = "https://github.com/$Repo/releases/download/v$($env:MYRN_VERSION.TrimStart('v'))"
} else {
    $base = "https://github.com/$Repo/releases/latest/download"
}
$dir = $env:MYRN_INSTALL_DIR
if (-not $dir) { $dir = Join-Path $env:LOCALAPPDATA 'Programs\myrn' }

$tmp = Join-Path ([IO.Path]::GetTempPath()) ("myrn-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $tmp | Out-Null
try {
    Say "downloading $archive"
    Invoke-WebRequest -UseBasicParsing -Uri "$base/$archive" -OutFile (Join-Path $tmp $archive)
    Invoke-WebRequest -UseBasicParsing -Uri "$base/sha256sums.txt" -OutFile (Join-Path $tmp 'sha256sums.txt')

    $want = $null
    foreach ($line in Get-Content (Join-Path $tmp 'sha256sums.txt')) {
        $parts = $line.Trim() -split '\s+'
        if ($parts.Count -eq 2 -and $parts[1].TrimStart('*') -eq $archive) { $want = $parts[0].ToLower() }
    }
    if (-not $want) { throw "myrn: $archive is not listed in sha256sums.txt" }
    $got = (Get-FileHash -Algorithm SHA256 (Join-Path $tmp $archive)).Hash.ToLower()
    if ($got -ne $want) { throw "myrn: checksum mismatch for $archive (got $got, expected $want)" }

    $unpacked = Join-Path $tmp 'unpacked'
    Expand-Archive -LiteralPath (Join-Path $tmp $archive) -DestinationPath $unpacked -Force
    $exe = Join-Path $unpacked 'myrn.exe'
    if (-not (Test-Path $exe)) { throw "myrn: $archive does not contain myrn.exe" }

    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $dest = Join-Path $dir 'myrn.exe'
    # A running myrn.exe (a hook, the MCP server) cannot be overwritten, but it can be renamed.
    if (Test-Path $dest) {
        $old = "$dest.old"
        Remove-Item -Force $old -ErrorAction SilentlyContinue
        Move-Item -Force $dest $old
    }
    Copy-Item $exe $dest
    Remove-Item -Force "$dest.old" -ErrorAction SilentlyContinue
} finally {
    Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}

$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if (-not (($userPath -split ';') -contains $dir)) {
    $newPath = (@($userPath, $dir) | Where-Object { $_ }) -join ';'
    [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
    $env:Path = "$env:Path;$dir"
    Say "added $dir to your user PATH (open a new terminal to pick it up)"
}
Say "installed $(& (Join-Path $dir 'myrn.exe') --version) to $dir"
Say 'next, in a Git repository: myrn init; myrn connect --all'
