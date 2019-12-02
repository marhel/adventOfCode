#[derive(Debug, PartialEq, Copy, Clone)]
struct CPU {
    regs: [i32; 6]
}

impl CPU {
    fn new(regs: [i32; 6]) -> CPU {
        CPU { regs }
    }
}
const OP: usize = 0;
const A: usize = 1;
const B: usize = 2;
const C: usize = 3;
const D: usize = 4;
const IP: usize = 5;

const MULI: i32 = 15;
const SETR: i32 = 14;
const EQIR: i32 = 13;
const GTRI: i32 = 12;
const EQRR: i32 = 11;
const ADDI: i32 = 10;
const ADDR: i32 = 9;
const BANI: i32 = 8;
const EQRI: i32 = 7;
const BANR: i32 = 6;
const BORR: i32 = 5;
const BORI: i32 = 4;
const GTRR: i32 = 3;
const SETI: i32 = 2;
const MULR: i32 = 1;
const GTIR: i32 = 0;

// Addition:
// 
// addr (add register) stores into register C the result of adding register A and register B.
// addi (add immediate) stores into register C the result of adding register A and value B.
fn addr<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = cpu.regs[operands[A] as usize] + cpu.regs[operands[B] as usize];
    cpu
}

fn addi<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = cpu.regs[operands[A] as usize] + operands[B];
    cpu
}

// Multiplication:
// 
// mulr (multiply register) stores into register C the result of multiplying register A and register B.
// muli (multiply immediate) stores into register C the result of multiplying register A and value B.
fn mulr<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = cpu.regs[operands[A] as usize] * cpu.regs[operands[B] as usize];
    cpu
}

fn muli<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = cpu.regs[operands[A] as usize] * operands[B];
    cpu
}

// Bitwise AND:
// 
// banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
// bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
fn banr<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = cpu.regs[operands[A] as usize] & cpu.regs[operands[B] as usize];
    cpu
}

fn bani<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = cpu.regs[operands[A] as usize] & operands[B];
    cpu
}

// Bitwise OR:
// 
// borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
// bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
fn borr<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = cpu.regs[operands[A] as usize] | cpu.regs[operands[B] as usize];
    cpu
}

fn bori<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = cpu.regs[operands[A] as usize] | operands[B];
    cpu
}

// Assignment:
// 
// setr (set register) copies the contents of register A into register C. (Input B is ignored.)
// seti (set immediate) stores value A into register C. (Input B is ignored.)
fn setr<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = cpu.regs[operands[A] as usize];
    cpu
}

fn seti<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = operands[A];
    cpu
}

// Greater-than testing:
// 
// gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
// gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
// gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.

fn gtir<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = if operands[A] > cpu.regs[operands[B] as usize] { 1 } else { 0 };
    cpu
}

fn gtri<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = if cpu.regs[operands[A] as usize] > operands[B] { 1 } else { 0 };
    cpu
}

fn gtrr<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = if cpu.regs[operands[A] as usize] > cpu.regs[operands[B] as usize] { 1 } else { 0 };
    cpu
}

// Equality testing:
// 
// eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
// eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
// eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.

fn eqir<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = if operands[A] == cpu.regs[operands[B] as usize] { 1 } else { 0 };
    cpu
}

fn eqri<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = if cpu.regs[operands[A] as usize] == operands[B] { 1 } else { 0 };
    cpu
}

fn eqrr<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    cpu.regs[operands[C] as usize] = if cpu.regs[operands[A] as usize] == cpu.regs[operands[B] as usize] { 1 } else { 0 };
    cpu
}

// execute the opcodes
fn exec<'a>(cpu: &'a mut CPU, operands: &[i32]) -> &'a CPU {
    match operands[0] {
        MULI => muli(cpu, operands),
        SETR => setr(cpu, operands),
        EQIR => eqir(cpu, operands),
        GTRI => gtri(cpu, operands),
        EQRR => eqrr(cpu, operands),
        ADDI => addi(cpu, operands),
        MULR => mulr(cpu, operands),
        BANI => bani(cpu, operands),
        EQRI => eqri(cpu, operands),
        BANR => banr(cpu, operands),
        BORR => borr(cpu, operands),
        BORI => bori(cpu, operands),
        GTRR => gtrr(cpu, operands),
        SETI => seti(cpu, operands),
        ADDR => addr(cpu, operands),
        GTIR => gtir(cpu, operands),        
        _ => panic!("unknown opcode {:?}", operands[0]),
    }
}

