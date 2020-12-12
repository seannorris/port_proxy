@Echo Off
cd %1
powershell /c "start mountSambaFolder -Args \"%2\" -WindowStyle Hidden"
set /p OLD_PID=<%TEMP%\port_proxy_12345_445.lock
taskkill /F /PID %OLD_PID%
port_proxy 12345 445