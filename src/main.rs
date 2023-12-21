struct ValuesForAlice {
    max_score: u16,
    window_size: u16,
}


fn task_2_vanilla(max_test_length: u16, threshold: i32) -> ValuesForAlice {
    // It's just a wrapper, see the desciption within the `_task_2_vanilla` function
    let remainder = max_test_length % 2;
    ValuesForAlice {
        max_score: 100 * (max_test_length / 2 + remainder) * (max_test_length / 2),
        window_size: _task_2_vanilla(max_test_length, threshold),
    }
}


fn _task_2_vanilla(max_test_length: u16, threshold: i32) -> u16 {
    // Preliminary comments: 
    // in the vanilla version of the task it's NOT requested to assume the number of questions as a variable.
    // I also believe that the length of the test in minutes is an integer; 
    // that the score should either exceed or be equal to the threshold.
    //
    // `total_score = right_answers * time_elapsed * time_remaining`
    // Let's replace `time_elapsed := max_test_length / 2 + delta`; then `time_remaining = max_test_length / 2 - delta`;
    // (it doesn't matter if `delta` is positive or negative)
    // Then `total_score = right_answers * ( max_test_length^2 / 4 - delta^2)`
    // It's a quadratic function with respect to `delta`. The condition to exceed a threshold is as follows:
    // `delta^2 <= ( max_test_length^2 / 4 ) - (threshold / right_answers)`
    // 
    // Now, let's deduce the size of the window.
    // There may be no valid `delta` meeting the inequality, then the size is 0 (no such window).
    // For the window [2nd min; 8th min] from the task description (corresponding to `max_delta = 3`) the window size is 7,
    // so among `time_elapsed` and `time_remaining` one is ceiled while another is floored
    // (in fact it's `time_elapsed` which is ceiled, as submitting the results by the 1st minute gives 900 scores instead of 0, but it doesn't matter).
    // So `window_size = 2 * max_delta + 1` (which is stricly less then `max_test_length`) for a positive threshold
    // and `window_size = max_test_length` (the answers can be submitted any time) for non-positive ones.

    if threshold <= 0 {
        return max_test_length
    }

    let length_is_even = max_test_length % 2 == 0;
    let right_answers: i32 = 100;
    let quadruple_right_side = (max_test_length as i32).pow(2) - (4 * threshold  / right_answers);

    let mut max_double_delta = (quadruple_right_side as f32).sqrt().floor() as u16;
    if length_is_even {  // => `delta` is an integer
        if quadruple_right_side < 0 {
            return 0
        }
        max_double_delta -= max_double_delta % 2;
    } else {  // => double `delta` is an integer, while `delta` itself is not
        if quadruple_right_side < 1 {
            return 0
        }
        if max_double_delta % 2 == 0 {
            max_double_delta -= 1;
        }
    }

    #[cfg(test)]
    {
        assert!(right_answers * (max_test_length + max_double_delta) as i32 * (max_test_length - max_double_delta) as i32 >= 4 * threshold, "Found delta doesn't meet the inequality");
        assert!(right_answers * (max_test_length + max_double_delta + 2) as i32 * ((max_test_length - max_double_delta - 2) as i32) < 4 * threshold, "Found delta can be increased");
    }

    max_double_delta + 1
}


struct Variables{
    max_test_length: u16,
    threshold: i32,
    question_num: u16,
}


impl Variables {
    fn get_score(&self, x: u16) -> u16 {
        (x * self.question_num / self.max_test_length) * x * (self.max_test_length - x)
    }
}


