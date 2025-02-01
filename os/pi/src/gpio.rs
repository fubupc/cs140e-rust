use core::marker::PhantomData;

use crate::common::{states, IO_BASE};
use volatile::prelude::*;
use volatile::{ReadVolatile, Reserved, Volatile, WriteVolatile};

/// An alternative GPIO function.
#[derive(Debug)]
#[repr(u8)]
pub enum Function {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}
impl TryFrom<u8> for Function {
    type Error = u8;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0b000 => Ok(Self::Input),
            0b001 => Ok(Self::Output),
            0b100 => Ok(Self::Alt0),
            0b101 => Ok(Self::Alt1),
            0b110 => Ok(Self::Alt2),
            0b111 => Ok(Self::Alt3),
            0b011 => Ok(Self::Alt4),
            0b010 => Ok(Self::Alt5),
            _ => Err(v),
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    FSEL: [Volatile<u32>; 6],
    __r0: Reserved<u32>,
    SET: [WriteVolatile<u32>; 2],
    __r1: Reserved<u32>,
    CLR: [WriteVolatile<u32>; 2],
    __r2: Reserved<u32>,
    LEV: [ReadVolatile<u32>; 2],
    __r3: Reserved<u32>,
    EDS: [Volatile<u32>; 2],
    __r4: Reserved<u32>,
    REN: [Volatile<u32>; 2],
    __r5: Reserved<u32>,
    FEN: [Volatile<u32>; 2],
    __r6: Reserved<u32>,
    HEN: [Volatile<u32>; 2],
    __r7: Reserved<u32>,
    LEN: [Volatile<u32>; 2],
    __r8: Reserved<u32>,
    AREN: [Volatile<u32>; 2],
    __r9: Reserved<u32>,
    AFEN: [Volatile<u32>; 2],
    __r10: Reserved<u32>,
    PUD: Volatile<u32>,
    PUDCLK: [Volatile<u32>; 2],
}

/// Possible states for a GPIO pin.
states! {
    Uninitialized, Input, Output, Alt
}

/// A GPIO pin in state `State`.
///
/// The `State` generic always corresponds to an uninstantiatable type that is
/// use solely to mark and track the state of a given GPIO pin. A `Gpio`
/// structure starts in the `Uninitialized` state and must be transitions into
/// one of `Input`, `Output`, or `Alt` via the `into_input`, `into_output`, and
/// `into_alt` methods before it can be used.
pub struct Gpio<State> {
    pin: u8,
    registers: &'static mut Registers,
    _state: PhantomData<State>,
}

/// The base address of the `GPIO` registers.
const GPIO_BASE: usize = IO_BASE + 0x200000;

impl<T> Gpio<T> {
    /// Transitions `self` to state `S`, consuming `self` and returning a new
    /// `Gpio` instance in state `S`. This method should _never_ be exposed to
    /// the public!
    #[inline(always)]
    fn transition<S>(self) -> Gpio<S> {
        Gpio {
            pin: self.pin,
            registers: self.registers,
            _state: PhantomData,
        }
    }

    pub fn function(&self) -> Function {
        let reg_idx = self.pin as usize / 10;
        let shift_bit_num = (self.pin as usize % 10) * 3;

        let reg = &self.registers.FSEL[reg_idx];
        let f = (reg.read() >> shift_bit_num & 0b111) as u8;
        Function::try_from(f).unwrap()
    }
}

impl Gpio<Uninitialized> {
    /// Returns a new `GPIO` structure for pin number `pin`.
    ///
    /// # Panics
    ///
    /// Panics if `pin` > `53`.
    pub fn new(pin: u8) -> Gpio<Uninitialized> {
        if pin > 53 {
            panic!("Gpio::new(): pin {} exceeds maximum of 53", pin);
        }

        Gpio {
            registers: unsafe { &mut *(GPIO_BASE as *mut Registers) },
            pin: pin,
            _state: PhantomData,
        }
    }

    /// Enables the alternative function `function` for `self`. Consumes self
    /// and returns a `Gpio` structure in the `Alt` state.
    pub fn into_alt(self, function: Function) -> Gpio<Alt> {
        let reg_idx = self.pin as usize / 10;
        let shift_bit_num = (self.pin as usize % 10) * 3;
        let mask = !(0b111 << shift_bit_num);
        let pattern = (function as u32) << shift_bit_num;

        let reg = &mut self.registers.FSEL[reg_idx];
        reg.write(reg.read() & mask | pattern);

        Gpio {
            pin: self.pin,
            registers: self.registers,
            _state: PhantomData,
        }
    }

    /// Sets this pin to be an _output_ pin. Consumes self and returns a `Gpio`
    /// structure in the `Output` state.
    pub fn into_output(self) -> Gpio<Output> {
        self.into_alt(Function::Output).transition()
    }

    /// Sets this pin to be an _input_ pin. Consumes self and returns a `Gpio`
    /// structure in the `Input` state.
    pub fn into_input(self) -> Gpio<Input> {
        self.into_alt(Function::Input).transition()
    }
}

impl Gpio<Output> {
    /// Sets (turns on) the pin.
    pub fn set(&mut self) {
        let reg_idx = self.pin as usize / 32;
        let shift_bit_num = self.pin as usize % 32;
        let mask = 1 << shift_bit_num;
        self.registers.SET[reg_idx].write(mask);
    }

    /// Clears (turns off) the pin.
    pub fn clear(&mut self) {
        let reg_idx = self.pin as usize / 32;
        let shift_bit_num = self.pin as usize % 32;
        let mask = 1 << shift_bit_num;
        self.registers.CLR[reg_idx].write(mask);
    }
}

impl Gpio<Input> {
    /// Reads the pin's value. Returns `true` if the level is high and `false`
    /// if the level is low.
    pub fn level(&mut self) -> bool {
        let reg_idx = self.pin as usize / 32;
        let level = self.registers.LEV[reg_idx].read();

        let shift_bit_num = self.pin as usize % 32;
        level << (31 - shift_bit_num) >> 31 == 1
    }
}
