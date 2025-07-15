@echo off
FOR /F "eol=; tokens=2 delims==" %%i IN ( 'findstr /i "PackName" config.ini' ) DO set PackName= %%i
cd ..\..\debug
if not exist Cargo.toml (@echo Cargo.toml not find
goto endbat ) else (cargo objdump  --release -- --disassemble --source --traceback-table --x86-asm-syntax=intel> target\assembly\dse_objdump_analyze.txt
cargo-nm --lib > target\assembly\dse_nm_analyze.txt
cargo size  --release -- -A -x > target\assembly\dse_size_analyze.txt
)
:endbat
pause
exit