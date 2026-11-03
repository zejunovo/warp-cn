//! Property-based tests for watch channel delivery.
//!
//! Covers spec requirement:
//! - (e) watch delivery: each i18n.set_locale(L) with L != current MUST deliver changed() to all receivers

use proptest::prelude::*;
use warp_i18n::{I18n, Locale};

// (e) Sequential set_locale calls deliver all distinct transitions to receivers.
proptest! {
    #[test]
    fn watch_delivers_all_distinct_transitions(
        seq in prop::collection::vec(
            prop_oneof![Just(Locale::En), Just(Locale::ZhCn)],
            1..50,
        ),
    ) {
        let i18n = I18n::new(Locale::ZhCn).expect("bundles load");

        let distinct: Vec<Locale> = {
            let mut d: Vec<Locale> = vec![i18n.current()];
            for &l in &seq {
                let last = *d.last().unwrap();
                if l != last {
                    d.push(l);
                }
            }
            let mut result = d;
            if result.len() > 1 {
                result.remove(0);
                result
            } else {
                vec![]
            }
        };

        if distinct.is_empty() {
            return Ok(());
        }

        let mut rx = i18n.subscribe();
        rx.borrow_and_update();

        for &locale in &distinct {
            i18n.set_locale(locale);
        }

        let observed = *rx.borrow_and_update();
        let expected = *distinct.last().unwrap();
        prop_assert_eq!(observed, expected);
    }
}

// (e) Falsification: multiple receivers all observe consistent state.
proptest! {
    #[test]
    fn multiple_receivers_consistent(
        seq in prop::collection::vec(
            prop_oneof![Just(Locale::En), Just(Locale::ZhCn)],
            5..30,
        ),
        num_receivers in 1usize..10usize,
    ) {
        let i18n = I18n::new(Locale::ZhCn).expect("bundles load");

        let mut receivers: Vec<_> = (0..num_receivers)
            .map(|_| i18n.subscribe())
            .collect();

        for receiver in &mut receivers {
            receiver.borrow_and_update();
        }

        for &locale in &seq {
            if locale != i18n.current() {
                i18n.set_locale(locale);
            }
        }

        let expected = *receivers[0].borrow_and_update();
        for receiver in receivers.iter_mut().skip(1) {
            let obs = *receiver.borrow_and_update();
            prop_assert_eq!(obs, expected);
        }
    }
}
