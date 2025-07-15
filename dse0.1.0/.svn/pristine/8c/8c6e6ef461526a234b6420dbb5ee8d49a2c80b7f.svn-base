@echo off
FOR /F "eol=; tokens=2 delims==" %%i IN ( 'findstr /i "PackName" config.ini' ) DO set PackName= %%i
@echo on
cd ..\..\debug
@echo please name your newPack name
@echo off
dir
set /p PackName=
cargo new --help
@echo please choose your Pack Type
set /p PackType=
cargo new %PackName% %PackType%
pause
exit