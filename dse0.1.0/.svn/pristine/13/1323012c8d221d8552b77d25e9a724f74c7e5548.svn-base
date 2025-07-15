@echo off
FOR /F "eol=; tokens=2 delims==" %%i IN ( 'findstr /i "BuildName" config.ini' ) DO set BuildName= %%i
@echo on
cd ..\..\debug
@echo off
XCOPY  %cd%\asset %cd%\..\release /e
:endbat
pause
exit