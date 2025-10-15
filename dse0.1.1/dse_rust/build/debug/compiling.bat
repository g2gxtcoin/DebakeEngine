@echo off
cd ..\..\debug
@echo build folder name
dir
set /p folder_name=
cd %folder_name%
cargo build --help
@echo build Mode
set /p BuildType=
if not exist Cargo.toml (@echo Cargo.toml not find
goto endbat ) else (cargo build %BuildType%)

:endbat
pause
exit