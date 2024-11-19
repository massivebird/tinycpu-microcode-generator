#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
pub enum Signal {
    Reset,
    AluE,
    AluC,
    MemC,
    D0E,
    D0C,
    MbrC,
    BcdC,
    IrE,
    Z,
    PcC,
    PcInc,
    MemE,
    IrC,
    MarC,
    PcE,
}

impl From<Signal> for u16 {
    fn from(val: Signal) -> Self {
        match val {
            Signal::PcE => 1,
            Signal::MarC => 0x2,
            Signal::IrC => 0x4,
            Signal::MemE => 0x8,
            Signal::PcInc => 0x10,
            Signal::PcC => 0x20,
            Signal::Z => 0x40,
            Signal::IrE => 0x80,
            Signal::MbrC => 0x100,
            Signal::D0C => 0x200,
            Signal::D0E => 0x400,
            Signal::MemC => 0x800,
            Signal::AluC => 0x1000,
            Signal::AluE => 0x2000,
            Signal::Reset => 0x4000,
            Signal::BcdC => 0x8000,
        }
    }
}

/// Generates the code for a microstep triggering one or more signals.
fn generate_microstep_code(signals: &[Signal]) -> u16 {
    let mut code = 0;

    for signal in signals {
        code |= u16::from(*signal);
    }

    code
}

pub fn tinycpu_microcode(insts: &[Vec<Vec<Signal>>]) -> [u16; 64] {
    assert_eq!(insts.len(), 8, "Expected seven (7) instructions.");

    let mut microcode = [0u16; 64];

    for (opcode, microsteps) in insts.iter().enumerate() {
        for (step_num, signals) in microsteps.iter().enumerate() {
            microcode[opcode + (8 * step_num)] = generate_microstep_code(signals);
        }
    }

    microcode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_signal_test() {
        assert_eq!(u16::from(Signal::PcE), 0x0001);
        assert_eq!(u16::from(Signal::D0C), 0x0200);
        assert_eq!(u16::from(Signal::Reset), 0x4000);
    }

    #[test]
    fn microstep_test() {
        assert_eq!(
            generate_microstep_code(&[Signal::Reset, Signal::D0C]),
            0x4200
        );
        assert_eq!(
            generate_microstep_code(&[Signal::AluC, Signal::AluE]),
            0x3000
        );
        assert_eq!(
            generate_microstep_code(&[Signal::IrE, Signal::MarC]),
            0x0082
        );
    }
}
