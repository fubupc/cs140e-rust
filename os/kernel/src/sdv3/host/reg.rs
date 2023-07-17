//! SD Host Standard Register

#![allow(non_snake_case)]

use bitfield::bitfield;

use super::super::command::Argument;
use super::super::common::BusWidth;

/// SD Host Control Register Map
#[repr(C, packed)]
pub struct RegMap {
    pub Argument2: Argument2,                                     // 0x000
    pub BlockSize: BlockSize,                                     // 0x004
    pub BlockCount: BlockCount,                                   // 0x006
    pub Argument1: Argument1,                                     // 0x008
    pub TransferMode: TransferMode,                               // 0x00C
    pub Command: Command,                                         // 0x00E
    pub Response: Response,                                       // 0x010
    pub BufferDataPort: BufferDataPort,                           // 0x020
    pub PresentState: PresentState,                               // 0x024
    pub HostControl1: HostControl1,                               // 0x028
    pub PowerControl: PowerControl,                               // 0x029
    pub BlockGapControl: BlockGapControl,                         // 0x02A
    pub WakeupControl: WakeupControl,                             // 0x02B
    pub ClockControl: ClockControl,                               // 0x02C
    pub TimeoutControl: TimeoutControl,                           // 0x02E
    pub SoftwareReset: SoftwareReset,                             // 0x02F
    pub NormalInterruptStatus: NormalInterruptStatus,             // 0x030
    pub ErrorInterruptStatus: ErrorInterruptStatus,               // 0x032
    pub NormalInterruptStatusEnable: NormalInterruptStatusEnable, // 0x034
    pub ErrorInterruptStatusEnable: ErrorInterruptStatusEnable,   // 0x036
    pub NormalInterruptSignalEnable: NormalInterruptSignalEnable, // 0x038
    pub ErrorInterruptSignalEnable: ErrorInterruptSignalEnable,   // 0x03A
    pub AutoCMDErrorStatus: AutoCMDErrorStatus,                   // 0x03C
    pub HostControl2: HostControl2,                               // 0x03E
    pub Capabilities: Capabilities,                               // 0x040
    pub MaxCurrentCapabilities: MaxCurrentCapabilities,           // 0x048
    pub ForceEventForAutoCMDError: ForceEventForAutoCMDError,     // 0x050
    pub ForceEventForErrorInterrupt: ForceEventForErrorInterrupt, // 0x052
    pub AMDAErrorStatus: AMDAErrorStatus,                         // 0x054
    pub _RESERVED1: u8,                                           // 0x055
    pub _RESERVED2: [u8; 2],                                      // 0x056
    pub AMDASystemAddress: AMDASystemAddress,                     // 0x058
    pub PresetValueInit: PresetValue,                             // 0x060
    pub PresetValueDefaultSpeed: PresetValue,                     // 0x062
    pub PresetValueHighSpeed: PresetValue,                        // 0x064
    pub PresetValueSDR12: PresetValue,                            // 0x066
    pub PresetValueSDR25: PresetValue,                            // 0x068
    pub PresetValueSDR50: PresetValue,                            // 0x06A
    pub PresetValueSDR104: PresetValue,                           // 0x06C
    pub PresetValueDDR50: PresetValue,                            // 0x06E
    pub _RESERVED3: [u8; 7 * 16],                                 // 0x070
    pub SharedBusControl: SharedBusControl,                       // 0x0E0
    pub _RESERVED4: [u8; 24],                                     // 0x0E4
    pub SlotInterruptStatus: SlotInterruptStatus,                 // 0x0FC
    pub HostControllerVersion: HostControllerVersion,             // 0x0FE
}

/// SDMA System Address / Argument 2 Register
///
/// This register is used with the Auto CMD23 to set a 32-bit block count value to the argument of the CMD23 while
/// executing Auto CMD23. If Auto CMD23 is used with ADMA, the full 32-bit block count value can be used. If Auto
/// CMD23 is used without AMDA, the available block count value is limited by the Block Count register. 65535 blocks
/// is the maximum value in this case.
#[derive(Debug, Copy, Clone)]
pub struct Argument2(u32);

bitfield! {
    /// Block Size Register
    #[derive(Copy, Clone)]
    pub struct BlockSize(u16);

    impl Debug;

    // TODO: [14:12] SDMA Buffer Boundary

    /// Transfer Block Size
    ///
    /// This register specifies the block size of data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. Values
    /// anging from 1 up to the maximum buffer size can be set. In case of memory, it shall be set up to 512 bytes.
    pub u16, block_size, set_block_size: 11, 0;
}

bitfield! {
    /// Block Count Register
    #[derive(Copy, Clone)]
    pub struct BlockCount(u16);

    impl Debug;

    /// Blocks Count For Current Transfer
    ///
    /// This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only
    /// for multiple block transfers.
    pub u16, block_count, set_block_count: 15, 0;
}

/// Argument 1 Register
#[derive(Debug, Copy, Clone)]
pub struct Argument1(u32);
impl From<Argument> for Argument1 {
    fn from(v: Argument) -> Self {
        Argument1(v.into())
    }
}
impl From<Argument1> for Argument {
    fn from(v: Argument1) -> Self {
        Argument::from(v.0)
    }
}

