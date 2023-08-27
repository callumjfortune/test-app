fn main() {

    let mut current_day = "Thursday";
    let mut days_remaining = 365;
    let mut next_day;

    while days_remaining > 0 {
        next_day = match current_day {
            "Monday" => "Tuesday",
            "Tuesday" => "Wednesday",
            "Wednesday" => "Thursday",
            "Thursday" => "Friday",
            "Friday" => "Saturday",
            "Saturday" => "Sunday",
            "Sunday" => "Monday",
            _ => ""
        };

        current_day = next_day;
        println!("The next day after that will be {}", current_day);
        days_remaining -= 1;

    }

}
