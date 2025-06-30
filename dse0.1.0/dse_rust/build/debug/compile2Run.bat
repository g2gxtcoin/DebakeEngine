@echo off
cd ..\..\debug
@echo build folder name
dir
set /p folder_name=
cd %folder_name%
cargo build --help
@echo build Mode
dir
set /p BuildType=
cargo run --help
@echo Run Mode
set /p RunMode=
if not exist Cargo.toml (@echo Cargo.toml not find
goto endbat ) else (cargo build %BuildType%)
cargo run %RunMode%
:endbat
pause
exit

:another
@echo compile quiet?Y/N
set /p AksParameter1=
if %AksParameter1%==y (if not exist Cargo.toml (@echo Cargo.toml not find
goto endbat ) else (cargo build --quiet)
cargo run --quiet
goto endbat )