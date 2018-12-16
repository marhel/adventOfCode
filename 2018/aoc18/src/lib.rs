#[derive(Debug, PartialEq, Copy, Clone)]
struct CPU {
    regs[i32; 4]
}

impl CPU {
    fn new(regs: [i32; 4]) -> CPU {
        CPU { regs }
    }
}
const OP: usize = 0;
const A: usize = 1;
const B: usize = 2;
const C: usize = 3;

// Addition:
// 
// addr (add register) stores into register C the result of adding register A and register B.
// addi (add immediate) stores into register C the result of adding register A and value B.
fn addr(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = cpu.regs[operands[A]] + cpu.regs[operands[B]];
    cpu
}

fn addi(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = cpu.regs[operands[A]] + operands[B];
    cpu
}

// Multiplication:
// 
// mulr (multiply register) stores into register C the result of multiplying register A and register B.
// muli (multiply immediate) stores into register C the result of multiplying register A and value B.
fn mulr(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = cpu.regs[operands[A]] * cpu.regs[operands[B]];
    cpu
}

fn muli(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = cpu.regs[operands[A]] * operands[B];
    cpu
}

// Bitwise AND:
// 
// banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
// bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
fn banr(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = cpu.regs[operands[A]] & cpu.regs[operands[B]];
    cpu
}

fn bani(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = cpu.regs[operands[A]] & operands[B];
    cpu
}

// Bitwise OR:
// 
// borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
// bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
fn borr(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = cpu.regs[operands[A]] | cpu.regs[operands[B]];
    cpu
}

fn bori(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = cpu.regs[operands[A]] | operands[B];
    cpu
}

// Assignment:
// 
// setr (set register) copies the contents of register A into register C. (Input B is ignored.)
// seti (set immediate) stores value A into register C. (Input B is ignored.)
fn setr(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = cpu.regs[operands[A]];
    cpu
}

fn seti(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = operands[A];
    cpu
}

// Greater-than testing:
// 
// gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
// gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
// gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.

fn gtir(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = if operands[A] > cpu.regs[operands[B]] { 1 } else { 0 };
    cpu
}

fn gtri(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = if cpu.regs[operands[A]] > operands[B] { 1 } else { 0 };
    cpu
}

fn gtrr(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = if cpu.regs[operands[A]] > cpu.regs[operands[B]] { 1 } else { 0 };
    cpu
}

// Equality testing:
// 
// eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
// eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
// eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.

fn eqir(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = if operands[A] == cpu.regs[operands[B]] { 1 } else { 0 };
    cpu
}

fn eqri(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = if cpu.regs[operands[A]] == operands[B] { 1 } else { 0 };
    cpu
}

