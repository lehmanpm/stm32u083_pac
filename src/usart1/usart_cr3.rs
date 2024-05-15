#[doc = "Register `USART_CR3` reader"]
pub type R = crate::R<UsartCr3Spec>;
#[doc = "Register `USART_CR3` writer"]
pub type W = crate::W<UsartCr3Spec>;
#[doc = "Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: interrupt generated when FE=1 or ORE=1 or NE=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
    B0x1 = 1,
}
impl From<Eie> for bool {
    #[inline(always)]
    fn from(variant: Eie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register)."]
pub type EieR = crate::BitReader<Eie>;
impl EieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eie {
        match self.bits {
            false => Eie::B0x0,
            true => Eie::B0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eie::B0x0
    }
    #[doc = "interrupt generated when FE=1 or ORE=1 or NE=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eie::B0x1
    }
}
#[doc = "Field `EIE` writer - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register)."]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG, Eie>;
impl<'a, REG> EieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eie::B0x0)
    }
    #[doc = "interrupt generated when FE=1 or ORE=1 or NE=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eie::B0x1)
    }
}
#[doc = "IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iren {
    #[doc = "0: IrDA disabled"]
    B0x0 = 0,
    #[doc = "1: IrDA enabled"]
    B0x1 = 1,
}
impl From<Iren> for bool {
    #[inline(always)]
    fn from(variant: Iren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type IrenR = crate::BitReader<Iren>;
impl IrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iren {
        match self.bits {
            false => Iren::B0x0,
            true => Iren::B0x1,
        }
    }
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Iren::B0x0
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Iren::B0x1
    }
}
#[doc = "Field `IREN` writer - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG, Iren>;
impl<'a, REG> IrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::B0x0)
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::B0x1)
    }
}
#[doc = "IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irlp {
    #[doc = "0: Normal mode"]
    B0x0 = 0,
    #[doc = "1: Low-power mode"]
    B0x1 = 1,
}
impl From<Irlp> for bool {
    #[inline(always)]
    fn from(variant: Irlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRLP` reader - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type IrlpR = crate::BitReader<Irlp>;
impl IrlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irlp {
        match self.bits {
            false => Irlp::B0x0,
            true => Irlp::B0x1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Irlp::B0x0
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Irlp::B0x1
    }
}
#[doc = "Field `IRLP` writer - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type IrlpW<'a, REG> = crate::BitWriter<'a, REG, Irlp>;
impl<'a, REG> IrlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Irlp::B0x0)
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Irlp::B0x1)
    }
}
#[doc = "Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdsel {
    #[doc = "0: Half-duplex mode is not selected"]
    B0x0 = 0,
    #[doc = "1: Half-duplex mode is selected"]
    B0x1 = 1,
}
impl From<Hdsel> for bool {
    #[inline(always)]
    fn from(variant: Hdsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSEL` reader - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0)."]
pub type HdselR = crate::BitReader<Hdsel>;
impl HdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdsel {
        match self.bits {
            false => Hdsel::B0x0,
            true => Hdsel::B0x1,
        }
    }
    #[doc = "Half-duplex mode is not selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hdsel::B0x0
    }
    #[doc = "Half-duplex mode is selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hdsel::B0x1
    }
}
#[doc = "Field `HDSEL` writer - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0)."]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG, Hdsel>;
impl<'a, REG> HdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half-duplex mode is not selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdsel::B0x0)
    }
    #[doc = "Half-duplex mode is selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdsel::B0x1)
    }
}
#[doc = "Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nack {
    #[doc = "0: NACK transmission in case of parity error is disabled"]
    B0x0 = 0,
    #[doc = "1: NACK transmission during parity error is enabled"]
    B0x1 = 1,
}
impl From<Nack> for bool {
    #[inline(always)]
    fn from(variant: Nack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type NackR = crate::BitReader<Nack>;
impl NackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nack {
        match self.bits {
            false => Nack::B0x0,
            true => Nack::B0x1,
        }
    }
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Nack::B0x0
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Nack::B0x1
    }
}
#[doc = "Field `NACK` writer - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG, Nack>;
impl<'a, REG> NackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nack::B0x0)
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nack::B0x1)
    }
}
#[doc = "Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scen {
    #[doc = "0: Smartcard mode disabled"]
    B0x0 = 0,
    #[doc = "1: Smartcard mode enabled"]
    B0x1 = 1,
}
impl From<Scen> for bool {
    #[inline(always)]
    fn from(variant: Scen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCEN` reader - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type ScenR = crate::BitReader<Scen>;
impl ScenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scen {
        match self.bits {
            false => Scen::B0x0,
            true => Scen::B0x1,
        }
    }
    #[doc = "Smartcard mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Scen::B0x0
    }
    #[doc = "Smartcard mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Scen::B0x1
    }
}
#[doc = "Field `SCEN` writer - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type ScenW<'a, REG> = crate::BitWriter<'a, REG, Scen>;
impl<'a, REG> ScenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Smartcard mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Scen::B0x0)
    }
    #[doc = "Smartcard mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Scen::B0x1)
    }
}
#[doc = "DMA enable receiver This bit is set/reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmar {
    #[doc = "1: DMA mode is enabled for reception"]
    B0x1 = 1,
    #[doc = "0: DMA mode is disabled for reception"]
    B0x0 = 0,
}
impl From<Dmar> for bool {
    #[inline(always)]
    fn from(variant: Dmar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAR` reader - DMA enable receiver This bit is set/reset by software"]
pub type DmarR = crate::BitReader<Dmar>;
impl DmarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmar {
        match self.bits {
            true => Dmar::B0x1,
            false => Dmar::B0x0,
        }
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmar::B0x1
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmar::B0x0
    }
}
#[doc = "Field `DMAR` writer - DMA enable receiver This bit is set/reset by software"]
pub type DmarW<'a, REG> = crate::BitWriter<'a, REG, Dmar>;
impl<'a, REG> DmarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmar::B0x1)
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmar::B0x0)
    }
}
#[doc = "DMA enable transmitter This bit is set/reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmat {
    #[doc = "1: DMA mode is enabled for transmission"]
    B0x1 = 1,
    #[doc = "0: DMA mode is disabled for transmission"]
    B0x0 = 0,
}
impl From<Dmat> for bool {
    #[inline(always)]
    fn from(variant: Dmat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAT` reader - DMA enable transmitter This bit is set/reset by software"]
pub type DmatR = crate::BitReader<Dmat>;
impl DmatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmat {
        match self.bits {
            true => Dmat::B0x1,
            false => Dmat::B0x0,
        }
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmat::B0x1
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmat::B0x0
    }
}
#[doc = "Field `DMAT` writer - DMA enable transmitter This bit is set/reset by software"]
pub type DmatW<'a, REG> = crate::BitWriter<'a, REG, Dmat>;
impl<'a, REG> DmatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmat::B0x1)
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmat::B0x0)
    }
}
#[doc = "RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtse {
    #[doc = "0: RTS hardware flow control disabled"]
    B0x0 = 0,
    #[doc = "1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The RTS output is deasserted (pulled to 0) when data can be received."]
    B0x1 = 1,
}
impl From<Rtse> for bool {
    #[inline(always)]
    fn from(variant: Rtse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSE` reader - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type RtseR = crate::BitReader<Rtse>;
impl RtseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtse {
        match self.bits {
            false => Rtse::B0x0,
            true => Rtse::B0x1,
        }
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtse::B0x0
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The RTS output is deasserted (pulled to 0) when data can be received."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtse::B0x1
    }
}
#[doc = "Field `RTSE` writer - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type RtseW<'a, REG> = crate::BitWriter<'a, REG, Rtse>;
impl<'a, REG> RtseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtse::B0x0)
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The RTS output is deasserted (pulled to 0) when data can be received."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtse::B0x1)
    }
}
#[doc = "CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctse {
    #[doc = "0: CTS hardware flow control disabled"]
    B0x0 = 0,
    #[doc = "1: CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0). If the CTS input is asserted while data is being transmitted, then the transmission is completed before stopping.If data is written into the data register while CTS is asserted, the transmission is postponed until CTS is deasserted."]
    B0x1 = 1,
}
impl From<Ctse> for bool {
    #[inline(always)]
    fn from(variant: Ctse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSE` reader - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type CtseR = crate::BitReader<Ctse>;
impl CtseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctse {
        match self.bits {
            false => Ctse::B0x0,
            true => Ctse::B0x1,
        }
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ctse::B0x0
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0). If the CTS input is asserted while data is being transmitted, then the transmission is completed before stopping.If data is written into the data register while CTS is asserted, the transmission is postponed until CTS is deasserted."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ctse::B0x1
    }
}
#[doc = "Field `CTSE` writer - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG, Ctse>;
impl<'a, REG> CtseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctse::B0x0)
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0). If the CTS input is asserted while data is being transmitted, then the transmission is completed before stopping.If data is written into the data register while CTS is asserted, the transmission is postponed until CTS is deasserted."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctse::B0x1)
    }
}
#[doc = "CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
    B0x1 = 1,
}
impl From<Ctsie> for bool {
    #[inline(always)]
    fn from(variant: Ctsie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type CtsieR = crate::BitReader<Ctsie>;
impl CtsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsie {
        match self.bits {
            false => Ctsie::B0x0,
            true => Ctsie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ctsie::B0x0
    }
    #[doc = "An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ctsie::B0x1
    }
}
#[doc = "Field `CTSIE` writer - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG, Ctsie>;
impl<'a, REG> CtsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsie::B0x0)
    }
    #[doc = "An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsie::B0x1)
    }
}
#[doc = "One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onebit {
    #[doc = "0: Three sample bit method"]
    B0x0 = 0,
    #[doc = "1: One sample bit method"]
    B0x1 = 1,
}
impl From<Onebit> for bool {
    #[inline(always)]
    fn from(variant: Onebit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONEBIT` reader - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0)."]
pub type OnebitR = crate::BitReader<Onebit>;
impl OnebitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onebit {
        match self.bits {
            false => Onebit::B0x0,
            true => Onebit::B0x1,
        }
    }
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Onebit::B0x0
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Onebit::B0x1
    }
}
#[doc = "Field `ONEBIT` writer - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0)."]
pub type OnebitW<'a, REG> = crate::BitWriter<'a, REG, Onebit>;
impl<'a, REG> OnebitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Onebit::B0x0)
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Onebit::B0x1)
    }
}
#[doc = "Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrdis {
    #[doc = "0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data."]
    B0x0 = 0,
    #[doc = "1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set"]
    B0x1 = 1,
}
impl From<Ovrdis> for bool {
    #[inline(always)]
    fn from(variant: Ovrdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRDIS` reader - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data"]
pub type OvrdisR = crate::BitReader<Ovrdis>;
impl OvrdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrdis {
        match self.bits {
            false => Ovrdis::B0x0,
            true => Ovrdis::B0x1,
        }
    }
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ovrdis::B0x0
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ovrdis::B0x1
    }
}
#[doc = "Field `OVRDIS` writer - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data"]
pub type OvrdisW<'a, REG> = crate::BitWriter<'a, REG, Ovrdis>;
impl<'a, REG> OvrdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrdis::B0x0)
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrdis::B0x1)
    }
}
#[doc = "DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddre {
    #[doc = "0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred. (used for Smartcard mode)"]
    B0x0 = 0,
    #[doc = "1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag."]
    B0x1 = 1,
}
impl From<Ddre> for bool {
    #[inline(always)]
    fn from(variant: Ddre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
pub type DdreR = crate::BitReader<Ddre>;
impl DdreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddre {
        match self.bits {
            false => Ddre::B0x0,
            true => Ddre::B0x1,
        }
    }
    #[doc = "DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred. (used for Smartcard mode)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ddre::B0x0
    }
    #[doc = "DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ddre::B0x1
    }
}
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
pub type DdreW<'a, REG> = crate::BitWriter<'a, REG, Ddre>;
impl<'a, REG> DdreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred. (used for Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ddre::B0x0)
    }
    #[doc = "DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ddre::B0x1)
    }
}
#[doc = "Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dem {
    #[doc = "0: DE function is disabled."]
    B0x0 = 0,
    #[doc = "1: DE function is enabled. The DE signal is output on the RTS pin."]
    B0x1 = 1,
}
impl From<Dem> for bool {
    #[inline(always)]
    fn from(variant: Dem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEM` reader - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826."]
pub type DemR = crate::BitReader<Dem>;
impl DemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dem {
        match self.bits {
            false => Dem::B0x0,
            true => Dem::B0x1,
        }
    }
    #[doc = "DE function is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dem::B0x0
    }
    #[doc = "DE function is enabled. The DE signal is output on the RTS pin."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dem::B0x1
    }
}
#[doc = "Field `DEM` writer - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826."]
pub type DemW<'a, REG> = crate::BitWriter<'a, REG, Dem>;
impl<'a, REG> DemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE function is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dem::B0x0)
    }
    #[doc = "DE function is enabled. The DE signal is output on the RTS pin."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dem::B0x1)
    }
}
#[doc = "Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dep {
    #[doc = "0: DE signal is active high."]
    B0x0 = 0,
    #[doc = "1: DE signal is active low."]
    B0x1 = 1,
}
impl From<Dep> for bool {
    #[inline(always)]
    fn from(variant: Dep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEP` reader - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type DepR = crate::BitReader<Dep>;
impl DepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dep {
        match self.bits {
            false => Dep::B0x0,
            true => Dep::B0x1,
        }
    }
    #[doc = "DE signal is active high."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dep::B0x0
    }
    #[doc = "DE signal is active low."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dep::B0x1
    }
}
#[doc = "Field `DEP` writer - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type DepW<'a, REG> = crate::BitWriter<'a, REG, Dep>;
impl<'a, REG> DepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE signal is active high."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dep::B0x0)
    }
    #[doc = "DE signal is active low."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dep::B0x1)
    }
}
#[doc = "Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scarcnt {
    #[doc = "0: retransmission disabled - No automatic retransmission in Transmission mode."]
    B0x0 = 0,
}
impl From<Scarcnt> for u8 {
    #[inline(always)]
    fn from(variant: Scarcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scarcnt {
    type Ux = u8;
}
impl crate::IsEnum for Scarcnt {}
#[doc = "Field `SCARCNT` reader - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type ScarcntR = crate::FieldReader<Scarcnt>;
impl ScarcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scarcnt> {
        match self.bits {
            0 => Some(Scarcnt::B0x0),
            _ => None,
        }
    }
    #[doc = "retransmission disabled - No automatic retransmission in Transmission mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Scarcnt::B0x0
    }
}
#[doc = "Field `SCARCNT` writer - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type ScarcntW<'a, REG> = crate::FieldWriter<'a, REG, 3, Scarcnt>;
impl<'a, REG> ScarcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "retransmission disabled - No automatic retransmission in Transmission mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Scarcnt::B0x0)
    }
}
#[doc = "Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wus {
    #[doc = "0: WUF active on address match (as defined by ADD\\[7:0\\]
and ADDM7)"]
    B0x0 = 0,
    #[doc = "1: Reserved."]
    B0x1 = 1,
    #[doc = "2: WUF active on start bit detection"]
    B0x2 = 2,
    #[doc = "3: WUF active on RXNE/RXFNE."]
    B0x3 = 3,
}
impl From<Wus> for u8 {
    #[inline(always)]
    fn from(variant: Wus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wus {
    type Ux = u8;
}
impl crate::IsEnum for Wus {}
#[doc = "Field `WUS` reader - Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type WusR = crate::FieldReader<Wus>;
impl WusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wus {
        match self.bits {
            0 => Wus::B0x0,
            1 => Wus::B0x1,
            2 => Wus::B0x2,
            3 => Wus::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "WUF active on address match (as defined by ADD\\[7:0\\]
and ADDM7)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wus::B0x0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wus::B0x1
    }
    #[doc = "WUF active on start bit detection"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Wus::B0x2
    }
    #[doc = "WUF active on RXNE/RXFNE."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Wus::B0x3
    }
}
#[doc = "Field `WUS` writer - Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type WusW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wus, crate::Safe>;
impl<'a, REG> WusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WUF active on address match (as defined by ADD\\[7:0\\]
and ADDM7)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wus::B0x0)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wus::B0x1)
    }
    #[doc = "WUF active on start bit detection"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Wus::B0x2)
    }
    #[doc = "WUF active on RXNE/RXFNE."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Wus::B0x3)
    }
}
#[doc = "Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wufie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated whenever WUF=1 in the USART_ISR register"]
    B0x1 = 1,
}
impl From<Wufie> for bool {
    #[inline(always)]
    fn from(variant: Wufie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUFIE` reader - Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type WufieR = crate::BitReader<Wufie>;
impl WufieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wufie {
        match self.bits {
            false => Wufie::B0x0,
            true => Wufie::B0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wufie::B0x0
    }
    #[doc = "USART interrupt generated whenever WUF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wufie::B0x1
    }
}
#[doc = "Field `WUFIE` writer - Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type WufieW<'a, REG> = crate::BitWriter<'a, REG, Wufie>;
impl<'a, REG> WufieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wufie::B0x0)
    }
    #[doc = "USART interrupt generated whenever WUF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wufie::B0x1)
    }
}
#[doc = "TXFIFO threshold interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txftie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    B0x1 = 1,
}
impl From<Txftie> for bool {
    #[inline(always)]
    fn from(variant: Txftie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFTIE` reader - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type TxftieR = crate::BitReader<Txftie>;
impl TxftieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txftie {
        match self.bits {
            false => Txftie::B0x0,
            true => Txftie::B0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txftie::B0x0
    }
    #[doc = "USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txftie::B0x1
    }
}
#[doc = "Field `TXFTIE` writer - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type TxftieW<'a, REG> = crate::BitWriter<'a, REG, Txftie>;
impl<'a, REG> TxftieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txftie::B0x0)
    }
    #[doc = "USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txftie::B0x1)
    }
}
#[doc = "Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcbgtie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated whenever TCBGT=1 in the USART_ISR register"]
    B0x1 = 1,
}
impl From<Tcbgtie> for bool {
    #[inline(always)]
    fn from(variant: Tcbgtie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCBGTIE` reader - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type TcbgtieR = crate::BitReader<Tcbgtie>;
impl TcbgtieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcbgtie {
        match self.bits {
            false => Tcbgtie::B0x0,
            true => Tcbgtie::B0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcbgtie::B0x0
    }
    #[doc = "USART interrupt generated whenever TCBGT=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcbgtie::B0x1
    }
}
#[doc = "Field `TCBGTIE` writer - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type TcbgtieW<'a, REG> = crate::BitWriter<'a, REG, Tcbgtie>;
impl<'a, REG> TcbgtieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcbgtie::B0x0)
    }
    #[doc = "USART interrupt generated whenever TCBGT=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcbgtie::B0x1)
    }
}
#[doc = "Receive FIFO threshold configuration Remaining combinations: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxftcfg {
    #[doc = "0: Receive FIFO reaches 1/8 of its depth"]
    B0x0 = 0,
    #[doc = "1: Receive FIFO reaches 1/4 of its depth"]
    B0x1 = 1,
    #[doc = "2: Receive FIFO reaches 1/2 of its depth"]
    B0x2 = 2,
    #[doc = "3: Receive FIFO reaches 3/4 of its depth"]
    B0x3 = 3,
    #[doc = "4: Receive FIFO reaches 7/8 of its depth"]
    B0x4 = 4,
    #[doc = "5: Receive FIFO becomes full"]
    B0x5 = 5,
}
impl From<Rxftcfg> for u8 {
    #[inline(always)]
    fn from(variant: Rxftcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxftcfg {
    type Ux = u8;
}
impl crate::IsEnum for Rxftcfg {}
#[doc = "Field `RXFTCFG` reader - Receive FIFO threshold configuration Remaining combinations: Reserved"]
pub type RxftcfgR = crate::FieldReader<Rxftcfg>;
impl RxftcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxftcfg> {
        match self.bits {
            0 => Some(Rxftcfg::B0x0),
            1 => Some(Rxftcfg::B0x1),
            2 => Some(Rxftcfg::B0x2),
            3 => Some(Rxftcfg::B0x3),
            4 => Some(Rxftcfg::B0x4),
            5 => Some(Rxftcfg::B0x5),
            _ => None,
        }
    }
    #[doc = "Receive FIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxftcfg::B0x0
    }
    #[doc = "Receive FIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxftcfg::B0x1
    }
    #[doc = "Receive FIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Rxftcfg::B0x2
    }
    #[doc = "Receive FIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Rxftcfg::B0x3
    }
    #[doc = "Receive FIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Rxftcfg::B0x4
    }
    #[doc = "Receive FIFO becomes full"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Rxftcfg::B0x5
    }
}
#[doc = "Field `RXFTCFG` writer - Receive FIFO threshold configuration Remaining combinations: Reserved"]
pub type RxftcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxftcfg>;
impl<'a, REG> RxftcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive FIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxftcfg::B0x0)
    }
    #[doc = "Receive FIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxftcfg::B0x1)
    }
    #[doc = "Receive FIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxftcfg::B0x2)
    }
    #[doc = "Receive FIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxftcfg::B0x3)
    }
    #[doc = "Receive FIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxftcfg::B0x4)
    }
    #[doc = "Receive FIFO becomes full"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Rxftcfg::B0x5)
    }
}
#[doc = "RXFIFO threshold interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxftie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    B0x1 = 1,
}
impl From<Rxftie> for bool {
    #[inline(always)]
    fn from(variant: Rxftie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFTIE` reader - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type RxftieR = crate::BitReader<Rxftie>;
impl RxftieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxftie {
        match self.bits {
            false => Rxftie::B0x0,
            true => Rxftie::B0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxftie::B0x0
    }
    #[doc = "USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxftie::B0x1
    }
}
#[doc = "Field `RXFTIE` writer - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type RxftieW<'a, REG> = crate::BitWriter<'a, REG, Rxftie>;
impl<'a, REG> RxftieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxftie::B0x0)
    }
    #[doc = "USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxftie::B0x1)
    }
}
#[doc = "TXFIFO threshold configuration Remaining combinations: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txftcfg {
    #[doc = "0: TXFIFO reaches 1/8 of its depth"]
    B0x0 = 0,
    #[doc = "1: TXFIFO reaches 1/4 of its depth"]
    B0x1 = 1,
    #[doc = "2: TXFIFO reaches 1/2 of its depth"]
    B0x2 = 2,
    #[doc = "3: TXFIFO reaches 3/4 of its depth"]
    B0x3 = 3,
    #[doc = "4: TXFIFO reaches 7/8 of its depth"]
    B0x4 = 4,
    #[doc = "5: TXFIFO becomes empty"]
    B0x5 = 5,
}
impl From<Txftcfg> for u8 {
    #[inline(always)]
    fn from(variant: Txftcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txftcfg {
    type Ux = u8;
}
impl crate::IsEnum for Txftcfg {}
#[doc = "Field `TXFTCFG` reader - TXFIFO threshold configuration Remaining combinations: Reserved"]
pub type TxftcfgR = crate::FieldReader<Txftcfg>;
impl TxftcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txftcfg> {
        match self.bits {
            0 => Some(Txftcfg::B0x0),
            1 => Some(Txftcfg::B0x1),
            2 => Some(Txftcfg::B0x2),
            3 => Some(Txftcfg::B0x3),
            4 => Some(Txftcfg::B0x4),
            5 => Some(Txftcfg::B0x5),
            _ => None,
        }
    }
    #[doc = "TXFIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txftcfg::B0x0
    }
    #[doc = "TXFIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txftcfg::B0x1
    }
    #[doc = "TXFIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Txftcfg::B0x2
    }
    #[doc = "TXFIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Txftcfg::B0x3
    }
    #[doc = "TXFIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Txftcfg::B0x4
    }
    #[doc = "TXFIFO becomes empty"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Txftcfg::B0x5
    }
}
#[doc = "Field `TXFTCFG` writer - TXFIFO threshold configuration Remaining combinations: Reserved"]
pub type TxftcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txftcfg>;
impl<'a, REG> TxftcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TXFIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txftcfg::B0x0)
    }
    #[doc = "TXFIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txftcfg::B0x1)
    }
    #[doc = "TXFIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Txftcfg::B0x2)
    }
    #[doc = "TXFIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Txftcfg::B0x3)
    }
    #[doc = "TXFIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Txftcfg::B0x4)
    }
    #[doc = "TXFIFO becomes empty"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Txftcfg::B0x5)
    }
}
impl R {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register)."]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn irlp(&self) -> IrlpR {
        IrlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn scen(&self) -> ScenR {
        ScenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmar(&self) -> DmarR {
        DmarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmat(&self) -> DmatR {
        DmatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn rtse(&self) -> RtseR {
        RtseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn onebit(&self) -> OnebitR {
        OnebitR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OvrdisR {
        OvrdisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    pub fn ddre(&self) -> DdreR {
        DdreR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn dem(&self) -> DemR {
        DemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn dep(&self) -> DepR {
        DepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn scarcnt(&self) -> ScarcntR {
        ScarcntR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn wus(&self) -> WusR {
        WusR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn wufie(&self) -> WufieR {
        WufieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txftie(&self) -> TxftieR {
        TxftieR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn tcbgtie(&self) -> TcbgtieR {
        TcbgtieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn rxftcfg(&self) -> RxftcfgR {
        RxftcfgR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxftie(&self) -> RxftieR {
        RxftieR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn txftcfg(&self) -> TxftcfgR {
        TxftcfgR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1or UDR = 1 in the USART_ISR register)."]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EieW<UsartCr3Spec> {
        EieW::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IrenW<UsartCr3Spec> {
        IrenW::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IrlpW<UsartCr3Spec> {
        IrlpW::new(self, 2)
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HdselW<UsartCr3Spec> {
        HdselW::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<UsartCr3Spec> {
        NackW::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> ScenW<UsartCr3Spec> {
        ScenW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA enable receiver This bit is set/reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DmarW<UsartCr3Spec> {
        DmarW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter This bit is set/reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DmatW<UsartCr3Spec> {
        DmatW::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn rtse(&mut self) -> RtseW<UsartCr3Spec> {
        RtseW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CtseW<UsartCr3Spec> {
        CtseW::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CtsieW<UsartCr3Spec> {
        CtsieW::new(self, 10)
    }
    #[doc = "Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> OnebitW<UsartCr3Spec> {
        OnebitW::new(self, 11)
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data are written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data"]
    #[inline(always)]
    #[must_use]
    pub fn ovrdis(&mut self) -> OvrdisW<UsartCr3Spec> {
        OvrdisW::new(self, 12)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    #[must_use]
    pub fn ddre(&mut self) -> DdreW<UsartCr3Spec> {
        DdreW::new(self, 13)
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn dem(&mut self) -> DemW<UsartCr3Spec> {
        DemW::new(self, 14)
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DepW<UsartCr3Spec> {
        DepW::new(self, 15)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In Transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In Reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn scarcnt(&mut self) -> ScarcntW<UsartCr3Spec> {
        ScarcntW::new(self, 17)
    }
    #[doc = "Bits 20:21 - Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn wus(&mut self) -> WusW<UsartCr3Spec> {
        WusW::new(self, 20)
    }
    #[doc = "Bit 22 - Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn wufie(&mut self) -> WufieW<UsartCr3Spec> {
        WufieW::new(self, 22)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txftie(&mut self) -> TxftieW<UsartCr3Spec> {
        TxftieW::new(self, 23)
    }
    #[doc = "Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn tcbgtie(&mut self) -> TcbgtieW<UsartCr3Spec> {
        TcbgtieW::new(self, 24)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rxftcfg(&mut self) -> RxftcfgW<UsartCr3Spec> {
        RxftcfgW::new(self, 25)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxftie(&mut self) -> RxftieW<UsartCr3Spec> {
        RxftieW::new(self, 28)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn txftcfg(&mut self) -> TxftcfgW<UsartCr3Spec> {
        TxftcfgW::new(self, 29)
    }
}
#[doc = "USART control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartCr3Spec;
impl crate::RegisterSpec for UsartCr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_cr3::R`](R) reader structure"]
impl crate::Readable for UsartCr3Spec {}
#[doc = "`write(|w| ..)` method takes [`usart_cr3::W`](W) writer structure"]
impl crate::Writable for UsartCr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_CR3 to value 0"]
impl crate::Resettable for UsartCr3Spec {
    const RESET_VALUE: u32 = 0;
}
