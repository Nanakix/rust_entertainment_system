/* 
    cpu.rs

    This will define every little piece of work of the 6502 CPU:
    Instructions, addressing modes, registers
*/
pub mod cpu;

let CLOCK_SPEED: u16 = 1790000 // 1.79MHz

enum Instructions {
    // source : https://www.nesdev.org/obelisk-6502-guide/reference.html
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
};


enum Registries {
    A, // general purpose
    X, // general purpose
    Y, // general purpose
    P, // status
    SP, // Stack Pointer
    PC, // Program Counter
};