param([Parameter(Mandatory)][string]$Exe, [Parameter(Mandatory)][string]$Output)
$ErrorActionPreference = 'Stop'
$root = Join-Path $env:RUNNER_TEMP 'etuschlopf-start-example'
New-Item -ItemType Directory -Path $root | Out-Null
$previous = Join-Path $root 'EtuSchlopf-Beispiel-r1537.zip'
gh release download v0.10.1839-r1537-windows-preview --repo $env:GITHUB_REPOSITORY --pattern 'EtuSchlopf-Beispiel-r1537.zip' --output $previous
if ($LASTEXITCODE -ne 0) { throw 'Could not retrieve the public example inputs' }
Expand-Archive $previous (Join-Path $root 'original')
$midi = @(Get-ChildItem (Join-Path $root 'original') -Recurse -File | Where-Object { $_.Extension -in '.mid', '.midi' })
if ($midi.Count -ne 1) { throw 'Expected one example MIDI' }
$wav = Join-Path $midi[0].DirectoryName ($midi[0].BaseName + '.wav')
if (!(Test-Path -LiteralPath $wav -PathType Leaf)) { throw 'Example WAV missing' }
$input = New-Item -ItemType Directory -Path (Join-Path $root 'input')
$inputMidi = Join-Path $input.FullName 'Beispiel.mid'
$inputWav = Join-Path $input.FullName 'Beispiel.wav'
Copy-Item -LiteralPath $midi[0].FullName -Destination $inputMidi
Copy-Item -LiteralPath $wav -Destination $inputWav
$before = (Get-FileHash $inputMidi,$inputWav).Hash
$msi = Join-Path $root 'MuseScore.msi'
Invoke-WebRequest 'https://github.com/musescore/MuseScore/releases/download/v4.7.5/MuseScore-Studio-4.7.5.260831071-x86_64.msi' -OutFile $msi
if ((Get-FileHash $msi -Algorithm SHA256).Hash.ToLowerInvariant() -ne '07b0f3292612e2df4c7e0632deb790add51b218f0ee33f7fecaa6111dd279ec6') { throw 'MuseScore MSI checksum mismatch' }
$rendererRoot = Join-Path $root 'renderer'
$extract = Start-Process msiexec.exe -ArgumentList "/a `"$msi`" /qn TARGETDIR=`"$rendererRoot`"" -Wait -PassThru
if ($extract.ExitCode -ne 0) { throw "MuseScore extraction failed ($($extract.ExitCode))" }
$renderers = @(Get-ChildItem $rendererRoot -Recurse -File | Where-Object { $_.Name -in 'MuseScore4.exe','MuseScore.exe','MuseScore Studio.exe','MuseScore Studio 4.exe' })
if ($renderers.Count -ne 1) { throw 'Expected one MuseScore executable' }
$env:ETUSCHLOPF_PDF_RENDERER = $renderers[0].FullName
try {
    & $Exe $inputMidi
    if ($LASTEXITCODE -ne 0) { throw "Windows example failed ($LASTEXITCODE)" }
} finally {
    Remove-Item Env:ETUSCHLOPF_PDF_RENDERER
}
if (Compare-Object $before (Get-FileHash $inputMidi,$inputWav).Hash) { throw 'Example inputs changed' }
$pdf = @(Get-ChildItem (Join-Path $input.FullName 'EtuSchlopf-Ergebnisse') -Recurse -Filter 'Beispiel.pdf')
if ($pdf.Count -ne 1 -or $pdf[0].Length -lt 5) { throw 'Fresh example PDF missing' }
if ([Text.Encoding]::ASCII.GetString([IO.File]::ReadAllBytes($pdf[0].FullName), 0, 5) -ne '%PDF-') { throw 'Invalid example PDF' }
Compress-Archive -LiteralPath $inputMidi,$inputWav,$pdf[0].FullName -DestinationPath $Output
