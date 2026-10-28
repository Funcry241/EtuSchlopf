param([Parameter(Mandatory=$true)][string]$Exe, [Parameter(Mandatory=$true)][string]$Inputs, [Parameter(Mandatory=$true)][string]$Evidence)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force $Evidence | Out-Null
$midi = Join-Path $Inputs 'Beispiel.mid'
$wav = Join-Path $Inputs 'Beispiel.wav'
$identity = Get-Content (Join-Path $Inputs 'input-manifest.json') -Raw | ConvertFrom-Json
$midiHash = $identity.midi_sha256
$wavHash = $identity.wav_sha256
if ((Get-FileHash $midi -Algorithm SHA256).Hash.ToLowerInvariant() -ne $midiHash) { throw 'MIDI input differs from the generated synthetic fixture' }
if ((Get-FileHash $wav -Algorithm SHA256).Hash.ToLowerInvariant() -ne $wavHash) { throw 'WAV input differs from the generated synthetic fixture' }
$version = (& $Exe --version | Out-String)
if ($LASTEXITCODE -ne 0 -or $version -notmatch 'r1537') { throw 'Download revision is not r1537' }
$version | Set-Content (Join-Path $Evidence 'version.txt')
$originalPath = $env:PATH
try {
    $env:PATH = "$env:windir\System32;$env:windir"
    $process = Start-Process -FilePath $Exe -ArgumentList ('"' + $midi + '"') -PassThru -RedirectStandardOutput (Join-Path $Evidence 'console.txt') -RedirectStandardError (Join-Path $Evidence 'console-errors.txt')
    if (-not $process.WaitForExit(900000)) {
        taskkill.exe /PID $process.Id /T /F | Out-Null
        throw 'Example processing timed out after 15 minutes'
    }
    $process.WaitForExit()
    $exitCode = $process.ExitCode
} finally { $env:PATH = $originalPath }
Get-Content (Join-Path $Evidence 'console.txt') | Select-Object -Last 20
Get-Content (Join-Path $Evidence 'console-errors.txt') | Select-Object -Last 20
$folders = @(Get-ChildItem (Join-Path $Inputs 'EtuSchlopf-Ergebnisse') -Directory)
if ($folders.Count -ne 1) { throw 'Expected one fresh result workspace' }
$result = $folders[0].FullName
Copy-Item (Join-Path $result '*') $Evidence -Recurse -Force
if ($exitCode -ne 0) {
    Get-Content (Join-Path $result 'Laufprotokoll.txt') | Select-Object -Last 40
    throw "Packaged example failed with exit $exitCode"
}
foreach ($name in @('Beispiel.pdf','Beispiel.musicxml','Beispiel.render.musicxml','Beispiel.result.txt','Laufprotokoll.txt')) {
    $path = Join-Path $result $name
    if (-not (Test-Path $path) -or (Get-Item $path).Length -eq 0) { throw "Missing output: $name" }
}
[xml]$master = Get-Content (Join-Path $result 'Beispiel.musicxml') -Raw
[xml]$render = Get-Content (Join-Path $result 'Beispiel.render.musicxml') -Raw
$report = Get-Content (Join-Path $result 'Beispiel.result.txt') -Raw
if ($report -notmatch '(?m)^Ergebnis: OK\s*$') { throw 'Report does not confirm OK' }
if ($report -notmatch 'WavObservation: Qualified ja;') { throw 'WAV observation did not qualify in the packaged runtime' }
if ($report -notmatch $wavHash) { throw 'Report does not bind the generated WAV' }
if ($report -match 'Reason wav_file_unreadable') { throw 'The packaged WAV path is still broken' }
$console = Get-Content (Join-Path $Evidence 'console.txt') -Raw
if ($console.Contains([string][char]27)) { throw 'ANSI control bytes leaked to the user console' }
if ((Get-FileHash $midi -Algorithm SHA256).Hash.ToLowerInvariant() -ne $midiHash) { throw 'Original MIDI changed' }
if ((Get-FileHash $wav -Algorithm SHA256).Hash.ToLowerInvariant() -ne $wavHash) { throw 'Original WAV changed' }
"Engine r1537; packaged synthetic example exit 0; report OK; both XMLs parse; PDF exists; real WAV qualified; inputs unchanged; user console without ANSI bytes." | Set-Content (Join-Path $Evidence 'example-verdict.txt')
