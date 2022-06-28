use crate::util;

#[test]
fn weighted_median() {
    const SETS: &[(&[(f64, f64)], f64)] = &[
        (&[(1.0, 0.25), (2.0, 0.25), (3.0, 0.25), (4.0, 0.25)], 2.5),
        (
            &[(1.0, 0.15), (2.0, 0.1), (3.0, 0.2), (4.0, 0.3), (5.0, 0.25)],
            4.0,
        ),
        (&[(1.0, 0.49), (2.0, 0.01), (3.0, 0.25), (4.0, 0.25)], 2.5),
    ];

    for (set, res) in SETS {
        assert_eq!(&util::weighted_median(set).unwrap(), res);
    }
}
