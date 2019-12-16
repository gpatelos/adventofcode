use crate::util::intcode::{IntCode, Result, Value};

const INPUT: &str = include_str!("../../static/day05.txt");

const PROGRAM_INPUT1: [Value; 1] = [Value::from(1)];
const PROGRAM_INPUT2: [Value; 1] = [Value::from(5)];

pub fn main() {
    let answer1 = solve1(INPUT).unwrap();
    println!("{}", answer1);
    let answer2 = solve2(INPUT).unwrap();
    println!("{}", answer2);
}

fn solve1(input: &str) -> Result<Value> {
    IntCode::from_str(input)?
        .with_inputs(PROGRAM_INPUT1.to_vec())
        .run()?
        .last_output()
}

fn solve2(input: &str) -> Result<Value> {
    IntCode::from_str(input)?
        .with_inputs(PROGRAM_INPUT2.to_vec())
        .run()?
        .last_output()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_output() -> Result<()> {
        let input = Value::from(78);
        let output = IntCode::from_str("3,0,4,0,99")?
            .with_inputs(vec![Value::from(input)])
            .run()?
            .last_output()?;
        assert_eq!(output, input);
        Ok(())
    }

    #[test]
    fn immediate() -> Result<()> {
        let product = IntCode::from_str("1002,4,3,4,33")?.run()?;
        let expected_memory: Vec<Value> = [1002, 4, 3, 4, 99]
            .iter()
            .map(|&x| Value::from(x))
            .collect::<Vec<_>>();
        assert_eq!(product.memory().to_vec(), expected_memory);
        Ok(())
    }

    #[test]
    fn equal_position_mode() -> Result<()> {
        let program_str = "3,9,8,9,10,9,4,9,99,-1,8";
        let target = Value::from(8);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(1));
        let output_not_eq_target1 = IntCode::from_str(program_str)?
            .with_inputs(vec![target + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target1, Value::from(0));
        let output_not_eq_target2 = IntCode::from_str(program_str)?
            .with_inputs(vec![target - 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target2, Value::from(0));
        Ok(())
    }

    #[test]
    fn less_than_position_mode() -> Result<()> {
        let program_str = "3,9,7,9,10,9,4,9,99,-1,8";
        let target = Value::from(8);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(0));
        let output_less_than_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target - 1])
            .run()?
            .last_output()?;
        assert_eq!(output_less_than_target, Value::from(1));
        let output_greater_than_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_greater_than_target, Value::from(0));
        Ok(())
    }

    #[test]
    fn equal_immediate_mode() -> Result<()> {
        let program_str = "3,3,1108,-1,8,3,4,3,99";
        let target = Value::from(8);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(1));
        let output_not_eq_target1 = IntCode::from_str(program_str)?
            .with_inputs(vec![target + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target1, Value::from(0));
        let output_not_eq_target2 = IntCode::from_str(program_str)?
            .with_inputs(vec![target - 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target2, Value::from(0));
        Ok(())
    }

    #[test]
    fn less_than_immediate_mode() -> Result<()> {
        let program_str = "3,3,1107,-1,8,3,4,3,99";
        let target = Value::from(8);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(0));
        let output_less_than_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target - 1])
            .run()?
            .last_output()?;
        assert_eq!(output_less_than_target, Value::from(1));
        let output_greater_than_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_greater_than_target, Value::from(0));
        Ok(())
    }

    #[test]
    fn jump_position_mode() -> Result<()> {
        let program_str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let target = Value::from(0);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(0));
        let output_not_eq_target1 = IntCode::from_str(program_str)?
            .with_inputs(vec![target + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target1, Value::from(1));
        let output_not_eq_target2 = IntCode::from_str(program_str)?
            .with_inputs(vec![target + 7])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target2, Value::from(1));
        Ok(())
    }

    #[test]
    fn answer01a() {
        assert_eq!(solve1(INPUT), Ok(Value::from(6069343)));
    }

    #[test]
    fn answer01b() {
        assert_eq!(solve2(INPUT), Ok(Value::from(3188550)));
    }
}
