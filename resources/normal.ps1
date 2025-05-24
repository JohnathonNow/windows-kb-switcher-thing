Enable-Display -DisplayId 1,2
Disable-Display -DisplayId 3
$steamProcess = Get-Process -Name "Steam" -ErrorAction SilentlyContinue
if ($steamProcess) {
    Stop-Process -Id $steamProcess.Id -Force
}
$audioDevice = Get-AudioDevice -List | Where-Object { $_.Name -like "*Headphones*" }
if ($audioDevice) {
    Set-AudioDevice -Id $audioDevice.Id
} else {
    Write-Host "Headphone audio device not found."
}