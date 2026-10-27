param([Parameter(Mandatory=$true)][string]$Exe)
$ErrorActionPreference = 'Stop'
& (Join-Path $PSScriptRoot 'package-check.ps1') -Exe $Exe -Renderer Missing -Case synthetic
if ($LASTEXITCODE -ne 0) { throw 'Audible synthetic packaged run failed' }
