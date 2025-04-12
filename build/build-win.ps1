$target = 'x86_64-pc-windows-msvc'
$package = 'roast-Windows-x86_64.zip'

cargo build --locked --release --target $target

7z a $package target\$target\release\*

$hash = $(Get-FileHash -Path $package -Algorithm SHA256)
Write-Output "---- SHA256 hash of $package ----"
Write-Output $h.Hash

$h.Hash > "$package.sha256"
