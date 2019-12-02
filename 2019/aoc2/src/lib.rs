const ADD: i32 = 1;
const MUL: i32 = 2;
const HLT: i32 = 99;

pub fn execute(mut v: Vec<i32>) -> Vec<i32> {
    let mut pc: usize = 0;
    loop {
        // println!("PC: {:?}", pc);
        match v[pc] {
            ADD => {
                let op1 = v[pc + 1] as usize;
                let op2 = v[pc + 2] as usize;
                let dst = v[pc + 3] as usize;
                v[dst] = v[op1] + v[op2];
                // println!("ADD: *{:?} + *{:?} => &{:?} ({:?})", op1, op2, dst, v[dst]);
            }
            MUL => {
                let op1 = v[pc + 1] as usize;
                let op2 = v[pc + 2] as usize;
                let dst = v[pc + 3] as usize;
                v[dst] = v[op1] * v[op2];
                // println!("MUL: *{:?} * *{:?} => &{:?} ({:?})", op1, op2, dst, v[dst]);
            }
            HLT => break,
            v => panic!("Unknown opcode {:?}", v),
        }
        pc += 4;
    }
    v
}

#[cfg(test)]
mod tests {
    use crate::execute;

    #[test]
    fn ex1() {
        let input = vec![1, 0, 0, 0, 99];
        let output = vec![2, 0, 0, 0, 99];
        assert_eq!(execute(input), output);
    }

    #[test]
    fn ex2() {
        let input = vec![2, 3, 0, 3, 99];
        let output = vec![2, 3, 0, 6, 99];
        assert_eq!(execute(input), output);
    }

    #[test]
    fn ex3() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let output = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(execute(input), output);
    }

    #[test]
    fn ex4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(execute(input), output);
    }

    #[test]
    fn part1() {
        let mut input = vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 6, 23, 2, 13, 23,
            27, 1, 27, 13, 31, 1, 9, 31, 35, 1, 35, 9, 39, 1, 39, 5, 43, 2, 6, 43, 47, 1, 47, 6,
            51, 2, 51, 9, 55, 2, 55, 13, 59, 1, 59, 6, 63, 1, 10, 63, 67, 2, 67, 9, 71, 2, 6, 71,
            75, 1, 75, 5, 79, 2, 79, 10, 83, 1, 5, 83, 87, 2, 9, 87, 91, 1, 5, 91, 95, 2, 13, 95,
            99, 1, 99, 10, 103, 1, 103, 2, 107, 1, 107, 6, 0, 99, 2, 14, 0, 0,
        ];

        input[1] = 12;
        input[2] = 2;

        let output = execute(input);
        assert_eq!(output[0], 2842648);
    }

    #[test]
    fn part2() {
        let original = vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 6, 23, 2, 13, 23,
            27, 1, 27, 13, 31, 1, 9, 31, 35, 1, 35, 9, 39, 1, 39, 5, 43, 2, 6, 43, 47, 1, 47, 6,
            51, 2, 51, 9, 55, 2, 55, 13, 59, 1, 59, 6, 63, 1, 10, 63, 67, 2, 67, 9, 71, 2, 6, 71,
            75, 1, 75, 5, 79, 2, 79, 10, 83, 1, 5, 83, 87, 2, 9, 87, 91, 1, 5, 91, 95, 2, 13, 95,
            99, 1, 99, 10, 103, 1, 103, 2, 107, 1, 107, 6, 0, 99, 2, 14, 0, 0,
        ];
        let mut answer = 0;

        'outer: for noun in 0..original.len() {
            print!(".");
            for verb in 0..original.len() {
                let mut input = original.clone();
                input[1] = noun as i32;
                input[2] = verb as i32;
                let output = execute(input);
                if output[0] == 19690720 {
                    answer = 100 * noun + verb;
                    break 'outer;
                }
            }
        }
        assert_eq!(answer, 9074);
    }
}
