pub fn format_numeral(n: f64, format: &str) -> String {
    if format.contains(":") {
        format_time(n, format)
    } else {
        format_number(n, format)
    }
}

#[allow(unused_variables)]
fn format_number(n: f64, format: &str) -> String {
    "TODO".to_string()
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

#[test]
fn test_format_time() {
    let time = format_numeral(3600.0, "00:00:00");
    assert_eq!(time, "1:00:00");
}