fn task_2_bonus(vars: Variables) -> ValuesForAlice {
    // New formula is `total_score = ((x / tnm) * tnq) * x * (tnm - x)`
    // (let's ignore for now that `((x / tnm) * tnq)` should be rounded)
    // or `total_score = x^2 (tnm - x) * (tnq / tnm)`
    // Let's examine `f(x) = x^2 (tnm - x)`:
    // `f'(x) = -3x^2 + 2x*tnm`
    // i.e. f(x) has a local minimum at `x = 0` (this point is of no interest to us)
    // and has a local maximum at `x = 2/3 * tnm` (it's here that Alice gets her best score).
    //
    // Now, to determine the window size we should find two points:
    // the point within [0, 2/3*tnm] where the score reaches the threshold for the first time;
    // and the point within [2/3*tnm, tnm] where the score is above the threshold for the last time.
    // There'are at least three ways to do it:
    // - analytically though using quite bulky formulae;
    // - using 2 binary searches (which makes sense only for really long tests,
    // where it's usefull to minimize the amount of calculations and the rounding can't really distort the function;
    // - to just calculate the score for each minute.
    // I will follow the last approach - I know that I'm expected to show off doing an algorithmic test task,
    // but the simplest way is often the best one for production code (which must be as readable and reliable as possible)
    // and for real-life tests of real-life Alices (they never include even a 1000 minutes to make straightforward calculation too long). 

    let threshold = if vars.threshold < 0 {0} else {vars.threshold as u16};

    let mut max_score = 0;
    let mut window_size = 0;
    for i in 1..(vars.max_test_length + 1) {
        let score = vars.get_score(i);
        if score >= threshold {
            window_size += 1;  // in fact we don't check if it's actually a continuous window
        }
        if score > max_score {
            max_score = score;
        }
    }

    ValuesForAlice {max_score, window_size}
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_vanilla() {
        // max score
        assert_eq!(task_2_vanilla(10, 0).max_score, 2500);
        assert_eq!(task_2_vanilla(11, 0).max_score, 3000);

        // window size, even test length
        assert_eq!(task_2_vanilla(10, 3000).window_size, 0);
        assert_eq!(task_2_vanilla(10, 2500).window_size, 1);
        assert_eq!(task_2_vanilla(10, 2450).window_size, 1);
        assert_eq!(task_2_vanilla(10, 2400).window_size, 3);
        assert_eq!(task_2_vanilla(10, 1500).window_size, 7);
        assert_eq!(task_2_vanilla(10, 800).window_size, 9);
        assert_eq!(task_2_vanilla(10, 0).window_size, 10);

        // window size, odd test length
        assert_eq!(task_2_vanilla(11, 3500).window_size, 0);
        assert_eq!(task_2_vanilla(11, 3000).window_size, 2);
        assert_eq!(task_2_vanilla(11, 2700).window_size, 4);
        assert_eq!(task_2_vanilla(11, 100).window_size, 10);
        assert_eq!(task_2_vanilla(11, 0).window_size, 11);
    }

    #[test]
    fn test_bonus() {
        // max score
        assert_eq!(task_2_bonus(Variables{max_test_length: 5, threshold: 0, question_num: 50}).max_score, 180);
        assert_eq!(task_2_bonus(Variables{max_test_length: 5, threshold: 0, question_num: 100}).max_score, 360);
        assert_eq!(task_2_bonus(Variables{max_test_length: 3, threshold: 0, question_num: 30}).max_score, 40);

        // window size
        assert_eq!(task_2_bonus(Variables{max_test_length: 3, threshold: 0, question_num: 30}).window_size, 3);
        assert_eq!(task_2_bonus(Variables{max_test_length: 3, threshold: 30, question_num: 30}).window_size, 1);
        assert_eq!(task_2_bonus(Variables{max_test_length: 3, threshold: 300, question_num: 300}).window_size, 1);
        assert_eq!(task_2_bonus(Variables{max_test_length: 3, threshold: 10, question_num: 30}).window_size, 2);
        assert_eq!(task_2_bonus(Variables{max_test_length: 6, threshold: 200, question_num: 60}).window_size, 3);
        assert_eq!(task_2_bonus(Variables{max_test_length: 6, threshold: 600, question_num: 180}).window_size, 3);
        assert_eq!(task_2_bonus(Variables{max_test_length: 6, threshold: 320, question_num: 60}).window_size, 1);
    }

}