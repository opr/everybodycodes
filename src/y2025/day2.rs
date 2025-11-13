pub fn main() {

}

fn part1(input: String) -> String {
    let cleaned_input = input.replace("A=[", "").replace("]", "");
    let input_numbers = cleaned_input.split(",").collect::<Vec<&str>>();
    let input1 = input_numbers.get(0).unwrap().parse::<isize>().unwrap();
    let input2 = input_numbers.get(1).unwrap().parse::<isize>().unwrap();
    let mut result = [0,0];
    for n in 0..3
    {
        // Cycle 1 - multiply result by itself
        let result2 = multiply(result, result);
        let result3 = divide(result2, [10, 10]);
        let result4 = add(result3, [input1, input2]);
        result = result4;
    }

    format!("[{:?},{:?}]",result[0],result[1])
}

#[derive(Clone,Debug,Copy)]
struct ComplexNumber {
    first: isize,
    second: isize,
}

fn part2(input: String) -> usize {
    let cleaned_input = input.replace("A=[", "").replace("]", "");
    let input_numbers = cleaned_input.split(",").collect::<Vec<&str>>();
    let input1 = input_numbers.get(0).unwrap().parse::<isize>().unwrap();
    let input2 = input_numbers.get(1).unwrap().parse::<isize>().unwrap();

    let mut x_vec: Vec<ComplexNumber> = vec![];
    let mut grid: Vec<ComplexNumber> = vec![];

    // Build grid by looping 101 times over each axis.
    for y in 0..101 {
        x_vec.clear();
        for x in 0..101 {
            let res = add([input1, input2], [10*x, 10*y]);
            grid.push(ComplexNumber{first:res[0], second:res[1]});
        }
    }

    grid.iter().filter(|&complex_number | {
       should_engrave( *complex_number )
    } ).count()
}

fn should_engrave ( input: ComplexNumber ) -> bool {
    let mut result = [0,0];
    for i in 0..100 {
        let res1 = multiply([result[0], result[1]], [result[0], result[1]]);
        let res2 = divide([res1[0], res1[1]], [100000, 100000]);
        result = add(res2, [input.first, input.second]);
        if result[0] > 1000000 || result[0] < -1000000 || result[1] > 1000000 || result[1] < -1000000 {
            return false;
        }
    }
    true
}

fn multiply( complex1: [isize;2], complex2: [isize;2]) -> [isize;2] {
    let x = [complex1[0], complex2[0]];
    let y = [complex1[1], complex2[1]];
    [ ( x[0] * x[1] ) - (y[0] * y[1]), (x[0] * y[1]) + (y[0] * x[1]) ]
}

fn divide( complex1: [isize;2], complex2: [isize;2]) -> [isize;2] {
    let x = [complex1[0], complex2[0]];
    let y = [complex1[1], complex2[1]];
    [ x[0] / x[1], y[0] / y[1] ]
}

fn add( complex1: [isize;2], complex2: [isize;2]) -> [isize;2] {
    let x = [complex1[0], complex2[0]];
    let y = [complex1[1], complex2[1]];
    [ x[0] + x[1], y[0] + y[1] ]
}

#[cfg(test)]
mod tests {
    use crate::input_reader::input_reader::read_input_for_day;
    use crate::y2025::day2::{divide, multiply, add, part1, part2, should_engrave, ComplexNumber};

    #[test]
    fn test_multiply() {
        assert_eq!([0,0], multiply([0,0], [0,0]));
        assert_eq!([0,4], multiply([1,1], [2,2]));
        assert_eq!([-29,29], multiply([2,5], [3,7]));
        assert_eq!([-5,10], multiply([-1,-2], [-3,-4]));
    }

    #[test]
    fn test_divide() {
        assert_eq!([5,6], divide([10,12], [2,2]));
        assert_eq!([3,2], divide([11,12], [3,5]));
        assert_eq!([-5,-6], divide([-10,-12], [2,2]));
    }

    #[test]
    fn test_add() {
        assert_eq!([3,3], add([1,1], [2,2]));
        assert_eq!([5,12], add([2,5], [3,7]));
        assert_eq!([8,4], add([-2,5], [10,-1]));

    }

    #[test]
    fn test_part1_test() {
        let input = read_input_for_day(2025, 2, true);
        let result = part1(input);
        assert_eq!(result, "[357,862]")
    }

    #[test]
    fn test_part2_test() {
        let input = read_input_for_day(2025, "2.2", true);
        let result = part2(input);
        assert_eq!(result, 4076)
    }

    #[test]
    fn test_part2() {
        let input = read_input_for_day(2025, "2.2", false);
        let result = part2(input);
        assert_eq!(result, 1236)
    }

    #[test]
    fn test_should_engrave() {
        assert_eq!(should_engrave(ComplexNumber{first:35630,second:-64880}), true);
        assert_eq!(should_engrave(ComplexNumber{first:35630,second:-64870}), true);
        assert_eq!(should_engrave(ComplexNumber{first:35640,second:-64860}), true);
        assert_eq!(should_engrave(ComplexNumber{first:36230,second:-64270}), true);
        assert_eq!(should_engrave(ComplexNumber{first:36250,second:-64270}), true);

        assert_eq!(should_engrave(ComplexNumber{first:35460,second:-64910}), false);
        assert_eq!(should_engrave(ComplexNumber{first:35470,second:-64910}), false);
        assert_eq!(should_engrave(ComplexNumber{first:35680,second:-64850}), false);
        assert_eq!(should_engrave(ComplexNumber{first:35630,second:-64830}), false);
    }

    #[test]
    fn test_part1() {
        let input = read_input_for_day(2025, 2, false);
        let result = part1(input);
        assert_eq!(result, "[338000,815150]")
    }
}