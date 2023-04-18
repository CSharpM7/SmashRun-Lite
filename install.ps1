function Get-TimeStamp {
    
    return "[{0:MM/dd/yy} {0:HH:mm:ss}]" -f (Get-Date)
    
}

$modPath = Get-Item -Path .\modPath.txt | Get-Content -Tail 1

cargo skyline install --install-path $modPath
"$(Get-TimeStamp) Installed plugin!"