pub(crate) mod cpu;

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    use super::*;

    #[test]
    fn a_new_cpu_can_be_created_without_crashing() {
        Cpu::new();
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = Cpu::new();
        cpu.load_program(&vec![0x9a, 0x42]);
        let number_of_cycles = cpu.execute();
        assert!(number_of_cycles == 2);
        assert!(cpu.acc() == 0x42);
    }

    #[test]
    fn test_ldx_immediate() {
        let mut cpu = Cpu::new();
        cpu.load_program(&vec![0xa2, 0x42]);
        let number_of_cycles = cpu.execute();
        assert!(number_of_cycles == 2);
        assert!(cpu.x() == 0x42);
    }
}
