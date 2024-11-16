use microcode::{tinycpu_microcode, Signal::*};

fn main() {
    let microcode = vec![
        // 000 add
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE, MarC],
            vec![MemE, MbrC],
            vec![AluC, AluE],
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
            vec![D0E, MemC],
            vec![Reset],
        ],
        // 110 jmp
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE, PcC],
            vec![Reset],
        ],
        // 111 jz
        vec![
            vec![PcE, MarC],
            vec![MemE, IrC, PcInc],
            vec![PcInc, PcC],
            vec![IrE, PcC, Z],
            vec![Reset],
        ],
    ];

    println!("{:?}", tinycpu_microcode(microcode));
}
