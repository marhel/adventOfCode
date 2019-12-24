const ADD: i64 = 1;
const MUL: i64 = 2;
const INP: i64 = 3;
const OUT: i64 = 4;

const JNZ: i64 = 5;
const JZ: i64 = 6;
const LES: i64 = 7;
const EQU: i64 = 8;
const ADJ: i64 = 9;

const HLT: i64 = 99;

const POSITION: i64 = 0;
const IMMEDIATE: i64 = 1;
const RELATIVE: i64 = 2;

pub fn address(v: &Vec<i64>, base: usize, offset: usize, mode: i64) -> usize {
    match mode {
        POSITION => {
            v[offset] as usize
        }
        IMMEDIATE => {
            offset
        }
        RELATIVE => {
            let op = v[offset];
            let index = (base as i64 + op) as usize;
            index
        }
        v => panic!("Unknown mode {:?}", v),
    }
}

pub fn operand(v: &Vec<i64>, base: usize, offset: usize, mode: i64) -> i64 {
    let address = address(v, base, offset, mode);
    v[address]
}

pub fn execute(v: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    execute_with_input(v, 0)
}

pub fn execute_with_input(mut v: Vec<i64>, input: i64) -> (Vec<i64>, Vec<i64>) {
    let mut pc: usize = 0;
    let mut output = Vec::new();
    let mut relative_base: usize = 0;
    loop {
        // println!("PC: {:?}", pc);
        let opcode = v[pc];
        let instruction = opcode % 100;
        let m1 = (opcode / 100) % 10;
        let m2 = (opcode / 1000) % 10;
        let m3 = (opcode / 10000) % 10;
        // println!(
        //     "\n  {:04}: ins {:?} ({:?}) [{:?}, {:?}, {:?}]",
        //     pc, instruction, opcode, m1, m2, m3
        // );

        match instruction {
            ADD => {
                let op1 = operand(&v, relative_base, pc + 1, m1);
                let op2 = operand(&v, relative_base, pc + 2, m2);
                let dst = address(&v, relative_base, pc + 3, m3);
                v[dst] = op1 + op2;
                // println!("{:04}: ADD: {:?} + {:?} => &{:?} ({:?})", pc, op1, op2, dst, v[dst]);
                pc += 4;
            }
            MUL => {
                let op1 = operand(&v, relative_base, pc + 1, m1);
                let op2 = operand(&v, relative_base, pc + 2, m2);
                let dst = address(&v, relative_base, pc + 3, m3);
                v[dst] = op1 * op2;
                // println!("{:04}: MUL: {:?} * {:?} => &{:?} ({:?})", pc, op1, op2, dst, v[dst]);
                pc += 4;
            }
            INP => {
                let dst = address(&v, relative_base, pc + 1, m1);
                // println!("{:04}: INP: {:?} => {:?}", pc, input, dst);
                v[dst] = input;
                pc += 2;
            }
            OUT => {
                let op1 = operand(&v, relative_base, pc + 1, m1);
                output.push(op1);
                println!("{:04}: OUT: {:?}", pc, output.last().unwrap());
                pc += 2;
            }
            JNZ => {
                let op1 = operand(&v, relative_base, pc + 1, m1);
                let dst = operand(&v, relative_base, pc + 2, m2);
                if op1 != 0 {
                    pc = dst as usize;
                } else {
                    pc += 3;
                }
            }
            JZ => {
                let op1 = operand(&v, relative_base, pc + 1, m1);
                let dst = operand(&v, relative_base, pc + 2, m2);
                if op1 == 0 {
                    pc = dst as usize;
                } else {
                    pc += 3;
                }
            }
            LES => {
                let op1 = operand(&v, relative_base, pc + 1, m1);
                let op2 = operand(&v, relative_base, pc + 2, m2);
                let dst = address(&v, relative_base, pc + 3, m3);
                v[dst] = if op1 < op2 { 1 } else { 0 };
                pc += 4;
            }
            EQU => {
                let op1 = operand(&v, relative_base, pc + 1, m1);
                let op2 = operand(&v, relative_base, pc + 2, m2);
                let dst = address(&v, relative_base, pc + 3, m3);
                v[dst] = if op1 == op2 { 1 } else { 0 };
                pc += 4;
            }
            ADJ => {
                let adjustment = operand(&v, relative_base, pc + 1, m1);
                relative_base = (relative_base as i64 + adjustment) as usize;
                // println!("{:04}: ADJ: {:?} => {:?}", pc, adjustment, relative_base);
                pc += 2;
            }
            HLT => break,
            v => panic!("Unknown opcode {:?}", v),
        }
    }
    (output, v)
}

