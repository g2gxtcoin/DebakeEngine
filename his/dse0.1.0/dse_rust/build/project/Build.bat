@echo off
FOR /F "eol=; tokens=2 delims==" %%i IN ( 'findstr /i "BuildName" config.ini' ) DO set BuildName= %%i
@echo on
cd ..\..\debug
@echo off
@echo please name your cargo Build folder name
dir
set /p BuildName=
cd %BuildName%
cargo build --release --target-dir %cd%..\..\..\release

:endbat
pause
exit