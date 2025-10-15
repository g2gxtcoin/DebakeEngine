@echo off
FOR /F "eol=; tokens=2 delims==" %%i IN ( 'findstr /i "PackName" config.ini' ) DO set PackName= %%i
cd ..\..\debug
cd %PackName%
if not exist Cargo.toml (@echo Cargo.toml not find
goto endbat ) else (cargo build)
cargo test --quiet
:endbat
pause
exit