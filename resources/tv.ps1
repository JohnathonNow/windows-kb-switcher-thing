Enable-Display -DisplayId 3
Disable-Display -DisplayId 1,2
Start-Process -FilePath "C:\Program Files (x86)\Steam\Steam.exe" -ArgumentList "-bigpicture" -WorkingDirectory "C:\Program Files (x86)\Steam"
# Set audio output to TV
$audioDevice = Get-AudioDevice -List | Where-Object { $_.Name -like "*TV*" }
if ($audioDevice) {
    Set-AudioDevice -Id $audioDevice.Id
} else {
    Write-Host "TV audio device not found."
}