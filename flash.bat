cargo rustc --release --target thumbv6m-none-eabi -- -C link-arg=-Tlink.x 
if %errorlevel% neq 0 exit /b %errorlevel%
arm-none-eabi-objcopy.exe -I elf32-littlearm -O srec ./target/thumbv6m-none-eabi/release/t4-rs ./target/thumbv6m-none-eabi/release/t4-rs.srec
ST-LINK_CLI.exe -c UR -P "./target/thumbv6m-none-eabi/release/t4-rs.srec" -V -Rst