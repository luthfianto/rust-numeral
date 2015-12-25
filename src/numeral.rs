pub fn format_numeral(n: f64, format: &str) -> String {
    if format.contains(":") {
        format_time(n, format)
    } else {
        format_number(n, format)
    }
}

pub fn unformat_numeral(string: &str) -> f64 {
    if string.contains(":") {
        unformat_time(string)
    } else {
        unformat_number(string)
    }
}

#[allow(unused_variables)]
fn format_number(n: f64, format: &str) -> String {
    unimplemented!()
}

#[allow(unused_variables)]
fn unformat_number(string: &str) -> f64 {
    unimplemented!()
}

#[allow(unused_variables)]
fn format_time(n: f64, format: &str) -> String {
    let hours: f64 = (n/60./60.).floor();
    let minutes = ((n - (hours * 60. * 60.))/60.).floor();
    let seconds = (n - (hours * 60. * 60.) - (minutes * 60.)).round();

    let hours_string = hours.to_string();

    let minutes_string = & if minutes < 10. {
        "0".to_string() + &minutes.to_string()
    } else {
        minutes.to_string()
    };

    let seconds_string = & if seconds < 10. {
        "0".to_string() + &seconds.to_string()
    } else {
        seconds.to_string()
    };
    
    hours_string + ":" + minutes_string + ":" + seconds_string
}

fn unformat_time(string: &str) -> f64 {
    let split = string.split(":");
    let time_array: Vec<&str> = split.collect();
    let mut seconds = 0.0;

    if time_array.len() == 3 {
        // hours
        seconds = seconds + (time_array[0].parse::<f64>().unwrap() * 60.0 * 60.0);
        // minutes
        seconds = seconds + (time_array[1].parse::<f64>().unwrap() * 60.0);
        // seconds
        seconds = seconds + (time_array[2].parse::<f64>().unwrap());
    } else if time_array.len() == 2 {
        // minutes
        seconds = seconds + (time_array[0].parse::<f64>().unwrap() * 60.0);
        // seconds
        seconds = seconds + (time_array[1].parse::<f64>().unwrap());
    }
    seconds
}

#[test]
fn test_format_time() {
    let time = format_numeral(3600.0, "00:00:00");
    assert_eq!(time, "1:00:00");
}

#[test]
fn test_unformat_time() {
    let number = unformat_numeral("1:00:00");
    assert_eq!(number, 3600.0);
}
