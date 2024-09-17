@echo off
cargo build
Rem Build the project using Cargo two times because sometimes the exe is not copied to the target folder
cargo build
timeout 1
site.exe