fn eqrr(cpu: &mut CPU, operands: [i32; 4]) -> &CPU {
    cpu.regs[operands[C]] = if cpu.regs[operands[A]] == cpu.regs[operands[B]] { 1 } else { 0 };
    cpu
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addr() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,3+4,6]);
        assert_eq!(*addr(&mut before, operands), after);
    }
    #[test]
    fn test_addi() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,3+8,6]);
        assert_eq!(*addi(&mut before, operands), after);
    }
    #[test]
    fn test_mulr() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,3*4,6]);
        assert_eq!(*mulr(&mut before, operands), after);
    }
    #[test]
    fn test_muli() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,3*8,6]);
        assert_eq!(*muli(&mut before, operands), after);
    }
    #[test]
    fn test_banr() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,3&4,6]);
        assert_eq!(*banr(&mut before, operands), after);
    }
    #[test]
    fn test_bani() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,3&8,6]);
        assert_eq!(*bani(&mut before, operands), after);
    }
    #[test]
    fn test_borr() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,3|4,6]);
        assert_eq!(*borr(&mut before, operands), after);
    }
    #[test]
    fn test_bori() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,3|8,6]);
        assert_eq!(*bori(&mut before, operands), after);
    }
    #[test]
    fn test_setr() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,3,6]);
        assert_eq!(*setr(&mut before, operands), after);
    }
    #[test]
    fn test_seti() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,9,6]);
        assert_eq!(*seti(&mut before, operands), after);
    }
    #[test]
    fn test_gtir() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,1,6]);
        assert_eq!(*gtir(&mut before, operands), after);
    }
    fn test_gtir_2() {
        let operands = [0,0,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,0,6]);
        assert_eq!(*gtir(&mut before, operands), after);
    }
    #[test]
    fn test_gtri() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,0,6]);
        assert_eq!(*gtri(&mut before, operands), after);
    }
    #[test]
    fn test_gtri_2() {
        let operands = [0,9,0,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,1,6]);
        assert_eq!(*gtri(&mut before, operands), after);
    }
    #[test]
    fn test_gtrr() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,0,6]);
        assert_eq!(*gtrr(&mut before, operands), after);
    }
    #[test]
    fn test_gtrr_2() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([4,3,5,6]);
        let after = CPU::new([4,3,1,6]);
        assert_eq!(*gtrr(&mut before, operands), after);
    }
    #[test]
    fn test_eqir() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,0,6]);
        assert_eq!(*eqir(&mut before, operands), after);
    }
    #[test]
    fn test_eqri() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,0,6]);
        assert_eq!(*eqri(&mut before, operands), after);
    }
    #[test]
    fn test_eqrr() {
        let operands = [0,9,8,7];
        let mut before = CPU::new([3,4,5,6]);
        let after = CPU::new([3,4,0,6]);
        assert_eq!(*eqrr(&mut before, operands), after);
    }

    fn check_sample(before: [i32; 4], operands: [i32; 4], after: [i32; 4]) -> i32 {
        let mut behaves = 0;
        let before = CPU::new(before);
        let after = CPU::new(after);
        behaves += if *addr(&mut before.clone(), operands) == after { println!("as addr"); 1 } else { 0 };
        behaves += if *addi(&mut before.clone(), operands) == after { println!("as addi"); 1 } else { 0 };
        behaves += if *mulr(&mut before.clone(), operands) == after { println!("as mulr"); 1 } else { 0 };
        behaves += if *muli(&mut before.clone(), operands) == after { println!("as muli"); 1 } else { 0 };
        behaves += if *banr(&mut before.clone(), operands) == after { println!("as banr"); 1 } else { 0 };
        behaves += if *bani(&mut before.clone(), operands) == after { println!("as bani"); 1 } else { 0 };
        behaves += if *borr(&mut before.clone(), operands) == after { println!("as borr"); 1 } else { 0 };
        behaves += if *bori(&mut before.clone(), operands) == after { println!("as bori"); 1 } else { 0 };
        behaves += if *setr(&mut before.clone(), operands) == after { println!("as setr"); 1 } else { 0 };
        behaves += if *seti(&mut before.clone(), operands) == after { println!("as seti"); 1 } else { 0 };
        behaves += if *gtir(&mut before.clone(), operands) == after { println!("as gtir"); 1 } else { 0 };
        behaves += if *gtri(&mut before.clone(), operands) == after { println!("as gtri"); 1 } else { 0 };
        behaves += if *gtrr(&mut before.clone(), operands) == after { println!("as gtrr"); 1 } else { 0 };
        behaves += if *eqir(&mut before.clone(), operands) == after { println!("as eqir"); 1 } else { 0 };
        behaves += if *eqri(&mut before.clone(), operands) == after { println!("as eqri"); 1 } else { 0 };
        behaves += if *eqrr(&mut before.clone(), operands) == after { println!("as eqrr"); 1 } else { 0 };
        if behaves >= 3 {
            // println!("behaves {:?}", behaves);
            1 
        } else { 0 }
    }

    #[test]
    fn part1_test() {
        assert_eq!(1, check_sample([3, 2, 1, 1], [9, 2, 1, 2], [3, 2, 2, 1]));
    }
    #[test]
    fn part1_test2() {
        let operands = [9, 2, 1, 2];
        let before = CPU::new([3, 2, 1, 1]);
        let after = CPU::new([3, 2, 2, 1]);
        println!("addi {:?}", addi(&mut before.clone(), operands));
        println!("mulr {:?}", mulr(&mut before.clone(), operands));
        println!("seti {:?}", seti(&mut before.clone(), operands));
        assert_eq!(1, check_sample([3, 2, 1, 1], [9, 2, 1, 2], [3, 2, 2, 1]));
    }

    #[test]
    fn part1() {
        // how many samples in your puzzle input behave like three or more opcodes?

        let mut behaves_like_3 = 0;
        behaves_like_3 += check_sample([2, 0, 0, 1], [15, 3, 1, 3], [2, 0, 0, 1]); 
        behaves_like_3 += check_sample([3, 2, 3, 3], [4, 3, 3, 0], [3, 2, 3, 3]); 
        behaves_like_3 += check_sample([3, 2, 1, 3], [12, 3, 0, 0], [1, 2, 1, 3]); 
        behaves_like_3 += check_sample([1, 2, 2, 2], [2, 0, 2, 0], [0, 2, 2, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 1], [10, 1, 0, 1], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 3, 2], [11, 0, 3, 3], [2, 1, 3, 1]); 
        behaves_like_3 += check_sample([1, 2, 1, 1], [9, 2, 0, 2], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([3, 0, 1, 1], [15, 3, 1, 3], [3, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 1], [10, 1, 0, 3], [0, 1, 1, 1]); 
        behaves_like_3 += check_sample([1, 3, 2, 2], [2, 0, 2, 2], [1, 3, 0, 2]); 
        behaves_like_3 += check_sample([0, 2, 2, 3], [13, 3, 2, 3], [0, 2, 2, 0]); 
        behaves_like_3 += check_sample([2, 1, 3, 1], [12, 2, 3, 2], [2, 1, 0, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [7, 0, 2, 1], [2, 1, 2, 2]); 
        behaves_like_3 += check_sample([2, 2, 0, 2], [11, 0, 3, 2], [2, 2, 1, 2]); 
        behaves_like_3 += check_sample([3, 0, 3, 2], [3, 3, 3, 3], [3, 0, 3, 0]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [9, 2, 0, 2], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 0], [5, 1, 3, 0], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([1, 0, 1, 0], [9, 2, 0, 3], [1, 0, 1, 2]); 
        behaves_like_3 += check_sample([2, 1, 0, 3], [0, 1, 3, 0], [0, 1, 0, 3]); 
        behaves_like_3 += check_sample([2, 3, 2, 0], [12, 2, 0, 3], [2, 3, 2, 1]); 
        behaves_like_3 += check_sample([0, 0, 3, 2], [1, 0, 0, 0], [0, 0, 3, 2]); 
        behaves_like_3 += check_sample([2, 0, 2, 1], [14, 3, 2, 2], [2, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 0, 2, 1], [15, 3, 1, 3], [0, 0, 2, 1]); 
        behaves_like_3 += check_sample([2, 0, 1, 3], [13, 3, 2, 3], [2, 0, 1, 0]); 
        behaves_like_3 += check_sample([3, 1, 3, 3], [7, 2, 3, 2], [3, 1, 1, 3]); 
        behaves_like_3 += check_sample([1, 1, 1, 1], [3, 2, 3, 3], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [14, 3, 2, 1], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [2, 0, 2, 3], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 3, 0, 1], [3, 3, 3, 2], [2, 3, 0, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [2, 0, 2, 2], [1, 1, 0, 0]); 
        behaves_like_3 += check_sample([2, 1, 3, 0], [5, 1, 3, 3], [2, 1, 3, 1]); 
        behaves_like_3 += check_sample([2, 3, 2, 1], [3, 3, 3, 0], [0, 3, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 0, 2], [10, 1, 0, 2], [0, 1, 1, 2]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [4, 2, 2, 3], [1, 1, 2, 2]); 
        behaves_like_3 += check_sample([3, 1, 0, 2], [6, 1, 3, 1], [3, 0, 0, 2]); 
        behaves_like_3 += check_sample([3, 0, 3, 1], [15, 3, 1, 0], [1, 0, 3, 1]); 
        behaves_like_3 += check_sample([3, 1, 2, 0], [8, 1, 2, 0], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 1, 1], [9, 2, 3, 2], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 3, 0], [5, 1, 3, 3], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([3, 1, 1, 3], [7, 0, 3, 3], [3, 1, 1, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [14, 3, 2, 0], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 0], [5, 1, 3, 0], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 1, 0], [5, 1, 3, 1], [0, 1, 1, 0]); 
        behaves_like_3 += check_sample([2, 1, 1, 3], [13, 3, 2, 1], [2, 0, 1, 3]); 
        behaves_like_3 += check_sample([2, 0, 1, 3], [15, 2, 1, 0], [1, 0, 1, 3]); 
        behaves_like_3 += check_sample([2, 3, 2, 2], [11, 0, 3, 3], [2, 3, 2, 1]); 
        behaves_like_3 += check_sample([3, 3, 2, 1], [14, 3, 2, 0], [1, 3, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 1, 2], [6, 1, 3, 1], [2, 0, 1, 2]); 
        behaves_like_3 += check_sample([3, 1, 0, 3], [7, 0, 3, 0], [1, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 3, 1, 3], [13, 3, 3, 2], [0, 3, 1, 3]); 
        behaves_like_3 += check_sample([2, 1, 3, 2], [6, 1, 3, 2], [2, 1, 0, 2]); 
        behaves_like_3 += check_sample([3, 2, 2, 3], [13, 3, 1, 0], [0, 2, 2, 3]); 
        behaves_like_3 += check_sample([1, 1, 1, 0], [5, 1, 3, 1], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 1], [14, 3, 2, 0], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([3, 3, 0, 3], [4, 3, 3, 1], [3, 3, 0, 3]); 
        behaves_like_3 += check_sample([0, 2, 0, 3], [13, 3, 3, 3], [0, 2, 0, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [2, 0, 2, 1], [1, 0, 2, 2]); 
        behaves_like_3 += check_sample([1, 3, 1, 3], [9, 2, 0, 2], [1, 3, 2, 3]); 
        behaves_like_3 += check_sample([3, 1, 1, 2], [6, 1, 3, 2], [3, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 0, 1, 3], [13, 0, 0, 3], [0, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [8, 1, 2, 0], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 2, 2, 1], [12, 2, 0, 1], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 0, 2, 0], [15, 0, 1, 1], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 3, 0], [5, 1, 3, 3], [1, 1, 3, 1]); 
        behaves_like_3 += check_sample([0, 0, 2, 1], [14, 3, 2, 2], [0, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 0, 3], [4, 3, 3, 3], [1, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 0, 2, 3], [1, 0, 0, 2], [0, 0, 0, 3]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [8, 1, 2, 3], [2, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [10, 1, 0, 1], [0, 1, 0, 3]); 
        behaves_like_3 += check_sample([1, 2, 2, 3], [2, 0, 2, 3], [1, 2, 2, 0]); 
        behaves_like_3 += check_sample([2, 1, 0, 2], [11, 0, 3, 3], [2, 1, 0, 1]); 
        behaves_like_3 += check_sample([0, 2, 0, 2], [1, 0, 0, 0], [0, 2, 0, 2]); 
        behaves_like_3 += check_sample([1, 3, 2, 0], [2, 0, 2, 1], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [3, 3, 3, 0], [0, 0, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [4, 2, 2, 1], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [15, 0, 1, 0], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 3], [10, 1, 0, 1], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([0, 3, 1, 1], [3, 3, 3, 1], [0, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 2], [6, 1, 3, 1], [0, 0, 1, 2]); 
        behaves_like_3 += check_sample([0, 0, 3, 3], [1, 0, 0, 1], [0, 0, 3, 3]); 
        behaves_like_3 += check_sample([1, 3, 1, 1], [9, 2, 0, 2], [1, 3, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 3, 3], [10, 1, 0, 0], [1, 1, 3, 3]); 
        behaves_like_3 += check_sample([2, 0, 1, 3], [4, 3, 3, 2], [2, 0, 3, 3]); 
        behaves_like_3 += check_sample([2, 1, 1, 1], [9, 2, 3, 0], [2, 1, 1, 1]); 
        behaves_like_3 += check_sample([3, 0, 3, 3], [12, 3, 2, 3], [3, 0, 3, 1]); 
        behaves_like_3 += check_sample([3, 1, 0, 2], [6, 1, 3, 3], [3, 1, 0, 0]); 
        behaves_like_3 += check_sample([0, 3, 1, 3], [4, 3, 3, 3], [0, 3, 1, 3]); 
        behaves_like_3 += check_sample([3, 0, 1, 3], [12, 3, 0, 3], [3, 0, 1, 1]); 
        behaves_like_3 += check_sample([3, 1, 3, 3], [0, 1, 3, 1], [3, 0, 3, 3]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [8, 1, 2, 0], [0, 1, 2, 2]); 
        behaves_like_3 += check_sample([2, 2, 3, 1], [3, 3, 3, 1], [2, 0, 3, 1]); 
        behaves_like_3 += check_sample([1, 0, 1, 0], [9, 2, 0, 2], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 0], [8, 1, 2, 2], [3, 1, 0, 0]); 
        behaves_like_3 += check_sample([3, 2, 1, 3], [7, 0, 3, 1], [3, 1, 1, 3]); 
        behaves_like_3 += check_sample([0, 1, 0, 1], [10, 1, 0, 1], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([1, 1, 3, 3], [13, 3, 1, 2], [1, 1, 0, 3]); 
        behaves_like_3 += check_sample([3, 1, 0, 3], [13, 3, 3, 2], [3, 1, 1, 3]); 
        behaves_like_3 += check_sample([0, 0, 0, 2], [3, 3, 3, 3], [0, 0, 0, 0]); 
        behaves_like_3 += check_sample([2, 0, 3, 2], [11, 0, 3, 3], [2, 0, 3, 1]); 
        behaves_like_3 += check_sample([3, 1, 3, 3], [12, 3, 2, 2], [3, 1, 1, 3]); 
        behaves_like_3 += check_sample([2, 0, 2, 1], [4, 2, 2, 0], [2, 0, 2, 1]); 
        behaves_like_3 += check_sample([0, 3, 3, 1], [12, 2, 3, 3], [0, 3, 3, 0]); 
        behaves_like_3 += check_sample([2, 1, 2, 1], [8, 1, 2, 2], [2, 1, 0, 1]); 
        behaves_like_3 += check_sample([2, 2, 2, 2], [11, 0, 3, 1], [2, 1, 2, 2]); 
        behaves_like_3 += check_sample([1, 2, 1, 2], [3, 3, 3, 2], [1, 2, 0, 2]); 
        behaves_like_3 += check_sample([3, 2, 1, 1], [9, 2, 3, 3], [3, 2, 1, 2]); 
        behaves_like_3 += check_sample([3, 0, 3, 1], [3, 3, 3, 1], [3, 0, 3, 1]); 
        behaves_like_3 += check_sample([2, 2, 0, 2], [11, 0, 3, 0], [1, 2, 0, 2]); 
        behaves_like_3 += check_sample([2, 2, 1, 1], [3, 2, 3, 3], [2, 2, 1, 0]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [6, 1, 3, 2], [1, 1, 0, 2]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [6, 1, 3, 0], [0, 1, 2, 2]); 
        behaves_like_3 += check_sample([2, 1, 2, 0], [7, 0, 2, 1], [2, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [6, 1, 3, 3], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([3, 0, 2, 3], [7, 0, 3, 2], [3, 0, 1, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 1], [8, 1, 2, 2], [3, 1, 0, 1]); 
        behaves_like_3 += check_sample([1, 0, 2, 3], [15, 0, 1, 2], [1, 0, 1, 3]); 
        behaves_like_3 += check_sample([1, 0, 2, 3], [15, 0, 1, 1], [1, 1, 2, 3]); 
        behaves_like_3 += check_sample([2, 2, 1, 2], [11, 0, 3, 2], [2, 2, 1, 2]); 
        behaves_like_3 += check_sample([0, 2, 1, 1], [9, 2, 3, 0], [2, 2, 1, 1]); 
        behaves_like_3 += check_sample([0, 0, 1, 1], [15, 2, 1, 0], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 0], [2, 0, 2, 1], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([0, 0, 2, 3], [13, 0, 0, 3], [0, 0, 2, 1]); 
        behaves_like_3 += check_sample([2, 0, 1, 2], [11, 0, 3, 0], [1, 0, 1, 2]); 
        behaves_like_3 += check_sample([3, 0, 2, 1], [14, 3, 2, 0], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([0, 3, 3, 3], [7, 2, 3, 3], [0, 3, 3, 1]); 
        behaves_like_3 += check_sample([0, 2, 1, 3], [0, 2, 3, 1], [0, 0, 1, 3]); 
        behaves_like_3 += check_sample([3, 1, 0, 2], [6, 1, 3, 2], [3, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 3, 2, 2], [11, 0, 3, 1], [2, 1, 2, 2]); 
        behaves_like_3 += check_sample([2, 1, 1, 0], [5, 1, 3, 2], [2, 1, 1, 0]); 
        behaves_like_3 += check_sample([2, 1, 2, 0], [8, 1, 2, 3], [2, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [4, 3, 3, 1], [1, 3, 2, 3]); 
        behaves_like_3 += check_sample([1, 1, 1, 0], [9, 2, 0, 3], [1, 1, 1, 2]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [11, 0, 3, 1], [2, 1, 2, 2]); 
        behaves_like_3 += check_sample([0, 0, 1, 1], [1, 0, 0, 1], [0, 0, 1, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [6, 1, 3, 0], [0, 1, 2, 2]); 
        behaves_like_3 += check_sample([0, 1, 1, 0], [5, 1, 3, 3], [0, 1, 1, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [2, 0, 2, 1], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 1], [8, 1, 2, 0], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [4, 2, 2, 2], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 2, 1, 1], [9, 2, 3, 3], [0, 2, 1, 2]); 
        behaves_like_3 += check_sample([1, 0, 1, 3], [9, 2, 0, 1], [1, 2, 1, 3]); 
        behaves_like_3 += check_sample([2, 3, 2, 1], [14, 3, 2, 0], [1, 3, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 1], [14, 3, 2, 3], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 3, 3], [4, 3, 3, 2], [0, 1, 3, 3]); 
        behaves_like_3 += check_sample([0, 2, 1, 3], [0, 1, 3, 3], [0, 2, 1, 0]); 
        behaves_like_3 += check_sample([0, 1, 0, 0], [10, 1, 0, 2], [0, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [10, 1, 0, 0], [1, 1, 2, 2]); 
        behaves_like_3 += check_sample([1, 1, 0, 0], [5, 1, 3, 0], [1, 1, 0, 0]); 
        behaves_like_3 += check_sample([0, 0, 1, 3], [0, 2, 3, 0], [0, 0, 1, 3]); 
        behaves_like_3 += check_sample([1, 3, 2, 2], [2, 0, 2, 0], [0, 3, 2, 2]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [10, 1, 0, 0], [1, 1, 0, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 0], [5, 1, 3, 2], [3, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 0, 2, 1], [14, 3, 2, 0], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([1, 3, 2, 2], [2, 0, 2, 3], [1, 3, 2, 0]); 
        behaves_like_3 += check_sample([0, 3, 2, 1], [1, 0, 0, 1], [0, 0, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 1, 2], [9, 2, 0, 1], [1, 2, 1, 2]); 
        behaves_like_3 += check_sample([0, 1, 1, 1], [10, 1, 0, 2], [0, 1, 1, 1]); 
        behaves_like_3 += check_sample([2, 3, 1, 1], [9, 2, 3, 0], [2, 3, 1, 1]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [15, 3, 1, 0], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 2], [10, 1, 0, 2], [0, 1, 1, 2]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [14, 3, 2, 0], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 1, 3], [4, 3, 3, 1], [3, 3, 1, 3]); 
        behaves_like_3 += check_sample([3, 0, 0, 3], [7, 0, 3, 2], [3, 0, 1, 3]); 
        behaves_like_3 += check_sample([1, 3, 3, 3], [4, 3, 3, 3], [1, 3, 3, 3]); 
        behaves_like_3 += check_sample([0, 1, 3, 2], [10, 1, 0, 3], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([0, 3, 3, 3], [13, 3, 3, 1], [0, 1, 3, 3]); 
        behaves_like_3 += check_sample([0, 0, 1, 2], [3, 3, 3, 2], [0, 0, 0, 2]); 
        behaves_like_3 += check_sample([3, 2, 3, 3], [12, 3, 0, 0], [1, 2, 3, 3]); 
        behaves_like_3 += check_sample([1, 1, 3, 3], [7, 2, 3, 0], [1, 1, 3, 3]); 
        behaves_like_3 += check_sample([2, 0, 1, 3], [15, 2, 1, 1], [2, 1, 1, 3]); 
        behaves_like_3 += check_sample([0, 3, 2, 3], [0, 2, 3, 1], [0, 0, 2, 3]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [7, 0, 2, 3], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 0, 0], [5, 1, 3, 2], [2, 1, 1, 0]); 
        behaves_like_3 += check_sample([3, 3, 2, 1], [3, 3, 3, 0], [0, 3, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [5, 1, 3, 3], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [2, 0, 2, 1], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([2, 0, 1, 1], [9, 2, 3, 3], [2, 0, 1, 2]); 
        behaves_like_3 += check_sample([1, 0, 2, 0], [2, 0, 2, 2], [1, 0, 0, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 1], [8, 1, 2, 0], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 3, 1, 3], [13, 3, 2, 1], [1, 0, 1, 3]); 
        behaves_like_3 += check_sample([0, 1, 3, 0], [10, 1, 0, 0], [1, 1, 3, 0]); 
        behaves_like_3 += check_sample([0, 2, 2, 2], [1, 0, 0, 2], [0, 2, 0, 2]); 
        behaves_like_3 += check_sample([1, 1, 0, 1], [3, 3, 3, 2], [1, 1, 0, 1]); 
        behaves_like_3 += check_sample([3, 1, 3, 3], [12, 3, 2, 1], [3, 1, 3, 3]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [12, 2, 1, 1], [2, 1, 2, 3]); 
        behaves_like_3 += check_sample([0, 1, 0, 0], [5, 1, 3, 0], [1, 1, 0, 0]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [9, 2, 3, 3], [1, 0, 1, 2]); 
        behaves_like_3 += check_sample([0, 1, 3, 2], [6, 1, 3, 1], [0, 0, 3, 2]); 
        behaves_like_3 += check_sample([2, 0, 2, 1], [14, 3, 2, 0], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([1, 0, 2, 0], [2, 0, 2, 3], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 1, 2], [6, 1, 3, 3], [0, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 0, 2, 0], [1, 0, 0, 0], [0, 0, 2, 0]); 
        behaves_like_3 += check_sample([0, 0, 3, 1], [15, 3, 1, 1], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([1, 0, 1, 3], [9, 2, 0, 3], [1, 0, 1, 2]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [12, 2, 1, 0], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 1, 1], [9, 2, 3, 1], [3, 2, 1, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 0], [5, 1, 3, 3], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([3, 2, 1, 3], [0, 1, 3, 2], [3, 2, 0, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 3], [12, 3, 0, 0], [1, 1, 2, 3]); 
        behaves_like_3 += check_sample([0, 0, 0, 1], [3, 3, 3, 3], [0, 0, 0, 0]); 
        behaves_like_3 += check_sample([0, 2, 2, 1], [1, 0, 0, 3], [0, 2, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [8, 1, 2, 0], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 3, 0], [5, 1, 3, 2], [2, 1, 1, 0]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [8, 1, 2, 1], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([3, 0, 2, 3], [0, 2, 3, 1], [3, 0, 2, 3]); 
        behaves_like_3 += check_sample([0, 3, 2, 1], [14, 3, 2, 0], [1, 3, 2, 1]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [0, 2, 3, 3], [2, 2, 2, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 2], [6, 1, 3, 0], [0, 1, 2, 2]); 
        behaves_like_3 += check_sample([1, 2, 1, 1], [9, 2, 0, 3], [1, 2, 1, 2]); 
        behaves_like_3 += check_sample([3, 1, 0, 3], [12, 3, 0, 3], [3, 1, 0, 1]); 
        behaves_like_3 += check_sample([1, 2, 1, 1], [9, 2, 3, 1], [1, 2, 1, 1]); 
        behaves_like_3 += check_sample([2, 0, 2, 0], [7, 0, 2, 3], [2, 0, 2, 1]); 
        behaves_like_3 += check_sample([3, 2, 0, 3], [12, 3, 0, 1], [3, 1, 0, 3]); 
        behaves_like_3 += check_sample([2, 3, 2, 0], [7, 0, 2, 1], [2, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 1, 3], [0, 2, 3, 3], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [2, 0, 2, 0], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 3, 2, 1], [14, 3, 2, 3], [1, 3, 2, 1]); 
        behaves_like_3 += check_sample([1, 2, 1, 3], [9, 2, 0, 2], [1, 2, 2, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 3], [8, 1, 2, 2], [3, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 3, 3, 3], [12, 3, 2, 0], [1, 3, 3, 3]); 
        behaves_like_3 += check_sample([1, 0, 2, 2], [15, 0, 1, 2], [1, 0, 1, 2]); 
        behaves_like_3 += check_sample([2, 2, 3, 2], [11, 0, 3, 0], [1, 2, 3, 2]); 
        behaves_like_3 += check_sample([3, 1, 2, 1], [14, 3, 2, 2], [3, 1, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 1, 0], [5, 1, 3, 0], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([3, 1, 1, 1], [3, 2, 3, 1], [3, 0, 1, 1]); 
        behaves_like_3 += check_sample([2, 1, 3, 3], [7, 2, 3, 0], [1, 1, 3, 3]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [2, 0, 2, 2], [1, 1, 0, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 0], [5, 1, 3, 0], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 2, 1, 1], [9, 2, 3, 1], [2, 2, 1, 1]); 
        behaves_like_3 += check_sample([2, 0, 2, 0], [4, 2, 2, 2], [2, 0, 2, 0]); 
        behaves_like_3 += check_sample([0, 2, 1, 2], [3, 3, 3, 3], [0, 2, 1, 0]); 
        behaves_like_3 += check_sample([3, 0, 1, 3], [7, 0, 3, 1], [3, 1, 1, 3]); 
        behaves_like_3 += check_sample([2, 2, 3, 2], [11, 0, 3, 3], [2, 2, 3, 1]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [0, 2, 3, 1], [2, 0, 2, 3]); 
        behaves_like_3 += check_sample([0, 1, 3, 0], [5, 1, 3, 2], [0, 1, 1, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 2], [6, 1, 3, 2], [3, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 2, 0, 3], [4, 3, 3, 3], [0, 2, 0, 3]); 
        behaves_like_3 += check_sample([2, 2, 3, 3], [7, 2, 3, 3], [2, 2, 3, 1]); 
        behaves_like_3 += check_sample([0, 3, 1, 2], [3, 3, 3, 2], [0, 3, 0, 2]); 
        behaves_like_3 += check_sample([0, 3, 3, 0], [1, 0, 0, 0], [0, 3, 3, 0]); 
        behaves_like_3 += check_sample([0, 3, 3, 3], [13, 0, 0, 0], [1, 3, 3, 3]); 
        behaves_like_3 += check_sample([3, 0, 1, 2], [3, 3, 3, 3], [3, 0, 1, 0]); 
        behaves_like_3 += check_sample([0, 1, 1, 1], [3, 2, 3, 1], [0, 0, 1, 1]); 
        behaves_like_3 += check_sample([2, 2, 2, 1], [12, 2, 1, 2], [2, 2, 1, 1]); 
        behaves_like_3 += check_sample([2, 2, 3, 2], [11, 0, 3, 2], [2, 2, 1, 2]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [4, 3, 3, 0], [3, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 3, 1, 1], [9, 2, 3, 3], [0, 3, 1, 2]); 
        behaves_like_3 += check_sample([2, 1, 0, 0], [5, 1, 3, 1], [2, 1, 0, 0]); 
        behaves_like_3 += check_sample([0, 1, 3, 2], [6, 1, 3, 0], [0, 1, 3, 2]); 
        behaves_like_3 += check_sample([2, 3, 0, 2], [11, 0, 3, 2], [2, 3, 1, 2]); 
        behaves_like_3 += check_sample([2, 1, 2, 1], [14, 3, 2, 0], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 3], [7, 0, 2, 2], [2, 1, 1, 3]); 
        behaves_like_3 += check_sample([2, 1, 3, 3], [7, 2, 3, 2], [2, 1, 1, 3]); 
        behaves_like_3 += check_sample([1, 2, 1, 1], [3, 2, 3, 2], [1, 2, 0, 1]); 
        behaves_like_3 += check_sample([2, 3, 1, 1], [9, 2, 3, 2], [2, 3, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 1], [8, 1, 2, 3], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([3, 2, 2, 1], [14, 3, 2, 0], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 3], [8, 1, 2, 1], [2, 0, 2, 3]); 
        behaves_like_3 += check_sample([2, 1, 2, 1], [14, 3, 2, 3], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 0, 3, 0], [1, 0, 0, 0], [0, 0, 3, 0]); 
        behaves_like_3 += check_sample([3, 2, 2, 3], [7, 0, 3, 1], [3, 1, 2, 3]); 
        behaves_like_3 += check_sample([1, 3, 2, 0], [2, 0, 2, 2], [1, 3, 0, 0]); 
        behaves_like_3 += check_sample([1, 2, 0, 3], [13, 3, 1, 0], [0, 2, 0, 3]); 
        behaves_like_3 += check_sample([2, 3, 0, 2], [11, 0, 3, 3], [2, 3, 0, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [14, 3, 2, 2], [1, 2, 1, 1]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [4, 3, 3, 3], [0, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [5, 1, 3, 1], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 2, 2, 3], [1, 0, 0, 3], [0, 2, 2, 0]); 
        behaves_like_3 += check_sample([2, 1, 3, 1], [12, 2, 3, 0], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [10, 1, 0, 1], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 1, 3], [0, 1, 3, 2], [1, 1, 0, 3]); 
        behaves_like_3 += check_sample([2, 2, 2, 2], [11, 0, 3, 0], [1, 2, 2, 2]); 
        behaves_like_3 += check_sample([0, 2, 1, 0], [1, 0, 0, 2], [0, 2, 0, 0]); 
        behaves_like_3 += check_sample([0, 3, 1, 1], [9, 2, 3, 2], [0, 3, 2, 1]); 
        behaves_like_3 += check_sample([0, 3, 3, 1], [13, 0, 0, 0], [1, 3, 3, 1]); 
        behaves_like_3 += check_sample([0, 1, 3, 3], [4, 3, 3, 3], [0, 1, 3, 3]); 
        behaves_like_3 += check_sample([0, 2, 2, 0], [1, 0, 0, 1], [0, 0, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 1, 3], [0, 1, 3, 0], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([1, 3, 1, 3], [9, 2, 0, 3], [1, 3, 1, 2]); 
        behaves_like_3 += check_sample([3, 1, 1, 1], [3, 3, 3, 2], [3, 1, 0, 1]); 
        behaves_like_3 += check_sample([3, 1, 2, 3], [0, 1, 3, 1], [3, 0, 2, 3]); 
        behaves_like_3 += check_sample([1, 3, 1, 1], [9, 2, 3, 0], [2, 3, 1, 1]); 
        behaves_like_3 += check_sample([3, 0, 1, 1], [15, 2, 1, 3], [3, 0, 1, 1]); 
        behaves_like_3 += check_sample([2, 0, 3, 2], [11, 0, 3, 1], [2, 1, 3, 2]); 
        behaves_like_3 += check_sample([1, 2, 1, 3], [4, 3, 3, 2], [1, 2, 3, 3]); 
        behaves_like_3 += check_sample([0, 3, 1, 3], [1, 0, 0, 2], [0, 3, 0, 3]); 
        behaves_like_3 += check_sample([0, 1, 2, 3], [8, 1, 2, 2], [0, 1, 0, 3]); 
        behaves_like_3 += check_sample([2, 3, 3, 1], [12, 2, 3, 2], [2, 3, 0, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [6, 1, 3, 1], [1, 0, 2, 2]); 
        behaves_like_3 += check_sample([0, 2, 2, 3], [4, 3, 3, 0], [3, 2, 2, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 1], [8, 1, 2, 3], [3, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 0, 1, 2], [11, 0, 3, 3], [2, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 0, 0, 2], [1, 0, 0, 0], [0, 0, 0, 2]); 
        behaves_like_3 += check_sample([0, 3, 0, 1], [1, 0, 0, 1], [0, 0, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 2], [6, 1, 3, 2], [0, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 0, 1, 2], [1, 0, 0, 1], [0, 0, 1, 2]); 
        behaves_like_3 += check_sample([3, 0, 0, 3], [12, 3, 0, 3], [3, 0, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 0, 0], [5, 1, 3, 2], [0, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 0, 3, 3], [1, 0, 0, 0], [0, 0, 3, 3]); 
        behaves_like_3 += check_sample([2, 1, 1, 2], [11, 0, 3, 0], [1, 1, 1, 2]); 
        behaves_like_3 += check_sample([1, 1, 0, 2], [6, 1, 3, 3], [1, 1, 0, 0]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [6, 1, 3, 1], [0, 0, 2, 2]); 
        behaves_like_3 += check_sample([1, 2, 2, 0], [2, 0, 2, 2], [1, 2, 0, 0]); 
        behaves_like_3 += check_sample([1, 1, 0, 2], [3, 3, 3, 1], [1, 0, 0, 2]); 
        behaves_like_3 += check_sample([2, 0, 1, 1], [15, 3, 1, 3], [2, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 2, 1, 0], [9, 2, 0, 1], [1, 2, 1, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 2], [6, 1, 3, 1], [3, 0, 2, 2]); 
        behaves_like_3 += check_sample([1, 3, 2, 3], [13, 3, 3, 0], [1, 3, 2, 3]); 
        behaves_like_3 += check_sample([1, 3, 2, 2], [2, 0, 2, 1], [1, 0, 2, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [6, 1, 3, 2], [0, 1, 0, 2]); 
        behaves_like_3 += check_sample([3, 0, 2, 2], [13, 2, 2, 1], [3, 1, 2, 2]); 
        behaves_like_3 += check_sample([0, 0, 1, 1], [3, 3, 3, 3], [0, 0, 1, 0]); 
        behaves_like_3 += check_sample([1, 1, 0, 1], [3, 3, 3, 1], [1, 0, 0, 1]); 
        behaves_like_3 += check_sample([0, 2, 2, 1], [14, 3, 2, 2], [0, 2, 1, 1]); 
        behaves_like_3 += check_sample([3, 1, 3, 1], [12, 2, 3, 2], [3, 1, 0, 1]); 
        behaves_like_3 += check_sample([2, 0, 2, 2], [7, 0, 2, 2], [2, 0, 1, 2]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [11, 0, 3, 3], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 0], [10, 1, 0, 0], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([3, 3, 2, 2], [13, 2, 2, 1], [3, 1, 2, 2]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [15, 0, 1, 3], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 0, 1, 2], [15, 2, 1, 0], [1, 0, 1, 2]); 
        behaves_like_3 += check_sample([2, 1, 0, 2], [6, 1, 3, 2], [2, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [10, 1, 0, 3], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([2, 0, 0, 2], [11, 0, 3, 0], [1, 0, 0, 2]); 
        behaves_like_3 += check_sample([0, 1, 1, 2], [13, 0, 0, 3], [0, 1, 1, 1]); 
        behaves_like_3 += check_sample([0, 2, 3, 0], [13, 0, 0, 0], [1, 2, 3, 0]); 
        behaves_like_3 += check_sample([3, 3, 1, 3], [4, 3, 3, 1], [3, 3, 1, 3]); 
        behaves_like_3 += check_sample([1, 1, 3, 2], [6, 1, 3, 2], [1, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 0, 2, 1], [14, 3, 2, 1], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [8, 1, 2, 1], [1, 0, 2, 2]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [8, 1, 2, 2], [1, 1, 0, 0]); 
        behaves_like_3 += check_sample([3, 0, 1, 3], [0, 2, 3, 1], [3, 0, 1, 3]); 
        behaves_like_3 += check_sample([2, 0, 2, 2], [7, 0, 2, 3], [2, 0, 2, 1]); 
        behaves_like_3 += check_sample([1, 3, 2, 0], [13, 2, 2, 1], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 3, 3], [10, 1, 0, 1], [0, 1, 3, 3]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [0, 1, 3, 3], [0, 1, 0, 0]); 
        behaves_like_3 += check_sample([1, 3, 2, 3], [2, 0, 2, 1], [1, 0, 2, 3]); 
        behaves_like_3 += check_sample([0, 1, 0, 2], [10, 1, 0, 0], [1, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 0, 1, 3], [0, 2, 3, 0], [0, 0, 1, 3]); 
        behaves_like_3 += check_sample([3, 0, 3, 1], [12, 2, 3, 3], [3, 0, 3, 0]); 
        behaves_like_3 += check_sample([2, 1, 1, 0], [5, 1, 3, 3], [2, 1, 1, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [10, 1, 0, 0], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 0, 2, 1], [15, 3, 1, 1], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 3, 2, 1], [14, 3, 2, 2], [2, 3, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [8, 1, 2, 1], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([3, 0, 0, 3], [12, 3, 0, 1], [3, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 2, 1, 1], [9, 2, 3, 2], [0, 2, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [10, 1, 0, 2], [0, 1, 1, 0]); 
        behaves_like_3 += check_sample([2, 0, 2, 1], [15, 3, 1, 3], [2, 0, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 0, 2], [6, 1, 3, 0], [0, 1, 0, 2]); 
        behaves_like_3 += check_sample([3, 0, 2, 1], [14, 3, 2, 2], [3, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 0, 2, 0], [15, 0, 1, 2], [1, 0, 1, 0]); 
        behaves_like_3 += check_sample([0, 2, 2, 1], [14, 3, 2, 1], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 0, 2, 3], [0, 2, 3, 0], [0, 0, 2, 3]); 
        behaves_like_3 += check_sample([0, 2, 3, 1], [3, 3, 3, 3], [0, 2, 3, 0]); 
        behaves_like_3 += check_sample([1, 0, 2, 2], [2, 0, 2, 3], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([2, 3, 2, 3], [13, 3, 2, 3], [2, 3, 2, 0]); 
        behaves_like_3 += check_sample([1, 3, 2, 1], [14, 3, 2, 1], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 3, 3], [12, 3, 0, 1], [3, 1, 3, 3]); 
        behaves_like_3 += check_sample([0, 0, 2, 0], [1, 0, 0, 3], [0, 0, 2, 0]); 
        behaves_like_3 += check_sample([2, 0, 3, 3], [7, 2, 3, 2], [2, 0, 1, 3]); 
        behaves_like_3 += check_sample([1, 3, 2, 3], [0, 2, 3, 2], [1, 3, 0, 3]); 
        behaves_like_3 += check_sample([3, 0, 2, 1], [14, 3, 2, 1], [3, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 0, 3, 1], [15, 3, 1, 0], [1, 0, 3, 1]); 
        behaves_like_3 += check_sample([2, 3, 1, 1], [3, 2, 3, 0], [0, 3, 1, 1]); 
        behaves_like_3 += check_sample([3, 1, 3, 1], [12, 2, 3, 0], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([2, 1, 0, 1], [3, 3, 3, 0], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [13, 0, 0, 2], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 2], [8, 1, 2, 0], [0, 1, 2, 2]); 
        behaves_like_3 += check_sample([2, 3, 1, 2], [11, 0, 3, 0], [1, 3, 1, 2]); 
        behaves_like_3 += check_sample([1, 0, 1, 3], [13, 3, 3, 2], [1, 0, 1, 3]); 
        behaves_like_3 += check_sample([3, 2, 3, 3], [13, 3, 1, 0], [0, 2, 3, 3]); 
        behaves_like_3 += check_sample([3, 0, 1, 1], [9, 2, 3, 1], [3, 2, 1, 1]); 
        behaves_like_3 += check_sample([1, 0, 2, 2], [2, 0, 2, 2], [1, 0, 0, 2]); 
        behaves_like_3 += check_sample([3, 0, 3, 3], [7, 0, 3, 3], [3, 0, 3, 1]); 
        behaves_like_3 += check_sample([0, 1, 0, 2], [6, 1, 3, 3], [0, 1, 0, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 3], [8, 1, 2, 1], [3, 0, 2, 3]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [2, 0, 2, 3], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 3, 0, 2], [1, 0, 0, 3], [0, 3, 0, 0]); 
        behaves_like_3 += check_sample([2, 2, 2, 2], [12, 2, 1, 3], [2, 2, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 3, 2], [11, 0, 3, 2], [2, 1, 1, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [6, 1, 3, 3], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 1, 3, 2], [11, 0, 3, 0], [1, 1, 3, 2]); 
        behaves_like_3 += check_sample([3, 1, 3, 2], [6, 1, 3, 3], [3, 1, 3, 0]); 
        behaves_like_3 += check_sample([2, 3, 2, 0], [7, 0, 2, 2], [2, 3, 1, 0]); 
        behaves_like_3 += check_sample([3, 3, 2, 1], [14, 3, 2, 1], [3, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [14, 3, 2, 1], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 0, 2], [6, 1, 3, 1], [0, 0, 0, 2]); 
        behaves_like_3 += check_sample([2, 1, 2, 0], [5, 1, 3, 2], [2, 1, 1, 0]); 
        behaves_like_3 += check_sample([2, 3, 3, 3], [7, 2, 3, 1], [2, 1, 3, 3]); 
        behaves_like_3 += check_sample([3, 1, 0, 3], [0, 1, 3, 0], [0, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 3, 2, 3], [4, 2, 2, 1], [0, 2, 2, 3]); 
        behaves_like_3 += check_sample([2, 0, 0, 2], [11, 0, 3, 1], [2, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 2, 2, 3], [13, 0, 0, 2], [0, 2, 1, 3]); 
        behaves_like_3 += check_sample([1, 1, 1, 0], [9, 2, 0, 1], [1, 2, 1, 0]); 
        behaves_like_3 += check_sample([1, 1, 3, 3], [0, 1, 3, 0], [0, 1, 3, 3]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [0, 2, 3, 0], [0, 2, 2, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 3], [0, 2, 3, 0], [0, 1, 2, 3]); 
        behaves_like_3 += check_sample([2, 2, 2, 2], [7, 0, 2, 3], [2, 2, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [14, 3, 2, 2], [1, 1, 1, 1]); 
        behaves_like_3 += check_sample([0, 0, 2, 0], [4, 2, 2, 3], [0, 0, 2, 2]); 
        behaves_like_3 += check_sample([3, 3, 0, 3], [13, 3, 3, 3], [3, 3, 0, 1]); 
        behaves_like_3 += check_sample([0, 2, 3, 1], [13, 0, 0, 2], [0, 2, 1, 1]); 
        behaves_like_3 += check_sample([2, 3, 2, 0], [7, 0, 2, 3], [2, 3, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 0, 2], [1, 0, 0, 3], [0, 1, 0, 0]); 
        behaves_like_3 += check_sample([2, 2, 0, 2], [3, 3, 3, 1], [2, 0, 0, 2]); 
        behaves_like_3 += check_sample([1, 3, 2, 3], [2, 0, 2, 2], [1, 3, 0, 3]); 
        behaves_like_3 += check_sample([1, 1, 3, 3], [13, 3, 3, 3], [1, 1, 3, 1]); 
        behaves_like_3 += check_sample([0, 0, 1, 1], [15, 3, 1, 3], [0, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 3, 0], [5, 1, 3, 1], [1, 1, 3, 0]); 
        behaves_like_3 += check_sample([2, 0, 3, 2], [11, 0, 3, 2], [2, 0, 1, 2]); 
        behaves_like_3 += check_sample([3, 0, 3, 3], [7, 2, 3, 0], [1, 0, 3, 3]); 
        behaves_like_3 += check_sample([0, 1, 2, 1], [10, 1, 0, 3], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 0, 0, 2], [11, 0, 3, 3], [2, 0, 0, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 3], [2, 0, 2, 1], [1, 0, 2, 3]); 
        behaves_like_3 += check_sample([1, 2, 1, 2], [9, 2, 0, 1], [1, 2, 1, 2]); 
        behaves_like_3 += check_sample([0, 2, 2, 1], [1, 0, 0, 0], [0, 2, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 0], [5, 1, 3, 2], [0, 1, 1, 0]); 
        behaves_like_3 += check_sample([2, 0, 3, 3], [13, 3, 3, 0], [1, 0, 3, 3]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [5, 1, 3, 1], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 1], [14, 3, 2, 1], [3, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 0, 2], [10, 1, 0, 3], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([1, 1, 3, 0], [5, 1, 3, 2], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([2, 1, 2, 3], [8, 1, 2, 2], [2, 1, 0, 3]); 
        behaves_like_3 += check_sample([3, 3, 3, 3], [12, 3, 0, 1], [3, 1, 3, 3]); 
        behaves_like_3 += check_sample([1, 0, 1, 0], [15, 0, 1, 3], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([2, 0, 1, 1], [15, 3, 1, 0], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 0, 0], [5, 1, 3, 1], [1, 1, 0, 0]); 
        behaves_like_3 += check_sample([0, 0, 2, 1], [14, 3, 2, 1], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 0, 0], [5, 1, 3, 1], [0, 1, 0, 0]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [0, 1, 3, 3], [2, 2, 2, 0]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [14, 3, 2, 0], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([1, 0, 1, 3], [9, 2, 0, 0], [2, 0, 1, 3]); 
        behaves_like_3 += check_sample([3, 0, 1, 0], [15, 2, 1, 3], [3, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 3, 2, 1], [14, 3, 2, 3], [0, 3, 2, 1]); 
        behaves_like_3 += check_sample([0, 3, 0, 0], [13, 0, 0, 2], [0, 3, 1, 0]); 
        behaves_like_3 += check_sample([1, 2, 1, 1], [9, 2, 0, 0], [2, 2, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 0, 3], [0, 1, 3, 1], [1, 0, 0, 3]); 
        behaves_like_3 += check_sample([0, 0, 1, 3], [15, 2, 1, 1], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([3, 1, 1, 3], [0, 1, 3, 0], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([0, 1, 3, 0], [13, 0, 0, 0], [1, 1, 3, 0]); 
        behaves_like_3 += check_sample([1, 1, 3, 3], [7, 2, 3, 1], [1, 1, 3, 3]); 
        behaves_like_3 += check_sample([1, 0, 2, 3], [2, 0, 2, 3], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([2, 0, 2, 1], [15, 3, 1, 2], [2, 0, 1, 1]); 
        behaves_like_3 += check_sample([2, 0, 2, 3], [4, 3, 3, 0], [3, 0, 2, 3]); 
        behaves_like_3 += check_sample([0, 1, 0, 1], [13, 0, 0, 1], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [10, 1, 0, 3], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 2], [12, 2, 1, 1], [1, 1, 2, 2]); 
        behaves_like_3 += check_sample([0, 2, 2, 3], [0, 2, 3, 2], [0, 2, 0, 3]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [8, 1, 2, 1], [2, 0, 2, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [8, 1, 2, 0], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([3, 3, 2, 0], [4, 2, 2, 2], [3, 3, 2, 0]); 
        behaves_like_3 += check_sample([2, 0, 1, 2], [11, 0, 3, 1], [2, 1, 1, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 3], [10, 1, 0, 2], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([3, 3, 1, 1], [9, 2, 3, 2], [3, 3, 2, 1]); 
        behaves_like_3 += check_sample([2, 0, 1, 2], [11, 0, 3, 2], [2, 0, 1, 2]); 
        behaves_like_3 += check_sample([0, 1, 0, 0], [10, 1, 0, 0], [1, 1, 0, 0]); 
        behaves_like_3 += check_sample([0, 1, 3, 1], [3, 3, 3, 0], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([1, 1, 3, 3], [7, 2, 3, 2], [1, 1, 1, 3]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [6, 1, 3, 2], [2, 1, 0, 2]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [15, 3, 1, 0], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 1, 0], [5, 1, 3, 2], [3, 1, 1, 0]); 
        behaves_like_3 += check_sample([3, 0, 3, 3], [7, 2, 3, 3], [3, 0, 3, 1]); 
        behaves_like_3 += check_sample([3, 0, 3, 3], [12, 3, 2, 2], [3, 0, 1, 3]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [14, 3, 2, 3], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 1, 1], [3, 3, 3, 0], [0, 1, 1, 1]); 
        behaves_like_3 += check_sample([0, 0, 3, 3], [12, 3, 2, 3], [0, 0, 3, 1]); 
        behaves_like_3 += check_sample([2, 3, 2, 1], [14, 3, 2, 1], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 0], [2, 0, 2, 0], [0, 2, 2, 0]); 
        behaves_like_3 += check_sample([2, 1, 3, 3], [4, 3, 3, 0], [3, 1, 3, 3]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [1, 0, 0, 2], [0, 1, 0, 0]); 
        behaves_like_3 += check_sample([0, 2, 2, 2], [3, 3, 3, 1], [0, 0, 2, 2]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [12, 2, 1, 3], [2, 2, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 0, 0], [5, 1, 3, 2], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([2, 1, 1, 2], [6, 1, 3, 3], [2, 1, 1, 0]); 
        behaves_like_3 += check_sample([1, 3, 2, 3], [2, 0, 2, 3], [1, 3, 2, 0]); 
        behaves_like_3 += check_sample([3, 0, 3, 1], [15, 3, 1, 3], [3, 0, 3, 1]); 
        behaves_like_3 += check_sample([0, 1, 3, 2], [1, 0, 0, 1], [0, 0, 3, 2]); 
        behaves_like_3 += check_sample([2, 0, 2, 0], [12, 2, 0, 0], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([3, 3, 1, 3], [4, 3, 3, 2], [3, 3, 3, 3]); 
        behaves_like_3 += check_sample([3, 0, 2, 3], [13, 2, 2, 0], [1, 0, 2, 3]); 
        behaves_like_3 += check_sample([3, 2, 2, 3], [0, 1, 3, 3], [3, 2, 2, 0]); 
        behaves_like_3 += check_sample([3, 0, 2, 0], [4, 2, 2, 3], [3, 0, 2, 2]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [15, 3, 1, 3], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([3, 3, 2, 1], [14, 3, 2, 3], [3, 3, 2, 1]); 
        behaves_like_3 += check_sample([1, 3, 1, 2], [9, 2, 0, 3], [1, 3, 1, 2]); 
        behaves_like_3 += check_sample([2, 1, 2, 3], [8, 1, 2, 0], [0, 1, 2, 3]); 
        behaves_like_3 += check_sample([2, 2, 2, 2], [7, 0, 2, 1], [2, 1, 2, 2]); 
        behaves_like_3 += check_sample([1, 1, 0, 2], [6, 1, 3, 0], [0, 1, 0, 2]); 
        behaves_like_3 += check_sample([3, 1, 1, 2], [6, 1, 3, 3], [3, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 1, 3, 1], [10, 1, 0, 3], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([0, 2, 2, 0], [12, 2, 1, 2], [0, 2, 1, 0]); 
        behaves_like_3 += check_sample([1, 2, 1, 1], [3, 3, 3, 3], [1, 2, 1, 0]); 
        behaves_like_3 += check_sample([2, 3, 2, 3], [12, 2, 0, 1], [2, 1, 2, 3]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [8, 1, 2, 0], [0, 1, 2, 2]); 
        behaves_like_3 += check_sample([0, 1, 1, 2], [6, 1, 3, 0], [0, 1, 1, 2]); 
        behaves_like_3 += check_sample([2, 3, 3, 2], [11, 0, 3, 1], [2, 1, 3, 2]); 
        behaves_like_3 += check_sample([2, 0, 0, 1], [3, 3, 3, 2], [2, 0, 0, 1]); 
        behaves_like_3 += check_sample([2, 0, 3, 2], [11, 0, 3, 0], [1, 0, 3, 2]); 
        behaves_like_3 += check_sample([0, 1, 1, 1], [10, 1, 0, 0], [1, 1, 1, 1]); 
        behaves_like_3 += check_sample([1, 3, 2, 3], [0, 2, 3, 1], [1, 0, 2, 3]); 
        behaves_like_3 += check_sample([3, 1, 1, 2], [6, 1, 3, 0], [0, 1, 1, 2]); 
        behaves_like_3 += check_sample([0, 1, 0, 0], [10, 1, 0, 3], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([1, 1, 1, 0], [5, 1, 3, 2], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [8, 1, 2, 2], [2, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 1, 1, 1], [9, 2, 3, 2], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [6, 1, 3, 3], [2, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 2, 1], [14, 3, 2, 0], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [2, 0, 2, 0], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [8, 1, 2, 3], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [14, 3, 2, 1], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [4, 2, 2, 2], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 0, 2], [11, 0, 3, 1], [2, 1, 0, 2]); 
        behaves_like_3 += check_sample([3, 2, 2, 0], [4, 2, 2, 2], [3, 2, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 0, 1], [10, 1, 0, 0], [1, 1, 0, 1]); 
        behaves_like_3 += check_sample([2, 2, 0, 2], [3, 3, 3, 3], [2, 2, 0, 0]); 
        behaves_like_3 += check_sample([2, 2, 2, 2], [11, 0, 3, 2], [2, 2, 1, 2]); 
        behaves_like_3 += check_sample([2, 3, 2, 1], [14, 3, 2, 3], [2, 3, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 3, 2], [6, 1, 3, 1], [1, 0, 3, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 3], [10, 1, 0, 3], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([3, 0, 2, 2], [4, 2, 2, 2], [3, 0, 2, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 1], [14, 3, 2, 1], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 3, 2], [6, 1, 3, 1], [3, 0, 3, 2]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [2, 0, 2, 1], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [2, 0, 2, 2], [1, 1, 0, 1]); 
        behaves_like_3 += check_sample([1, 3, 2, 1], [13, 2, 2, 3], [1, 3, 2, 1]); 
        behaves_like_3 += check_sample([3, 3, 2, 0], [13, 2, 2, 2], [3, 3, 1, 0]); 
        behaves_like_3 += check_sample([2, 2, 1, 3], [0, 1, 3, 3], [2, 2, 1, 0]); 
        behaves_like_3 += check_sample([1, 0, 0, 2], [15, 0, 1, 0], [1, 0, 0, 2]); 
        behaves_like_3 += check_sample([3, 2, 1, 1], [3, 3, 3, 3], [3, 2, 1, 0]); 
        behaves_like_3 += check_sample([3, 2, 1, 3], [0, 1, 3, 1], [3, 0, 1, 3]); 
        behaves_like_3 += check_sample([3, 3, 2, 3], [7, 0, 3, 0], [1, 3, 2, 3]); 
        behaves_like_3 += check_sample([0, 2, 0, 2], [1, 0, 0, 1], [0, 0, 0, 2]); 
        behaves_like_3 += check_sample([2, 0, 2, 1], [7, 0, 2, 1], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([3, 2, 2, 1], [14, 3, 2, 2], [3, 2, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [8, 1, 2, 3], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 0, 2], [6, 1, 3, 2], [1, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 1, 3, 2], [6, 1, 3, 1], [2, 0, 3, 2]); 
        behaves_like_3 += check_sample([1, 0, 3, 3], [4, 3, 3, 1], [1, 3, 3, 3]); 
        behaves_like_3 += check_sample([0, 2, 3, 3], [0, 1, 3, 0], [0, 2, 3, 3]); 
        behaves_like_3 += check_sample([2, 1, 1, 3], [0, 2, 3, 1], [2, 0, 1, 3]); 
        behaves_like_3 += check_sample([1, 0, 0, 1], [15, 3, 1, 2], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 3, 3, 2], [1, 0, 0, 2], [0, 3, 0, 2]); 
        behaves_like_3 += check_sample([2, 1, 2, 3], [7, 0, 2, 1], [2, 1, 2, 3]); 
        behaves_like_3 += check_sample([0, 0, 3, 2], [13, 0, 0, 3], [0, 0, 3, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [5, 1, 3, 2], [0, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 2, 2, 1], [12, 2, 1, 0], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 1], [8, 1, 2, 2], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 0], [10, 1, 0, 3], [0, 1, 1, 1]); 
        behaves_like_3 += check_sample([0, 3, 1, 0], [1, 0, 0, 2], [0, 3, 0, 0]); 
        behaves_like_3 += check_sample([2, 1, 1, 2], [11, 0, 3, 1], [2, 1, 1, 2]); 
        behaves_like_3 += check_sample([3, 3, 2, 1], [14, 3, 2, 2], [3, 3, 1, 1]); 
        behaves_like_3 += check_sample([3, 1, 2, 2], [4, 2, 2, 3], [3, 1, 2, 2]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [0, 2, 3, 2], [1, 1, 0, 3]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [0, 1, 3, 1], [2, 0, 2, 3]); 
        behaves_like_3 += check_sample([3, 3, 2, 3], [4, 3, 3, 2], [3, 3, 3, 3]); 
        behaves_like_3 += check_sample([2, 1, 2, 0], [7, 0, 2, 3], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 2, 1, 1], [9, 2, 3, 1], [0, 2, 1, 1]); 
        behaves_like_3 += check_sample([2, 1, 0, 3], [0, 1, 3, 2], [2, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [13, 3, 3, 0], [1, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 0, 3, 1], [3, 3, 3, 2], [0, 0, 0, 1]); 
        behaves_like_3 += check_sample([1, 0, 0, 0], [15, 0, 1, 0], [1, 0, 0, 0]); 
        behaves_like_3 += check_sample([0, 0, 1, 3], [0, 2, 3, 1], [0, 0, 1, 3]); 
        behaves_like_3 += check_sample([1, 1, 1, 3], [9, 2, 0, 1], [1, 2, 1, 3]); 
        behaves_like_3 += check_sample([3, 0, 1, 3], [0, 2, 3, 2], [3, 0, 0, 3]); 
        behaves_like_3 += check_sample([3, 1, 3, 2], [6, 1, 3, 0], [0, 1, 3, 2]); 
        behaves_like_3 += check_sample([3, 3, 1, 3], [12, 3, 0, 1], [3, 1, 1, 3]); 
        behaves_like_3 += check_sample([3, 1, 1, 3], [0, 2, 3, 0], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([2, 1, 1, 2], [6, 1, 3, 2], [2, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 1, 0, 2], [10, 1, 0, 1], [0, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 1, 1, 0], [10, 1, 0, 1], [0, 1, 1, 0]); 
        behaves_like_3 += check_sample([3, 2, 1, 3], [13, 3, 1, 3], [3, 2, 1, 0]); 
        behaves_like_3 += check_sample([2, 1, 1, 0], [5, 1, 3, 0], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 1, 3, 1], [10, 1, 0, 1], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([0, 0, 3, 1], [12, 2, 3, 2], [0, 0, 0, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [0, 2, 3, 0], [0, 1, 2, 3]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [0, 2, 3, 3], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 3, 2, 1], [14, 3, 2, 0], [1, 3, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 3], [10, 1, 0, 0], [1, 1, 2, 3]); 
        behaves_like_3 += check_sample([2, 3, 3, 1], [12, 2, 3, 3], [2, 3, 3, 0]); 
        behaves_like_3 += check_sample([2, 0, 2, 2], [11, 0, 3, 2], [2, 0, 1, 2]); 
        behaves_like_3 += check_sample([3, 1, 0, 2], [3, 3, 3, 2], [3, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [1, 0, 0, 3], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 3, 2, 1], [14, 3, 2, 2], [0, 3, 1, 1]); 
        behaves_like_3 += check_sample([3, 2, 3, 3], [12, 3, 2, 1], [3, 1, 3, 3]); 
        behaves_like_3 += check_sample([3, 2, 2, 1], [3, 3, 3, 2], [3, 2, 0, 1]); 
        behaves_like_3 += check_sample([2, 0, 3, 2], [3, 3, 3, 2], [2, 0, 0, 2]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [8, 1, 2, 2], [1, 1, 0, 1]); 
        behaves_like_3 += check_sample([3, 1, 2, 0], [5, 1, 3, 1], [3, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 1, 1, 1], [9, 2, 3, 0], [2, 1, 1, 1]); 
        behaves_like_3 += check_sample([2, 0, 2, 0], [13, 2, 2, 1], [2, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 0, 2, 2], [13, 2, 2, 3], [0, 0, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 1], [8, 1, 2, 3], [2, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 1, 0, 2], [3, 3, 3, 3], [2, 1, 0, 0]); 
        behaves_like_3 += check_sample([0, 2, 3, 0], [1, 0, 0, 1], [0, 0, 3, 0]); 
        behaves_like_3 += check_sample([3, 2, 1, 3], [12, 3, 0, 1], [3, 1, 1, 3]); 
        behaves_like_3 += check_sample([0, 1, 3, 2], [1, 0, 0, 2], [0, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [0, 1, 3, 2], [2, 2, 0, 3]); 
        behaves_like_3 += check_sample([3, 0, 1, 0], [15, 2, 1, 2], [3, 0, 1, 0]); 
        behaves_like_3 += check_sample([1, 0, 3, 1], [12, 2, 3, 2], [1, 0, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 2], [10, 1, 0, 1], [0, 1, 1, 2]); 
        behaves_like_3 += check_sample([3, 1, 1, 3], [13, 3, 2, 2], [3, 1, 0, 3]); 
        behaves_like_3 += check_sample([1, 0, 2, 0], [2, 0, 2, 0], [0, 0, 2, 0]); 
        behaves_like_3 += check_sample([1, 2, 0, 3], [0, 1, 3, 1], [1, 0, 0, 3]); 
        behaves_like_3 += check_sample([0, 3, 2, 3], [4, 2, 2, 2], [0, 3, 2, 3]); 
        behaves_like_3 += check_sample([2, 2, 1, 2], [11, 0, 3, 3], [2, 2, 1, 1]); 
        behaves_like_3 += check_sample([1, 0, 1, 2], [3, 3, 3, 0], [0, 0, 1, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [8, 1, 2, 3], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 0, 3, 3], [12, 3, 2, 1], [2, 1, 3, 3]); 
        behaves_like_3 += check_sample([2, 1, 3, 2], [11, 0, 3, 1], [2, 1, 3, 2]); 
        behaves_like_3 += check_sample([1, 2, 1, 2], [9, 2, 0, 0], [2, 2, 1, 2]); 
        behaves_like_3 += check_sample([2, 2, 2, 0], [12, 2, 0, 1], [2, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 3, 0, 2], [3, 3, 3, 3], [2, 3, 0, 0]); 
        behaves_like_3 += check_sample([1, 1, 3, 1], [12, 2, 3, 3], [1, 1, 3, 0]); 
        behaves_like_3 += check_sample([1, 0, 3, 1], [15, 0, 1, 0], [1, 0, 3, 1]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [9, 2, 0, 1], [1, 2, 1, 1]); 
        behaves_like_3 += check_sample([0, 1, 3, 2], [10, 1, 0, 0], [1, 1, 3, 2]); 
        behaves_like_3 += check_sample([0, 1, 1, 2], [10, 1, 0, 3], [0, 1, 1, 1]); 
        behaves_like_3 += check_sample([3, 1, 3, 1], [3, 3, 3, 2], [3, 1, 0, 1]); 
        behaves_like_3 += check_sample([0, 0, 3, 3], [12, 3, 2, 0], [1, 0, 3, 3]); 
        behaves_like_3 += check_sample([3, 0, 1, 1], [9, 2, 3, 3], [3, 0, 1, 2]); 
        behaves_like_3 += check_sample([2, 2, 0, 3], [13, 3, 3, 3], [2, 2, 0, 1]); 
        behaves_like_3 += check_sample([0, 3, 1, 3], [13, 3, 2, 2], [0, 3, 0, 3]); 
        behaves_like_3 += check_sample([0, 1, 0, 2], [6, 1, 3, 0], [0, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 2, 2, 1], [14, 3, 2, 0], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 1, 0], [5, 1, 3, 1], [3, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 1, 3, 3], [0, 1, 3, 3], [0, 1, 3, 0]); 
        behaves_like_3 += check_sample([0, 0, 0, 1], [15, 3, 1, 1], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([3, 1, 2, 3], [7, 0, 3, 1], [3, 1, 2, 3]); 
        behaves_like_3 += check_sample([1, 1, 2, 1], [2, 0, 2, 3], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([0, 0, 2, 1], [13, 0, 0, 2], [0, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 0, 1, 3], [13, 3, 3, 0], [1, 0, 1, 3]); 
        behaves_like_3 += check_sample([1, 0, 1, 2], [9, 2, 0, 3], [1, 0, 1, 2]); 
        behaves_like_3 += check_sample([2, 0, 3, 3], [7, 2, 3, 3], [2, 0, 3, 1]); 
        behaves_like_3 += check_sample([0, 3, 1, 3], [0, 2, 3, 0], [0, 3, 1, 3]); 
        behaves_like_3 += check_sample([2, 1, 1, 1], [9, 2, 3, 3], [2, 1, 1, 2]); 
        behaves_like_3 += check_sample([0, 0, 1, 0], [15, 2, 1, 0], [1, 0, 1, 0]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [2, 0, 2, 0], [0, 0, 2, 1]); 
        behaves_like_3 += check_sample([0, 0, 1, 3], [4, 3, 3, 3], [0, 0, 1, 3]); 
        behaves_like_3 += check_sample([0, 1, 1, 2], [10, 1, 0, 0], [1, 1, 1, 2]); 
        behaves_like_3 += check_sample([2, 2, 2, 2], [11, 0, 3, 3], [2, 2, 2, 1]); 
        behaves_like_3 += check_sample([2, 3, 3, 1], [3, 3, 3, 3], [2, 3, 3, 0]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [2, 0, 2, 0], [0, 1, 2, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 1], [8, 1, 2, 1], [3, 0, 2, 1]); 
        behaves_like_3 += check_sample([2, 2, 0, 2], [11, 0, 3, 3], [2, 2, 0, 1]); 
        behaves_like_3 += check_sample([2, 2, 3, 1], [12, 2, 3, 2], [2, 2, 0, 1]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [13, 2, 2, 3], [2, 2, 2, 1]); 
        behaves_like_3 += check_sample([2, 0, 2, 1], [14, 3, 2, 3], [2, 0, 2, 1]); 
        behaves_like_3 += check_sample([0, 0, 2, 3], [1, 0, 0, 1], [0, 0, 2, 3]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [5, 1, 3, 3], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 2, 2, 2], [12, 2, 1, 1], [2, 1, 2, 2]); 
        behaves_like_3 += check_sample([3, 2, 0, 3], [7, 0, 3, 3], [3, 2, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 1, 3], [4, 3, 3, 0], [3, 1, 1, 3]); 
        behaves_like_3 += check_sample([1, 1, 1, 2], [6, 1, 3, 1], [1, 0, 1, 2]); 
        behaves_like_3 += check_sample([1, 1, 1, 2], [6, 1, 3, 3], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([1, 0, 3, 1], [12, 2, 3, 1], [1, 0, 3, 1]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [2, 0, 2, 2], [1, 0, 0, 1]); 
        behaves_like_3 += check_sample([1, 0, 1, 0], [15, 2, 1, 2], [1, 0, 1, 0]); 
        behaves_like_3 += check_sample([1, 3, 3, 3], [4, 3, 3, 1], [1, 3, 3, 3]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [9, 2, 3, 2], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([0, 2, 2, 3], [0, 2, 3, 1], [0, 0, 2, 3]); 
        behaves_like_3 += check_sample([3, 2, 2, 3], [0, 2, 3, 0], [0, 2, 2, 3]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [8, 1, 2, 3], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([3, 1, 0, 0], [5, 1, 3, 2], [3, 1, 1, 0]); 
        behaves_like_3 += check_sample([2, 0, 2, 2], [7, 0, 2, 0], [1, 0, 2, 2]); 
        behaves_like_3 += check_sample([1, 2, 1, 3], [0, 2, 3, 2], [1, 2, 0, 3]); 
        behaves_like_3 += check_sample([0, 0, 2, 2], [4, 2, 2, 1], [0, 2, 2, 2]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [15, 0, 1, 0], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 0, 2, 0], [2, 0, 2, 1], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([1, 0, 0, 1], [15, 0, 1, 1], [1, 1, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 3, 0], [10, 1, 0, 1], [0, 1, 3, 0]); 
        behaves_like_3 += check_sample([2, 3, 0, 2], [11, 0, 3, 0], [1, 3, 0, 2]); 
        behaves_like_3 += check_sample([1, 0, 2, 2], [15, 0, 1, 3], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([3, 0, 1, 1], [15, 3, 1, 2], [3, 0, 1, 1]); 
        behaves_like_3 += check_sample([0, 0, 2, 1], [3, 3, 3, 1], [0, 0, 2, 1]); 
        behaves_like_3 += check_sample([1, 3, 2, 0], [2, 0, 2, 3], [1, 3, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [10, 1, 0, 3], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([3, 2, 2, 3], [13, 3, 1, 1], [3, 0, 2, 3]); 
        behaves_like_3 += check_sample([3, 0, 3, 2], [3, 3, 3, 1], [3, 0, 3, 2]); 
        behaves_like_3 += check_sample([0, 0, 2, 3], [13, 2, 2, 1], [0, 1, 2, 3]); 
        behaves_like_3 += check_sample([1, 1, 1, 2], [6, 1, 3, 2], [1, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 3, 2, 1], [12, 2, 0, 2], [2, 3, 1, 1]); 
        behaves_like_3 += check_sample([0, 0, 0, 1], [15, 3, 1, 3], [0, 0, 0, 1]); 
        behaves_like_3 += check_sample([2, 2, 2, 1], [14, 3, 2, 2], [2, 2, 1, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 3], [13, 3, 2, 0], [0, 1, 2, 3]); 
        behaves_like_3 += check_sample([2, 1, 0, 1], [3, 3, 3, 2], [2, 1, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [5, 1, 3, 0], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([3, 3, 1, 3], [0, 2, 3, 3], [3, 3, 1, 0]); 
        behaves_like_3 += check_sample([1, 2, 3, 3], [0, 1, 3, 2], [1, 2, 0, 3]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [10, 1, 0, 1], [0, 1, 2, 2]); 
        behaves_like_3 += check_sample([2, 1, 1, 3], [13, 3, 1, 0], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([1, 0, 3, 3], [4, 3, 3, 0], [3, 0, 3, 3]); 
        behaves_like_3 += check_sample([2, 3, 1, 1], [9, 2, 3, 3], [2, 3, 1, 2]); 
        behaves_like_3 += check_sample([2, 2, 2, 0], [7, 0, 2, 2], [2, 2, 1, 0]); 
        behaves_like_3 += check_sample([0, 0, 3, 1], [15, 3, 1, 0], [1, 0, 3, 1]); 
        behaves_like_3 += check_sample([0, 2, 2, 1], [14, 3, 2, 0], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 3, 2], [6, 1, 3, 3], [1, 1, 3, 0]); 
        behaves_like_3 += check_sample([2, 2, 1, 1], [9, 2, 3, 2], [2, 2, 2, 1]); 
        behaves_like_3 += check_sample([1, 0, 0, 1], [15, 0, 1, 3], [1, 0, 0, 1]); 
        behaves_like_3 += check_sample([1, 1, 3, 1], [3, 3, 3, 0], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([3, 2, 2, 3], [0, 1, 3, 0], [0, 2, 2, 3]); 
        behaves_like_3 += check_sample([3, 2, 3, 3], [12, 3, 0, 2], [3, 2, 1, 3]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [2, 0, 2, 2], [1, 2, 0, 1]); 
        behaves_like_3 += check_sample([3, 1, 1, 1], [9, 2, 3, 0], [2, 1, 1, 1]); 
        behaves_like_3 += check_sample([1, 2, 1, 2], [3, 3, 3, 1], [1, 0, 1, 2]); 
        behaves_like_3 += check_sample([3, 0, 2, 2], [13, 2, 2, 0], [1, 0, 2, 2]); 
        behaves_like_3 += check_sample([2, 3, 2, 2], [3, 3, 3, 2], [2, 3, 0, 2]); 
        behaves_like_3 += check_sample([1, 0, 2, 3], [2, 0, 2, 1], [1, 0, 2, 3]); 
        behaves_like_3 += check_sample([2, 3, 2, 2], [7, 0, 2, 0], [1, 3, 2, 2]); 
        behaves_like_3 += check_sample([2, 3, 0, 2], [11, 0, 3, 1], [2, 1, 0, 2]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [4, 2, 2, 1], [1, 2, 2, 0]); 
        behaves_like_3 += check_sample([2, 1, 0, 2], [11, 0, 3, 0], [1, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [1, 0, 0, 1], [0, 0, 2, 0]); 
        behaves_like_3 += check_sample([2, 1, 1, 0], [5, 1, 3, 1], [2, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [13, 2, 2, 2], [0, 1, 1, 2]); 
        behaves_like_3 += check_sample([0, 3, 2, 1], [14, 3, 2, 1], [0, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 1, 0], [5, 1, 3, 3], [1, 1, 1, 1]); 
        behaves_like_3 += check_sample([1, 0, 0, 0], [15, 0, 1, 1], [1, 1, 0, 0]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [1, 0, 0, 3], [0, 1, 0, 0]); 
        behaves_like_3 += check_sample([2, 1, 2, 0], [5, 1, 3, 1], [2, 1, 2, 0]); 
        behaves_like_3 += check_sample([3, 1, 0, 3], [7, 0, 3, 2], [3, 1, 1, 3]); 
        behaves_like_3 += check_sample([1, 3, 1, 1], [9, 2, 0, 0], [2, 3, 1, 1]); 
        behaves_like_3 += check_sample([3, 1, 1, 0], [5, 1, 3, 3], [3, 1, 1, 1]); 
        behaves_like_3 += check_sample([2, 1, 3, 0], [5, 1, 3, 0], [1, 1, 3, 0]); 
        behaves_like_3 += check_sample([0, 2, 0, 3], [0, 1, 3, 1], [0, 0, 0, 3]); 
        behaves_like_3 += check_sample([2, 3, 2, 2], [11, 0, 3, 2], [2, 3, 1, 2]); 
        behaves_like_3 += check_sample([3, 1, 2, 1], [14, 3, 2, 3], [3, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 2, 2, 1], [14, 3, 2, 1], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 3, 2, 0], [2, 0, 2, 0], [0, 3, 2, 0]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [2, 0, 2, 0], [0, 2, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 2, 0], [8, 1, 2, 1], [3, 0, 2, 0]); 
        behaves_like_3 += check_sample([0, 2, 0, 3], [13, 0, 0, 1], [0, 1, 0, 3]); 
        behaves_like_3 += check_sample([1, 2, 1, 0], [9, 2, 0, 2], [1, 2, 2, 0]); 
        behaves_like_3 += check_sample([0, 0, 0, 0], [13, 0, 0, 3], [0, 0, 0, 1]); 
        behaves_like_3 += check_sample([0, 3, 2, 0], [1, 0, 0, 1], [0, 0, 2, 0]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [14, 3, 2, 2], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [2, 0, 2, 0], [0, 1, 2, 2]); 
        behaves_like_3 += check_sample([0, 1, 0, 1], [10, 1, 0, 3], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([2, 2, 0, 2], [11, 0, 3, 1], [2, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 2, 3, 3], [7, 2, 3, 1], [2, 1, 3, 3]); 
        behaves_like_3 += check_sample([0, 1, 3, 0], [10, 1, 0, 3], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([2, 1, 0, 2], [6, 1, 3, 0], [0, 1, 0, 2]); 
        behaves_like_3 += check_sample([0, 2, 2, 1], [14, 3, 2, 3], [0, 2, 2, 1]); 
        behaves_like_3 += check_sample([3, 2, 2, 1], [14, 3, 2, 1], [3, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 3, 3, 3], [7, 2, 3, 2], [2, 3, 1, 3]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [15, 2, 1, 2], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [7, 0, 2, 0], [1, 1, 2, 2]); 
        behaves_like_3 += check_sample([0, 2, 3, 3], [7, 2, 3, 2], [0, 2, 1, 3]); 
        behaves_like_3 += check_sample([1, 2, 0, 3], [0, 1, 3, 2], [1, 2, 0, 3]); 
        behaves_like_3 += check_sample([1, 0, 2, 0], [15, 0, 1, 0], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 0, 0], [10, 1, 0, 1], [0, 1, 0, 0]); 
        behaves_like_3 += check_sample([2, 1, 2, 1], [7, 0, 2, 0], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 1, 2, 1], [14, 3, 2, 1], [2, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 3], [10, 1, 0, 1], [0, 1, 2, 3]); 
        behaves_like_3 += check_sample([2, 1, 2, 2], [6, 1, 3, 1], [2, 0, 2, 2]); 
        behaves_like_3 += check_sample([1, 1, 3, 2], [3, 3, 3, 1], [1, 0, 3, 2]); 
        behaves_like_3 += check_sample([3, 1, 0, 0], [5, 1, 3, 3], [3, 1, 0, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 0], [1, 0, 0, 0], [0, 1, 2, 0]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [2, 0, 2, 3], [1, 0, 2, 0]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [6, 1, 3, 0], [0, 1, 2, 2]); 
        behaves_like_3 += check_sample([1, 1, 0, 0], [5, 1, 3, 3], [1, 1, 0, 1]); 
        behaves_like_3 += check_sample([0, 3, 2, 0], [4, 2, 2, 0], [2, 3, 2, 0]); 
        behaves_like_3 += check_sample([3, 0, 0, 1], [15, 3, 1, 0], [1, 0, 0, 1]); 
        behaves_like_3 += check_sample([0, 3, 3, 0], [1, 0, 0, 1], [0, 0, 3, 0]); 
        behaves_like_3 += check_sample([0, 2, 3, 3], [1, 0, 0, 1], [0, 0, 3, 3]); 
        behaves_like_3 += check_sample([3, 1, 2, 2], [8, 1, 2, 1], [3, 0, 2, 2]); 
        behaves_like_3 += check_sample([0, 3, 1, 2], [1, 0, 0, 0], [0, 3, 1, 2]); 
        behaves_like_3 += check_sample([2, 0, 2, 2], [11, 0, 3, 0], [1, 0, 2, 2]); 
        behaves_like_3 += check_sample([1, 0, 1, 3], [15, 2, 1, 0], [1, 0, 1, 3]); 
        behaves_like_3 += check_sample([1, 3, 2, 3], [0, 2, 3, 0], [0, 3, 2, 3]); 
        behaves_like_3 += check_sample([1, 2, 1, 3], [13, 3, 2, 2], [1, 2, 0, 3]); 
        behaves_like_3 += check_sample([2, 2, 1, 2], [11, 0, 3, 1], [2, 1, 1, 2]); 
        behaves_like_3 += check_sample([2, 2, 2, 2], [4, 2, 2, 2], [2, 2, 2, 2]); 
        behaves_like_3 += check_sample([3, 0, 2, 1], [14, 3, 2, 3], [3, 0, 2, 1]); 
        behaves_like_3 += check_sample([2, 3, 3, 2], [11, 0, 3, 0], [1, 3, 3, 2]); 
        behaves_like_3 += check_sample([3, 2, 0, 2], [3, 3, 3, 3], [3, 2, 0, 0]); 
        behaves_like_3 += check_sample([1, 1, 3, 1], [12, 2, 3, 0], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [3, 3, 3, 1], [0, 0, 2, 2]); 
        behaves_like_3 += check_sample([1, 2, 2, 2], [12, 2, 1, 3], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [9, 2, 0, 0], [2, 0, 1, 1]); 
        behaves_like_3 += check_sample([2, 1, 0, 3], [4, 3, 3, 3], [2, 1, 0, 3]); 
        behaves_like_3 += check_sample([2, 1, 2, 1], [12, 2, 0, 0], [1, 1, 2, 1]); 
        behaves_like_3 += check_sample([0, 1, 2, 2], [10, 1, 0, 2], [0, 1, 1, 2]); 
        behaves_like_3 += check_sample([1, 0, 1, 2], [9, 2, 0, 2], [1, 0, 2, 2]); 
        behaves_like_3 += check_sample([3, 1, 3, 3], [7, 0, 3, 3], [3, 1, 3, 1]); 
        behaves_like_3 += check_sample([0, 1, 3, 3], [10, 1, 0, 2], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([1, 3, 1, 1], [3, 2, 3, 1], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([1, 3, 2, 3], [2, 0, 2, 0], [0, 3, 2, 3]); 
        behaves_like_3 += check_sample([1, 3, 1, 1], [9, 2, 0, 1], [1, 2, 1, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 0], [2, 0, 2, 3], [1, 1, 2, 0]); 
        behaves_like_3 += check_sample([2, 3, 1, 2], [11, 0, 3, 1], [2, 1, 1, 2]); 
        behaves_like_3 += check_sample([0, 0, 1, 0], [1, 0, 0, 0], [0, 0, 1, 0]); 
        behaves_like_3 += check_sample([2, 1, 0, 2], [6, 1, 3, 1], [2, 0, 0, 2]); 
        behaves_like_3 += check_sample([2, 3, 2, 3], [7, 0, 2, 0], [1, 3, 2, 3]); 
        behaves_like_3 += check_sample([0, 3, 2, 3], [0, 2, 3, 0], [0, 3, 2, 3]); 
        behaves_like_3 += check_sample([2, 2, 3, 3], [0, 1, 3, 1], [2, 0, 3, 3]); 
        behaves_like_3 += check_sample([1, 0, 2, 1], [14, 3, 2, 3], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([3, 1, 2, 3], [0, 2, 3, 1], [3, 0, 2, 3]); 
        behaves_like_3 += check_sample([0, 0, 3, 3], [13, 3, 3, 3], [0, 0, 3, 1]); 
        behaves_like_3 += check_sample([0, 3, 0, 3], [1, 0, 0, 3], [0, 3, 0, 0]); 
        behaves_like_3 += check_sample([0, 1, 3, 2], [6, 1, 3, 3], [0, 1, 3, 0]); 
        behaves_like_3 += check_sample([0, 3, 3, 3], [7, 2, 3, 0], [1, 3, 3, 3]); 
        behaves_like_3 += check_sample([0, 1, 0, 0], [5, 1, 3, 3], [0, 1, 0, 1]); 
        behaves_like_3 += check_sample([1, 3, 2, 1], [2, 0, 2, 1], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([1, 0, 1, 1], [15, 2, 1, 0], [1, 0, 1, 1]); 
        behaves_like_3 += check_sample([2, 0, 0, 2], [11, 0, 3, 2], [2, 0, 1, 2]); 
        behaves_like_3 += check_sample([3, 1, 3, 2], [6, 1, 3, 2], [3, 1, 0, 2]); 
        behaves_like_3 += check_sample([1, 0, 1, 0], [15, 2, 1, 1], [1, 1, 1, 0]); 
        behaves_like_3 += check_sample([0, 1, 3, 3], [10, 1, 0, 3], [0, 1, 3, 1]); 
        behaves_like_3 += check_sample([2, 1, 3, 0], [5, 1, 3, 1], [2, 1, 3, 0]); 
        behaves_like_3 += check_sample([0, 1, 0, 3], [10, 1, 0, 2], [0, 1, 1, 3]); 
        behaves_like_3 += check_sample([1, 3, 2, 1], [2, 0, 2, 2], [1, 3, 0, 1]); 
        behaves_like_3 += check_sample([2, 1, 3, 2], [6, 1, 3, 3], [2, 1, 3, 0]); 
        behaves_like_3 += check_sample([0, 0, 2, 1], [15, 3, 1, 0], [1, 0, 2, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 2], [2, 0, 2, 3], [1, 2, 2, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 0], [5, 1, 3, 3], [3, 1, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [8, 1, 2, 2], [1, 1, 0, 3]); 
        behaves_like_3 += check_sample([0, 1, 3, 1], [10, 1, 0, 0], [1, 1, 3, 1]); 
        behaves_like_3 += check_sample([0, 0, 0, 2], [1, 0, 0, 3], [0, 0, 0, 0]); 
        behaves_like_3 += check_sample([3, 1, 2, 3], [13, 3, 3, 3], [3, 1, 2, 1]); 
        behaves_like_3 += check_sample([2, 2, 2, 1], [7, 0, 2, 3], [2, 2, 2, 1]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [14, 3, 2, 3], [1, 2, 2, 1]); 
        behaves_like_3 += check_sample([1, 1, 2, 3], [8, 1, 2, 1], [1, 0, 2, 3]); 
        behaves_like_3 += check_sample([1, 3, 2, 1], [2, 0, 2, 3], [1, 3, 2, 0]); 
        behaves_like_3 += check_sample([2, 2, 2, 3], [13, 3, 2, 2], [2, 2, 0, 3]); 
        behaves_like_3 += check_sample([1, 2, 2, 1], [2, 0, 2, 3], [1, 2, 2, 0]); 
        behaves_like_3 += check_sample([3, 1, 3, 0], [5, 1, 3, 0], [1, 1, 3, 0]); 
        behaves_like_3 += check_sample([2, 3, 1, 2], [11, 0, 3, 2], [2, 3, 1, 2]); 
        behaves_like_3 += check_sample([1, 1, 3, 2], [6, 1, 3, 0], [0, 1, 3, 2]); 
        behaves_like_3 += check_sample([0, 1, 2, 3], [8, 1, 2, 1], [0, 0, 2, 3]); 
        behaves_like_3 += check_sample([1, 1, 2, 2], [8, 1, 2, 2], [1, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 1, 2, 1], [14, 3, 2, 2], [2, 1, 1, 1]); 
        behaves_like_3 += check_sample([0, 1, 3, 2], [6, 1, 3, 2], [0, 1, 0, 2]); 
        behaves_like_3 += check_sample([2, 0, 1, 3], [0, 2, 3, 1], [2, 0, 1, 3]); 
        behaves_like_3 += check_sample([2, 1, 0, 2], [6, 1, 3, 3], [2, 1, 0, 0]);
        assert_eq!(0, behaves_like_3);
    }
}
