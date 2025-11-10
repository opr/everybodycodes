pub fn main() {

}

fn part1(input: String) -> String {
    let cleaned_input = input.replace("A=[", "").replace("]", "");
    let input_numbers = cleaned_input.split(",").collect::<Vec<&str>>();
    let input1 = input_numbers.get(0).unwrap().parse::<i32>().unwrap();
    let input2 = input_numbers.get(1).unwrap().parse::<i32>().unwrap();
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

fn multiply( complex1: [i32;2], complex2: [i32;2]) -> [i32;2] {
    let x = [complex1[0], complex2[0]];
    let y = [complex1[1], complex2[1]];
    [ ( x[0] * x[1] ) - (y[0] * y[1]), (x[0] * y[1]) + (y[0] * x[1]) ]
}

fn divide( complex1: [i32;2], complex2: [i32;2]) -> [i32;2] {
    let x = [complex1[0], complex2[0]];
    let y = [complex1[1], complex2[1]];
    [ x[0] / x[1], y[0] / y[1] ]
}

fn add( complex1: [i32;2], complex2: [i32;2]) -> [i32;2] {
    let x = [complex1[0], complex2[0]];
    let y = [complex1[1], complex2[1]];
    [ x[0] + x[1], y[0] + y[1] ]
}

#[cfg(test)]
mod tests {
    use crate::input_reader::input_reader::read_input_for_day;
    use crate::y2025::day2::{divide, multiply, add, part1};

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
    fn test_part1() {
        let input = read_input_for_day(2025, 2, false);
        let result = part1(input);
        assert_eq!(result, "[338000,815150]")
    }
}