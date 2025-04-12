cargo build --locked --release --target x86_64-pc-windows-msvc

7z a roast-Windows-x86_64.zip target\x86_64-pc-windows-msvc\release\*