bitfield! {
    /// Transfer Mode Register
    ///
    /// This register is used to control the operation of data transfers. The Host Driver shall set this register
    /// before issuing a command which transfers data (Refer to Data Present Select in the Command register), or
    /// before issuing a Resume command. The Host Driver shall save the value of this register when the data
    /// transfer is suspended (as a result of a Suspend command) and restore it before issuing a Resume
    /// command. To prevent data loss, the Host Controller shall implement write protection for this register
    /// during data transactions. Writes to this register shall be ignored when the Command Inhibit (DAT) in the
    /// Present State register is 1.
    #[derive(Copy, Clone)]
    pub struct TransferMode(u16);

    impl Debug;

    // [15:6] Reserved

    /// Multi / Single Block Select
    ///
    /// This bit is set when issuing multiple-block transfer commands using DAT line. For any other commands, this bit
    /// shall be set to 0. If this bit is 0, it is not necessary to set the Block Count register.
    pub multi_block, set_multi_block: 5;

    /// Data Transfer Direction Select
    ///
    /// - 1: Read (Card to Host)
    /// - 0: Write (Host to Card)
    pub u8, from into TransferDirection, transfer_direction, set_transfer_direction: 4, 4;

    /// Auto CMD Enable
    ///
    /// - 00b: Auto Command Disabled
    /// - 01b: Auto CMD12 Enable
    /// - 10b: Auto CMD23 Enable
    /// - 11b: Reserved
    pub u8, auto_cmd, set_auto_cmd: 3, 2;

    /// Block Count Enable
    ///
    /// Only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is
    /// useful in executing an infinite transfer.
    pub block_count_enable, set_block_count_enable: 1;

    /// DMA Enable
    pub dma_enable, set_dma_enable: 0;
}

/// Direction of DAT line data transfers
#[derive(Debug, Clone, Copy)]
pub enum TransferDirection {
    Read = 1,  // Card to Host
    Write = 0, // Host to Card
}
impl From<u8> for TransferDirection {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Read,
            0 => Self::Write,
            _ => unreachable!(),
        }
    }
}
impl From<TransferDirection> for u8 {
    fn from(v: TransferDirection) -> Self {
        v as u8
    }
}

bitfield! {
    /// Command Register
    ///
    /// The Host Driver shall check the Command Inhibit (DAT) bit and Command Inhibit (CMD) bit in the Present State
    /// register before writing to this register.
    #[derive(Copy, Clone)]
    pub struct Command(u16);

    impl Debug;

    /// Command Index
    pub u8, command_index, set_command_index: 13, 8;

    /// Command Type
    ///
    /// - 11b: Abort    CMD12, CMD52 for writing "I/O Abort" in CCCR
    /// - 10b: Resume   CMD52 for writing "Function Select" in CCCR
    /// - 01b: Suspend  CMD52 for writing "Bus Suspend" in CCCR
    /// - 00b: Normal   Other commands
    pub u8, from into CommandType, command_type, set_command_type: 7, 6;

    /// Data Present Select
    ///
    /// This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. It is set to
    /// 0 for the following:
    /// 1. Commands using only CMD line (ex. CMD52).
    /// 2. Commands with no data transfer but using busy signal on DAT[0] line (R1b or R5b ex. CMD38)
    /// 3. Resume command
    pub data_present, set_data_present: 5;

    /// Command Index Check Enable
    pub command_index_check, set_command_index_check: 4;

    /// Command CRC Check Enable
    pub command_crc_check, set_command_crc_check: 3;

    /// Response Type Select
    ///
    /// - 00: No Response
    /// - 01: Response Length 136
    /// - 10: Response Length 48
    /// - 11: Response Length 48 check Busy after response
    pub u8, from into ResponseType, response_type, set_response_type: 1, 0;
}

/// Represents *Command Type* field of [`Command Register`](`Command`) .
///
/// NOTE: Don't confuse it with command type like [`BC`](super::super::command::CommandType::BC), etc.
#[derive(Debug, Clone, Copy)]
pub enum CommandType {
    Normal = 0b00,
    Suspend = 0b01,
    Resume = 0b10,
    Abort = 0b11,
}
impl From<u8> for CommandType {
    fn from(v: u8) -> Self {
        match v {
            0b00 => Self::Normal,
            0b01 => Self::Suspend,
            0b10 => Self::Resume,
            0b11 => Self::Abort,
            _ => unreachable!(),
        }
    }
}
impl From<CommandType> for u8 {
    fn from(v: CommandType) -> Self {
        v as u8
    }
}

/// Represents *Response Type* field of [Command Register](`Command`).
///
/// NOTE: Don't confuse it with response type like [`R1`](super::super::response::R1), etc.
#[derive(Debug, Clone, Copy)]
pub enum ResponseType {
    NoResponse = 0b00,
    _136Bits = 0b01,
    _48Bits = 0b10,
    _48BitsBusy = 0b11,
}
impl From<u8> for ResponseType {
    fn from(v: u8) -> Self {
        match v {
            0b00 => Self::NoResponse,
            0b01 => Self::_136Bits,
            0b10 => Self::_48Bits,
            0b11 => Self::_48BitsBusy,
            _ => unreachable!(),
        }
    }
}
impl From<ResponseType> for u8 {
    fn from(v: ResponseType) -> Self {
        v as u8
    }
}

/// Response Register
///
/// This register is used to store responses from SD cards.
///
/// NOTE: Little-Endian
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Response([u32; 4]);
impl Response {
    /// Bit range [31:0]
    pub fn bit_31_0(self) -> u32 {
        self.0[0]
    }

    /// Bit range [127:96]
    pub fn bit_127_96(self) -> u32 {
        self.0[3]
    }

    /// Bit range [119:0]
    pub fn bit_119_0(self) -> u128 {
        let mut u = self.0[0] as u128;
        u |= (self.0[1] as u128) << 32;
        u |= (self.0[2] as u128) << 64;
        u |= (self.0[3] as u128) << 96;
        u
    }
}