#[cfg(test)]
mod tests {
    use crate::{execute, execute_with_input};

    #[test]
    fn ex1() {
        let input = vec![1, 0, 0, 0, 99];
        let output = vec![2, 0, 0, 0, 99];
        assert_eq!(execute(input).1, output);
    }

    #[test]
    fn ex_imm_add() {
        let input = vec![1101, 1, 1, 0, 99];
        let output = vec![2, 1, 1, 0, 99];
        assert_eq!(execute(input).1, output);
    }

    #[test]
    fn ex_imm_mul() {
        let input = vec![102, 3, 1, 0, 99];
        let output = vec![9, 3, 1, 0, 99];
        assert_eq!(execute(input).1, output);
    }

    #[test]
    fn ex2() {
        let input = vec![2, 3, 0, 3, 99];
        let output = vec![2, 3, 0, 6, 99];
        assert_eq!(execute(input).1, output);
    }

    #[test]
    fn input_output() {
        let input = vec![3, 0, 4, 0, 99];
        let res = execute_with_input(input, 33);
        assert_eq!(*res.0.last().unwrap(), 33);
    }

    #[test]
    fn ex4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(execute(input).1, output);
    }

    #[test]
    fn not_equal_to_8() {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let output = execute_with_input(input, 5);
        assert_eq!(*output.0.last().unwrap(), 0);
    }

    #[test]
    fn equal_to_8() {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let output = execute_with_input(input, 8);
        assert_eq!(*output.0.last().unwrap(), 1);
    }
    #[test]
    fn less_than_8() {
        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let output = execute_with_input(input, 5);
        assert_eq!(*output.0.last().unwrap(), 1);
    }
    #[test]
    fn not_less_than_8() {
        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let output = execute_with_input(input, 55);
        assert_eq!(*output.0.last().unwrap(), 0);
    }
    #[test]
    fn equal_to_8_imm() {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let output = execute_with_input(input, 8);
        assert_eq!(*output.0.last().unwrap(), 1);
    }
    #[test]
    fn not_equal_to_8_imm() {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let output = execute_with_input(input, 9);
        assert_eq!(*output.0.last().unwrap(), 0);
    }

    #[test]
    fn quine_rel() {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut i2 = input.clone();
        i2.resize(2048, 0);
        let output = execute(i2);
        assert_eq!(output.0, input);
    }

    #[test]
    fn large_num_output() {
        let input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let output = execute(input);
        assert_eq!(*output.0.last().unwrap(), 34915192 * 34915192);
    }

    #[test]
    fn large_num_input() {
        let input = vec![104, 1125899906842624, 99];

        let output = execute(input);
        assert_eq!(*output.0.last().unwrap(), 1125899906842624);
    }
    #[test]
    fn rel_base_test() {
        let input = vec![109, 2000, 109, 19, 204, -2018, 99];

        let output = execute(input);
        assert_eq!(*output.0.last().unwrap(), 2000);
    }
    // For example, if the relative base is 2000, then after the instruction , the relative base would be 2019. If the next instruction were 204,-34, then the value at address 1985 would be output.

    #[test]

    fn old1() {
        let input = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1002, 148, 28, 224, 1001, 224, -672,
            224, 4, 224, 1002, 223, 8, 223, 101, 3, 224, 224, 1, 224, 223, 223, 1102, 8, 21, 225,
            1102, 13, 10, 225, 1102, 21, 10, 225, 1102, 6, 14, 225, 1102, 94, 17, 225, 1, 40, 173,
            224, 1001, 224, -90, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224, 1, 224, 223,
            223, 2, 35, 44, 224, 101, -80, 224, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1,
            223, 224, 223, 1101, 26, 94, 224, 101, -120, 224, 224, 4, 224, 102, 8, 223, 223, 1001,
            224, 7, 224, 1, 224, 223, 223, 1001, 52, 70, 224, 101, -87, 224, 224, 4, 224, 1002,
            223, 8, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 1101, 16, 92, 225, 1101, 59, 24, 225,
            102, 83, 48, 224, 101, -1162, 224, 224, 4, 224, 102, 8, 223, 223, 101, 4, 224, 224, 1,
            223, 224, 223, 1101, 80, 10, 225, 101, 5, 143, 224, 1001, 224, -21, 224, 4, 224, 1002,
            223, 8, 223, 1001, 224, 6, 224, 1, 223, 224, 223, 1102, 94, 67, 224, 101, -6298, 224,
            224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 4, 223, 99, 0, 0,
            0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1,
            99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265,
            1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1,
            99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 108, 677,
            677, 224, 102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223, 1107, 677, 226, 224, 102,
            2, 223, 223, 1006, 224, 344, 101, 1, 223, 223, 1107, 226, 226, 224, 102, 2, 223, 223,
            1006, 224, 359, 101, 1, 223, 223, 1108, 677, 677, 224, 102, 2, 223, 223, 1005, 224,
            374, 101, 1, 223, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 389, 101, 1,
            223, 223, 108, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 404, 1001, 223, 1, 223,
            107, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 419, 101, 1, 223, 223, 1007, 226, 226,
            224, 102, 2, 223, 223, 1005, 224, 434, 101, 1, 223, 223, 1007, 677, 677, 224, 102, 2,
            223, 223, 1005, 224, 449, 1001, 223, 1, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1006,
            224, 464, 101, 1, 223, 223, 1108, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 479,
            101, 1, 223, 223, 7, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 494, 101, 1, 223,
            223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 509, 1001, 223, 1, 223, 1007,
            226, 677, 224, 1002, 223, 2, 223, 1006, 224, 524, 1001, 223, 1, 223, 107, 226, 226,
            224, 1002, 223, 2, 223, 1006, 224, 539, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2,
            223, 223, 1005, 224, 554, 101, 1, 223, 223, 1108, 226, 677, 224, 102, 2, 223, 223,
            1006, 224, 569, 101, 1, 223, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224,
            584, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 599, 101, 1,
            223, 223, 8, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 614, 101, 1, 223, 223, 7, 226,
            677, 224, 1002, 223, 2, 223, 1005, 224, 629, 101, 1, 223, 223, 1008, 226, 677, 224,
            1002, 223, 2, 223, 1006, 224, 644, 101, 1, 223, 223, 107, 226, 677, 224, 1002, 223, 2,
            223, 1005, 224, 659, 1001, 223, 1, 223, 1008, 226, 226, 224, 1002, 223, 2, 223, 1006,
            224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
        ];

        let mut i2 = input.clone();
        i2.resize(8096, 0);
        let output = execute_with_input(i2, 1);
        assert_eq!(*output.0.last().unwrap(), 7566643);
    }
    #[test]
    fn part1() {
        let input = vec![
            1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1102, 1, 3, 1000,
            109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65,
            1008, 1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99,
            4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1101, 0, 493, 1024, 1102, 1, 38, 1015, 1101,
            20, 0, 1011, 1101, 0, 509, 1026, 1101, 0, 32, 1018, 1101, 0, 333, 1022, 1102, 1, 0,
            1020, 1101, 326, 0, 1023, 1101, 0, 33, 1010, 1101, 21, 0, 1016, 1101, 25, 0, 1004,
            1102, 28, 1, 1008, 1102, 1, 506, 1027, 1102, 488, 1, 1025, 1101, 0, 27, 1013, 1101, 1,
            0, 1021, 1101, 0, 34, 1019, 1101, 607, 0, 1028, 1102, 1, 23, 1003, 1102, 26, 1, 1007,
            1102, 29, 1, 1009, 1101, 31, 0, 1000, 1102, 37, 1, 1012, 1101, 30, 0, 1005, 1101, 602,
            0, 1029, 1101, 36, 0, 1002, 1102, 1, 22, 1001, 1102, 1, 35, 1014, 1102, 24, 1, 1006,
            1102, 39, 1, 1017, 109, 4, 21102, 40, 1, 6, 1008, 1010, 40, 63, 1005, 63, 203, 4, 187,
            1106, 0, 207, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 13, 1206, 3, 221, 4, 213, 1106, 0,
            225, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -5, 1208, -9, 22, 63, 1005, 63, 241, 1106,
            0, 247, 4, 231, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -5, 21107, 41, 40, 3, 1005,
            1010, 263, 1106, 0, 269, 4, 253, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -1, 1202, 3, 1,
            63, 1008, 63, 29, 63, 1005, 63, 295, 4, 275, 1001, 64, 1, 64, 1106, 0, 295, 1002, 64,
            2, 64, 109, 16, 21108, 42, 42, -8, 1005, 1014, 313, 4, 301, 1105, 1, 317, 1001, 64, 1,
            64, 1002, 64, 2, 64, 109, -4, 2105, 1, 5, 1001, 64, 1, 64, 1105, 1, 335, 4, 323, 1002,
            64, 2, 64, 109, -5, 1207, -4, 28, 63, 1005, 63, 355, 1001, 64, 1, 64, 1105, 1, 357, 4,
            341, 1002, 64, 2, 64, 109, 2, 21102, 43, 1, -1, 1008, 1014, 45, 63, 1005, 63, 377,
            1106, 0, 383, 4, 363, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -10, 1208, -3, 36, 63,
            1005, 63, 401, 4, 389, 1106, 0, 405, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 6, 21107,
            44, 45, 1, 1005, 1012, 423, 4, 411, 1105, 1, 427, 1001, 64, 1, 64, 1002, 64, 2, 64,
            109, 4, 21101, 45, 0, 3, 1008, 1018, 45, 63, 1005, 63, 453, 4, 433, 1001, 64, 1, 64,
            1105, 1, 453, 1002, 64, 2, 64, 109, -23, 2101, 0, 10, 63, 1008, 63, 36, 63, 1005, 63,
            475, 4, 459, 1106, 0, 479, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 26, 2105, 1, 6, 4,
            485, 1105, 1, 497, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 4, 2106, 0, 5, 1105, 1, 515,
            4, 503, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -25, 1201, 10, 0, 63, 1008, 63, 26, 63,
            1005, 63, 537, 4, 521, 1105, 1, 541, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 18, 21101,
            46, 0, -1, 1008, 1014, 43, 63, 1005, 63, 565, 1001, 64, 1, 64, 1106, 0, 567, 4, 547,
            1002, 64, 2, 64, 109, -6, 1201, -4, 0, 63, 1008, 63, 33, 63, 1005, 63, 587, 1105, 1,
            593, 4, 573, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 22, 2106, 0, -3, 4, 599, 1105, 1,
            611, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -28, 2102, 1, -2, 63, 1008, 63, 22, 63,
            1005, 63, 633, 4, 617, 1105, 1, 637, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -1, 21108,
            47, 44, 9, 1005, 1011, 653, 1105, 1, 659, 4, 643, 1001, 64, 1, 64, 1002, 64, 2, 64,
            109, 10, 2107, 24, -8, 63, 1005, 63, 681, 4, 665, 1001, 64, 1, 64, 1105, 1, 681, 1002,
            64, 2, 64, 109, -11, 2107, 31, 4, 63, 1005, 63, 697, 1106, 0, 703, 4, 687, 1001, 64, 1,
            64, 1002, 64, 2, 64, 109, 8, 2101, 0, -8, 63, 1008, 63, 23, 63, 1005, 63, 727, 1001,
            64, 1, 64, 1105, 1, 729, 4, 709, 1002, 64, 2, 64, 109, -16, 2108, 21, 10, 63, 1005, 63,
            749, 1001, 64, 1, 64, 1106, 0, 751, 4, 735, 1002, 64, 2, 64, 109, 17, 2108, 36, -8, 63,
            1005, 63, 769, 4, 757, 1105, 1, 773, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -10, 1207,
            1, 23, 63, 1005, 63, 791, 4, 779, 1105, 1, 795, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
            -3, 2102, 1, 6, 63, 1008, 63, 22, 63, 1005, 63, 815, 1106, 0, 821, 4, 801, 1001, 64, 1,
            64, 1002, 64, 2, 64, 109, 16, 1205, 7, 837, 1001, 64, 1, 64, 1105, 1, 839, 4, 827,
            1002, 64, 2, 64, 109, -5, 1202, 0, 1, 63, 1008, 63, 30, 63, 1005, 63, 863, 1001, 64, 1,
            64, 1106, 0, 865, 4, 845, 1002, 64, 2, 64, 109, 4, 1205, 9, 883, 4, 871, 1001, 64, 1,
            64, 1106, 0, 883, 1002, 64, 2, 64, 109, 16, 1206, -7, 899, 1001, 64, 1, 64, 1106, 0,
            901, 4, 889, 4, 64, 99, 21102, 1, 27, 1, 21101, 915, 0, 0, 1105, 1, 922, 21201, 1,
            47633, 1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21102,
            942, 1, 0, 1105, 1, 922, 22102, 1, 1, -1, 21201, -2, -3, 1, 21101, 957, 0, 0, 1106, 0,
            922, 22201, 1, -1, -2, 1105, 1, 968, 22101, 0, -2, -2, 109, -3, 2106, 0, 0,
        ];

        let mut i2 = input.clone();
        i2.resize(8096, 0);
        let output = execute_with_input(i2, 1);
        assert_eq!(*output.0.last().unwrap(), 3345854957);
    }

    #[test]
    fn part2() {
        let input = vec![
            1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1102, 1, 3, 1000,
            109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65,
            1008, 1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99,
            4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1101, 0, 493, 1024, 1102, 1, 38, 1015, 1101,
            20, 0, 1011, 1101, 0, 509, 1026, 1101, 0, 32, 1018, 1101, 0, 333, 1022, 1102, 1, 0,
            1020, 1101, 326, 0, 1023, 1101, 0, 33, 1010, 1101, 21, 0, 1016, 1101, 25, 0, 1004,
            1102, 28, 1, 1008, 1102, 1, 506, 1027, 1102, 488, 1, 1025, 1101, 0, 27, 1013, 1101, 1,
            0, 1021, 1101, 0, 34, 1019, 1101, 607, 0, 1028, 1102, 1, 23, 1003, 1102, 26, 1, 1007,
            1102, 29, 1, 1009, 1101, 31, 0, 1000, 1102, 37, 1, 1012, 1101, 30, 0, 1005, 1101, 602,
            0, 1029, 1101, 36, 0, 1002, 1102, 1, 22, 1001, 1102, 1, 35, 1014, 1102, 24, 1, 1006,
            1102, 39, 1, 1017, 109, 4, 21102, 40, 1, 6, 1008, 1010, 40, 63, 1005, 63, 203, 4, 187,
            1106, 0, 207, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 13, 1206, 3, 221, 4, 213, 1106, 0,
            225, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -5, 1208, -9, 22, 63, 1005, 63, 241, 1106,
            0, 247, 4, 231, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -5, 21107, 41, 40, 3, 1005,
            1010, 263, 1106, 0, 269, 4, 253, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -1, 1202, 3, 1,
            63, 1008, 63, 29, 63, 1005, 63, 295, 4, 275, 1001, 64, 1, 64, 1106, 0, 295, 1002, 64,
            2, 64, 109, 16, 21108, 42, 42, -8, 1005, 1014, 313, 4, 301, 1105, 1, 317, 1001, 64, 1,
            64, 1002, 64, 2, 64, 109, -4, 2105, 1, 5, 1001, 64, 1, 64, 1105, 1, 335, 4, 323, 1002,
            64, 2, 64, 109, -5, 1207, -4, 28, 63, 1005, 63, 355, 1001, 64, 1, 64, 1105, 1, 357, 4,
            341, 1002, 64, 2, 64, 109, 2, 21102, 43, 1, -1, 1008, 1014, 45, 63, 1005, 63, 377,
            1106, 0, 383, 4, 363, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -10, 1208, -3, 36, 63,
            1005, 63, 401, 4, 389, 1106, 0, 405, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 6, 21107,
            44, 45, 1, 1005, 1012, 423, 4, 411, 1105, 1, 427, 1001, 64, 1, 64, 1002, 64, 2, 64,
            109, 4, 21101, 45, 0, 3, 1008, 1018, 45, 63, 1005, 63, 453, 4, 433, 1001, 64, 1, 64,
            1105, 1, 453, 1002, 64, 2, 64, 109, -23, 2101, 0, 10, 63, 1008, 63, 36, 63, 1005, 63,
            475, 4, 459, 1106, 0, 479, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 26, 2105, 1, 6, 4,
            485, 1105, 1, 497, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 4, 2106, 0, 5, 1105, 1, 515,
            4, 503, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -25, 1201, 10, 0, 63, 1008, 63, 26, 63,
            1005, 63, 537, 4, 521, 1105, 1, 541, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 18, 21101,
            46, 0, -1, 1008, 1014, 43, 63, 1005, 63, 565, 1001, 64, 1, 64, 1106, 0, 567, 4, 547,
            1002, 64, 2, 64, 109, -6, 1201, -4, 0, 63, 1008, 63, 33, 63, 1005, 63, 587, 1105, 1,
            593, 4, 573, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 22, 2106, 0, -3, 4, 599, 1105, 1,
            611, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -28, 2102, 1, -2, 63, 1008, 63, 22, 63,
            1005, 63, 633, 4, 617, 1105, 1, 637, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -1, 21108,
            47, 44, 9, 1005, 1011, 653, 1105, 1, 659, 4, 643, 1001, 64, 1, 64, 1002, 64, 2, 64,
            109, 10, 2107, 24, -8, 63, 1005, 63, 681, 4, 665, 1001, 64, 1, 64, 1105, 1, 681, 1002,
            64, 2, 64, 109, -11, 2107, 31, 4, 63, 1005, 63, 697, 1106, 0, 703, 4, 687, 1001, 64, 1,
            64, 1002, 64, 2, 64, 109, 8, 2101, 0, -8, 63, 1008, 63, 23, 63, 1005, 63, 727, 1001,
            64, 1, 64, 1105, 1, 729, 4, 709, 1002, 64, 2, 64, 109, -16, 2108, 21, 10, 63, 1005, 63,
            749, 1001, 64, 1, 64, 1106, 0, 751, 4, 735, 1002, 64, 2, 64, 109, 17, 2108, 36, -8, 63,
            1005, 63, 769, 4, 757, 1105, 1, 773, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -10, 1207,
            1, 23, 63, 1005, 63, 791, 4, 779, 1105, 1, 795, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
            -3, 2102, 1, 6, 63, 1008, 63, 22, 63, 1005, 63, 815, 1106, 0, 821, 4, 801, 1001, 64, 1,
            64, 1002, 64, 2, 64, 109, 16, 1205, 7, 837, 1001, 64, 1, 64, 1105, 1, 839, 4, 827,
            1002, 64, 2, 64, 109, -5, 1202, 0, 1, 63, 1008, 63, 30, 63, 1005, 63, 863, 1001, 64, 1,
            64, 1106, 0, 865, 4, 845, 1002, 64, 2, 64, 109, 4, 1205, 9, 883, 4, 871, 1001, 64, 1,
            64, 1106, 0, 883, 1002, 64, 2, 64, 109, 16, 1206, -7, 899, 1001, 64, 1, 64, 1106, 0,
            901, 4, 889, 4, 64, 99, 21102, 1, 27, 1, 21101, 915, 0, 0, 1105, 1, 922, 21201, 1,
            47633, 1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21102,
            942, 1, 0, 1105, 1, 922, 22102, 1, 1, -1, 21201, -2, -3, 1, 21101, 957, 0, 0, 1106, 0,
            922, 22201, 1, -1, -2, 1105, 1, 968, 22101, 0, -2, -2, 109, -3, 2106, 0, 0,
        ];

        let mut i2 = input.clone();
        i2.resize(65536, 0);

        let output = execute_with_input(i2, 2);

        assert_eq!(*output.0.last().unwrap(), 68938);
    }
}
