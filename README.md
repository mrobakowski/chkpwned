ChkPwned
===
A little command line utility to check if your password was compromised. You must provide the text file with sorted
password hashes. The (archived) file can be downloaded from 
https://downloads.pwnedpasswords.com/passwords/pwned-passwords-ordered-2.0.txt.7z

Building & Running
-

This application compiles on Rust stable (checked on version 1.24.1).
```bash
$ cargo build --release # produces chkpwned[.exe] in target/release
$ ./target/release/chkpasswd path/to/haveIBeenPwned/database/file.txt
enter password to check> ******
Searching...
██████████████████████████████████████████████████████████████████ 29/28
The password you entered was found 11063 times in various leaks

$ ./target/release/chkpasswd path/to/haveIBeenPwned/database/file.txt
enter password to check> **********************
Searching...
██████████████████████████████████████████████████████████████████ 29/29
The password you entered wasn't found! Lucky you!

$ # The 29/29 number can seem a bit weird, but the database contains 501m entries and we're using binary search
$ # which runs in O(log N). log2(501m) is approx. 29
```
