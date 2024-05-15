#[doc = "Register `LPUART_CR1` reader"]
pub type R = crate::R<LpuartCr1Spec>;
#[doc = "Register `LPUART_CR1` writer"]
pub type W = crate::W<LpuartCr1Spec>;
#[doc = "LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ue {
    #[doc = "0: LPUART prescaler and outputs disabled, low-power mode"]
    B0x0 = 0,
    #[doc = "1: LPUART enabled"]
    B0x1 = 1,
}
impl From<Ue> for bool {
    #[inline(always)]
    fn from(variant: Ue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UE` reader - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit."]
pub type UeR = crate::BitReader<Ue>;
impl UeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ue {
        match self.bits {
            false => Ue::B0x0,
            true => Ue::B0x1,
        }
    }
    #[doc = "LPUART prescaler and outputs disabled, low-power mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ue::B0x0
    }
    #[doc = "LPUART enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ue::B0x1
    }
}
#[doc = "Field `UE` writer - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit."]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG, Ue>;
impl<'a, REG> UeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART prescaler and outputs disabled, low-power mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ue::B0x0)
    }
    #[doc = "LPUART enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ue::B0x1)
    }
}
#[doc = "LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uesm {
    #[doc = "0: USART not able to wake up the MCU from low-power mode."]
    B0x0 = 0,
    #[doc = "1: USART able to wake up the MCU from low-power mode."]
    B0x1 = 1,
}
impl From<Uesm> for bool {
    #[inline(always)]
    fn from(variant: Uesm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UESM` reader - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
pub type UesmR = crate::BitReader<Uesm>;
impl UesmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uesm {
        match self.bits {
            false => Uesm::B0x0,
            true => Uesm::B0x1,
        }
    }
    #[doc = "USART not able to wake up the MCU from low-power mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uesm::B0x0
    }
    #[doc = "USART able to wake up the MCU from low-power mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uesm::B0x1
    }
}
#[doc = "Field `UESM` writer - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
pub type UesmW<'a, REG> = crate::BitWriter<'a, REG, Uesm>;
impl<'a, REG> UesmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART not able to wake up the MCU from low-power mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uesm::B0x0)
    }
    #[doc = "USART able to wake up the MCU from low-power mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uesm::B0x1)
    }
}
#[doc = "Receiver enable This bit enables the receiver. It is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Re {
    #[doc = "0: Receiver is disabled"]
    B0x0 = 0,
    #[doc = "1: Receiver is enabled and begins searching for a start bit"]
    B0x1 = 1,
}
impl From<Re> for bool {
    #[inline(always)]
    fn from(variant: Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Receiver enable This bit enables the receiver. It is set and cleared by software."]
pub type ReR = crate::BitReader<Re>;
impl ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Re {
        match self.bits {
            false => Re::B0x0,
            true => Re::B0x1,
        }
    }
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Re::B0x0
    }
    #[doc = "Receiver is enabled and begins searching for a start bit"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Re::B0x1
    }
}
#[doc = "Field `RE` writer - Receiver enable This bit enables the receiver. It is set and cleared by software."]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG, Re>;
impl<'a, REG> ReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Re::B0x0)
    }
    #[doc = "Receiver is enabled and begins searching for a start bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Re::B0x1)
    }
}
#[doc = "Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    #[doc = "0: Transmitter is disabled"]
    B0x0 = 0,
    #[doc = "1: Transmitter is enabled"]
    B0x1 = 1,
}
impl From<Te> for bool {
    #[inline(always)]
    fn from(variant: Te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
pub type TeR = crate::BitReader<Te>;
impl TeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te {
        match self.bits {
            false => Te::B0x0,
            true => Te::B0x1,
        }
    }
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Te::B0x0
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Te::B0x1
    }
}
#[doc = "Field `TE` writer - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG, Te>;
impl<'a, REG> TeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Te::B0x0)
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Te::B0x1)
    }
}
#[doc = "IDLE interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idleie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: An LPUART interrupt is generated whenever IDLE=1 in the LPUART_ISR register"]
    B0x1 = 1,
}
impl From<Idleie> for bool {
    #[inline(always)]
    fn from(variant: Idleie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable This bit is set and cleared by software."]
pub type IdleieR = crate::BitReader<Idleie>;
impl IdleieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idleie {
        match self.bits {
            false => Idleie::B0x0,
            true => Idleie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Idleie::B0x0
    }
    #[doc = "An LPUART interrupt is generated whenever IDLE=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Idleie::B0x1
    }
}
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable This bit is set and cleared by software."]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG, Idleie>;
impl<'a, REG> IdleieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Idleie::B0x0)
    }
    #[doc = "An LPUART interrupt is generated whenever IDLE=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Idleie::B0x1)
    }
}
#[doc = "RXFIFO not empty interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfneie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: A LPUART interrupt is generated whenever ORE=1 or RXFNE=1 in the LPUART_ISR register"]
    B0x1 = 1,
}
impl From<Rxfneie> for bool {
    #[inline(always)]
    fn from(variant: Rxfneie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFNEIE` reader - RXFIFO not empty interrupt enable This bit is set and cleared by software."]
pub type RxfneieR = crate::BitReader<Rxfneie>;
impl RxfneieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfneie {
        match self.bits {
            false => Rxfneie::B0x0,
            true => Rxfneie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxfneie::B0x0
    }
    #[doc = "A LPUART interrupt is generated whenever ORE=1 or RXFNE=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxfneie::B0x1
    }
}
#[doc = "Field `RXFNEIE` writer - RXFIFO not empty interrupt enable This bit is set and cleared by software."]
pub type RxfneieW<'a, REG> = crate::BitWriter<'a, REG, Rxfneie>;
impl<'a, REG> RxfneieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfneie::B0x0)
    }
    #[doc = "A LPUART interrupt is generated whenever ORE=1 or RXFNE=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfneie::B0x1)
    }
}
#[doc = "Transmission complete interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: An LPUART interrupt is generated whenever TC=1 in the LPUART_ISR register"]
    B0x1 = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable This bit is set and cleared by software."]
pub type TcieR = crate::BitReader<Tcie>;
impl TcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcie {
        match self.bits {
            false => Tcie::B0x0,
            true => Tcie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcie::B0x0
    }
    #[doc = "An LPUART interrupt is generated whenever TC=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcie::B0x1
    }
}
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable This bit is set and cleared by software."]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x0)
    }
    #[doc = "An LPUART interrupt is generated whenever TC=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x1)
    }
}
#[doc = "TXFIFO not full interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfnfie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: A LPUART interrupt is generated whenever TXFNF =1 in the LPUART_ISR register"]
    B0x1 = 1,
}
impl From<Txfnfie> for bool {
    #[inline(always)]
    fn from(variant: Txfnfie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFNFIE` reader - TXFIFO not full interrupt enable This bit is set and cleared by software."]
pub type TxfnfieR = crate::BitReader<Txfnfie>;
impl TxfnfieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfnfie {
        match self.bits {
            false => Txfnfie::B0x0,
            true => Txfnfie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txfnfie::B0x0
    }
    #[doc = "A LPUART interrupt is generated whenever TXFNF =1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txfnfie::B0x1
    }
}
#[doc = "Field `TXFNFIE` writer - TXFIFO not full interrupt enable This bit is set and cleared by software."]
pub type TxfnfieW<'a, REG> = crate::BitWriter<'a, REG, Txfnfie>;
impl<'a, REG> TxfnfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnfie::B0x0)
    }
    #[doc = "A LPUART interrupt is generated whenever TXFNF =1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnfie::B0x1)
    }
}
#[doc = "PE interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: An LPUART interrupt is generated whenever PE=1 in the LPUART_ISR register"]
    B0x1 = 1,
}
impl From<Peie> for bool {
    #[inline(always)]
    fn from(variant: Peie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIE` reader - PE interrupt enable This bit is set and cleared by software."]
pub type PeieR = crate::BitReader<Peie>;
impl PeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peie {
        match self.bits {
            false => Peie::B0x0,
            true => Peie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Peie::B0x0
    }
    #[doc = "An LPUART interrupt is generated whenever PE=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Peie::B0x1
    }
}
#[doc = "Field `PEIE` writer - PE interrupt enable This bit is set and cleared by software."]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG, Peie>;
impl<'a, REG> PeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::B0x0)
    }
    #[doc = "An LPUART interrupt is generated whenever PE=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::B0x1)
    }
}
#[doc = "Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ps {
    #[doc = "0: Even parity"]
    B0x0 = 0,
    #[doc = "1: Odd parity"]
    B0x1 = 1,
}
impl From<Ps> for bool {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type PsR = crate::BitReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            false => Ps::B0x0,
            true => Ps::B0x1,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ps::B0x0
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ps::B0x1
    }
}
#[doc = "Field `PS` writer - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG, Ps>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B0x0)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B0x1)
    }
}
#[doc = "Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pce {
    #[doc = "0: Parity control disabled"]
    B0x0 = 0,
    #[doc = "1: Parity control enabled"]
    B0x1 = 1,
}
impl From<Pce> for bool {
    #[inline(always)]
    fn from(variant: Pce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCE` reader - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type PceR = crate::BitReader<Pce>;
impl PceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pce {
        match self.bits {
            false => Pce::B0x0,
            true => Pce::B0x1,
        }
    }
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pce::B0x0
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pce::B0x1
    }
}
#[doc = "Field `PCE` writer - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG, Pce>;
impl<'a, REG> PceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::B0x0)
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::B0x1)
    }
}
#[doc = "Receiver wake-up method This bit determines the LPUART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wake {
    #[doc = "0: Idle line"]
    B0x0 = 0,
    #[doc = "1: Address mark"]
    B0x1 = 1,
}
impl From<Wake> for bool {
    #[inline(always)]
    fn from(variant: Wake) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKE` reader - Receiver wake-up method This bit determines the LPUART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type WakeR = crate::BitReader<Wake>;
impl WakeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wake {
        match self.bits {
            false => Wake::B0x0,
            true => Wake::B0x1,
        }
    }
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wake::B0x0
    }
    #[doc = "Address mark"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wake::B0x1
    }
}
#[doc = "Field `WAKE` writer - Receiver wake-up method This bit determines the LPUART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG, Wake>;
impl<'a, REG> WakeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wake::B0x0)
    }
    #[doc = "Address mark"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wake::B0x1)
    }
}
#[doc = "Field `M0` reader - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0)."]
pub type M0R = crate::BitReader;
#[doc = "Field `M0` writer - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0)."]
pub type M0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mme {
    #[doc = "0: Receiver in Active mode permanently"]
    B0x0 = 0,
    #[doc = "1: Receiver can switch between Mute mode and Active mode."]
    B0x1 = 1,
}
impl From<Mme> for bool {
    #[inline(always)]
    fn from(variant: Mme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MME` reader - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software."]
pub type MmeR = crate::BitReader<Mme>;
impl MmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mme {
        match self.bits {
            false => Mme::B0x0,
            true => Mme::B0x1,
        }
    }
    #[doc = "Receiver in Active mode permanently"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mme::B0x0
    }
    #[doc = "Receiver can switch between Mute mode and Active mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mme::B0x1
    }
}
#[doc = "Field `MME` writer - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software."]
pub type MmeW<'a, REG> = crate::BitWriter<'a, REG, Mme>;
impl<'a, REG> MmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver in Active mode permanently"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mme::B0x0)
    }
    #[doc = "Receiver can switch between Mute mode and Active mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mme::B0x1)
    }
}
#[doc = "Character match interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: A LPUART interrupt is generated when the CMF bit is set in the LPUART_ISR register."]
    B0x1 = 1,
}
impl From<Cmie> for bool {
    #[inline(always)]
    fn from(variant: Cmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMIE` reader - Character match interrupt enable This bit is set and cleared by software."]
pub type CmieR = crate::BitReader<Cmie>;
impl CmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmie {
        match self.bits {
            false => Cmie::B0x0,
            true => Cmie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cmie::B0x0
    }
    #[doc = "A LPUART interrupt is generated when the CMF bit is set in the LPUART_ISR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cmie::B0x1
    }
}
#[doc = "Field `CMIE` writer - Character match interrupt enable This bit is set and cleared by software."]
pub type CmieW<'a, REG> = crate::BitWriter<'a, REG, Cmie>;
impl<'a, REG> CmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmie::B0x0)
    }
    #[doc = "A LPUART interrupt is generated when the CMF bit is set in the LPUART_ISR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmie::B0x1)
    }
}
#[doc = "Field `DEDT` reader - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section132.4.14: RS232 Hardware flow control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DedtR = crate::FieldReader;
#[doc = "Field `DEDT` writer - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section132.4.14: RS232 Hardware flow control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DedtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEAT` reader - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section131.5.21: RS232 Hardware flow control and RS485 Driver Enable. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DeatR = crate::FieldReader;
#[doc = "Field `DEAT` writer - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section131.5.21: RS232 Hardware flow control and RS485 Driver Enable. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DeatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `M1` reader - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= 00: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= 01: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
pub type M1R = crate::BitReader;
#[doc = "Field `M1` writer - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= 00: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= 01: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
pub type M1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FIFO mode enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoen {
    #[doc = "0: FIFO mode is disabled."]
    B0x0 = 0,
    #[doc = "1: FIFO mode is enabled."]
    B0x1 = 1,
}
impl From<Fifoen> for bool {
    #[inline(always)]
    fn from(variant: Fifoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` reader - FIFO mode enable This bit is set and cleared by software."]
pub type FifoenR = crate::BitReader<Fifoen>;
impl FifoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoen {
        match self.bits {
            false => Fifoen::B0x0,
            true => Fifoen::B0x1,
        }
    }
    #[doc = "FIFO mode is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fifoen::B0x0
    }
    #[doc = "FIFO mode is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fifoen::B0x1
    }
}
#[doc = "Field `FIFOEN` writer - FIFO mode enable This bit is set and cleared by software."]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG, Fifoen>;
impl<'a, REG> FifoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO mode is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::B0x0)
    }
    #[doc = "FIFO mode is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::B0x1)
    }
}
#[doc = "TXFIFO empty interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfeie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: An LPUART interrupt is generated when TXFE=1 in the LPUART_ISR register"]
    B0x1 = 1,
}
impl From<Txfeie> for bool {
    #[inline(always)]
    fn from(variant: Txfeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFEIE` reader - TXFIFO empty interrupt enable This bit is set and cleared by software."]
pub type TxfeieR = crate::BitReader<Txfeie>;
impl TxfeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfeie {
        match self.bits {
            false => Txfeie::B0x0,
            true => Txfeie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txfeie::B0x0
    }
    #[doc = "An LPUART interrupt is generated when TXFE=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txfeie::B0x1
    }
}
#[doc = "Field `TXFEIE` writer - TXFIFO empty interrupt enable This bit is set and cleared by software."]
pub type TxfeieW<'a, REG> = crate::BitWriter<'a, REG, Txfeie>;
impl<'a, REG> TxfeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txfeie::B0x0)
    }
    #[doc = "An LPUART interrupt is generated when TXFE=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txfeie::B0x1)
    }
}
#[doc = "RXFIFO Full interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxffie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: An LPUART interrupt is generated when RXFF=1 in the LPUART_ISR register"]
    B0x1 = 1,
}
impl From<Rxffie> for bool {
    #[inline(always)]
    fn from(variant: Rxffie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFFIE` reader - RXFIFO Full interrupt enable This bit is set and cleared by software."]
pub type RxffieR = crate::BitReader<Rxffie>;
impl RxffieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxffie {
        match self.bits {
            false => Rxffie::B0x0,
            true => Rxffie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxffie::B0x0
    }
    #[doc = "An LPUART interrupt is generated when RXFF=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxffie::B0x1
    }
}
#[doc = "Field `RXFFIE` writer - RXFIFO Full interrupt enable This bit is set and cleared by software."]
pub type RxffieW<'a, REG> = crate::BitWriter<'a, REG, Rxffie>;
impl<'a, REG> RxffieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxffie::B0x0)
    }
    #[doc = "An LPUART interrupt is generated when RXFF=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxffie::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit."]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
    #[inline(always)]
    pub fn uesm(&self) -> UesmR {
        UesmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software."]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFIFO not empty interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxfneie(&self) -> RxfneieR {
        RxfneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFIFO not full interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txfnfie(&self) -> TxfnfieR {
        TxfnfieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wake-up method This bit determines the LPUART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn m0(&self) -> M0R {
        M0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software."]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cmie(&self) -> CmieR {
        CmieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section132.4.14: RS232 Hardware flow control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn dedt(&self) -> DedtR {
        DedtR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section131.5.21: RS232 Hardware flow control and RS485 Driver Enable. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn deat(&self) -> DeatR {
        DeatR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= 00: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= 01: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
    #[inline(always)]
    pub fn m1(&self) -> M1R {
        M1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFO mode enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TXFIFO empty interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txfeie(&self) -> TxfeieR {
        TxfeieR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RXFIFO Full interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxffie(&self) -> RxffieR {
        RxffieR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit."]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UeW<LpuartCr1Spec> {
        UeW::new(self, 0)
    }
    #[doc = "Bit 1 - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UesmW<LpuartCr1Spec> {
        UesmW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<LpuartCr1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<LpuartCr1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IdleieW<LpuartCr1Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - RXFIFO not empty interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxfneie(&mut self) -> RxfneieW<LpuartCr1Spec> {
        RxfneieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<LpuartCr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - TXFIFO not full interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txfnfie(&mut self) -> TxfnfieW<LpuartCr1Spec> {
        TxfnfieW::new(self, 7)
    }
    #[doc = "Bit 8 - PE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PeieW<LpuartCr1Spec> {
        PeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<LpuartCr1Spec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PceW<LpuartCr1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 11 - Receiver wake-up method This bit determines the LPUART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WakeW<LpuartCr1Spec> {
        WakeW::new(self, 11)
    }
    #[doc = "Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0W<LpuartCr1Spec> {
        M0W::new(self, 12)
    }
    #[doc = "Bit 13 - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MmeW<LpuartCr1Spec> {
        MmeW::new(self, 13)
    }
    #[doc = "Bit 14 - Character match interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CmieW<LpuartCr1Spec> {
        CmieW::new(self, 14)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section132.4.14: RS232 Hardware flow control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn dedt(&mut self) -> DedtW<LpuartCr1Spec> {
        DedtW::new(self, 16)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section131.5.21: RS232 Hardware flow control and RS485 Driver Enable. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn deat(&mut self) -> DeatW<LpuartCr1Spec> {
        DeatW::new(self, 21)
    }
    #[doc = "Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= 00: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= 01: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1W<LpuartCr1Spec> {
        M1W::new(self, 28)
    }
    #[doc = "Bit 29 - FIFO mode enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FifoenW<LpuartCr1Spec> {
        FifoenW::new(self, 29)
    }
    #[doc = "Bit 30 - TXFIFO empty interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txfeie(&mut self) -> TxfeieW<LpuartCr1Spec> {
        TxfeieW::new(self, 30)
    }
    #[doc = "Bit 31 - RXFIFO Full interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxffie(&mut self) -> RxffieW<LpuartCr1Spec> {
        RxffieW::new(self, 31)
    }
}
#[doc = "LPUART control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpuartCr1Spec;
impl crate::RegisterSpec for LpuartCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpuart_cr1::R`](R) reader structure"]
impl crate::Readable for LpuartCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`lpuart_cr1::W`](W) writer structure"]
impl crate::Writable for LpuartCr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPUART_CR1 to value 0"]
impl crate::Resettable for LpuartCr1Spec {
    const RESET_VALUE: u32 = 0;
}
