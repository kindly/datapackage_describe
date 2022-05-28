use chrono::prelude::*;
use chrono::DateTime;

fn descriptions() -> Vec<(usize, String)> {
    let output = vec![
        (1, "".into()),
        (1, "".into()),
        (1, "".into()),
        (1, "".into()),
    ];

    output
}

fn datetime_formats() -> Vec<String> {
    vec! [
        "%Y-%m-%d %H:%M:%S%#z",
        "%Y-%m-%d %H:%M:%S.%f%#z",
        "%Y-%m-%d %H:%M%#z",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%d %H:%M:%S.%f",
        "%Y-%m-%d %I:%M:%S %P",
        "%Y-%m-%d %I:%M %P",
        "%Y-%m-%d %H:%M:%S %Z",
        "%Y-%m-%d %H:%M %Z",
        "%Y-%m-%d %H:%M:%S.%f %Z",
        "%Y %b %d %H:%M:%S",
        "%B %d %Y %H:%M:%S",
        "%B %d %Y %I:%M:%S %P",
        "%B %d %Y %I:%M %P"
    ]
}

fn date_formats() -> Vec<String> {
    vec! [
        "%Y-%m-%d %Z",
        "%Y-%m-%d %Z",
        "%Y-%m-%d",
        "%Y-%b-%d",
        "%B %d %Y %H:%M",
    ]
}

fn time_formats() -> Vec<String> {
    vec! [
        "%H:%M",
        "%I:%M:%S %P",
        "%I:%M %P",
        "%H:%M:%S %Z",
        "%H:%M %Z",
        "%I:%M:%S %P %Z",
        "%I:%M %P %Z"
    ]
}

pub struct Describer {
    count: usize

    
}

impl Describer {
    pub fn new() -> Describer {
        return Describer {
            count: 0
        }
    }

    pub fn process(&mut self, string: &str){
        self.count += 1;

    }
}

#[cfg(test)]
mod tests {
}
