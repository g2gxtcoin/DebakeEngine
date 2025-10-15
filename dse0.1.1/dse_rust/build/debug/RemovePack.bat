cd ..\..\debug
@echo please name your want remove
@echo off
dir
set /p PackName=
cd %PackName%
cargo clean --help
@echo please choose your clean mode
set /p CleanMode=
cargo clean %CleanMode%
pause
exit