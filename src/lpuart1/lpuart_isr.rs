#[doc = "Register `LPUART_ISR` reader"]
pub type R = crate::R<LpuartIsrSpec>;
#[doc = "Parity error This bit is set by hardware when a parity error occurs in Reception mode. It is cleared by software, writing 1 to the PECF in the LPUART_ICR register. An interrupt is generated if PEIE = 1 in the LPUART_CR1 register. Note: This error is associated with the character in the LPUART_RDR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: No parity error"]
    B0x0 = 0,
    #[doc = "1: Parity error"]
    B0x1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity error This bit is set by hardware when a parity error occurs in Reception mode. It is cleared by software, writing 1 to the PECF in the LPUART_ICR register. An interrupt is generated if PEIE = 1 in the LPUART_CR1 register. Note: This error is associated with the character in the LPUART_RDR."]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::B0x0,
            true => Pe::B0x1,
        }
    }
    #[doc = "No parity error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pe::B0x0
    }
    #[doc = "Parity error"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pe::B0x1
    }
}
#[doc = "Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the LPUART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE1=11 in the LPUART_CR3 register. Note: This error is associated with the character in the LPUART_RDR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: No Framing error is detected"]
    B0x0 = 0,
    #[doc = "1: Framing error or break character is detected"]
    B0x1 = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the LPUART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE1=11 in the LPUART_CR3 register. Note: This error is associated with the character in the LPUART_RDR."]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::B0x0,
            true => Fe::B0x1,
        }
    }
    #[doc = "No Framing error is detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fe::B0x0
    }
    #[doc = "Framing error or break character is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fe::B0x1
    }
}
#[doc = "Start bit noise detection flag This bit is set by hardware when noise is detected on the start bit of a received frame. It is cleared by software, writing 1 to the NFCF bit in the LPUART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. Note: This error is associated with the character in the LPUART_RDR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ne {
    #[doc = "0: No noise is detected"]
    B0x0 = 0,
    #[doc = "1: Noise is detected"]
    B0x1 = 1,
}
impl From<Ne> for bool {
    #[inline(always)]
    fn from(variant: Ne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NE` reader - Start bit noise detection flag This bit is set by hardware when noise is detected on the start bit of a received frame. It is cleared by software, writing 1 to the NFCF bit in the LPUART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. Note: This error is associated with the character in the LPUART_RDR."]
pub type NeR = crate::BitReader<Ne>;
impl NeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ne {
        match self.bits {
            false => Ne::B0x0,
            true => Ne::B0x1,
        }
    }
    #[doc = "No noise is detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ne::B0x0
    }
    #[doc = "Noise is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ne::B0x1
    }
}
#[doc = "Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the LPUART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the LPUART_ICR register. An interrupt is generated if RXFNEIE=1 in the LPUART_CR1 register, or EIE = 1 in the LPUART_CR3 register. Note: When this bit is set, the LPUART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. Note: This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the LPUART_CR3 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ore {
    #[doc = "1: Overrun error is detected"]
    B0x1 = 1,
}
impl From<Ore> for bool {
    #[inline(always)]
    fn from(variant: Ore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORE` reader - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the LPUART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the LPUART_ICR register. An interrupt is generated if RXFNEIE=1 in the LPUART_CR1 register, or EIE = 1 in the LPUART_CR3 register. Note: When this bit is set, the LPUART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. Note: This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the LPUART_CR3 register."]
pub type OreR = crate::BitReader<Ore>;
impl OreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ore> {
        match self.bits {
            true => Some(Ore::B0x1),
            _ => None,
        }
    }
    #[doc = "Overrun error is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ore::B0x1
    }
}
#[doc = "Idle line detected This bit is set by hardware when an Idle line is detected. An interrupt is generated if IDLEIE=1 in the LPUART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the LPUART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). Note: If Mute mode is enabled (MME=1), IDLE is set if the LPUART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: No Idle line is detected"]
    B0x0 = 0,
    #[doc = "1: Idle line is detected"]
    B0x1 = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - Idle line detected This bit is set by hardware when an Idle line is detected. An interrupt is generated if IDLEIE=1 in the LPUART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the LPUART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). Note: If Mute mode is enabled (MME=1), IDLE is set if the LPUART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set."]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::B0x0,
            true => Idle::B0x1,
        }
    }
    #[doc = "No Idle line is detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Idle::B0x0
    }
    #[doc = "Idle line is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Idle::B0x1
    }
}
#[doc = "RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, and so data can be read from the LPUART_RDR register. Every read of the LPUART_RDR frees a location in the RXFIFO. It is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the LPUART_RQR register. An interrupt is generated if RXFNEIE=1 in the LPUART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfne {
    #[doc = "0: Data is not received"]
    B0x0 = 0,
    #[doc = "1: Received data is ready to be read."]
    B0x1 = 1,
}
impl From<Rxfne> for bool {
    #[inline(always)]
    fn from(variant: Rxfne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFNE` reader - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, and so data can be read from the LPUART_RDR register. Every read of the LPUART_RDR frees a location in the RXFIFO. It is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the LPUART_RQR register. An interrupt is generated if RXFNEIE=1 in the LPUART_CR1 register."]
pub type RxfneR = crate::BitReader<Rxfne>;
impl RxfneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfne {
        match self.bits {
            false => Rxfne::B0x0,
            true => Rxfne::B0x1,
        }
    }
    #[doc = "Data is not received"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxfne::B0x0
    }
    #[doc = "Received data is ready to be read."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxfne::B0x1
    }
}
#[doc = "Field `TC` reader - Transmission complete This bit indicates that the last data written in the LPUART_TDR has been transmitted out of the shift register. The TC flag behaves as follows: When TDN = 0, the TC flag is set when the transmission of a frame containing data is complete and when TXFE is set. When TDN is equal to the number of data in the TXFIFO, the TC flag is set when TXFIFO is empty and TDN is reached. When TDN is greater than the number of data in the TXFIFO, TC remains cleared until the TXFIFO is filled again to reach the programmed number of data to be transferred. When TDN is less than the number of data in the TXFIFO, TC is set when TDN is reached even if the TXFIFO is not empty. An interrupt is generated if TCIE=1 in the LPUART_CR1 register. TC bit is cleared by software by writing 1 to the TCCF in the LPUART_ICR register or by writing to the LPUART_TDR register."]
pub type TcR = crate::BitReader;
#[doc = "TXFIFO not full TXFNF is set by hardware when TXFIFO is not full, and so data can be written in the LPUART_TDR. Every write in the LPUART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the LPUART_TDR. The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF must be checked prior to writing in TXFIFO (TXFNF and TXFE are set at the same time). An interrupt is generated if the TXFNFIE bit =1 in the LPUART_CR1 register. Note: This bit is used during single buffer transmission.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfnf {
    #[doc = "0: Data register is full/Transmit FIFO is full."]
    B0x0 = 0,
    #[doc = "1: Data register/Transmit FIFO is not full."]
    B0x1 = 1,
}
impl From<Txfnf> for bool {
    #[inline(always)]
    fn from(variant: Txfnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFNF` reader - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full, and so data can be written in the LPUART_TDR. Every write in the LPUART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the LPUART_TDR. The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF must be checked prior to writing in TXFIFO (TXFNF and TXFE are set at the same time). An interrupt is generated if the TXFNFIE bit =1 in the LPUART_CR1 register. Note: This bit is used during single buffer transmission."]
pub type TxfnfR = crate::BitReader<Txfnf>;
impl TxfnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfnf {
        match self.bits {
            false => Txfnf::B0x0,
            true => Txfnf::B0x1,
        }
    }
    #[doc = "Data register is full/Transmit FIFO is full."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txfnf::B0x0
    }
    #[doc = "Data register/Transmit FIFO is not full."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txfnf::B0x1
    }
}
#[doc = "CTS interrupt flag This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the LPUART_ICR register. An interrupt is generated if CTSIE=1 in the LPUART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsif {
    #[doc = "0: No change occurred on the CTS status line"]
    B0x0 = 0,
    #[doc = "1: A change occurred on the CTS status line"]
    B0x1 = 1,
}
impl From<Ctsif> for bool {
    #[inline(always)]
    fn from(variant: Ctsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIF` reader - CTS interrupt flag This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the LPUART_ICR register. An interrupt is generated if CTSIE=1 in the LPUART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CtsifR = crate::BitReader<Ctsif>;
impl CtsifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsif {
        match self.bits {
            false => Ctsif::B0x0,
            true => Ctsif::B0x1,
        }
    }
    #[doc = "No change occurred on the CTS status line"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ctsif::B0x0
    }
    #[doc = "A change occurred on the CTS status line"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ctsif::B0x1
    }
}
#[doc = "CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: CTS line set"]
    B0x0 = 0,
    #[doc = "1: CTS line reset"]
    B0x1 = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CtsR = crate::BitReader<Cts>;
impl CtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cts {
        match self.bits {
            false => Cts::B0x0,
            true => Cts::B0x1,
        }
    }
    #[doc = "CTS line set"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cts::B0x0
    }
    #[doc = "CTS line reset"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cts::B0x1
    }
}
#[doc = "Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: LPUART is idle (no reception)"]
    B0x0 = 0,
    #[doc = "1: Reception on going"]
    B0x1 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::B0x0,
            true => Busy::B0x1,
        }
    }
    #[doc = "LPUART is idle (no reception)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Busy::B0x0
    }
    #[doc = "Reception on going"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Busy::B0x1
    }
}
#[doc = "Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the LPUART_ICR register. An interrupt is generated if CMIE=1in the LPUART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmf {
    #[doc = "0: No Character match detected"]
    B0x0 = 0,
    #[doc = "1: Character match detected"]
    B0x1 = 1,
}
impl From<Cmf> for bool {
    #[inline(always)]
    fn from(variant: Cmf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMF` reader - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the LPUART_ICR register. An interrupt is generated if CMIE=1in the LPUART_CR1 register."]
pub type CmfR = crate::BitReader<Cmf>;
impl CmfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmf {
        match self.bits {
            false => Cmf::B0x0,
            true => Cmf::B0x1,
        }
    }
    #[doc = "No Character match detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cmf::B0x0
    }
    #[doc = "Character match detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cmf::B0x1
    }
}
#[doc = "Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the LPUART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbkf {
    #[doc = "0: No break character transmitted"]
    B0x0 = 0,
    #[doc = "1: Break character transmitted"]
    B0x1 = 1,
}
impl From<Sbkf> for bool {
    #[inline(always)]
    fn from(variant: Sbkf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKF` reader - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the LPUART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
pub type SbkfR = crate::BitReader<Sbkf>;
impl SbkfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbkf {
        match self.bits {
            false => Sbkf::B0x0,
            true => Sbkf::B0x1,
        }
    }
    #[doc = "No break character transmitted"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sbkf::B0x0
    }
    #[doc = "Break character transmitted"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sbkf::B0x1
    }
}
#[doc = "Receiver wake-up from Mute mode This bit indicates if the LPUART is in Mute mode. It is cleared/set by hardware when a wake-up/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the LPUART_CR1 register. When wake-up on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the LPUART_RQR register. Note: If the LPUART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwu {
    #[doc = "0: Receiver in Active mode"]
    B0x0 = 0,
    #[doc = "1: Receiver in Mute mode"]
    B0x1 = 1,
}
impl From<Rwu> for bool {
    #[inline(always)]
    fn from(variant: Rwu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWU` reader - Receiver wake-up from Mute mode This bit indicates if the LPUART is in Mute mode. It is cleared/set by hardware when a wake-up/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the LPUART_CR1 register. When wake-up on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the LPUART_RQR register. Note: If the LPUART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value."]
pub type RwuR = crate::BitReader<Rwu>;
impl RwuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwu {
        match self.bits {
            false => Rwu::B0x0,
            true => Rwu::B0x1,
        }
    }
    #[doc = "Receiver in Active mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rwu::B0x0
    }
    #[doc = "Receiver in Mute mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rwu::B0x1
    }
}
#[doc = "Field `WUF` reader - Wake-up from low-power mode flag This bit is set by hardware, when a wake-up event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the LPUART_ICR register. An interrupt is generated if WUFIE=1 in the LPUART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value. Refer to Section132.3: LPUART implementation on page1914."]
pub type WufR = crate::BitReader;
#[doc = "Field `TEACK` reader - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the LPUART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the LPUART_CR1 register, in order to respect the TE=0 minimum period."]
pub type TeackR = crate::BitReader;
#[doc = "Field `REACK` reader - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the LPUART. It can be used to verify that the LPUART is ready for reception before entering low-power mode. Note: If the LPUART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value."]
pub type ReackR = crate::BitReader;
#[doc = "TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the LPUART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the LPUART_CR1 register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfe {
    #[doc = "0: TXFIFO is not empty."]
    B0x0 = 0,
    #[doc = "1: TXFIFO is empty."]
    B0x1 = 1,
}
impl From<Txfe> for bool {
    #[inline(always)]
    fn from(variant: Txfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFE` reader - TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the LPUART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the LPUART_CR1 register."]
pub type TxfeR = crate::BitReader<Txfe>;
impl TxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfe {
        match self.bits {
            false => Txfe::B0x0,
            true => Txfe::B0x1,
        }
    }
    #[doc = "TXFIFO is not empty."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txfe::B0x0
    }
    #[doc = "TXFIFO is empty."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txfe::B0x1
    }
}
#[doc = "RXFIFO Full This bit is set by hardware when the number of received data corresponds to RXFIFO1size1+11 (RXFIFO full + 1 data in the LPUART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the LPUART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxff {
    #[doc = "0: RXFIFO is not Full."]
    B0x0 = 0,
    #[doc = "1: RXFIFO is Full."]
    B0x1 = 1,
}
impl From<Rxff> for bool {
    #[inline(always)]
    fn from(variant: Rxff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFF` reader - RXFIFO Full This bit is set by hardware when the number of received data corresponds to RXFIFO1size1+11 (RXFIFO full + 1 data in the LPUART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the LPUART_CR1 register."]
pub type RxffR = crate::BitReader<Rxff>;
impl RxffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxff {
        match self.bits {
            false => Rxff::B0x0,
            true => Rxff::B0x1,
        }
    }
    #[doc = "RXFIFO is not Full."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxff::B0x0
    }
    #[doc = "RXFIFO is Full."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxff::B0x1
    }
}
#[doc = "RXFIFO threshold flag This bit is set by hardware when the RXFIFO reaches the threshold programmed in RXFTCFG in LPUART_CR3 register i.e. the Receive FIFO contains RXFTCFG data. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the LPUART_CR3 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxft {
    #[doc = "0: Receive FIFO does not reach the programmed threshold."]
    B0x0 = 0,
    #[doc = "1: Receive FIFO reached the programmed threshold."]
    B0x1 = 1,
}
impl From<Rxft> for bool {
    #[inline(always)]
    fn from(variant: Rxft) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFT` reader - RXFIFO threshold flag This bit is set by hardware when the RXFIFO reaches the threshold programmed in RXFTCFG in LPUART_CR3 register i.e. the Receive FIFO contains RXFTCFG data. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the LPUART_CR3 register."]
pub type RxftR = crate::BitReader<Rxft>;
impl RxftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxft {
        match self.bits {
            false => Rxft::B0x0,
            true => Rxft::B0x1,
        }
    }
    #[doc = "Receive FIFO does not reach the programmed threshold."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxft::B0x0
    }
    #[doc = "Receive FIFO reached the programmed threshold."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxft::B0x1
    }
}
#[doc = "TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG in LPUART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the LPUART_CR3 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txft {
    #[doc = "0: TXFIFO does not reach the programmed threshold."]
    B0x0 = 0,
    #[doc = "1: TXFIFO reached the programmed threshold."]
    B0x1 = 1,
}
impl From<Txft> for bool {
    #[inline(always)]
    fn from(variant: Txft) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFT` reader - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG in LPUART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the LPUART_CR3 register."]
pub type TxftR = crate::BitReader<Txft>;
impl TxftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txft {
        match self.bits {
            false => Txft::B0x0,
            true => Txft::B0x1,
        }
    }
    #[doc = "TXFIFO does not reach the programmed threshold."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txft::B0x0
    }
    #[doc = "TXFIFO reached the programmed threshold."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txft::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - Parity error This bit is set by hardware when a parity error occurs in Reception mode. It is cleared by software, writing 1 to the PECF in the LPUART_ICR register. An interrupt is generated if PEIE = 1 in the LPUART_CR1 register. Note: This error is associated with the character in the LPUART_RDR."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the LPUART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE1=11 in the LPUART_CR3 register. Note: This error is associated with the character in the LPUART_RDR."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start bit noise detection flag This bit is set by hardware when noise is detected on the start bit of a received frame. It is cleared by software, writing 1 to the NFCF bit in the LPUART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. Note: This error is associated with the character in the LPUART_RDR."]
    #[inline(always)]
    pub fn ne(&self) -> NeR {
        NeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the LPUART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the LPUART_ICR register. An interrupt is generated if RXFNEIE=1 in the LPUART_CR1 register, or EIE = 1 in the LPUART_CR3 register. Note: When this bit is set, the LPUART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. Note: This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the LPUART_CR3 register."]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle line detected This bit is set by hardware when an Idle line is detected. An interrupt is generated if IDLEIE=1 in the LPUART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the LPUART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). Note: If Mute mode is enabled (MME=1), IDLE is set if the LPUART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, and so data can be read from the LPUART_RDR register. Every read of the LPUART_RDR frees a location in the RXFIFO. It is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the LPUART_RQR register. An interrupt is generated if RXFNEIE=1 in the LPUART_CR1 register."]
    #[inline(always)]
    pub fn rxfne(&self) -> RxfneR {
        RxfneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete This bit indicates that the last data written in the LPUART_TDR has been transmitted out of the shift register. The TC flag behaves as follows: When TDN = 0, the TC flag is set when the transmission of a frame containing data is complete and when TXFE is set. When TDN is equal to the number of data in the TXFIFO, the TC flag is set when TXFIFO is empty and TDN is reached. When TDN is greater than the number of data in the TXFIFO, TC remains cleared until the TXFIFO is filled again to reach the programmed number of data to be transferred. When TDN is less than the number of data in the TXFIFO, TC is set when TDN is reached even if the TXFIFO is not empty. An interrupt is generated if TCIE=1 in the LPUART_CR1 register. TC bit is cleared by software by writing 1 to the TCCF in the LPUART_ICR register or by writing to the LPUART_TDR register."]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full, and so data can be written in the LPUART_TDR. Every write in the LPUART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the LPUART_TDR. The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF must be checked prior to writing in TXFIFO (TXFNF and TXFE are set at the same time). An interrupt is generated if the TXFNFIE bit =1 in the LPUART_CR1 register. Note: This bit is used during single buffer transmission."]
    #[inline(always)]
    pub fn txfnf(&self) -> TxfnfR {
        TxfnfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the LPUART_ICR register. An interrupt is generated if CTSIE=1 in the LPUART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn ctsif(&self) -> CtsifR {
        CtsifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the LPUART_ICR register. An interrupt is generated if CMIE=1in the LPUART_CR1 register."]
    #[inline(always)]
    pub fn cmf(&self) -> CmfR {
        CmfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the LPUART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
    #[inline(always)]
    pub fn sbkf(&self) -> SbkfR {
        SbkfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver wake-up from Mute mode This bit indicates if the LPUART is in Mute mode. It is cleared/set by hardware when a wake-up/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the LPUART_CR1 register. When wake-up on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the LPUART_RQR register. Note: If the LPUART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wake-up from low-power mode flag This bit is set by hardware, when a wake-up event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the LPUART_ICR register. An interrupt is generated if WUFIE=1 in the LPUART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value. Refer to Section132.3: LPUART implementation on page1914."]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the LPUART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the LPUART_CR1 register, in order to respect the TE=0 minimum period."]
    #[inline(always)]
    pub fn teack(&self) -> TeackR {
        TeackR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the LPUART. It can be used to verify that the LPUART is ready for reception before entering low-power mode. Note: If the LPUART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn reack(&self) -> ReackR {
        ReackR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the LPUART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the LPUART_CR1 register."]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RXFIFO Full This bit is set by hardware when the number of received data corresponds to RXFIFO1size1+11 (RXFIFO full + 1 data in the LPUART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the LPUART_CR1 register."]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RXFIFO threshold flag This bit is set by hardware when the RXFIFO reaches the threshold programmed in RXFTCFG in LPUART_CR3 register i.e. the Receive FIFO contains RXFTCFG data. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the LPUART_CR3 register."]
    #[inline(always)]
    pub fn rxft(&self) -> RxftR {
        RxftR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG in LPUART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the LPUART_CR3 register."]
    #[inline(always)]
    pub fn txft(&self) -> TxftR {
        TxftR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "LPUART interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpuartIsrSpec;
impl crate::RegisterSpec for LpuartIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpuart_isr::R`](R) reader structure"]
impl crate::Readable for LpuartIsrSpec {}
#[doc = "`reset()` method sets LPUART_ISR to value 0x0080_00c0"]
impl crate::Resettable for LpuartIsrSpec {
    const RESET_VALUE: u32 = 0x0080_00c0;
}
