@echo off
cd ..\..\debug
@echo Run folder name
dir
set /p folder_name=
cd %folder_name%
cargo run --help
@echo Run Mode
set /p RunMode=
if not exist Cargo.toml (@echo Cargo.toml not find
goto endbat ) else (cargo run %RunMode%)

:endbat
pause
exit
