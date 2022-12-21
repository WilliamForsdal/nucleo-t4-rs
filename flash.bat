cargo rustc --release --target thumbv6m-none-eabi -- -C link-arg=-Tlink.x 
if %errorlevel% neq 0 exit /b %errorlevel%
arm-none-eabi-objcopy -I elf32-littlearm -O srec ./target/thumbv6m-none-eabi/release/t4-rs ./target/thumbv6m-none-eabi/release/t4-rs.srec
arm-none-eabi-objdump -x --wide --disassemble -l ./target/thumbv6m-none-eabi/release/t4-rs > ./target/thumbv6m-none-eabi/release/bin.srec.objdump
arm-none-eabi-size.exe ./target/thumbv6m-none-eabi/release/t4-rs
ST-LINK_CLI.exe -c UR -P "./target/thumbv6m-none-eabi/release/t4-rs.srec" -V -Rst