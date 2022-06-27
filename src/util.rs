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
        s.push_str(&secs.to_string());
    }

    s
}

pub fn mean(v: Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }

    Some(v.iter().sum::<f64>() / v.len() as f64)
}

pub fn median(mut v: Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }

    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    if v.len() % 2 == 1 {
        Some(v[v.len() / 2])
    } else {
        Some((v[v.len() / 2] + v[(v.len() / 2) + 1]) / 2.0)
    }
}
