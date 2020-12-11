@Echo Off
timeout /t 7
net use h: /delete
net use h: \\localhost\%1
exit