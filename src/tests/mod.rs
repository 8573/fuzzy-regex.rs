use super::*;

#[test]
fn worked_example_1() {
    let regex = FuzzyRegex::new("a(b|c)d").unwrap();
    let input = "ward";
    let mut state = regex.start_match(input);

    // NOTE: `MatchStep`'s field `regex_state` is ignored in comparisons, including equality.

    assert_eq!(state.mode, MatchAttemptKind::Exact);
    assert_eq!(state.current.pos, 0);
    assert_eq!(state.current.trace, 0);
    assert_eq!(state.current.fuzz, 0);
    assert_eq!(state.queue.clone().into_sorted_vec(), vec![]);

    assert_eq!(state.advance(), MatchStatus::InProgress);
    assert_eq!(state.mode, MatchAttemptKind::Exact);
    assert_eq!(state.current.pos, 1);
    assert_eq!(state.current.trace, 0);
    assert_eq!(state.current.fuzz, 0);
    assert_eq!(
        state.queue.clone().into_sorted_vec(),
        vec![
            MatchStep {
                regex_state: None,
                pos: 0,
                trace: 0,
                fuzz: 0,
                tried: 0,
            },
        ]
    );

    assert_eq!(state.advance(), MatchStatus::InProgress);
    assert_eq!(state.mode, MatchAttemptKind::Exact);
    assert_eq!(state.current.pos, 2);
    assert_eq!(state.current.trace, 1);
    assert_eq!(state.current.fuzz, 0);
    assert_eq!(
        state.queue.clone().into_sorted_vec(),
        vec![
            MatchStep {
                regex_state: None,
                pos: 0,
                trace: 0,
                fuzz: 0,
                tried: 0,
            },
            MatchStep {
                regex_state: None,
                pos: 1,
                trace: 0,
                fuzz: 0,
                tried: 0,
            },
        ]
    );

    assert_eq!(state.advance(), MatchStatus::InProgress);
    assert_eq!(state.mode, MatchAttemptKind::Exact);
    assert_eq!(state.current.pos, 3);
    assert_eq!(state.current.trace, 0);
    assert_eq!(state.current.fuzz, 0);
    assert_eq!(
        state.queue.clone().into_sorted_vec(),
        vec![
            MatchStep {
                regex_state: None,
                pos: 2,
                trace: 1,
                fuzz: 0,
                tried: 0,
            },
            MatchStep {
                regex_state: None,
                pos: 0,
                trace: 0,
                fuzz: 0,
                tried: 0,
            },
            MatchStep {
                regex_state: None,
                pos: 1,
                trace: 0,
                fuzz: 0,
                tried: 0,
            },
        ]
    );

    // TODO: Partway through transcribing eternaleye's worked example into this test-case, I
    // realize that it appears to assume that the regex can match any part of the string, whereas
    // `fst_regex` only matches whole strings. Resolve this problem?
    //
    // ...At least, `fst_regex` only matches whole strings when used on an `fst` data-structure.
    // Maybe it doesn't have that behavior when used independently?
}