/// Buffer Data Port Register
///
/// 32-bit data port register to access internal buffer.
#[derive(Debug, Copy, Clone)]
pub struct BufferDataPort(u32);
impl From<u32> for BufferDataPort {
    fn from(v: u32) -> Self {
        BufferDataPort(v)
    }
}
impl From<BufferDataPort> for u32 {
    fn from(v: BufferDataPort) -> Self {
        v.0
    }
}

bitfield! {
    /// Present State Register
    #[derive(Copy, Clone)]
    pub struct PresentState(u32);

    impl Debug;

    // [31:25] Reserved

    /// CMD Line Signal Level
    ///
    /// This status is used to check the CMD line level to recover from errors, and for debugging.
    pub cmd_line_level, _: 24;

    /// DAT[3:0] Line Signal Level
    ///
    /// This status is used to check the DAT line level to recover from errors, and for debugging. This is especially
    /// useful in detecting the busy signal level from DAT[0].
    pub dat3_line_level, _: 23;
    pub dat2_line_level, _: 22;
    pub dat1_line_level, _: 21;
    pub dat0_line_level, _: 20;

    /// Write Protect Switch Pin Level
    pub write_protect, _: 19;

    /// Card Detect Pin Level
    pub card_present, _: 18;

    /// Card State Stable
    pub card_state_stable, _: 17;

    /// Card Inserted
    pub card_inserted, _: 16;

    // [15:12] Reserved

    /// Buffer Read Enable
    pub buffer_read, _: 11;

    /// Buffer Write Enable
    pub buffer_write, _: 10;

    /// Read Transfer Active
    pub read_active, _: 9;

    /// Write Transfer Active
    pub write_active, _: 8;

    // [7:4] Reserved

    /// Re-Tuning Request
    pub re_tuning_request, _: 3;

    /// DAT Line Active
    pub dat_line_active, _: 2;

    /// Command Inhibit (DAT)
    ///
    /// This status bit is generated if either the DAT Line Active or the Read Transfer Active is set to 1. If this bit is 0,
    /// it indicates the Host Controller can issue the next SD Command. Commands with busy signal belong to Command Inhibit
    /// (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal Interrupt Status
    /// register.
    /// - 1: Cannot issue command which uses the DAT line
    /// - 0: Can issue command which uses the DAT line
    pub command_inhibit_dat, _: 1;

    /// Command Inhibit (CMD)
    ///
    /// - 1: Cannot issue command
    /// - 0: Can issue command using only CMD line
    pub command_inhibit_cmd, _: 0;
}

bitfield! {
    /// Host Control 1 Register
    #[derive(Copy, Clone)]
    pub struct HostControl1(u8);

    impl Debug;

    /// Card Detect Signal Selection
    ///
    /// - 1: The Card Detect Test Level is selected (for test purpose)
    /// - 0: SDCD# is selected (for normal use)
    pub card_detect_test, set_card_detect_test: 7;

    /// Card Detect Test Level
    ///
    /// This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not.
    /// - 1: Card Inserted
    /// - 0: No Card
    pub card_inserted, set_card_inserted: 6;

    /// Extended Data Transfer Width
    ///
    /// - 1: 8-bit Bus Width
    /// - 0: Bus Width is Selected by Data Transfer Width
    _8bit_bus, _set_8bit_bus: 5;

    /// DMA Select
    ///
    /// - 00: SDMA is selected
    /// - 01: Reserved (New assignment is not allowed)
    /// - 10: 32-bit Address ADMA2 is selected
    /// - 11: Reserved (will be modified by Version 4.00)
    pub u8, dma_mode, set_dma_mode: 4, 3;

    /// High Speed Enable
    pub high_speed_mode, set_high_speed_mode: 2;

    /// Data Transfer Width
    ///
    /// This bit selects the data width of the Host Controller. The Host Driver shall set it to match the data width
    /// of the SD card.
    ///
    /// - 1: 4-bit mode
    /// - 0: 1-bit mode
    _4bit_bus, _set_4bit_bus: 1;

    /// LED Control
    pub led_on, set_led_on: 0;
}
impl HostControl1 {
    pub fn bus_width(&self) -> BusWidth {
        match self._8bit_bus() {
            true => BusWidth::_8Bit,
            false => match self._4bit_bus() {
                true => BusWidth::_4Bit,
                false => BusWidth::_1Bit,
            },
        }
    }

    pub fn set_bus_width(&mut self, w: BusWidth) {
        match w {
            BusWidth::_1Bit => {
                self._set_8bit_bus(false);
                self._set_4bit_bus(false);
            }
            BusWidth::_4Bit => {
                self._set_8bit_bus(false);
                self._set_4bit_bus(true);
            }
            BusWidth::_8Bit => self._set_8bit_bus(true),
        }
    }
}

bitfield! {
    /// Power Control Register
    #[derive(Copy, Clone)]
    pub struct PowerControl(u8);

    impl Debug;

    /// SD Bus Voltage Select
    ///
    /// By setting these bits, the Host Driver selects the voltage level for the SD card.
    /// Before setting this register, the Host Driver shall check the Voltage Support bits in
    /// the Capabilities register. If an unsupported voltage is selected, the Host System
    /// shall not supply SD Bus voltage.
    ///
    /// - 111b: 3.3V (Typ.)
    /// - 110b: 3.0V (Typ.)
    /// - 101b: 1.8V (Typ.)
    /// - 100b – 000b Reserved
    pub u8, into BusVoltage, bus_voltage, set_bus_voltage: 3, 1;

    /// SD Bus Power
    ///
    /// - 1: power on
    /// - 0: power off
    pub bus_power_on, set_bus_power_on: 0;
}

