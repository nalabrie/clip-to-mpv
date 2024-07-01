@echo off

@REM dir of user's bin directory
@REM ! change this to your bin directory
set BIN_DIR=%USERPROFILE%\bin

@REM copy release binary to user's bin directory
echo Copying release binary to %BIN_DIR%
copy target\release\clip-to-mpv.exe %BIN_DIR%\clip-to-mpv.exe
echo Done.
