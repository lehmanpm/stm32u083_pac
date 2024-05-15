#[doc = "Register `USART_CR1_ALTERNATE` reader"]
pub type R = crate::R<UsartCr1AlternateSpec>;
#[doc = "Register `USART_CR1_ALTERNATE` writer"]
pub type W = crate::W<UsartCr1AlternateSpec>;
#[doc = "USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ue {
    #[doc = "0: USART prescaler and outputs disabled, low-power mode"]
    B0x0 = 0,
    #[doc = "1: USART enabled"]
    B0x1 = 1,
}
impl From<Ue> for bool {
    #[inline(always)]
    fn from(variant: Ue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UE` reader - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value."]
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
    #[doc = "USART prescaler and outputs disabled, low-power mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ue::B0x0
    }
    #[doc = "USART enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ue::B0x1
    }
}
#[doc = "Field `UE` writer - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value."]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG, Ue>;
impl<'a, REG> UeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART prescaler and outputs disabled, low-power mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ue::B0x0)
    }
    #[doc = "USART enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ue::B0x1)
    }
}
#[doc = "USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode.\n\nValue on reset: 0"]
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
#[doc = "Field `UESM` reader - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
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
#[doc = "Field `UESM` writer - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
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
#[doc = "Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.\n\nValue on reset: 0"]
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
#[doc = "Field `TE` reader - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
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
#[doc = "Field `TE` writer - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
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
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated whenever IDLE=1 in the USART_ISR register"]
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
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Idleie::B0x0
    }
    #[doc = "USART interrupt generated whenever IDLE=1 in the USART_ISR register"]
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
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Idleie::B0x0)
    }
    #[doc = "USART interrupt generated whenever IDLE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Idleie::B0x1)
    }
}
#[doc = "Receive data register not empty This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxneie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated whenever ORE=1 or RXNE=1 in the USART_ISR register"]
    B0x1 = 1,
}
impl From<Rxneie> for bool {
    #[inline(always)]
    fn from(variant: Rxneie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - Receive data register not empty This bit is set and cleared by software."]
pub type RxneieR = crate::BitReader<Rxneie>;
impl RxneieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxneie {
        match self.bits {
            false => Rxneie::B0x0,
            true => Rxneie::B0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxneie::B0x0
    }
    #[doc = "USART interrupt generated whenever ORE=1 or RXNE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxneie::B0x1
    }
}
#[doc = "Field `RXNEIE` writer - Receive data register not empty This bit is set and cleared by software."]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG, Rxneie>;
impl<'a, REG> RxneieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxneie::B0x0)
    }
    #[doc = "USART interrupt generated whenever ORE=1 or RXNE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxneie::B0x1)
    }
}
#[doc = "Transmission complete interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated whenever TC=1 in the USART_ISR register"]
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
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcie::B0x0
    }
    #[doc = "USART interrupt generated whenever TC=1 in the USART_ISR register"]
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
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x0)
    }
    #[doc = "USART interrupt generated whenever TC=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x1)
    }
}
#[doc = "Transmit data register empty This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txeie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated whenever TXE =1 in the USART_ISR register"]
    B0x1 = 1,
}
impl From<Txeie> for bool {
    #[inline(always)]
    fn from(variant: Txeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - Transmit data register empty This bit is set and cleared by software."]
pub type TxeieR = crate::BitReader<Txeie>;
impl TxeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txeie {
        match self.bits {
            false => Txeie::B0x0,
            true => Txeie::B0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txeie::B0x0
    }
    #[doc = "USART interrupt generated whenever TXE =1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txeie::B0x1
    }
}
#[doc = "Field `TXEIE` writer - Transmit data register empty This bit is set and cleared by software."]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG, Txeie>;
impl<'a, REG> TxeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txeie::B0x0)
    }
    #[doc = "USART interrupt generated whenever TXE =1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txeie::B0x1)
    }
}
#[doc = "PE interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated whenever PE=1 in the USART_ISR register"]
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
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Peie::B0x0
    }
    #[doc = "USART interrupt generated whenever PE=1 in the USART_ISR register"]
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
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::B0x0)
    }
    #[doc = "USART interrupt generated whenever PE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::B0x1)
    }
}
#[doc = "Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
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
#[doc = "Field `PS` reader - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0)."]
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
#[doc = "Field `PS` writer - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0)."]
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
#[doc = "Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
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
#[doc = "Field `PCE` reader - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0)."]
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
#[doc = "Field `PCE` writer - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0)."]
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
#[doc = "Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
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
#[doc = "Field `WAKE` reader - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
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
#[doc = "Field `WAKE` writer - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
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
#[doc = "Field `M0` reader - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE=0)."]
pub type M0R = crate::BitReader;
#[doc = "Field `M0` writer - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE=0)."]
pub type M0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.\n\nValue on reset: 0"]
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
#[doc = "Field `MME` reader - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software."]
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
#[doc = "Field `MME` writer - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software."]
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
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated when the CMF bit is set in the USART_ISR register."]
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
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cmie::B0x0
    }
    #[doc = "USART interrupt generated when the CMF bit is set in the USART_ISR register."]
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
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmie::B0x0)
    }
    #[doc = "USART interrupt generated when the CMF bit is set in the USART_ISR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmie::B0x1)
    }
}
#[doc = "Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Over8 {
    #[doc = "0: Oversampling by 16"]
    B0x0 = 0,
    #[doc = "1: Oversampling by 8"]
    B0x1 = 1,
}
impl From<Over8> for bool {
    #[inline(always)]
    fn from(variant: Over8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVER8` reader - Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared."]
pub type Over8R = crate::BitReader<Over8>;
impl Over8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Over8 {
        match self.bits {
            false => Over8::B0x0,
            true => Over8::B0x1,
        }
    }
    #[doc = "Oversampling by 16"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Over8::B0x0
    }
    #[doc = "Oversampling by 8"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Over8::B0x1
    }
}
#[doc = "Field `OVER8` writer - Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared."]
pub type Over8W<'a, REG> = crate::BitWriter<'a, REG, Over8>;
impl<'a, REG> Over8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampling by 16"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Over8::B0x0)
    }
    #[doc = "Oversampling by 8"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Over8::B0x1)
    }
}
#[doc = "Field `DEDT` reader - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type DedtR = crate::FieldReader;
#[doc = "Field `DEDT` writer - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type DedtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEAT` reader - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type DeatR = crate::FieldReader;
#[doc = "Field `DEAT` writer - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type DeatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtoie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated when the RTOF bit is set in the USART_ISR register."]
    B0x1 = 1,
}
impl From<Rtoie> for bool {
    #[inline(always)]
    fn from(variant: Rtoie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOIE` reader - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826."]
pub type RtoieR = crate::BitReader<Rtoie>;
impl RtoieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtoie {
        match self.bits {
            false => Rtoie::B0x0,
            true => Rtoie::B0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtoie::B0x0
    }
    #[doc = "USART interrupt generated when the RTOF bit is set in the USART_ISR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtoie::B0x1
    }
}
#[doc = "Field `RTOIE` writer - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826."]
pub type RtoieW<'a, REG> = crate::BitWriter<'a, REG, Rtoie>;
impl<'a, REG> RtoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtoie::B0x0)
    }
    #[doc = "USART interrupt generated when the RTOF bit is set in the USART_ISR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtoie::B0x1)
    }
}
#[doc = "End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eobie {
    #[doc = "0: Interrupt inhibited"]
    B0x0 = 0,
    #[doc = "1: USART interrupt generated when the EOBF flag is set in the USART_ISR register"]
    B0x1 = 1,
}
impl From<Eobie> for bool {
    #[inline(always)]
    fn from(variant: Eobie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOBIE` reader - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type EobieR = crate::BitReader<Eobie>;
impl EobieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eobie {
        match self.bits {
            false => Eobie::B0x0,
            true => Eobie::B0x1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eobie::B0x0
    }
    #[doc = "USART interrupt generated when the EOBF flag is set in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eobie::B0x1
    }
}
#[doc = "Field `EOBIE` writer - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
pub type EobieW<'a, REG> = crate::BitWriter<'a, REG, Eobie>;
impl<'a, REG> EobieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eobie::B0x0)
    }
    #[doc = "USART interrupt generated when the EOBF flag is set in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eobie::B0x1)
    }
}
#[doc = "Field `M1` reader - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= 00: 1 start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= 01: 1 start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= 10: 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
pub type M1R = crate::BitReader;
#[doc = "Field `M1` writer - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= 00: 1 start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= 01: 1 start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= 10: 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
pub type M1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI Master/Slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.\n\nValue on reset: 0"]
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
#[doc = "Field `FIFOEN` reader - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI Master/Slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes."]
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
#[doc = "Field `FIFOEN` writer - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI Master/Slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes."]
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
impl R {
    #[doc = "Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value."]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
    #[inline(always)]
    pub fn uesm(&self) -> UesmR {
        UesmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software."]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive data register not empty This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn m0(&self) -> M0R {
        M0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software."]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cmie(&self) -> CmieR {
        CmieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared."]
    #[inline(always)]
    pub fn over8(&self) -> Over8R {
        Over8R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn dedt(&self) -> DedtR {
        DedtR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn deat(&self) -> DeatR {
        DeatR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn rtoie(&self) -> RtoieR {
        RtoieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    pub fn eobie(&self) -> EobieR {
        EobieR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= 00: 1 start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= 01: 1 start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= 10: 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
    #[inline(always)]
    pub fn m1(&self) -> M1R {
        M1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI Master/Slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes."]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value."]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UeW<UsartCr1AlternateSpec> {
        UeW::new(self, 0)
    }
    #[doc = "Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode."]
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UesmW<UsartCr1AlternateSpec> {
        UesmW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<UsartCr1AlternateSpec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<UsartCr1AlternateSpec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IdleieW<UsartCr1AlternateSpec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive data register not empty This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RxneieW<UsartCr1AlternateSpec> {
        RxneieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<UsartCr1AlternateSpec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit data register empty This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TxeieW<UsartCr1AlternateSpec> {
        TxeieW::new(self, 7)
    }
    #[doc = "Bit 8 - PE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PeieW<UsartCr1AlternateSpec> {
        PeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<UsartCr1AlternateSpec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PceW<UsartCr1AlternateSpec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 11 - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WakeW<UsartCr1AlternateSpec> {
        WakeW::new(self, 11)
    }
    #[doc = "Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0W<UsartCr1AlternateSpec> {
        M0W::new(self, 12)
    }
    #[doc = "Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MmeW<UsartCr1AlternateSpec> {
        MmeW::new(self, 13)
    }
    #[doc = "Bit 14 - Character match interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CmieW<UsartCr1AlternateSpec> {
        CmieW::new(self, 14)
    }
    #[doc = "Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared."]
    #[inline(always)]
    #[must_use]
    pub fn over8(&mut self) -> Over8W<UsartCr1AlternateSpec> {
        Over8W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn dedt(&mut self) -> DedtW<UsartCr1AlternateSpec> {
        DedtW::new(self, 16)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn deat(&mut self) -> DeatW<UsartCr1AlternateSpec> {
        DeatW::new(self, 21)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn rtoie(&mut self) -> RtoieW<UsartCr1AlternateSpec> {
        RtoieW::new(self, 26)
    }
    #[doc = "Bit 27 - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826."]
    #[inline(always)]
    #[must_use]
    pub fn eobie(&mut self) -> EobieW<UsartCr1AlternateSpec> {
        EobieW::new(self, 27)
    }
    #[doc = "Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= 00: 1 start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= 01: 1 start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= 10: 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported."]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1W<UsartCr1AlternateSpec> {
        M1W::new(self, 28)
    }
    #[doc = "Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI Master/Slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes."]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FifoenW<UsartCr1AlternateSpec> {
        FifoenW::new(self, 29)
    }
}
#[doc = "USART control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_cr1_alternate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_cr1_alternate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartCr1AlternateSpec;
impl crate::RegisterSpec for UsartCr1AlternateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_cr1_alternate::R`](R) reader structure"]
impl crate::Readable for UsartCr1AlternateSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_cr1_alternate::W`](W) writer structure"]
impl crate::Writable for UsartCr1AlternateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_CR1_ALTERNATE to value 0"]
impl crate::Resettable for UsartCr1AlternateSpec {
    const RESET_VALUE: u32 = 0;
}