/// SD Bus Voltage
#[derive(Debug, Copy, Clone)]
pub enum BusVoltage {
    _3_3V = 0b111,
    _3_0V = 0b110,
    _1_8V = 0b101,

    /// All 100b – 000b are reserved
    Reserved = 0b000,
}
impl From<u8> for BusVoltage {
    fn from(v: u8) -> Self {
        match v {
            0b111 => BusVoltage::_3_3V,
            0b110 => BusVoltage::_3_0V,
            0b101 => BusVoltage::_1_8V,
            _ => BusVoltage::Reserved,
        }
    }
}

bitfield! {
    /// Block Gap Control Register
    ///
    // TODO: only for SDIO?
    #[derive(Copy, Clone)]
    pub struct BlockGapControl(u8);

    impl Debug;

}

bitfield! {
    /// Wakeup Control Register
    #[derive(Copy, Clone)]
    pub struct WakeupControl(u8);

    impl Debug;

    /// Wakeup Event Enable On SD Card Removal
    pub wakeup_on_removal, set_wakeup_on_removal: 2;

    /// Wakeup Event Enable On SD Card Insertion
    pub wakeup_on_insertion, set_wakeup_on_insertion: 1;

    /// Wakeup Event Enable On Card Interrupt
    pub wakeup_on_interrupt, set_wakeup_on_interrupt: 0;
}

bitfield! {
    /// Clock Control Register
    #[derive(Copy, Clone)]
    pub struct ClockControl(u16);

    impl Debug;

    /// SDCLK Frequency Select
    ///
    /// This field depends on setting of Preset Value Enable in the Host Control 2 register:
    /// - Preset Value Enable = 0, this field is set by Host Driver
    /// - Preset Value Enable = 1, this field is automatically set to a value specified in one of
    ///   Preset Value registers.
    ///
    /// The definition of this field is dependent on the Host Controller Version:
    /// 1. For v1.00 and v2.00: 8-bit Divided Clock Mode
    /// 2. For v3.00: 10-bit Divided Clock Mode
    /// 3. For v3.00 (optional): Programmable Clock Mode
    _sdclk_freq, _set_sdclk_freq: 15, 8;

    /// Upper Bits of SDCLK Frequency Select (Only for v3.00)
    _sdclk_freq_upper, _set_sdclk_freq_upper: 7, 6;

    /// Clock Generator Select (Only for v3.00)
    ///
    /// 1: Programmable Clock Mode
    /// 0: Divided Clock Mode
    pub u8, into ClockGenerator, clock_generator, set_clock_generator: 5, 5;

    /// SD Clock Enable
    pub sdclk_enable, set_sdclk_enable: 2;

    /// Internal Clock Stable (Read-Only)
    ///
    /// This bit is set to 1 when SD Clock is stable after writing to Internal Clock Enable in this
    /// register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1.
    pub internal_clock_stable, _: 1;

    /// Internal Clock Enable
    pub internal_clock_enable, set_interal_clock_enable: 0;
}

/// Clock Generator
#[derive(Debug, Copy, Clone)]
pub enum ClockGenerator {
    Programmable = 1,
    Divided = 0,
}
impl From<u8> for ClockGenerator {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Programmable,
            0 => Self::Divided,
            _ => unreachable!(),
        }
    }
}
impl From<ClockGenerator> for u8 {
    fn from(v: ClockGenerator) -> Self {
        v as u8
    }
}

bitfield! {
    /// Timeout Control Register
    #[derive(Copy, Clone)]
    pub struct TimeoutControl(u8);

    impl Debug;

    /// Data Timeout Counter
    ///
    /// This value determines the interval by which DAT line timeouts are detected.
    pub u8, data_timeout_count, set_data_timeout_count: 3, 0;
}

bitfield! {
    /// Software Reset Register
    ///
    /// A reset pulse is generated when writing 1 to each bit of this register. After completing the reset, the Host
    /// Controller shall clear each bit. Because it takes some time to complete software reset, the SD Host Driver
    /// shall confirm that these bits are 0.
    #[derive(Copy, Clone)]
    pub struct SoftwareReset(u8);

    impl Debug;

    /// Software Reset For DAT Line
    pub srst_dat, set_srst_dat: 2;

    /// Software Reset For CMD Line
    pub srst_cmd, set_srst_cmd: 1;

    /// Software Reset For All
    pub srst_all, set_srst_all: 0;
}

