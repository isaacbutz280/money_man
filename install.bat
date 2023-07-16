@echo off

REM Build the Rust application
cargo build --release

REM Set the name of the shortcut
set shortcutName=MoneyMan

REM Set the path to the target .exe file
set targetPath=%CD%\target\release\gui.exe

REM Set the path to the desktop folder
set desktopPath=%USERPROFILE%\Desktop

REM Set the path for the shortcut file
set shortcutPath=%desktopPath%\%shortcutName%.lnk

REM Set the path to the icon file
set iconPath=%cd%\docs\vault.ico

REM Create the ButzIndustries directory if it doesn't exist
mkdir "C:\Users\%USERNAME%\AppData\Local\ButzIndustries\"
mkdir "C:\Users\%USERNAME%\AppData\Local\ButzIndustries\MoneyMan"

REM Move the executable to the desired location
copy "%targetPath%" "C:\\Users\\%USERNAME%\\AppData\\Local\\ButzIndustries\\MoneyMan\\gui.exe"
copy "%iconPath%" "C:\\Users\\%USERNAME%\\AppData\\Local\\ButzIndustries\\MoneyMan\\vault.ico"

REM Create a VBScript file to create the shortcut
echo Set oWS = WScript.CreateObject("WScript.Shell") > CreateShortcut.vbs
echo sLinkFile = "%shortcutPath%" >> CreateShortcut.vbs
echo Set oLink = oWS.CreateShortcut(sLinkFile) >> CreateShortcut.vbs
echo oLink.TargetPath = "C:\\Users\\%USERNAME%\\AppData\\Local\\ButzIndustries\\MoneyMan\\gui.exe" >> CreateShortcut.vbs
echo oLink.IconLocation = "C:\\Users\\%USERNAME%\\AppData\\Local\\ButzIndustries\\MoneyMan\\vault.ico" >> CreateShortcut.vbs
echo oLink.Save >> CreateShortcut.vbs

REM Execute the VBScript to create the shortcut
cscript //NoLogo CreateShortcut.vbs

REM Delete the temporary VBScript file
del CreateShortcut.vbs

echo Desktop shortcut created successfully!

pause
