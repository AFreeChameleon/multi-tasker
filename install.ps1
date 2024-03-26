curl https://afreechameleon.github.io/files/mlt-win.zip -o mlt-win.zip -s
New-Item "$env:USERPROFILE/.multi-tasker/bin" -ItemType Directory -Force
Expand-Archive -Force "$env:USERPROFILE/mlt-win.zip" "$env:USERPROFILE\.multi-tasker\bin"
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.multi-tasker\bin", "User")
$env:PATH += ";$env:USERPROFILE\.multi-tasker\bin"
echo "Install finished, run: 'mlt help' to get started"