bitfield! {
    /// Normal Interrupt Status Register
    ///
    /// The [`NormalInterruptStatusEnable`] affects reads of this register, but [`NormalInterruptSignalEnable`] does
    /// not affect these reads. An interrupt is generated when the [`NormalInterruptSignalEnable`] is enabled and
    /// at least one of the status bits is set to 1. Writing 1 to a bit of RW1C attribute clears it; writing 0 keeps the
    /// bit unchanged. Writing 1 to a bit of ROC attribute keeps the bit unchanged. More than one status can be
    /// cleared with a single register write. The Card Interrupt is cleared when the card stops asserting the
    /// interrupt; that is, when the Card Driver services the interrupt condition.
    #[derive(Copy, Clone)]
    pub struct NormalInterruptStatus(u16);

    impl Debug;

    /// Error Interrupt
    ///
    /// If any of the bits in the [`ErrorInterruptStatus`] register are set, then this bit is set.
    pub error, _: 15;

    /// Re-Tuning Event
    pub re_tuning_event, _: 12;

    /// INT_C
    pub int_c, _: 11;

    /// INT_B
    pub int_b, _: 10;

    /// INT_A
    pub int_a, _: 9;

    /// Card Interrupt
    pub card, _: 8;

    /// Card Removal (RW1C)
    ///
    /// - 1: Card removed
    /// - 0: Card state stable or Debouncing
    pub card_removal, set_card_removal: 7;

    /// Card Insertion (RW1C)
    ///
    /// - 1: Card inserted
    /// - 0: Card state stable or Debouncing
    pub card_insertion, set_card_insertion: 6;

    /// Buffer Read Ready (RW1C)
    pub buffer_read_ready, set_buffer_read_ready: 5;

    /// Buffer Write Ready (RW1C)
    pub buffer_write_ready, set_buffer_write_ready: 4;

    /// DMA Interrupt (RW1C)
    pub dma, set_dma: 3;

    /// Block Gap Event (RW1C)
    pub block_gap_event, set_block_gap_event: 2;

    /// Transfer Complete (RW1C)
    pub transfer_complete, set_transfer_complete: 1;

    /// Command Complete (RW1C)
    pub command_complete, set_command_complete: 0;
}

bitfield! {
    /// Error Interrupt Status Register
    ///
    /// Signals defined in this register can be enabled by the [`ErrorInterruptStatusEnable`] register, but not by the
    /// [`ErrorInterruptSignalEnable`] register. The interrupt is generated when the [`ErrorInterruptSignalEnable`] is
    /// enabled and at least one of the statuses is set to 1. Writing to 1 clears the bit and writing to 0 keeps the bit
    /// unchanged. More than one status can be cleared at the one register write.
    #[derive(Copy, Clone)]
    pub struct ErrorInterruptStatus(u16);

    impl Debug;

    /// Vendor Specific Error Status
    pub u8, vendor_errors, set_vendor_errors: 15, 12;

    /// Tuning Error
    pub tuning_error, set_tuning_error: 10;

    /// ADMA Error
    pub adma_error, set_adma_error: 9;

    /// Auto CMD Error
    pub auto_cmd_error, set_auto_cmd_error: 8;

    /// Current Limit Error
    pub current_limit_error, set_current_limit_error: 7;

    /// Data End Bit Error
    pub dat_end_bit_error, set_dat_end_bit_error: 6;

    /// Data CRC Error
    pub dat_crc_error, set_dat_crc_error: 5;

    /// Data Timeout Error
    pub dat_timeout_error, set_dat_timeout_error: 4;

    /// Command Index Error
    pub command_index_error, set_command_index_error: 3;

    /// Command End Bit Error
    pub command_end_bit_error, set_command_end_bit_error: 2;

    /// Command CRC Error
    pub command_crc_error, set_command_crc_error: 1;

    /// Command Timeout Error
    pub command_timeout_error, set_command_timeout_error: 0;
}

bitfield! {
    /// Normal Interrupt Status Enable Register
    #[derive(Copy, Clone)]
    pub struct NormalInterruptStatusEnable(u16);

    impl Debug;

    /// 15: Fixed to 0

    /// Re-Tuning Event Status Enable
    pub re_tuning_event, set_re_tuning_event: 12;

    /// INT_C Status Enable
    pub int_c, set_int_c: 11;

    /// INT_B Status Enable
    pub int_b, set_int_b: 10;

    /// INT_A Status Enable
    pub int_a, set_int_a: 9;

    /// Card Interrupt Status Enable
    pub card, set_card: 8;

    /// Card Removal Status Enable
    pub card_removal, set_card_removal: 7;

    /// Card Insertion Status Enable
    pub card_insertion, set_card_insertion: 6;

    /// Buffer Read Ready Status Enable
    pub buffer_read_ready, set_buffer_read_ready: 5;

    /// Buffer Write Ready Status Enable
    pub buffer_write_ready, set_buffer_write_ready: 4;

    /// DMA Interrupt Status Enable
    pub dma, set_dma: 3;

    /// Block Gap Event Status Enable
    pub block_gap_event, set_block_gap_event: 2;

    /// Transfer Complete Status Enable
    pub transfer_complete, set_transfer_complete: 1;

    /// Command Complete Status Enable
    pub command_complete, set_command_complete: 0;
}

bitfield! {
    /// Error Interrupt Status Enable Register
    #[derive(Copy, Clone)]
    pub struct ErrorInterruptStatusEnable(u16);

    impl Debug;

    /// Vendor Specific Error Status Enable
    pub u8, vendor_errors, set_vendor_errors: 15, 12;

    /// Tuning Error Enable
    pub tuning_error, set_tuning_error: 10;

    /// ADMA Error Enable
    pub adma_error, set_adma_error: 9;

    /// Auto CMD Error Enable
    pub auto_cmd_error, set_auto_cmd_error: 8;

    /// Current Limit Error Enable
    pub current_limit_error, set_current_limit_error: 7;

    /// Data End Bit Error Enable
    pub dat_end_bit_error, set_dat_end_bit_error: 6;

    /// Data CRC Error Enable
    pub dat_crc_error, set_dat_crc_error: 5;

    /// Data Timeout Error Enable
    pub dat_timeout_error, set_dat_timeout_error: 4;

    /// Command Index Error Enable
    pub command_index_error, set_command_index_error: 3;

    /// Command End Bit Error Enable
    pub command_end_bit_error, set_command_end_bit_error: 2;

    /// Command CRC Error Enable
    pub command_crc_error, set_command_crc_error: 1;

    /// Command Timeout Error Enable
    pub command_timeout_error, set_command_timeout_error: 0;
}

