curl https://afreechameleon.github.io/files/mlt-win.zip -o mlt-win.zip | Out-Null
New-Item "$env:USERPROFILE\.multi-tasker\bin" -ItemType Directory -Force | Out-Null
Expand-Archive -Force "$env:USERPROFILE\mlt-win.zip" "$env:USERPROFILE\.multi-tasker\bin"
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.multi-tasker\bin", "User")
$env:PATH += ";$env:USERPROFILE\.multi-tasker\bin"
Remove-Item "$env:USERPROFILE\mlt-win.zip"
echo "Install finished, run: 'mlt help' to get started"
