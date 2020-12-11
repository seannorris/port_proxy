# CS Home Folder Remote Mounter
The scripts in this folder can be used to mount your home folder in windows while outside of the St Andrews uni network.

To use this utility unzip the file ['Mount CS Home Folder Remote.zip'](./Mount%20CS%20Home%20Folder%20Remote.zip?raw=true) somewhere and then follow the following steps:
1. Run the following commands from an elevated command prompt:
   ```
   sc stop LanmanServer
   sc config LanmanServer start= disabled
   ```
2. Restart your computer.
3. Edit the variables on lines 2 and 3 of connectHomeCS.bat to be your cs username and ssh address (e.g. user.host.cs.st-andrews.ac.uk).
4. Run connectHomeCS.bat.
5. Click yes when the UAC prompt appears.
6. Minimise the connected SSH window and leave it running in the background *(You can press win+tab and drag it into another desktop; this will hide it from view)*.
7. After 10s access your home directory at H:

*Note: Steps 1-3 are only required once.*

If the SSH connection drops, the script will run again; repeat steps 5-7.

To run the script automatically on startup, add a shortcut to connectHomeCS.bat to ```%AppData%\Microsoft\Windows\Start Menu\Programs\Startup```

If you don't want to trust the pre-built executable, you can compile it yourself [here](../src/).

### **If this is your first time mounting your home folder, you will need to follow these additional steps after step 6 above:**
1. Run the following command (from a non-elevated command prompt):
   ```
   net use h: /delete
   ```
   (The result of this command doesn't matter).
2. Navigate to the 'This PC' folder in windows explorer.
3. In the 'Computer' tab at the top of the screen click 'Map network drive'.
4. Select 'H:' from the dropdown list.
5. For folder, enter ```\\localhost\<your cs username>```.
6. Unselect 'Reconnect at sign-in'.
7. Select 'Connect using different credentials'.
8. Click 'Finish'.
9. For Username, enter ```CSAD\<your cs username>```.
10. For Password, enter your cs password.
11. Tick 'Remember my credentials'.
12. Click Ok.
13. Repeat steps 10-12.
14. *(Optional)* Stop and re-run ConnectHomeCS.bat.

*Note: These steps are only required once.*
