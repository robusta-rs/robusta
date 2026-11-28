# Installs the `robusta.exe` executable of the release this script is attached to.
#
#   powershell -c "irm https://robusta.rs/install.ps1 | iex"
#
# The executable goes to `$env:ROBUSTA_INSTALL_DIR`, or `~\.local\bin` by default.

# Everything runs inside a function, so that `irm | iex` leaves no variables or
# preferences behind in the calling session.
function Install-Robusta {
  $ErrorActionPreference = 'Stop'
  # The progress bar slows downloads considerably in Windows PowerShell.
  $ProgressPreference = 'SilentlyContinue'

  # Replaced with the release's download URL when the release is published.
  $releaseUrl = '@RELEASE_URL@'

  # A 32-bit PowerShell on a 64-bit Windows reports the OS architecture here.
  $arch = $env:PROCESSOR_ARCHITEW6432
  if (-not $arch) { $arch = $env:PROCESSOR_ARCHITECTURE }
  if ($arch -ne 'AMD64') { throw "unsupported architecture: $arch" }
  $asset = 'robusta-x86_64-pc-windows-msvc.exe'

  if ($env:ROBUSTA_INSTALL_DIR) {
    $installDir = $env:ROBUSTA_INSTALL_DIR
  } else {
    $installDir = Join-Path $HOME '.local\bin'
  }

  # GitHub requires TLS 1.2, which older Windows PowerShell does not enable by default.
  [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12

  $tmp = Join-Path ([IO.Path]::GetTempPath()) ([Guid]::NewGuid())
  New-Item -ItemType Directory -Path $tmp | Out-Null
  try {
    $download = Join-Path $tmp $asset
    Write-Host "Downloading $releaseUrl/$asset"
    Invoke-WebRequest -Uri "$releaseUrl/$asset" -OutFile $download -UseBasicParsing
    Invoke-WebRequest -Uri "$releaseUrl/$asset.sha256" -OutFile "$download.sha256" -UseBasicParsing

    $expected = (Get-Content "$download.sha256" -Raw).Split(' ')[0].Trim().ToLowerInvariant()
    $actual = (Get-FileHash $download -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($expected -ne $actual) {
      throw "checksum mismatch for ${asset}: expected $expected, got $actual"
    }

    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    $executable = Join-Path $installDir 'robusta.exe'
    Move-Item -Path $download -Destination $executable -Force
    Write-Host "Installed robusta to $executable"
  } finally {
    Remove-Item -Path $tmp -Recurse -Force -ErrorAction SilentlyContinue
  }

  $onPath = $env:Path -split ';' | Where-Object { $_.TrimEnd('\') -eq $installDir.TrimEnd('\') }
  if (-not $onPath) {
    Write-Host ""
    Write-Host "$installDir is not on your PATH. To add it, run:"
    Write-Host ""
    Write-Host "  [Environment]::SetEnvironmentVariable('Path', `"$installDir;`" + [Environment]::GetEnvironmentVariable('Path', 'User'), 'User')"
  }
}

Install-Robusta
