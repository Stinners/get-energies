
use regex::Regex;

use results::FileResults;
use get_filetype::Reader;

pub fn check(line: &str) -> bool {
    lazy_static! { static ref CHECK_RE: Regex = Regex::new(r" +Welcome to Q-Chem").unwrap(); }
    CHECK_RE.is_match(line)
}
