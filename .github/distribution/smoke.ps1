param([Parameter(Mandatory=$true)][string]$Exe)
$ErrorActionPreference = 'Stop'
$root = Join-Path $env:RUNNER_TEMP ('etuschlopf-package-probe-' + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory $root | Out-Null
$midi = Join-Path $root 'Example Ü.midi'
$wav = Join-Path $root 'Example Ü.wav'
[IO.File]::WriteAllBytes($midi, [Convert]::FromBase64String('TVRoZAAAAAYAAAABAGBNVHJrAAACEwD/UQMHoSAA/1gEBAIYCACQMEAAkDxAYIAwAACAPAAAkDVAAJBAQGCANQAAgEAAAJA3QACQQ0BggDcAAIBDAACQMEAAkEBAYIAwAACAQAAAkDBAAJA8QGCAMAAAgDwAAJA1QACQQEBggDUAAIBAAACQN0AAkENAYIA3AACAQwAAkDBAAJBAQGCAMAAAgEAAAJAwQACQPEBggDAAAIA8AACQNUAAkEBAYIA1AACAQAAAkDdAAJBDQGCANwAAgEMAAJAwQACQQEBggDAAAIBAAACQMEAAkDxAYIAwAACAPAAAkDVAAJBAQGCANQAAgEAAAJA3QACQQ0BggDcAAIBDAACQMEAAkEBAYIAwAACAQAAAkDBAAJA8QGCAMAAAgDwAAJA1QACQQEBggDUAAIBAAACQN0AAkENAYIA3AACAQwAAkDBAAJBAQGCAMAAAgEAAAJAwQACQPEBggDAAAIA8AACQNUAAkEBAYIA1AACAQAAAkDdAAJBDQGCANwAAgEMAAJAwQACQQEBggDAAAIBAAACQMEAAkDxAYIAwAACAPAAAkDVAAJBAQGCANQAAgEAAAJA3QACQQ0BggDcAAIBDAACQMEAAkEBAYIAwAACAQAAAkDBAAJA8QGCAMAAAgDwAAJA1QACQQEBggDUAAIBAAACQN0AAkENAYIA3AACAQwAAkDBAAJBAQGCAMAAAgEAAAP8vAA=='))
$stream = [IO.MemoryStream]::new()
$writer = [IO.BinaryWriter]::new($stream)
$writer.Write([Text.Encoding]::ASCII.GetBytes('RIFF')); $writer.Write([uint32]256036)
$writer.Write([Text.Encoding]::ASCII.GetBytes('WAVEfmt ')); $writer.Write([uint32]16)
$writer.Write([uint16]1); $writer.Write([uint16]1); $writer.Write([uint32]8000)
$writer.Write([uint32]16000); $writer.Write([uint16]2); $writer.Write([uint16]16)
$writer.Write([Text.Encoding]::ASCII.GetBytes('data')); $writer.Write([uint32]256000)
$writer.Write([byte[]]::new(256000)); $writer.Flush()
[IO.File]::WriteAllBytes($wav,$stream.ToArray()); $writer.Dispose(); $stream.Dispose()
$beforeMidi = (Get-FileHash $midi -Algorithm SHA256).Hash
$beforeWav = (Get-FileHash $wav -Algorithm SHA256).Hash
$originalPath = $env:PATH
try {
  $env:PATH = "$env:windir\System32;$env:windir"
  & $Exe $midi --no-pdf *> "$root/probe.log"
  $exitCode = $LASTEXITCODE
} finally { $env:PATH = $originalPath }
if ((Get-FileHash $midi -Algorithm SHA256).Hash -ne $beforeMidi) { throw 'Original MIDI changed' }
if ((Get-FileHash $wav -Algorithm SHA256).Hash -ne $beforeWav) { throw 'Original WAV changed' }
$log = Get-Content "$root/probe.log" -Raw
Write-Output $log
if ($log -notmatch 'MIDI produktiv;') { throw 'Packaged engine did not reach productive MIDI processing' }
if ($log -match 'konnte EtuSchlopfs gebundene MusicXML') { throw 'Required runtime schema data unavailable' }
if ($exitCode -ne 0 -and $log -notmatch 'Aktiver menschlicher Sichtpruefungs-/Reviewvertrag') { throw "Unexpected pipeline failure: $exitCode" }
if ($exitCode -ne 0) { Write-Warning 'Synthetic piece was rejected by the existing report gate. This is not a successful musical acceptance.' }
$folders = @(Get-ChildItem "$root/EtuSchlopf-Ergebnisse" -Directory)
if ($folders.Count -ne 1) { throw 'Expected exactly one isolated result workspace' }
if (-not (Test-Path (Join-Path $folders[0].FullName 'Example Ü.mid'))) { throw 'Internal MIDI copy missing' }
if (-not (Test-Path (Join-Path $folders[0].FullName 'Example Ü.wav'))) { throw 'Internal WAV copy missing' }
Write-Output 'Packaged engine reached processing without developer PATH; original source hashes preserved.'
