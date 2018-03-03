ChkPwned
===
A little command line utility to check if your password was compromised. You must provide the text file with sorted
password hashes. The (archived) file can be downloaded from 
https://downloads.pwnedpasswords.com/passwords/pwned-passwords-ordered-2.0.txt.7z

Building & Running
-

This application compiles on Rust stable (checked on version 1.24.1).
```bash
cargo build --release # produces chkpwned[.exe] in target/release
./target/release/chkpasswd path/to/haveIBeenPwned/database/file.txt
```
