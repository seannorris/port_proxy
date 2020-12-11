@Echo Off
SET USER_CONNECT_CS_HOME=<user>
SET SSH_HOST=<ssh address>
echo User: %USER_CONNECT_CS_HOME%
:top
powershell /c "start forwardSambaPort -Args \"%CD% %USER_CONNECT_CS_HOME%\" -WindowStyle Hidden -v runAs"
powershell /c "start mountSambaFolder -Args \"%USER_CONNECT_CS_HOME%\" -WindowStyle Hidden"
ssh -CL 12345:%USER_CONNECT_CS_HOME%.home:445 %SSH_HOST%
goto top
exit