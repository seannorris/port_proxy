@Echo Off
timeout /t 7
net use p: /delete
net use p: \\localhost\private
net use h: /delete
net use h: \\localhost\%1
exit