use anyhow::{bail, Result};

use time::Duration;

pub fn pretty_duration(duration: Duration) -> String {
    let mut s = String::new();
    let mut secs = duration.whole_seconds();
    if secs >= 3600 {
        s.push_str(&(secs / 3600).to_string());
        s.push(':');
        secs %= 3600
    }

    if secs >= 60 {
        s.push_str(&(secs / 60).to_string());
        s.push(':');
        secs %= 60;
    }

    if s.is_empty() {
        s.push_str(&secs.to_string());
        s.push_str(" s");
    } else {
        s.push_str(&format!("{secs:02}"));
    }

    s
}

pub fn weighted_median(xs: &[(f64, f64)] /* (val, weight) */) -> Result<f64> {
    if xs.is_empty() {
        bail!("input was empty");
    } else if xs.iter().any(|(a, b)| a.is_nan() || b.is_nan()) {
        bail!("input contained NaN");
    }

    let half_sum = xs.iter().map(|s| s.1).sum::<f64>() / 2.0;
    let mut v = xs.to_vec();
    v.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let l = l_wt_median_unchecked(&v, &half_sum);
    if v.len() % 2 == 0 {
        let r = r_wt_median_unchecked(&v, &half_sum);
        Ok((r.0 + l.0) / 2.0)
    } else {
        Ok(l.0)
    }
}

fn l_wt_median_unchecked<'a>(xs: &'a [(f64, f64)], half_sum: &f64) -> &'a (f64, f64) {
    let mut wt_sum = 0.0;
    for t @ (_, wt) in xs {
        wt_sum += wt;
        if &wt_sum >= half_sum {
            return t;
        }
    }

    panic!("l_wt_median_unchecked recieved empty slice");
}

fn r_wt_median_unchecked<'a>(xs: &'a [(f64, f64)], half_sum: &f64) -> &'a (f64, f64) {
    let mut wt_sum = 0.0;
    let mut it = xs.iter();
    while let Some(t @ (_, wt)) = it.next_back() {
        wt_sum += wt;
        if &wt_sum >= half_sum {
            return t;
        }
    }

    panic!("r_wt_median_unchecked recieved empty slice");
}