bitfield! {
    /// Normal Interrupt Signal Enable Register
    #[derive(Copy, Clone)]
    pub struct NormalInterruptSignalEnable(u16);

    impl Debug;

    /// 15: Fixed to 0

    /// Re-Tuning Event Signal Enable
    pub re_tuning_event, set_re_tuning_event: 12;

    /// INT_C Signal Enable
    pub int_c, set_int_c: 11;

    /// INT_B Signal Enable
    pub int_b, set_int_b: 10;

    /// INT_A Signal Enable
    pub int_a, set_int_a: 9;

    /// Card Interrupt Signal Enable
    pub card, set_card: 8;

    /// Card Removal Signal Enable
    pub card_removal, set_card_removal: 7;

    /// Card Insertion Signal Enable
    pub card_insertion, set_card_insertion: 6;

    /// Buffer Read Ready Signal Enable
    pub buffer_read_ready, set_buffer_read_ready: 5;

    /// Buffer Write Ready Signal Enable
    pub buffer_write_ready, set_buffer_write_ready: 4;

    /// DMA Interrupt Signal Enable
    pub dma, set_dma: 3;

    /// Block Gap Event Signal Enable
    pub block_gap_event, set_block_gap_event: 2;

    /// Transfer Complete Signal Enable
    pub transfer_complete, set_transfer_complete: 1;

    /// Command Complete Signal Enable
    pub command_complete, set_command_complete: 0;
}

bitfield! {
    /// Error Interrupt Signal Enable Register
    #[derive(Copy, Clone)]
    pub struct ErrorInterruptSignalEnable(u16);

    impl Debug;

    /// Vendor Specific Error Signal Enable
    pub u8, vendor_errors, set_vendor_errors: 15, 12;

    /// Tuning Error Signal Enable
    pub tuning_error, set_tuning_error: 10;

    /// ADMA Error Signal Enable
    pub adma_error, set_adma_error: 9;

    /// Auto CMD Error Signal Enable
    pub auto_cmd_error, set_auto_cmd_error: 8;

    /// Current Limit Error Signal Enable
    pub current_limit_error, set_current_limit_error: 7;

    /// Data End Bit Error Signal Enable
    pub dat_end_bit_error, set_dat_end_bit_error: 6;

    /// Data CRC Error Signal Enable
    pub dat_crc_error, set_dat_crc_error: 5;

    /// Data Timeout Error Signal Enable
    pub dat_timeout_error, set_dat_timeout_error: 4;

    /// Command Index Error Signal Enable
    pub command_index_error, set_command_index_error: 3;

    /// Command End Bit Error Signal Enable
    pub command_end_bit_error, set_command_end_bit_error: 2;

    /// Command CRC Error Signal Enable
    pub command_crc_error, set_command_crc_error: 1;

    /// Command Timeout Error Signal Enable
    pub command_timeout_error, set_command_timeout_error: 0;
}

bitfield! {
    /// Auto CMD Error Status Register
    #[derive(Copy, Clone)]
    pub struct AutoCMDErrorStatus(u16);

    impl Debug;

    /// Command Not Issued By Auto CMD12 Error
    pub not_issued_by_auto_cmd12, _: 7;

    /// Auto CMD Index Error
    pub auto_cmd_index_error, _: 4;

    /// Auto CMD End Bit Error
    pub auto_cmd_end_bit_error, _: 3;

    /// Auto CMD CRC Error
    pub auto_cmd_crc_error, _: 2;

    /// Auto CMD Timeout Error
    pub auto_cmd_timeout_error, _: 1;

    /// Auto CMD12 Not Executed
    pub not_executed, _: 0;
}

bitfield! {
    /// Host Control 2 Register
    #[derive(Copy, Clone)]
    pub struct HostControl2(u16);

    impl Debug;

    /// Preset Value Enable
    ///
    /// Host Controller Version 3.00 supports this bit. As the operating SDCLK frequency and I/O driver strength
    /// depend on the Host System implementation, it is difficult to determine these parameters in the Standard
    /// Host Driver. When Preset Value Enable is set, automatic SDCLK frequency generation and driver strength
    /// selection is performed without considering system specific conditions. This bit enables the functions
    /// defined in the Preset Value registers.
    ///
    /// - 0: SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select
    ///   in Host Control 2 register are set by Host Driver.
    /// - 1: SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select
    ///   in Host Control 2 register are set by Host Controller as specified in the Preset Value registers.
    pub preset_value_enable, set_preset_value_enable: 15;

    /// Asynchronous Interrupt Enable
    pub async_interrupt_enable, set_async_interrupt_enable: 14;

    /// Sampling Clock Select
    ///
    /// Setting:
    /// - 1: tuning is completed successfully
    /// - 0: tuning is failed
    ///
    /// Writing:
    /// 1: meaningless and ignored
    /// 0: a tuning circuit is reset
    pub u8, from into SamplingClock, sampling_clock, set_sampling_clock: 7, 7;

    /// Execute Tuning
    ///
    /// This bit is set to 1 to start tuning procedure and automatically cleared when tuning procedure is completed.
    /// The result of tuning is indicated to Sampling Clock Select. Tuning procedure is aborted by writing 0.
    pub execute_tuning, set_execute_tuning: 6;

    /// Driver Strength Select
    ///
    /// Host Controller output driver in 1.8V signaling is selected by this bit. In 3.3V signaling, this field is not
    /// effective. This field can be set depends on Driver Type A, C and D support bits in the Capabilities register.
    pub u8, from into DriverStrength, driver_strength, set_driver_strength: 5, 4;

    /// 1.8V Signaling Enable
    pub _1_8v_signaling_enable, set_1_8v_signaling_enable: 3;

    // TODO: [2:0] UHS Mode Select
}

