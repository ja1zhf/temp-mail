New-Item -Path 'C:\Program Files\tmail' -ItemType Directory
Invoke-WebRequest "https://github.com/Ja1z666/temp-mail/releases/download/1.0.0/tmail.exe" -OutFile "C:\Program Files\tmail\tmail.exe"
$env:Path += ';C:\Program Files\tmail'
