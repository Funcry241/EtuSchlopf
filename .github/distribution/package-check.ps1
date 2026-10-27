param(
  [Parameter(Mandatory=$true)][string]$Exe,
  [Parameter(Mandatory=$true)][ValidateSet('Missing','Present')][string]$Renderer,
  [ValidateSet('synthetic','menuet')][string]$Case = 'synthetic'
)
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$Exe = (Resolve-Path $Exe).Path
$tempRoot = if ($env:RUNNER_TEMP) { $env:RUNNER_TEMP } else { [IO.Path]::GetTempPath() }
$root = Join-Path $tempRoot ('etuschlopf-real-package-' + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory $root | Out-Null
$fixtureRoot = Join-Path $PSScriptRoot 'fixtures'
$fixture = @(Get-Content (Join-Path $fixtureRoot 'manifest.json') -Raw | ConvertFrom-Json | Where-Object name -eq $Case)
if ($fixture.Count -ne 1) { throw 'Unknown fixture identity' }
$stem = if ($Case -eq 'synthetic') { 'Example Ü' } else { 'Menuet BWV Anh 114' }
$midi = Join-Path $root "$stem.midi"
$wav = Join-Path $root "$stem.wav"
Copy-Item (Join-Path $fixtureRoot "$Case.mid") $midi
$compressed = [IO.File]::OpenRead((Join-Path $fixtureRoot "$Case.wav.gz"))
$decompress = [IO.Compression.GZipStream]::new($compressed,[IO.Compression.CompressionMode]::Decompress)
$outputAudio = [IO.File]::Create($wav)
try { $decompress.CopyTo($outputAudio) } finally { $outputAudio.Dispose(); $decompress.Dispose(); $compressed.Dispose() }
if ((Get-FileHash $midi -Algorithm SHA256).Hash.ToLowerInvariant() -ne $fixture[0].midi_sha256) { throw 'Fixture MIDI hash mismatch' }
if ((Get-FileHash $wav -Algorithm SHA256).Hash.ToLowerInvariant() -ne $fixture[0].wav_sha256) { throw 'Audible WAV hash mismatch' }
$pcm = [IO.File]::ReadAllBytes($wav)
$energy = 0.0; $frames = 0
for ($i = 44; $i + 1 -lt $pcm.Length; $i += 2) { $sample = [BitConverter]::ToInt16($pcm,$i) / 32768.0; $energy += $sample*$sample; $frames++ }
if ($frames -eq 0 -or [Math]::Sqrt($energy/$frames) -lt 0.01) { throw 'Silent/invalid WAV cannot count as successful package input' }
function Hash-Tree([string]$Directory) {
  $map = @{}
  if (Test-Path $Directory) {
    Get-ChildItem $Directory -File -Recurse | ForEach-Object { $map[$_.FullName] = (Get-FileHash $_.FullName -Algorithm SHA256).Hash }
  }
  return $map
}
function Assert-Preserved($Before) {
  foreach ($path in $Before.Keys) {
    if (-not (Test-Path $path -PathType Leaf) -or (Get-FileHash $path -Algorithm SHA256).Hash -ne $Before[$path]) { throw "Earlier result changed: $path" }
  }
}
$outputRoot = Join-Path $root 'EtuSchlopf-Ergebnisse'
$originalMidi = [IO.File]::ReadAllBytes($midi)
$sourceHashes = @{ $midi = (Get-FileHash $midi -Algorithm SHA256).Hash; $wav = (Get-FileHash $wav -Algorithm SHA256).Hash }
$hint = "MuseScore ist nicht installiert. Die Notendateien werden trotzdem erstellt.`nFür eine PDF installiere MuseScore kostenlos: https://musescore.org/de/download`n`n"
function Successful-Run([bool]$NoPdf,[string]$Label) {
  $before = Hash-Tree $outputRoot
  $oldDirs = @(Get-ChildItem $outputRoot -Directory -ErrorAction SilentlyContinue | ForEach-Object FullName)
  $arguments = @($midi)
  if ($NoPdf) { $arguments += '--no-pdf' }
  & $Exe @arguments *> (Join-Path $root "$Label.log")
  $exitCode = $LASTEXITCODE
  if ($exitCode -ne 0) { throw "Real packaged run $Label failed: $exitCode" }
  Assert-Preserved $before
  Assert-Preserved $sourceHashes
  $newDirs = @(Get-ChildItem $outputRoot -Directory | Where-Object { $_.FullName -notin $oldDirs })
  if ($newDirs.Count -ne 1) { throw 'Each run must create exactly one fresh result directory' }
  $run = $newDirs[0].FullName
  if (Test-Path (Join-Path $run 'target')) { throw 'Technical intermediate files leaked into the musician result folder' }
  foreach ($extension in @('musicxml','render.musicxml','result.txt')) {
    $path = Join-Path $run "$stem.$extension"
    if (-not (Test-Path $path -PathType Leaf) -or (Get-Item $path).Length -eq 0) { throw "Missing actual output: $path" }
  }
  $log = (Get-Content (Join-Path $root "$Label.log") -Raw).Replace("`r`n","`n")
  if ($Renderer -eq 'Missing') {
    if (-not $log.Contains($hint)) { throw 'Exact missing-MuseScore startup text absent: this environment does not prove the missing case' }
    if (-not $log.Contains('Öffne danach die Datei')) { throw 'Missing renderer: installation must precede the opening instruction' }
  } elseif ($log.Contains('MuseScore ist nicht installiert.')) { throw 'Present-renderer case reported missing MuseScore' }
  $pdf = Join-Path $run "$stem.pdf"
  if ($Renderer -eq 'Present' -and -not $NoPdf) {
    if (-not (Test-Path $pdf -PathType Leaf) -or (Get-Item $pdf).Length -eq 0) { throw 'Actual MuseScore PDF missing' }
    if (-not $log.Contains('Öffne die PDF')) { throw 'Existing PDF is not offered' }
    if (-not [Text.Encoding]::ASCII.GetString([IO.File]::ReadAllBytes($pdf),0,5).StartsWith('%PDF-')) { throw 'Output is not a PDF' }
  } else {
    if (Test-Path $pdf) { throw 'No-PDF run produced an unexpected PDF' }
    if ($log.Contains('Öffne die PDF')) { throw 'Absent PDF is offered to the user' }
  }
  $report = Get-Content (Join-Path $run "$stem.result.txt") -Raw
  if ($report -notmatch '(?m)^Ergebnis: OK\s*$') { throw 'Exit 0 without corresponding OK report' }
}
$originalPath = $env:PATH
try {
  $env:PATH = "$env:windir\System32;$env:windir"
  Successful-Run $false 'default'
  Successful-Run $false 'repeat'
  Successful-Run $true 'explicit-no-pdf'
  $priorResults = Hash-Tree $outputRoot
  try {
    [IO.File]::WriteAllBytes($midi,[Text.Encoding]::ASCII.GetBytes('synthetic invalid MIDI'))
    & $Exe $midi *> (Join-Path $root 'intentional-failure.log')
    if ($LASTEXITCODE -eq 0) { throw 'Invalid MIDI was reported as successful' }
    Assert-Preserved $priorResults
  } finally { [IO.File]::WriteAllBytes($midi,$originalMidi) }
  Assert-Preserved $sourceHashes
 } finally { $env:PATH = $originalPath }
$proof = [ordered]@{ Package=(Get-FileHash $Exe -Algorithm SHA256).Hash; RendererEnvironment=$Renderer; Fixture=$Case; WavRms=[Math]::Sqrt($energy/$frames); DefaultExit=0; RepeatExit=0; ExplicitNoPdfExit=0; IntentionalFailureRejected=$true; EarlierResultsPreserved=$true; OriginalInputsPreserved=$true; ActualPdfRequired=($Renderer -eq 'Present'); Logs=$root }
$proof | ConvertTo-Json | Set-Content (Join-Path $root 'package-proof.json') -Encoding utf8
Write-Output ("Package checks passed for {0}; evidence: {1}" -f $Renderer,$root)
$global:LASTEXITCODE = 0