fn execm<'a>(cpu: &'a mut CPU, memory: &Vec<i32>) -> &'a CPU {
    let mip: usize = (cpu.regs[IP]*4) as usize;
    if mip > memory.len() {
        panic!("{:?} {:?} {:?}", cpu, mip, memory.len());
    }
    let r = mip..(mip+4);
    let operands = &memory[r];
    match operands[0] {
        MULI => muli(cpu, operands),
        SETR => setr(cpu, operands),
        EQIR => eqir(cpu, operands),
        GTRI => gtri(cpu, operands),
        EQRR => eqrr(cpu, operands),
        ADDI => addi(cpu, operands),
        MULR => mulr(cpu, operands),
        BANI => bani(cpu, operands),
        EQRI => eqri(cpu, operands),
        BANR => banr(cpu, operands),
        BORR => borr(cpu, operands),
        BORI => bori(cpu, operands),
        GTRR => gtrr(cpu, operands),
        SETI => seti(cpu, operands),
        ADDR => addr(cpu, operands),
        GTIR => gtir(cpu, operands),        
        _ => panic!("unknown opcode {:?}", operands[0]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn next_part1() {
        let cpu = &mut CPU::new([0, 0, 0, 0, 0, 0]);
        let mut mem = Vec::with_capacity(1000);
        mem.extend_from_slice(&[ADDI, 5, 16, 5]);
        mem.extend_from_slice(&[SETI, 1, 8, 2]);
        mem.extend_from_slice(&[SETI, 1, 1, 1]);
        mem.extend_from_slice(&[MULR, 2, 1, 4]);
        mem.extend_from_slice(&[EQRR, 4, 3, 4]);
        mem.extend_from_slice(&[ADDR, 4, 5, 5]);
        mem.extend_from_slice(&[ADDI, 5, 1, 5]);
        mem.extend_from_slice(&[ADDR, 2, 0, 0]);
        mem.extend_from_slice(&[ADDI, 1, 1, 1]);
        mem.extend_from_slice(&[GTRR, 1, 3, 4]);
        mem.extend_from_slice(&[ADDR, 5, 4, 5]);
        mem.extend_from_slice(&[SETI, 2, 8, 5]);
        mem.extend_from_slice(&[ADDI, 2, 1, 2]);
        mem.extend_from_slice(&[GTRR, 2, 3, 4]);
        mem.extend_from_slice(&[ADDR, 4, 5, 5]);
        mem.extend_from_slice(&[SETI, 1, 7, 5]);
        mem.extend_from_slice(&[MULR, 5, 5, 5]);
        mem.extend_from_slice(&[ADDI, 3, 2, 3]);
        mem.extend_from_slice(&[MULR, 3, 3, 3]);
        mem.extend_from_slice(&[MULR, 5, 3, 3]);
        mem.extend_from_slice(&[MULI, 3, 11, 3]);
        mem.extend_from_slice(&[ADDI, 4, 6, 4]);
        mem.extend_from_slice(&[MULR, 4, 5, 4]);
        mem.extend_from_slice(&[ADDI, 4, 5, 4]);
        mem.extend_from_slice(&[ADDR, 3, 4, 3]);
        mem.extend_from_slice(&[ADDR, 5, 0, 5]);
        mem.extend_from_slice(&[SETI, 0, 0, 5]);
        mem.extend_from_slice(&[SETR, 5, 3, 4]);
        mem.extend_from_slice(&[MULR, 4, 5, 4]);
        mem.extend_from_slice(&[ADDR, 5, 4, 4]);
        mem.extend_from_slice(&[MULR, 5, 4, 4]);
        mem.extend_from_slice(&[MULI, 4, 14, 4]);
        mem.extend_from_slice(&[MULR, 4, 5, 4]);
        mem.extend_from_slice(&[ADDR, 3, 4, 3]);
        mem.extend_from_slice(&[SETI, 0, 3, 0]);
        mem.extend_from_slice(&[SETI, 0, 0, 5]);

        cpu.regs[0] = 1;
        let mut last0 = 0; 
        let mut turns:i64 = 0;
        loop {
            if(cpu.regs[0] != last0) {
                last0 = cpu.regs[0];
                println!("{:?} {:?}", turns, cpu);
            }
            turns += 1;
            //println!("CPU {:?}", cpu);
            execm(cpu, &mem);
            cpu.regs[IP] += 1;
        }
    }

/*
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
    #[test]
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

    fn check_sample(before: [i32; 4], operands: &[i32], after: [i32; 4]) -> i32 {
        let mut behaves = 0;
        let before = CPU::new(before);
        let after = CPU::new(after);

        behaves += if *addr(&mut before.clone(), operands) == after { println!("# {} as addr", operands[0]); 1 } else { 0 };
        behaves += if *addi(&mut before.clone(), operands) == after { println!("# {} as addi", operands[0]); 1 } else { 0 };
        behaves += if *mulr(&mut before.clone(), operands) == after { println!("# {} as mulr", operands[0]); 1 } else { 0 };
        behaves += if *muli(&mut before.clone(), operands) == after { println!("# {} as muli", operands[0]); 1 } else { 0 };
        behaves += if *banr(&mut before.clone(), operands) == after { println!("# {} as banr", operands[0]); 1 } else { 0 };
        behaves += if *bani(&mut before.clone(), operands) == after { println!("# {} as bani", operands[0]); 1 } else { 0 };
        behaves += if *borr(&mut before.clone(), operands) == after { println!("# {} as borr", operands[0]); 1 } else { 0 };
        behaves += if *bori(&mut before.clone(), operands) == after { println!("# {} as bori", operands[0]); 1 } else { 0 };
        behaves += if *setr(&mut before.clone(), operands) == after { println!("# {} as setr", operands[0]); 1 } else { 0 };
        behaves += if *seti(&mut before.clone(), operands) == after { println!("# {} as seti", operands[0]); 1 } else { 0 };
        behaves += if *gtir(&mut before.clone(), operands) == after { println!("# {} as gtir", operands[0]); 1 } else { 0 };
        behaves += if *gtri(&mut before.clone(), operands) == after { println!("# {} as gtri", operands[0]); 1 } else { 0 };
        behaves += if *gtrr(&mut before.clone(), operands) == after { println!("# {} as gtrr", operands[0]); 1 } else { 0 };
        behaves += if *eqir(&mut before.clone(), operands) == after { println!("# {} as eqir", operands[0]); 1 } else { 0 };
        behaves += if *eqri(&mut before.clone(), operands) == after { println!("# {} as eqri", operands[0]); 1 } else { 0 };
        behaves += if *eqrr(&mut before.clone(), operands) == after { println!("# {} as eqrr", operands[0]); 1 } else { 0 };
        if behaves == 1 {
            println!("# {} IS above", operands[0])
        }
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
    fn part2() {
        let cpu = &mut CPU::new([0, 0, 0, 0]);

        exec(cpu, [2, 2, 3, 2]);
        exec(cpu, [15, 1, 0, 1]);
        exec(cpu, [10, 1, 0, 1]);
        exec(cpu, [2, 0, 1, 3]);
        exec(cpu, [0, 3, 2, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 0, 1, 0]);
        exec(cpu, [14, 0, 2, 2]);
        exec(cpu, [2, 2, 3, 0]);
        exec(cpu, [2, 1, 2, 1]);
        exec(cpu, [2, 3, 0, 3]);
        exec(cpu, [1, 1, 0, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [14, 2, 3, 1]);
        exec(cpu, [2, 1, 0, 0]);
        exec(cpu, [15, 1, 0, 2]);
        exec(cpu, [10, 2, 2, 2]);
        exec(cpu, [2, 2, 0, 3]);
        exec(cpu, [1, 0, 3, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [14, 1, 2, 3]);
        exec(cpu, [2, 3, 1, 1]);
        exec(cpu, [2, 1, 0, 0]);
        exec(cpu, [14, 0, 2, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 3, 3]);
        exec(cpu, [14, 3, 1, 1]);
        exec(cpu, [2, 3, 1, 2]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [2, 2, 1, 0]);
        exec(cpu, [1, 3, 0, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [14, 1, 1, 2]);
        exec(cpu, [2, 1, 3, 0]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [2, 3, 1, 3]);
        exec(cpu, [9, 0, 0, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 1, 2, 2]);
        exec(cpu, [14, 2, 0, 1]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 2, 0, 3]);
        exec(cpu, [2, 2, 3, 2]);
        exec(cpu, [11, 0, 3, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 1, 1]);
        exec(cpu, [2, 0, 0, 0]);
        exec(cpu, [2, 0, 3, 2]);
        exec(cpu, [13, 2, 3, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 1, 2, 1]);
        exec(cpu, [14, 1, 0, 0]);
        exec(cpu, [2, 0, 2, 2]);
        exec(cpu, [15, 0, 0, 1]);
        exec(cpu, [10, 1, 0, 1]);
        exec(cpu, [2, 2, 1, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 0, 2, 0]);
        exec(cpu, [14, 0, 1, 2]);
        exec(cpu, [2, 3, 0, 1]);
        exec(cpu, [2, 2, 1, 0]);
        exec(cpu, [12, 0, 1, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [14, 2, 0, 1]);
        exec(cpu, [2, 1, 3, 2]);
        exec(cpu, [15, 1, 0, 0]);
        exec(cpu, [10, 0, 3, 0]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [8, 0, 2, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 1, 2, 1]);
        exec(cpu, [14, 1, 1, 0]);
        exec(cpu, [2, 0, 3, 3]);
        exec(cpu, [2, 1, 3, 2]);
        exec(cpu, [2, 3, 2, 1]);
        exec(cpu, [8, 1, 2, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 1, 0, 0]);
        exec(cpu, [14, 0, 2, 1]);
        exec(cpu, [2, 3, 2, 2]);
        exec(cpu, [2, 1, 1, 3]);
        exec(cpu, [2, 0, 0, 0]);
        exec(cpu, [15, 3, 2, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [2, 2, 2, 2]);
        exec(cpu, [2, 1, 0, 0]);
        exec(cpu, [9, 3, 0, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [14, 1, 2, 0]);
        exec(cpu, [2, 3, 2, 1]);
        exec(cpu, [2, 0, 1, 3]);
        exec(cpu, [0, 3, 2, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 1, 0, 0]);
        exec(cpu, [14, 0, 2, 3]);
        exec(cpu, [15, 3, 0, 0]);
        exec(cpu, [10, 0, 3, 0]);
        exec(cpu, [2, 0, 1, 2]);
        exec(cpu, [2, 1, 0, 1]);
        exec(cpu, [7, 2, 0, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 3, 1, 3]);
        exec(cpu, [14, 3, 1, 1]);
        exec(cpu, [2, 0, 3, 3]);
        exec(cpu, [15, 0, 0, 2]);
        exec(cpu, [10, 2, 3, 2]);
        exec(cpu, [2, 1, 3, 0]);
        exec(cpu, [13, 3, 2, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [14, 1, 2, 3]);
        exec(cpu, [2, 1, 2, 1]);
        exec(cpu, [2, 2, 3, 0]);
        exec(cpu, [2, 1, 0, 2]);
        exec(cpu, [1, 1, 0, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 3, 0, 3]);
        exec(cpu, [14, 3, 3, 2]);
        exec(cpu, [2, 2, 3, 3]);
        exec(cpu, [15, 3, 0, 0]);
        exec(cpu, [10, 0, 3, 0]);
        exec(cpu, [1, 1, 3, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 1, 2, 2]);
        exec(cpu, [14, 2, 3, 1]);
        exec(cpu, [2, 0, 1, 0]);
        exec(cpu, [2, 0, 1, 2]);
        exec(cpu, [13, 2, 3, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [14, 1, 2, 2]);
        exec(cpu, [15, 0, 0, 3]);
        exec(cpu, [10, 3, 1, 3]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 0, 2, 1]);
        exec(cpu, [10, 3, 1, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [9, 2, 3, 2]);
        exec(cpu, [14, 2, 2, 0]);
        exec(cpu, [2, 3, 3, 1]);
        exec(cpu, [2, 2, 0, 2]);
        exec(cpu, [15, 3, 0, 3]);
        exec(cpu, [10, 3, 3, 3]);
        exec(cpu, [12, 2, 1, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 1, 0, 0]);
        exec(cpu, [15, 0, 0, 2]);
        exec(cpu, [10, 2, 3, 2]);
        exec(cpu, [15, 0, 0, 1]);
        exec(cpu, [10, 1, 0, 1]);
        exec(cpu, [2, 1, 3, 3]);
        exec(cpu, [10, 3, 1, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 1, 0, 0]);
        exec(cpu, [14, 0, 1, 3]);
        exec(cpu, [2, 1, 1, 1]);
        exec(cpu, [2, 3, 0, 0]);
        exec(cpu, [15, 1, 2, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 3, 0, 3]);
        exec(cpu, [14, 3, 3, 0]);
        exec(cpu, [15, 0, 0, 2]);
        exec(cpu, [10, 2, 1, 2]);
        exec(cpu, [2, 2, 0, 3]);
        exec(cpu, [1, 1, 3, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 0, 2, 0]);
        exec(cpu, [14, 0, 1, 3]);
        exec(cpu, [2, 1, 1, 2]);
        exec(cpu, [2, 3, 1, 1]);
        exec(cpu, [2, 2, 1, 0]);
        exec(cpu, [12, 0, 1, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 2, 3, 3]);
        exec(cpu, [2, 0, 1, 2]);
        exec(cpu, [2, 1, 2, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 1, 3, 3]);
        exec(cpu, [14, 3, 0, 1]);
        exec(cpu, [2, 1, 0, 3]);
        exec(cpu, [2, 1, 0, 2]);
        exec(cpu, [3, 0, 3, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [2, 2, 3, 3]);
        exec(cpu, [2, 1, 3, 2]);
        exec(cpu, [11, 0, 3, 3]);
        exec(cpu, [15, 3, 2, 3]);
        exec(cpu, [9, 1, 3, 1]);
        exec(cpu, [14, 1, 2, 2]);
        exec(cpu, [2, 3, 0, 0]);
        exec(cpu, [2, 3, 3, 1]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [10, 3, 1, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 2, 0, 2]);
        exec(cpu, [14, 2, 3, 0]);
        exec(cpu, [2, 0, 1, 3]);
        exec(cpu, [15, 0, 0, 2]);
        exec(cpu, [10, 2, 2, 2]);
        exec(cpu, [2, 0, 0, 1]);
        exec(cpu, [0, 3, 2, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 0, 2, 0]);
        exec(cpu, [15, 0, 0, 3]);
        exec(cpu, [10, 3, 2, 3]);
        exec(cpu, [2, 1, 1, 1]);
        exec(cpu, [2, 2, 1, 2]);
        exec(cpu, [4, 2, 3, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 0, 1, 0]);
        exec(cpu, [14, 0, 0, 3]);
        exec(cpu, [2, 3, 1, 0]);
        exec(cpu, [2, 0, 3, 2]);
        exec(cpu, [2, 2, 1, 1]);
        exec(cpu, [6, 0, 1, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 1, 3, 3]);
        exec(cpu, [15, 1, 0, 1]);
        exec(cpu, [10, 1, 2, 1]);
        exec(cpu, [2, 1, 2, 0]);
        exec(cpu, [2, 2, 0, 2]);
        exec(cpu, [14, 0, 2, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 3, 3]);
        exec(cpu, [14, 3, 3, 1]);
        exec(cpu, [2, 1, 1, 0]);
        exec(cpu, [2, 0, 2, 2]);
        exec(cpu, [2, 2, 0, 3]);
        exec(cpu, [13, 2, 3, 3]);
        exec(cpu, [15, 3, 2, 3]);
        exec(cpu, [9, 1, 3, 1]);
        exec(cpu, [14, 1, 3, 3]);
        exec(cpu, [2, 3, 3, 2]);
        exec(cpu, [2, 0, 1, 1]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [7, 0, 2, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 3, 1, 3]);
        exec(cpu, [14, 3, 3, 1]);
        exec(cpu, [2, 0, 1, 2]);
        exec(cpu, [2, 3, 0, 0]);
        exec(cpu, [2, 2, 2, 3]);
        exec(cpu, [8, 0, 2, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [14, 1, 1, 3]);
        exec(cpu, [2, 3, 0, 0]);
        exec(cpu, [2, 0, 2, 1]);
        exec(cpu, [7, 2, 0, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 2, 3, 3]);
        exec(cpu, [14, 3, 2, 0]);
        exec(cpu, [2, 3, 0, 3]);
        exec(cpu, [15, 1, 0, 2]);
        exec(cpu, [10, 2, 0, 2]);
        exec(cpu, [8, 3, 2, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 0, 2, 0]);
        exec(cpu, [14, 0, 3, 3]);
        exec(cpu, [2, 0, 1, 2]);
        exec(cpu, [15, 2, 0, 0]);
        exec(cpu, [10, 0, 3, 0]);
        exec(cpu, [7, 2, 0, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 3, 0, 3]);
        exec(cpu, [14, 3, 1, 2]);
        exec(cpu, [2, 2, 1, 0]);
        exec(cpu, [2, 2, 1, 3]);
        exec(cpu, [11, 0, 3, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [15, 3, 0, 0]);
        exec(cpu, [10, 0, 2, 0]);
        exec(cpu, [3, 0, 3, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 2, 1, 2]);
        exec(cpu, [14, 2, 0, 1]);
        exec(cpu, [2, 2, 0, 2]);
        exec(cpu, [3, 0, 3, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 0, 1, 1]);
        exec(cpu, [14, 1, 3, 0]);
        exec(cpu, [15, 3, 0, 3]);
        exec(cpu, [10, 3, 2, 3]);
        exec(cpu, [2, 3, 2, 1]);
        exec(cpu, [6, 1, 3, 3]);
        exec(cpu, [15, 3, 3, 3]);
        exec(cpu, [9, 0, 3, 0]);
        exec(cpu, [14, 0, 1, 2]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 2, 2, 1]);
        exec(cpu, [15, 1, 0, 3]);
        exec(cpu, [10, 3, 3, 3]);
        exec(cpu, [6, 3, 1, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [9, 2, 3, 2]);
        exec(cpu, [14, 2, 3, 1]);
        exec(cpu, [15, 0, 0, 2]);
        exec(cpu, [10, 2, 1, 2]);
        exec(cpu, [2, 1, 0, 0]);
        exec(cpu, [2, 2, 0, 3]);
        exec(cpu, [1, 0, 3, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [14, 1, 2, 0]);
        exec(cpu, [15, 2, 0, 1]);
        exec(cpu, [10, 1, 3, 1]);
        exec(cpu, [2, 3, 0, 2]);
        exec(cpu, [2, 0, 1, 3]);
        exec(cpu, [13, 3, 2, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 0, 1, 0]);
        exec(cpu, [14, 0, 0, 2]);
        exec(cpu, [2, 0, 0, 1]);
        exec(cpu, [2, 2, 1, 0]);
        exec(cpu, [2, 2, 2, 3]);
        exec(cpu, [11, 0, 3, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 1, 2, 2]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [2, 1, 2, 1]);
        exec(cpu, [3, 0, 3, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [14, 2, 0, 3]);
        exec(cpu, [2, 2, 0, 2]);
        exec(cpu, [2, 1, 0, 0]);
        exec(cpu, [14, 0, 2, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 3, 3]);
        exec(cpu, [14, 3, 3, 1]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 0, 2, 3]);
        exec(cpu, [4, 2, 3, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [14, 1, 1, 3]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 3, 1, 1]);
        exec(cpu, [2, 1, 3, 2]);
        exec(cpu, [6, 1, 0, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 3, 1, 3]);
        exec(cpu, [2, 2, 2, 1]);
        exec(cpu, [2, 2, 1, 2]);
        exec(cpu, [2, 3, 3, 0]);
        exec(cpu, [5, 2, 0, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 3, 0, 3]);
        exec(cpu, [14, 3, 2, 1]);
        exec(cpu, [2, 2, 0, 3]);
        exec(cpu, [2, 0, 2, 2]);
        exec(cpu, [2, 3, 3, 0]);
        exec(cpu, [13, 2, 3, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 1, 1]);
        exec(cpu, [2, 2, 3, 0]);
        exec(cpu, [2, 3, 2, 2]);
        exec(cpu, [11, 0, 3, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [15, 1, 0, 0]);
        exec(cpu, [10, 0, 1, 0]);
        exec(cpu, [15, 3, 0, 2]);
        exec(cpu, [10, 2, 3, 2]);
        exec(cpu, [1, 0, 3, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [15, 2, 0, 2]);
        exec(cpu, [10, 2, 1, 2]);
        exec(cpu, [2, 0, 0, 0]);
        exec(cpu, [2, 3, 1, 3]);
        exec(cpu, [8, 3, 2, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [9, 1, 3, 1]);
        exec(cpu, [14, 1, 0, 3]);
        exec(cpu, [15, 0, 0, 2]);
        exec(cpu, [10, 2, 3, 2]);
        exec(cpu, [2, 3, 1, 1]);
        exec(cpu, [2, 1, 3, 0]);
        exec(cpu, [10, 0, 1, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 3, 3]);
        exec(cpu, [14, 3, 2, 2]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [15, 0, 0, 0]);
        exec(cpu, [10, 0, 3, 0]);
        exec(cpu, [2, 3, 3, 3]);
        exec(cpu, [6, 0, 1, 3]);
        exec(cpu, [15, 3, 2, 3]);
        exec(cpu, [9, 2, 3, 2]);
        exec(cpu, [14, 2, 3, 1]);
        exec(cpu, [2, 2, 2, 0]);
        exec(cpu, [2, 3, 1, 2]);
        exec(cpu, [2, 2, 3, 3]);
        exec(cpu, [5, 0, 2, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [2, 2, 2, 2]);
        exec(cpu, [15, 0, 0, 0]);
        exec(cpu, [10, 0, 1, 0]);
        exec(cpu, [2, 3, 3, 3]);
        exec(cpu, [14, 0, 2, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 1, 2, 1]);
        exec(cpu, [15, 0, 0, 3]);
        exec(cpu, [10, 3, 1, 3]);
        exec(cpu, [2, 2, 3, 2]);
        exec(cpu, [15, 3, 0, 0]);
        exec(cpu, [10, 0, 2, 0]);
        exec(cpu, [3, 0, 3, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [2, 0, 2, 2]);
        exec(cpu, [2, 3, 2, 3]);
        exec(cpu, [2, 3, 2, 0]);
        exec(cpu, [7, 2, 0, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 1, 1]);
        exec(cpu, [14, 1, 3, 2]);
        exec(cpu, [2, 2, 3, 3]);
        exec(cpu, [15, 2, 0, 0]);
        exec(cpu, [10, 0, 2, 0]);
        exec(cpu, [2, 0, 0, 1]);
        exec(cpu, [2, 1, 3, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 2, 0, 2]);
        exec(cpu, [14, 2, 1, 3]);
        exec(cpu, [2, 2, 2, 2]);
        exec(cpu, [2, 1, 3, 0]);
        exec(cpu, [14, 0, 2, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 3, 1, 3]);
        exec(cpu, [14, 3, 2, 2]);
        exec(cpu, [2, 0, 3, 3]);
        exec(cpu, [2, 2, 1, 1]);
        exec(cpu, [2, 3, 0, 0]);
        exec(cpu, [4, 1, 3, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [14, 2, 1, 1]);
        exec(cpu, [2, 3, 1, 3]);
        exec(cpu, [15, 1, 0, 2]);
        exec(cpu, [10, 2, 2, 2]);
        exec(cpu, [2, 3, 0, 0]);
        exec(cpu, [12, 2, 0, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 1, 2, 1]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [2, 2, 2, 2]);
        exec(cpu, [9, 3, 3, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 1, 2, 1]);
        exec(cpu, [14, 1, 0, 2]);
        exec(cpu, [2, 2, 0, 3]);
        exec(cpu, [2, 2, 3, 0]);
        exec(cpu, [15, 1, 0, 1]);
        exec(cpu, [10, 1, 2, 1]);
        exec(cpu, [11, 0, 3, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [2, 2, 3, 0]);
        exec(cpu, [2, 3, 1, 1]);
        exec(cpu, [11, 0, 3, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 2, 1, 2]);
        exec(cpu, [14, 2, 0, 3]);
        exec(cpu, [2, 3, 3, 1]);
        exec(cpu, [2, 3, 1, 2]);
        exec(cpu, [6, 1, 0, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 3, 2, 3]);
        exec(cpu, [14, 3, 2, 0]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [15, 2, 0, 3]);
        exec(cpu, [10, 3, 0, 3]);
        exec(cpu, [2, 2, 0, 2]);
        exec(cpu, [0, 3, 2, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 0, 1, 0]);
        exec(cpu, [14, 0, 1, 2]);
        exec(cpu, [2, 2, 1, 3]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [15, 3, 0, 1]);
        exec(cpu, [10, 1, 3, 1]);
        exec(cpu, [11, 0, 3, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [14, 2, 2, 1]);
        exec(cpu, [2, 1, 3, 0]);
        exec(cpu, [15, 3, 0, 3]);
        exec(cpu, [10, 3, 3, 3]);
        exec(cpu, [2, 0, 3, 2]);
        exec(cpu, [15, 0, 2, 3]);
        exec(cpu, [15, 3, 3, 3]);
        exec(cpu, [9, 3, 1, 1]);
        exec(cpu, [14, 1, 2, 3]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [2, 3, 3, 0]);
        exec(cpu, [2, 3, 1, 2]);
        exec(cpu, [8, 0, 2, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 3, 0, 3]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 0, 2, 1]);
        exec(cpu, [5, 0, 2, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 3, 0, 3]);
        exec(cpu, [14, 3, 1, 0]);
        exec(cpu, [2, 0, 3, 2]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [2, 3, 0, 1]);
        exec(cpu, [10, 3, 1, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 0, 1, 0]);
        exec(cpu, [14, 0, 2, 3]);
        exec(cpu, [2, 1, 2, 0]);
        exec(cpu, [15, 2, 0, 2]);
        exec(cpu, [10, 2, 3, 2]);
        exec(cpu, [2, 3, 0, 1]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 3, 1, 3]);
        exec(cpu, [14, 3, 0, 1]);
        exec(cpu, [15, 0, 0, 3]);
        exec(cpu, [10, 3, 0, 3]);
        exec(cpu, [15, 2, 0, 0]);
        exec(cpu, [10, 0, 3, 0]);
        exec(cpu, [15, 3, 0, 2]);
        exec(cpu, [10, 2, 1, 2]);
        exec(cpu, [8, 0, 2, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 1, 2, 1]);
        exec(cpu, [14, 1, 3, 2]);
        exec(cpu, [2, 3, 2, 3]);
        exec(cpu, [2, 3, 0, 1]);
        exec(cpu, [2, 1, 1, 0]);
        exec(cpu, [10, 0, 1, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 2, 0, 2]);
        exec(cpu, [14, 2, 3, 0]);
        exec(cpu, [2, 3, 0, 2]);
        exec(cpu, [2, 0, 2, 3]);
        exec(cpu, [2, 0, 0, 1]);
        exec(cpu, [2, 2, 3, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 0, 2, 0]);
        exec(cpu, [14, 0, 1, 2]);
        exec(cpu, [2, 2, 2, 0]);
        exec(cpu, [4, 0, 3, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 2, 1, 2]);
        exec(cpu, [2, 2, 3, 3]);
        exec(cpu, [2, 0, 0, 1]);
        exec(cpu, [2, 1, 1, 0]);
        exec(cpu, [1, 0, 3, 3]);
        exec(cpu, [15, 3, 3, 3]);
        exec(cpu, [15, 3, 3, 3]);
        exec(cpu, [9, 2, 3, 2]);
        exec(cpu, [14, 2, 0, 3]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [2, 3, 1, 0]);
        exec(cpu, [2, 1, 3, 2]);
        exec(cpu, [5, 1, 0, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 2, 3, 3]);
        exec(cpu, [14, 3, 0, 2]);
        exec(cpu, [2, 3, 0, 3]);
        exec(cpu, [2, 3, 1, 1]);
        exec(cpu, [2, 2, 2, 0]);
        exec(cpu, [6, 1, 0, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [14, 2, 1, 0]);
        exec(cpu, [2, 0, 0, 1]);
        exec(cpu, [15, 2, 0, 2]);
        exec(cpu, [10, 2, 3, 2]);
        exec(cpu, [2, 2, 2, 3]);
        exec(cpu, [2, 1, 3, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 0, 1, 0]);
        exec(cpu, [14, 0, 0, 1]);
        exec(cpu, [15, 2, 0, 2]);
        exec(cpu, [10, 2, 2, 2]);
        exec(cpu, [2, 1, 2, 0]);
        exec(cpu, [4, 2, 3, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 1, 2, 1]);
        exec(cpu, [14, 1, 3, 2]);
        exec(cpu, [2, 2, 2, 1]);
        exec(cpu, [2, 3, 2, 0]);
        exec(cpu, [15, 3, 0, 3]);
        exec(cpu, [10, 3, 3, 3]);
        exec(cpu, [6, 3, 1, 3]);
        exec(cpu, [15, 3, 3, 3]);
        exec(cpu, [9, 3, 2, 2]);
        exec(cpu, [14, 2, 1, 3]);
        exec(cpu, [2, 0, 3, 2]);
        exec(cpu, [7, 2, 0, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 2, 3, 3]);
        exec(cpu, [14, 3, 2, 0]);
        exec(cpu, [2, 0, 2, 3]);
        exec(cpu, [2, 1, 0, 1]);
        exec(cpu, [2, 2, 3, 2]);
        exec(cpu, [0, 3, 2, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 2, 0, 0]);
        exec(cpu, [14, 0, 3, 1]);
        exec(cpu, [15, 2, 0, 3]);
        exec(cpu, [10, 3, 1, 3]);
        exec(cpu, [2, 0, 2, 2]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [3, 0, 3, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 1, 2, 1]);
        exec(cpu, [14, 1, 1, 2]);
        exec(cpu, [2, 1, 0, 1]);
        exec(cpu, [2, 0, 2, 3]);
        exec(cpu, [1, 1, 0, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [9, 3, 2, 2]);
        exec(cpu, [14, 2, 0, 1]);
        exec(cpu, [2, 3, 0, 0]);
        exec(cpu, [15, 1, 0, 3]);
        exec(cpu, [10, 3, 0, 3]);
        exec(cpu, [2, 3, 3, 2]);
        exec(cpu, [13, 3, 2, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 0, 1, 1]);
        exec(cpu, [14, 1, 2, 2]);
        exec(cpu, [15, 3, 0, 0]);
        exec(cpu, [10, 0, 2, 0]);
        exec(cpu, [2, 3, 2, 1]);
        exec(cpu, [2, 2, 2, 3]);
        exec(cpu, [4, 0, 3, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 2, 1, 2]);
        exec(cpu, [14, 2, 0, 1]);
        exec(cpu, [2, 1, 2, 2]);
        exec(cpu, [15, 3, 0, 3]);
        exec(cpu, [10, 3, 1, 3]);
        exec(cpu, [3, 0, 3, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 1, 1]);
        exec(cpu, [2, 2, 3, 2]);
        exec(cpu, [15, 3, 0, 0]);
        exec(cpu, [10, 0, 2, 0]);
        exec(cpu, [2, 0, 0, 3]);
        exec(cpu, [0, 3, 2, 2]);
        exec(cpu, [15, 2, 1, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [2, 0, 0, 0]);
        exec(cpu, [2, 2, 3, 2]);
        exec(cpu, [0, 3, 2, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [2, 2, 1, 0]);
        exec(cpu, [2, 3, 2, 2]);
        exec(cpu, [2, 2, 3, 3]);
        exec(cpu, [7, 0, 2, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [14, 1, 1, 3]);
        exec(cpu, [2, 2, 1, 2]);
        exec(cpu, [2, 3, 2, 0]);
        exec(cpu, [2, 2, 2, 1]);
        exec(cpu, [5, 2, 0, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 3, 1, 3]);
        exec(cpu, [14, 3, 2, 1]);
        exec(cpu, [15, 0, 0, 2]);
        exec(cpu, [10, 2, 3, 2]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 0, 2, 3]);
        exec(cpu, [7, 0, 2, 3]);
        exec(cpu, [15, 3, 2, 3]);
        exec(cpu, [9, 3, 1, 1]);
        exec(cpu, [2, 0, 3, 2]);
        exec(cpu, [2, 2, 2, 3]);
        exec(cpu, [11, 0, 3, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 1, 1]);
        exec(cpu, [14, 1, 0, 2]);
        exec(cpu, [2, 1, 0, 3]);
        exec(cpu, [2, 2, 2, 0]);
        exec(cpu, [2, 1, 1, 1]);
        exec(cpu, [3, 0, 3, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [9, 2, 3, 2]);
        exec(cpu, [14, 2, 2, 3]);
        exec(cpu, [2, 1, 0, 0]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [15, 0, 0, 2]);
        exec(cpu, [10, 2, 2, 2]);
        exec(cpu, [14, 0, 2, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 2, 3, 3]);
        exec(cpu, [2, 3, 1, 1]);
        exec(cpu, [2, 0, 3, 2]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 3, 3]);
        exec(cpu, [2, 3, 2, 2]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [7, 0, 2, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 3, 3]);
        exec(cpu, [14, 3, 2, 1]);
        exec(cpu, [2, 0, 0, 3]);
        exec(cpu, [2, 2, 1, 2]);
        exec(cpu, [2, 1, 1, 0]);
        exec(cpu, [0, 3, 2, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [2, 2, 1, 3]);
        exec(cpu, [2, 0, 3, 2]);
        exec(cpu, [2, 3, 0, 0]);
        exec(cpu, [13, 2, 3, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [2, 0, 1, 3]);
        exec(cpu, [2, 3, 1, 2]);
        exec(cpu, [2, 2, 1, 0]);
        exec(cpu, [7, 0, 2, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 1, 1]);
        exec(cpu, [2, 1, 3, 0]);
        exec(cpu, [2, 2, 3, 2]);
        exec(cpu, [14, 0, 2, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 1, 2, 1]);
        exec(cpu, [14, 1, 3, 3]);
        exec(cpu, [2, 1, 3, 1]);
        exec(cpu, [2, 0, 0, 2]);
        exec(cpu, [15, 0, 2, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 2, 3, 3]);
        exec(cpu, [15, 1, 0, 0]);
        exec(cpu, [10, 0, 2, 0]);
        exec(cpu, [15, 0, 0, 2]);
        exec(cpu, [10, 2, 3, 2]);
        exec(cpu, [5, 0, 2, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 0, 3, 3]);
        exec(cpu, [14, 3, 1, 1]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [15, 1, 0, 2]);
        exec(cpu, [10, 2, 0, 2]);
        exec(cpu, [3, 0, 3, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [9, 3, 1, 1]);
        exec(cpu, [2, 3, 3, 2]);
        exec(cpu, [2, 3, 2, 0]);
        exec(cpu, [15, 3, 0, 3]);
        exec(cpu, [10, 3, 0, 3]);
        exec(cpu, [13, 3, 2, 3]);
        exec(cpu, [15, 3, 2, 3]);
        exec(cpu, [9, 1, 3, 1]);
        exec(cpu, [14, 1, 1, 2]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [2, 1, 1, 1]);
        exec(cpu, [3, 0, 3, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 2, 1, 2]);
        exec(cpu, [14, 2, 1, 3]);
        exec(cpu, [2, 2, 0, 1]);
        exec(cpu, [2, 1, 0, 0]);
        exec(cpu, [2, 1, 2, 2]);
        exec(cpu, [9, 0, 0, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 3, 3]);
        exec(cpu, [14, 3, 3, 2]);
        exec(cpu, [2, 1, 1, 0]);
        exec(cpu, [15, 0, 0, 1]);
        exec(cpu, [10, 1, 0, 1]);
        exec(cpu, [2, 3, 1, 3]);
        exec(cpu, [10, 0, 1, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [14, 2, 1, 0]);
        exec(cpu, [2, 3, 2, 1]);
        exec(cpu, [2, 2, 1, 3]);
        exec(cpu, [2, 1, 1, 2]);
        exec(cpu, [6, 1, 3, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [9, 0, 3, 0]);
        exec(cpu, [14, 0, 0, 1]);
        exec(cpu, [2, 0, 1, 2]);
        exec(cpu, [15, 3, 0, 3]);
        exec(cpu, [10, 3, 2, 3]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [4, 0, 3, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [15, 3, 2, 3]);
        exec(cpu, [9, 3, 1, 1]);
        exec(cpu, [14, 1, 2, 0]);
        exec(cpu, [2, 3, 3, 1]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [2, 3, 3, 2]);
        exec(cpu, [15, 3, 2, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [9, 0, 2, 0]);
        exec(cpu, [14, 0, 2, 2]);
        exec(cpu, [2, 2, 2, 0]);
        exec(cpu, [2, 1, 3, 1]);
        exec(cpu, [2, 2, 0, 3]);
        exec(cpu, [1, 1, 3, 0]);
        exec(cpu, [15, 0, 3, 0]);
        exec(cpu, [9, 0, 2, 2]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [4, 0, 3, 3]);
        exec(cpu, [15, 3, 2, 3]);
        exec(cpu, [9, 3, 2, 2]);
        exec(cpu, [15, 1, 0, 1]);
        exec(cpu, [10, 1, 3, 1]);
        exec(cpu, [2, 1, 3, 3]);
        exec(cpu, [3, 0, 3, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 1, 2, 2]);
        exec(cpu, [14, 2, 2, 3]);
        exec(cpu, [2, 1, 0, 0]);
        exec(cpu, [2, 3, 3, 1]);
        exec(cpu, [2, 2, 1, 2]);
        exec(cpu, [14, 0, 2, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 1, 3, 3]);
        exec(cpu, [2, 1, 3, 2]);
        exec(cpu, [2, 2, 3, 1]);
        exec(cpu, [9, 0, 0, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 1, 3, 3]);
        exec(cpu, [2, 2, 1, 0]);
        exec(cpu, [2, 3, 1, 1]);
        exec(cpu, [15, 3, 0, 2]);
        exec(cpu, [10, 2, 2, 2]);
        exec(cpu, [12, 0, 1, 2]);
        exec(cpu, [15, 2, 2, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 2, 3, 3]);
        exec(cpu, [14, 3, 3, 0]);
        exec(cpu, [2, 1, 1, 3]);
        exec(cpu, [2, 0, 0, 2]);
        exec(cpu, [2, 1, 3, 1]);
        exec(cpu, [2, 3, 1, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 1, 0, 0]);
        exec(cpu, [14, 0, 3, 2]);
        exec(cpu, [2, 0, 2, 3]);
        exec(cpu, [15, 2, 0, 0]);
        exec(cpu, [10, 0, 1, 0]);
        exec(cpu, [2, 0, 0, 1]);
        exec(cpu, [10, 0, 1, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 2, 0, 2]);
        exec(cpu, [2, 1, 2, 1]);
        exec(cpu, [2, 1, 0, 0]);
        exec(cpu, [2, 1, 1, 3]);
        exec(cpu, [9, 0, 0, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [15, 3, 1, 3]);
        exec(cpu, [9, 2, 3, 2]);
        exec(cpu, [14, 2, 0, 1]);
        exec(cpu, [2, 2, 3, 3]);
        exec(cpu, [2, 2, 1, 0]);
        exec(cpu, [2, 1, 3, 2]);
        exec(cpu, [11, 0, 3, 2]);
        exec(cpu, [15, 2, 3, 2]);
        exec(cpu, [9, 2, 1, 1]);
        exec(cpu, [14, 1, 2, 2]);
        exec(cpu, [2, 1, 2, 3]);
        exec(cpu, [2, 1, 3, 0]);
        exec(cpu, [15, 3, 0, 1]);
        exec(cpu, [10, 1, 1, 1]);
        exec(cpu, [9, 1, 0, 1]);
        exec(cpu, [15, 1, 1, 1]);
        exec(cpu, [9, 1, 2, 2]);
        exec(cpu, [2, 2, 3, 3]);
        exec(cpu, [2, 0, 2, 1]);
        exec(cpu, [2, 2, 0, 0]);
        exec(cpu, [11, 0, 3, 3]);
        exec(cpu, [15, 3, 2, 3]);
        exec(cpu, [9, 3, 2, 2]);
        exec(cpu, [2, 1, 1, 0]);
        exec(cpu, [15, 1, 0, 3]);
        exec(cpu, [10, 3, 2, 3]);
        exec(cpu, [15, 1, 0, 1]);
        exec(cpu, [10, 1, 2, 1]);
        exec(cpu, [1, 0, 3, 1]);
        exec(cpu, [15, 1, 2, 1]);
        exec(cpu, [9, 2, 1, 2]);
        exec(cpu, [14, 2, 3, 3]);
        exec(cpu, [15, 1, 0, 2]);
        exec(cpu, [10, 2, 2, 2]);
        exec(cpu, [2, 2, 1, 1]);
        exec(cpu, [14, 0, 2, 1]);
        exec(cpu, [15, 1, 3, 1]);
        exec(cpu, [9, 1, 3, 3]);
        exec(cpu, [14, 3, 1, 2]);
        exec(cpu, [15, 2, 0, 1]);
        exec(cpu, [10, 1, 3, 1]);
        exec(cpu, [2, 2, 3, 0]);
        exec(cpu, [2, 0, 1, 3]);
        exec(cpu, [12, 0, 1, 3]);
        exec(cpu, [15, 3, 3, 3]);
        exec(cpu, [9, 2, 3, 2]);
        exec(cpu, [14, 2, 1, 1]);
        exec(cpu, [2, 3, 3, 3]);
        exec(cpu, [2, 2, 2, 2]);
        exec(cpu, [2, 1, 3, 0]);
        exec(cpu, [14, 0, 2, 0]);
        exec(cpu, [15, 0, 1, 0]);
        exec(cpu, [15, 0, 2, 0]);
        exec(cpu, [9, 0, 1, 1]);
        exec(cpu, [14, 1, 0, 0]);

        let dummy = CPU::new([0,0,0,0]);
        assert_eq!(dummy, *cpu);
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
*/
}
