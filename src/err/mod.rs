use fst_regex;

error_chain! {
    foreign_links {
        FstRegex(fst_regex::Error);
    }
}
