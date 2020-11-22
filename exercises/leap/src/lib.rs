pub fn is_leap_year(year: u64) -> bool {
    // Wikipedia
    // The following pseudocode determines whether a year is a leap year or a common year in the Gregorian calendar
    // (and in the proleptic Gregorian calendar before 1582). The year variable being tested is the integer representing
    // the number of the year in the Gregorian calendar.

    // if (year is not divisible by 4) then (it is a common year)
    // else if (year is not divisible by 100) then (it is a leap year)
    // else if (year is not divisible by 400) then (it is a common year)
    // else (it is a leap year)

    // The algorithm applies to proleptic Gregorian calendar years before 1, but only if the year is expressed
    // with astronomical year numbering. It is not valid for the BC or BCE notation. The algorithm is not
    // necessarily valid for years in the Julian calendar, such as years before 1752 in the British Empire.
    // The year 1700 was a leap year in the Julian calendar, but not in the Gregorian calendar.
    if year % 4 != 0 {
        return false;
    } else if year % 100 != 0 {
        return true;
    } else if year % 400 != 0 {
        return false;
    }
    true
}