#[derive(Debug, Copy, Clone)]
pub enum SamplingClock {
    Tuned = 1,
    Fixed = 0,
}
impl From<u8> for SamplingClock {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Tuned,
            0 => Self::Fixed,
            _ => unreachable!(),
        }
    }
}
impl From<SamplingClock> for u8 {
    fn from(v: SamplingClock) -> Self {
        v as u8
    }
}

/// Driver Strength is supported by 1.8V signaling bus speed modes. It's meaningless for 3.3V signaling.
#[derive(Debug, Copy, Clone)]
pub enum DriverStrength {
    /// Default
    TypeB = 0b00,
    TypeA = 0b01,
    TypeC = 0b10,
    TypeD = 0b11,
}
impl From<DriverStrength> for u8 {
    fn from(v: DriverStrength) -> Self {
        v as u8
    }
}
impl From<u8> for DriverStrength {
    fn from(v: u8) -> Self {
        match v {
            0b00 => DriverStrength::TypeB,
            0b01 => DriverStrength::TypeA,
            0b10 => DriverStrength::TypeC,
            0b11 => DriverStrength::TypeD,
            _ => unreachable!(),
        }
    }
}

bitfield! {
    /// Capabilities Register
    ///
    /// This register provides the Host Driver with information specific to the Host Controller implementation.
    #[derive(Copy, Clone)]
    pub struct Capabilities(u64);

    impl Debug;

    /// Clock Multiplier
    ///
    /// This field indicates clock multiplier value of programmable clock generator.
    ///
    /// Setting 00h means that Host Controller does not support programmable clock generator.
    pub u8, clock_multiplier, _: 55, 48;

    /// Re-Tuning Modes
    pub u8, into ReTuningMode, re_tuning_mode, _: 47, 46;

    /// Use Tuning for SDR50
    pub use_tuning_for_sdr50, _: 45;

    /// Timer Count for Re-Tuning
    pub u8, timer_count_for_re_tuning, _: 43, 40;

    /// Driver Type D Support
    pub driver_type_d_support, _: 38;

    /// Driver Type C Support
    pub driver_type_c_support, _: 37;

    /// Driver Type A Support
    pub driver_type_a_support, _: 36;

    /// DDR50 Support
    pub ddr50_support, _: 34;

    /// SDR104 Support
    pub sdr104_support, _: 33;

    /// SDR50 Support
    pub sdr50_support, _: 32;

    /// Slot Type
    pub u8, into SlotType, slot_type, _: 31, 30;

    /// Asynchronous Interrupt Support
    pub async_interrupt_support, _: 29;

    /// 64-bit System Bus Support
    pub _64bit_system_bus_support, _: 28;

    /// Voltage Support 1.8V
    pub voltage_1_8v_support, _: 26;

    /// Voltage Support 3.0V
    pub voltage_3_0v_support, _: 25;

    /// Voltage Support 3.3V
    pub voltage_3_3v_support, _: 24;

    /// Suspend/Resume Support
    pub suspend_resume_support, _: 23;

    /// SDMA Support
    pub sdma_support, _: 22;

    /// High Speed Support
    ///
    /// This bit indicates whether the Host Controller and the Host System support High Speed mode and they can
    /// supply SD Clock frequency from 25MHz to 50MHz.
    pub high_speed_support, _: 21;

    /// legacy ADMA1 Support
    pub legacy_adma1_support, _: 20;

    /// ADMA2 Support
    pub adma2_support, _: 19;

    /// 8-bit Support for Embedded Device
    pub _8bit_bus_support, _: 18;

    /// Max Block Length
    pub u8, into MaxBlockLength, max_block_length, _: 17, 16;

    /// Base Clock Frequency For SD Clock (Unit 1MHz)
    ///
    /// This value indicates the base (maximum) clock frequency for the SD Clock. Definition of this field depends
    /// on Host Controller Version:
    /// 1. For V1.00 and V2.00: 6-bit Base Clock Frequency, range 10MHz - 63MHz.
    /// 2. For V3.00: 8-bit Base Clock Frequency, range 10MHz - 255MHz.
    ///
    /// If these bits are all 0, the Host System has to get information via another method.
    pub u8, sdclk_base_freq, _: 15, 8;

    /// Timeout Clock Unit
    ///
    /// This bit shows the unit of base clock frequency used to detect Data Timeout Error.
    pub u8, into TimeoutClockUnit, timeout_clock_unit, _: 7, 7;

    /// Timeout Clock Frequency
    ///
    /// This bit shows the base clock frequency used to detect Data Timeout Error. The Timeout Clock Unit defines
    /// the unit of this field's value.
    pub u8, timeout_clock_freq, _: 5, 0;
}

/// Re-Tuning Modes
#[derive(Debug, Copy, Clone)]
pub enum ReTuningMode {
    /// Timer
    Mode1 = 0b00,
    /// Timer and Re-Tuning Request
    Mode2 = 0b01,
    /// Auto Re-Tuning (for transfer) Any Timer and Re-Tuning Request
    Mode3 = 0b10,
    /// Reserved
    Reserved,
}
impl From<u8> for ReTuningMode {
    fn from(v: u8) -> Self {
        match v {
            0b00 => ReTuningMode::Mode1,
            0b01 => ReTuningMode::Mode2,
            0b10 => ReTuningMode::Mode3,
            _ => ReTuningMode::Reserved,
        }
    }
}

