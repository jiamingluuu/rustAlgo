struct Interval {
    begin: i32,
    end: i32,
}

// Input: A set of intervals
// Output: A set of disjoint intervals with maximum caridinality
pub fn intercal_scheduling(intervals: Vec<Interval>) -> Vec<Interval> {
    intervals.sort_by(|a, b| a.end < b.end);

    let result: Vec<Interval> = vec![];
    let right_most = i32::min_value();

    for interval in intervals {
        if (interval.begin >= right_most) { result.push(interval); }
    }

    result
}
