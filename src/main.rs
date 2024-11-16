use std::fmt::Write;

use microcode::{tinycpu_microcode, Signal::*};

fn main() {
    let steps = vec![
        // 000 add
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE, MarC],
            vec![MemE, MbrC],
            vec![AluC, AluE],
            vec![AluE, D0C],
            vec![Reset],
        ],
        // 001 and
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE, MarC],
            vec![MemE, MbrC],
            vec![AluC, AluE],
            vec![AluE, D0C],
            vec![Reset],
        ],
        // 010 shr
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE, MarC],
            vec![MemE, MbrC],
            vec![AluC, AluE],
            vec![AluE, MemC],
            vec![Reset],
        ],
        // 011 disp
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE, MarC],
            vec![MemE, MbrC, BcdC],
            vec![Reset],
        ],
        // 100 load
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE, MarC],
            vec![MemE, D0C],
            vec![Reset],
        ],
        // 101 str
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE, MarC],
            vec![MemC, D0E],
            vec![Reset],
        ],
        // 110 jmp
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE],
            vec![IrE, PcC],
            vec![Reset],
        ],
        // 111 jz
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE],
            vec![IrE, Z],
            vec![Reset],
        ],
    ];

    let mut s = String::new();

    writeln!(&mut s, "v2.0 raw").unwrap();

    for code in tinycpu_microcode(steps) {
        write!(&mut s, "{code:x} ").unwrap()
    }

    print!("{s}");
}