/// SlotType
#[derive(Debug, Copy, Clone)]
pub enum SlotType {
    /// Removable Card Slot
    Removable = 0b00,
    /// Embedded Slot for One Device
    Embedded = 0b01,
    /// Shared Bus Slot
    SharedBus = 0b10,

    Reserved,
}
impl From<u8> for SlotType {
    fn from(v: u8) -> Self {
        match v {
            0b00 => SlotType::Removable,
            0b01 => SlotType::Embedded,
            0b10 => SlotType::SharedBus,
            _ => Self::Reserved,
        }
    }
}

/// Maximum block size that the Host Driver can read and write to the buffer in the Host Controller
#[derive(Debug, Copy, Clone)]
pub enum MaxBlockLength {
    _512 = 0b00,
    _1024 = 0b01,
    _2048 = 0b10,
    Reserved = 0b11,
}
impl From<u8> for MaxBlockLength {
    fn from(v: u8) -> Self {
        match v {
            0b00 => MaxBlockLength::_512,
            0b01 => MaxBlockLength::_1024,
            0b10 => MaxBlockLength::_512,
            _ => Self::Reserved,
        }
    }
}

/// Timeout Clock Unit
#[derive(Debug, Copy, Clone)]
pub enum TimeoutClockUnit {
    KHz = 0,
    MHz = 1,
}
impl From<u8> for TimeoutClockUnit {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::KHz,
            1 => Self::MHz,
            _ => unreachable!(),
        }
    }
}

bitfield! {
    /// TODO: Maximum Current Capabilities Register
    #[derive(Copy, Clone)]
    pub struct MaxCurrentCapabilities(u64);

    impl Debug;
}

bitfield! {
    /// TODO: Force Event Register for Auto CMD Error Status
    ///
    /// The Force Event Register is not a physically implemented register. Rather, it is an address at which the
    /// Auto CMD Error Status Register can be written.
    #[derive(Copy, Clone)]
    pub struct ForceEventForAutoCMDError(u16);

    impl Debug;
}

bitfield! {
    /// TODO: Force Event Register for Error Interrupt Status
    ///
    /// The Force Event Register is not a physically implemented register. Rather, it is an address at which the
    /// Error Interrupt Status register can be written. The effect of a write to this address will be reflected in the
    /// Error Interrupt Status Register if the corresponding bit of the Error Interrupt Status Enable Register is set.
    #[derive(Copy, Clone)]
    pub struct ForceEventForErrorInterrupt(u16);

    impl Debug;
}

bitfield! {
    /// TODO: ADMA Error Status Register
    #[derive(Copy, Clone)]
    pub struct AMDAErrorStatus(u8);

    impl Debug;
}

bitfield! {
    /// TODO: ADMA System Address Register
    #[derive(Copy, Clone)]
    pub struct AMDASystemAddress(u64);

    impl Debug;
}

bitfield! {
    /// Preset Value Register
    ///
    /// There are a set of preset values per card or device. One of the Preset Value registers (06Eh - 062h)
    /// is effective based on the Selected Bus Speed Mode. Before starting the initialization sequence, the
    /// Host Driver needs to set a clock preset value to SDCLK Frequency Select in the Clock Control Register.
    /// Preset Value Enable can be set after initialization completed.
    #[derive(Copy, Clone)]
    pub struct PresetValue(u16);

    impl Debug;

    /// Driver Strength Select Value
    pub u8, into DriverStrength, driver_strength, _: 15, 14;

    /// Clock Generator Select Value
    pub u8, into ClockGenerator, clock_generator, _: 10, 10;

    /// SDCLK Frequency Select Value
    pub u16, sdclk_freq, _: 9, 0;
}

bitfield! {
    /// TODO: Shared Bus Control Register (Optional)
    #[derive(Copy, Clone)]
    pub struct SharedBusControl(u32);

    impl Debug;
}

bitfield! {
    /// Slot Interrupt Status Register
    #[derive(Copy, Clone)]
    pub struct SlotInterruptStatus(u16);

    impl Debug;

    /// Interrupt Signal For Each Slot
    ///
    /// These status bits indicate the logical OR of Interrupt Signal and Wakeup Signal for each slot.
    /// A maximum of 8 slots can be defined.
    pub slot8, _: 7;
    pub slot7, _: 6;
    pub slot6, _: 5;
    pub slot5, _: 4;
    pub slot4, _: 3;
    pub slot3, _: 2;
    pub slot2, _: 1;
    pub slot1, _: 0;
}

bitfield! {
    /// Host Controller Version Register
    #[derive(Copy, Clone)]
    pub struct HostControllerVersion(u16);

    impl Debug;

    /// Vendor Version Number
    pub u8, vendor_version, _: 15, 8;

    /// Specification Version Number
    pub u8, into HostSpecVersion, host_spec_version, _: 7, 0;
}

/// SD Host Specification Version
#[derive(Debug, Copy, Clone)]
pub enum HostSpecVersion {
    /// Version 1.00
    V1 = 0,
    /// Version 2.00
    V2 = 1,
    /// Version 3.00
    V3 = 2,

    Reserved,
}
impl From<u8> for HostSpecVersion {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::V1,
            1 => Self::V2,
            2 => Self::V3,
            _ => Self::Reserved,
        }
    }
}
impl From<HostSpecVersion> for u8 {
    fn from(v: HostSpecVersion) -> Self {
        v as u8
    }
}
