extern crate indicatif;
extern crate sha1;
extern crate rpassword;
extern crate memmap;
#[macro_use]
extern crate quicli;
#[macro_use]
extern crate static_assertions;

use quicli::prelude::*;

use std::fs::File;
use sha1::Sha1;
use rpassword::prompt_password_stdout;
use memmap::Mmap;
use indicatif::ProgressBar;

// Add cool slogan for your app here, e.g.:
/// Find if your password was compromised
#[derive(Debug, StructOpt)]
struct Cli {
    /// The text file from https://haveibeenpwned.com/Passwords (must be the sorted one) to use as
    /// the password database
    file: String,
    /// Pass many times for more log output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

main!(|args: Cli, log_level: verbosity| {
    m(args.file)?;
});

// This is how one line of the haveIBeenPwned database file looks like
// "000000005AD76BD555C1D6D771DE417A4B87E4B4:3                   \r\n";
//  `-------------- 40 bytes --------------' `------ 22 bytes ------'
#[repr(C)]
struct HashLine {
    hash: [u8; 40],
    colon: u8,
    num_times_and_line_feed: [u8; 22],
}

impl std::fmt::Debug for HashLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HashLine {{ hash: {:?}, size_and_return: {:?} }}",
               unsafe { std::str::from_utf8_unchecked(&self.hash) },
               unsafe { std::str::from_utf8_unchecked(&self.num_times_and_line_feed) })?;
        Ok(())
    }
}
const_assert_eq!(hash_line_size; std::mem::size_of::<HashLine>(), 63);
const_assert_eq!(_2_hash_line_size; std::mem::size_of::<[HashLine; 2]>(), 126);

fn hash_lines(s: &[u8]) -> &[HashLine] {
    let ptr = s.as_ptr() as *const HashLine;
    let len = s.len() / std::mem::size_of::<HashLine>();
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

fn m(password_database: String) -> Result<()> {
    let f = File::open(password_database)?;
    let mmapped_file = unsafe { Mmap::map(&f) }?;
    let typed_hashes = hash_lines(&mmapped_file);

    assert_eq!(&typed_hashes[0].hash as &[u8], &mmapped_file[0..40]);

    debug!("first few hash lines: {:#?}", &typed_hashes[0..5]);
    debug!("num hashes: {}", typed_hashes.len());

    let password_hash = Sha1::from(prompt_password_stdout("enter password to check> ")?)
        .hexdigest().to_uppercase();
    debug!("password hash: {}", password_hash);

    println!("Searching...");
    let p = ProgressBar::new((typed_hashes.len() as f64).log2().ceil() as u64); // we're doing binary search here, so O(log2 N)
    let res =
        typed_hashes.binary_search_by_key(&password_hash.as_bytes(), |x| {
            p.inc(1);
            (&x.hash) as &[u8]
        });
    p.finish();

    if let Ok(idx) = res {
        debug!("found entry: {:?}", typed_hashes[idx]);
        let num_times: u64 = std::str::from_utf8(&typed_hashes[idx].num_times_and_line_feed)?.trim().parse()?;
        println!("The password you entered was found {} times in various leaks", num_times);
    } else {
        println!("The password you entered wasn't found! Lucky you!");
    }

    Ok(())
}
