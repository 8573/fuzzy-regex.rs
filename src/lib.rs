extern crate fst;
extern crate fst_regex;

#[macro_use]
extern crate error_chain;

pub use self::err::*;
use fst::Automaton;
use fst_regex::Regex as FstRegex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

mod err;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct FuzzyRegex {
    fst_regex: FstRegex,
}

#[derive(Debug)]
struct MatchInProgress<'r, 't> {
    fst_regex: &'r FstRegex,
    input: &'t str,
    mode: MatchAttemptKind,
    current: MatchStep,
    queue: BinaryHeap<MatchStep>,
}

#[derive(Debug, PartialEq)]
enum MatchAttemptKind {
    Exact,
    Substitution,
    Insertion,
    Deletion,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
struct MatchStep {
    /// The current state of the underlying `fst_regex::Regex`.
    regex_state: <FstRegex as Automaton>::State,

    /// How many octets of input we've consumed.
    pos: usize,

    /// The length in octets of the current match candidate.
    trace: usize,

    /// The Levenshtein distance of the current match candidate from the exact regex.
    fuzz: u32,

    /// How many times ?? TODO
    tried: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum MatchStatus {
    /// The match is still in progress.
    InProgress,

    /// The match succeeded, at the given Levenshtein distance from the exact regular expression.
    Matched(u32),

    /// The match failed.
    Failed,
}

impl FuzzyRegex {
    /// Compiles a regular expression that can be used to match strings approximately.
    ///
    /// The regex will behave as if anchored on both ends; e.g., `"a(b|c)d"` will behave as
    /// `"^a(b|c)d$"`.
    pub fn new(uncompiled_regex: &str) -> Result<Self> {
        Ok(FuzzyRegex { fst_regex: FstRegex::new(uncompiled_regex)? })
    }

    /// Returns whether the regex matches the given string.
    ///
    /// A string will be considered to match so long as it is within a Levenshtein distance of
    /// `max_fuzz` from the exact regular expression.
    pub fn is_match(&self, _text: &str, _max_fuzz: u32) -> bool {
        unimplemented!()
    }

    fn start_match<'r, 't>(&'r self, text: &'t str) -> MatchInProgress<'r, 't> {
        MatchInProgress {
            fst_regex: &self.fst_regex,
            input: text,
            mode: MatchAttemptKind::Exact,
            current: MatchStep {
                regex_state: self.fst_regex.start(),
                pos: 0,
                trace: 0,
                fuzz: 0,
                tried: 0,
            },
            queue: BinaryHeap::new(),
        }
    }
}

impl<'r, 't> MatchInProgress<'r, 't> {
    fn advance(&mut self) -> MatchStatus {
        unimplemented!()
    }
}

impl Eq for MatchStep {}

impl Ord for MatchStep {
    fn cmp(&self, other: &Self) -> Ordering {
        let &MatchStep {
            regex_state: _,
            pos,
            trace,
            fuzz,
            tried,
        } = self;

        // Sort ascending by Levenshtein distance.
        match fuzz.cmp(&other.fuzz) {
            Ordering::Equal => {}
            r => return r,
        }

        // Sort descending by match length.
        match other.trace.cmp(&trace) {
            Ordering::Equal => {}
            r => return r,
        }

        // Sort ascending by match position.
        match pos.cmp(&other.pos) {
            Ordering::Equal => {}
            r => return r,
        }

        // Sort ascending by tries.
        // XXX: This isn't in eternaleye's worked example. What does it do to the results?
        match tried.cmp(&other.tried) {
            Ordering::Equal => {}
            r => return r,
        }

        Ordering::Equal
    }
}

impl PartialEq for MatchStep {
    fn eq(&self, other: &Self) -> bool {
        let &MatchStep {
            regex_state: _,
            pos,
            trace,
            fuzz,
            tried,
        } = self;

        pos == other.pos && trace == other.trace && fuzz == other.fuzz && tried == other.tried
    }
}

impl PartialOrd for MatchStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
