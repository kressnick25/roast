$target = 'x86_64-pc-windows-msvc'
$package = 'roast-Windows-x86_64.zip'

cargo build --locked --release --target $target

7z a $package "$(Get-Location)\target\$target\release\roast.exe"

$hash = $(Get-FileHash -Path .\$package -Algorithm SHA256)
Write-Output "---- SHA256 hash of $package ----"
Write-Output $hash.Hash

$hash.Hash | Out-File "$package.sha256"
