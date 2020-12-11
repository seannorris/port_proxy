# CS Home Folder Remote Mounter
The scripts in this folder can be used to mount your home folder in windows while outside of the St Andrews uni network.

To use this utility follow the following steps:
1. Run the following commands from an elevated command prompt:
   ```
   sc stop LanmanServer
   sc config LanmanServer start= disabled
   ```
2. Restart your computer.
3. Edit the variables on lines 2 and 3 of connectHomeCS.bat to be your cs username and ssh address (e.g. user.host.cs.st-andrews.ac.uk).
4. Run connectHomeCS.bat.
5. Click yes when the UAC prompt appears.
6. Minimise the connected SSH window and leave it running in the background.
7. After 10s access your home directory at H:

If the SSH connection drops, the script will run again; repeat steps 5-7.

To run the script automatically on startup, add a shortcut to connectHomeCS.bat to %AppData%\Microsoft\Windows\Start Menu\Programs\Startup

If you don't want to trust the pre-built executable, you can compile it yourself [here](../src/).
