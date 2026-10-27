$ErrorActionPreference = 'Stop'
$version = '4.6.3.252940956'
$url = "https://github.com/musescore/MuseScore/releases/download/v4.6.3/MuseScore-Studio-$version-x86_64.msi"
$expected = '74d21efe0c01d45f3fcd0616c0cc75c5fb1dbacb5902b85a0169c857ba35eb9b'
$msi = Join-Path $env:RUNNER_TEMP 'musescore-4.6.3.msi'
Invoke-WebRequest $url -OutFile $msi
$actual = (Get-FileHash $msi -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actual -ne $expected) { throw 'Official MuseScore MSI SHA256 mismatch' }
$log = Join-Path $env:RUNNER_TEMP 'musescore-install.log'
$process = Start-Process msiexec.exe -ArgumentList @('/i', ('"'+$msi+'"'), '/qn', '/norestart', '/l*v', ('"'+$log+'"')) -Wait -PassThru
if ($process.ExitCode -notin @(0,3010)) { throw "MuseScore install failed: $($process.ExitCode); log=$log" }
Write-Output "Official MuseScore $version installed; MSI SHA256 $actual. Actual EXE PDF tests must still pass."
