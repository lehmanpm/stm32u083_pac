#[doc = "Register `I2C_CR1` reader"]
pub type R = crate::R<I2cCr1Spec>;
#[doc = "Register `I2C_CR1` writer"]
pub type W = crate::W<I2cCr1Spec>;
#[doc = "Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Peripheral disable"]
    B0x0 = 0,
    #[doc = "1: Peripheral enable"]
    B0x1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles."]
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
    #[doc = "Peripheral disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pe::B0x0
    }
    #[doc = "Peripheral enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pe::B0x1
    }
}
#[doc = "Field `PE` writer - Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles."]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::B0x0)
    }
    #[doc = "Peripheral enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::B0x1)
    }
}
#[doc = "TX interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txie {
    #[doc = "0: Transmit (TXIS) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Transmit (TXIS) interrupt enabled"]
    B0x1 = 1,
}
impl From<Txie> for bool {
    #[inline(always)]
    fn from(variant: Txie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIE` reader - TX interrupt enable"]
pub type TxieR = crate::BitReader<Txie>;
impl TxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txie {
        match self.bits {
            false => Txie::B0x0,
            true => Txie::B0x1,
        }
    }
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txie::B0x0
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txie::B0x1
    }
}
#[doc = "Field `TXIE` writer - TX interrupt enable"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG, Txie>;
impl<'a, REG> TxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txie::B0x0)
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txie::B0x1)
    }
}
#[doc = "RX interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxie {
    #[doc = "0: Receive (RXNE) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Receive (RXNE) interrupt enabled"]
    B0x1 = 1,
}
impl From<Rxie> for bool {
    #[inline(always)]
    fn from(variant: Rxie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIE` reader - RX interrupt enable"]
pub type RxieR = crate::BitReader<Rxie>;
impl RxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxie {
        match self.bits {
            false => Rxie::B0x0,
            true => Rxie::B0x1,
        }
    }
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxie::B0x0
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxie::B0x1
    }
}
#[doc = "Field `RXIE` writer - RX interrupt enable"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG, Rxie>;
impl<'a, REG> RxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::B0x0)
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::B0x1)
    }
}
#[doc = "Address match interrupt enable (slave only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrie {
    #[doc = "0: Address match (ADDR) interrupts disabled"]
    B0x0 = 0,
    #[doc = "1: Address match (ADDR) interrupts enabled"]
    B0x1 = 1,
}
impl From<Addrie> for bool {
    #[inline(always)]
    fn from(variant: Addrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRIE` reader - Address match interrupt enable (slave only)"]
pub type AddrieR = crate::BitReader<Addrie>;
impl AddrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrie {
        match self.bits {
            false => Addrie::B0x0,
            true => Addrie::B0x1,
        }
    }
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Addrie::B0x0
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Addrie::B0x1
    }
}
#[doc = "Field `ADDRIE` writer - Address match interrupt enable (slave only)"]
pub type AddrieW<'a, REG> = crate::BitWriter<'a, REG, Addrie>;
impl<'a, REG> AddrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Addrie::B0x0)
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Addrie::B0x1)
    }
}
#[doc = "Not acknowledge received interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nackie {
    #[doc = "0: Not acknowledge (NACKF) received interrupts disabled"]
    B0x0 = 0,
    #[doc = "1: Not acknowledge (NACKF) received interrupts enabled"]
    B0x1 = 1,
}
impl From<Nackie> for bool {
    #[inline(always)]
    fn from(variant: Nackie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKIE` reader - Not acknowledge received interrupt enable"]
pub type NackieR = crate::BitReader<Nackie>;
impl NackieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nackie {
        match self.bits {
            false => Nackie::B0x0,
            true => Nackie::B0x1,
        }
    }
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Nackie::B0x0
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Nackie::B0x1
    }
}
#[doc = "Field `NACKIE` writer - Not acknowledge received interrupt enable"]
pub type NackieW<'a, REG> = crate::BitWriter<'a, REG, Nackie>;
impl<'a, REG> NackieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nackie::B0x0)
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nackie::B0x1)
    }
}
#[doc = "Stop detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopie {
    #[doc = "0: Stop detection (STOPF) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Stop detection (STOPF) interrupt enabled"]
    B0x1 = 1,
}
impl From<Stopie> for bool {
    #[inline(always)]
    fn from(variant: Stopie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIE` reader - Stop detection interrupt enable"]
pub type StopieR = crate::BitReader<Stopie>;
impl StopieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopie {
        match self.bits {
            false => Stopie::B0x0,
            true => Stopie::B0x1,
        }
    }
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Stopie::B0x0
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Stopie::B0x1
    }
}
#[doc = "Field `STOPIE` writer - Stop detection interrupt enable"]
pub type StopieW<'a, REG> = crate::BitWriter<'a, REG, Stopie>;
impl<'a, REG> StopieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Stopie::B0x0)
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Stopie::B0x1)
    }
}
#[doc = "Transfer complete interrupt enable Note: Any of these events generate an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: Transfer complete interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Transfer complete interrupt enabled"]
    B0x1 = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable Note: Any of these events generate an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)"]
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
    #[doc = "Transfer complete interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcie::B0x0
    }
    #[doc = "Transfer complete interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcie::B0x1
    }
}
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable Note: Any of these events generate an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer complete interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x0)
    }
    #[doc = "Transfer complete interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x1)
    }
}
#[doc = "Error interrupts enable Note: Any of these errors generate an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/Underrun (OVR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Error detection interrupts disabled"]
    B0x0 = 0,
    #[doc = "1: Error detection interrupts enabled"]
    B0x1 = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupts enable Note: Any of these errors generate an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/Underrun (OVR)"]
pub type ErrieR = crate::BitReader<Errie>;
impl ErrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errie {
        match self.bits {
            false => Errie::B0x0,
            true => Errie::B0x1,
        }
    }
    #[doc = "Error detection interrupts disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Errie::B0x0
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Errie::B0x1
    }
}
#[doc = "Field `ERRIE` writer - Error interrupts enable Note: Any of these errors generate an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/Underrun (OVR)"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error detection interrupts disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x0)
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x1)
    }
}
#[doc = "Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* t&lt;sub>I2CCLK&lt;/sub> &lt;sub>...&lt;/sub> Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dnf {
    #[doc = "0: Digital filter disabled"]
    B0x0 = 0,
    #[doc = "1: Digital filter enabled and filtering capability up to 1 t&lt;sub>I2CCLK&lt;/sub>"]
    B0x1 = 1,
    #[doc = "15: digital filter enabled and filtering capability up to15 t&lt;sub>I2CCLK&lt;/sub>"]
    B0xF = 15,
}
impl From<Dnf> for u8 {
    #[inline(always)]
    fn from(variant: Dnf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dnf {
    type Ux = u8;
}
impl crate::IsEnum for Dnf {}
#[doc = "Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* t&lt;sub>I2CCLK&lt;/sub> &lt;sub>...&lt;/sub> Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0)."]
pub type DnfR = crate::FieldReader<Dnf>;
impl DnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dnf> {
        match self.bits {
            0 => Some(Dnf::B0x0),
            1 => Some(Dnf::B0x1),
            15 => Some(Dnf::B0xF),
            _ => None,
        }
    }
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dnf::B0x0
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 t&lt;sub>I2CCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dnf::B0x1
    }
    #[doc = "digital filter enabled and filtering capability up to15 t&lt;sub>I2CCLK&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Dnf::B0xF
    }
}
#[doc = "Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* t&lt;sub>I2CCLK&lt;/sub> &lt;sub>...&lt;/sub> Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0)."]
pub type DnfW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dnf>;
impl<'a, REG> DnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::B0x0)
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 t&lt;sub>I2CCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::B0x1)
    }
    #[doc = "digital filter enabled and filtering capability up to15 t&lt;sub>I2CCLK&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::B0xF)
    }
}
#[doc = "Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anfoff {
    #[doc = "0: Analog noise filter enabled"]
    B0x0 = 0,
    #[doc = "1: Analog noise filter disabled"]
    B0x1 = 1,
}
impl From<Anfoff> for bool {
    #[inline(always)]
    fn from(variant: Anfoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANFOFF` reader - Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0)."]
pub type AnfoffR = crate::BitReader<Anfoff>;
impl AnfoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anfoff {
        match self.bits {
            false => Anfoff::B0x0,
            true => Anfoff::B0x1,
        }
    }
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Anfoff::B0x0
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Anfoff::B0x1
    }
}
#[doc = "Field `ANFOFF` writer - Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0)."]
pub type AnfoffW<'a, REG> = crate::BitWriter<'a, REG, Anfoff>;
impl<'a, REG> AnfoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Anfoff::B0x0)
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Anfoff::B0x1)
    }
}
#[doc = "DMA transmission requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmaen {
    #[doc = "0: DMA mode disabled for transmission"]
    B0x0 = 0,
    #[doc = "1: DMA mode enabled for transmission"]
    B0x1 = 1,
}
impl From<Txdmaen> for bool {
    #[inline(always)]
    fn from(variant: Txdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - DMA transmission requests enable"]
pub type TxdmaenR = crate::BitReader<Txdmaen>;
impl TxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdmaen {
        match self.bits {
            false => Txdmaen::B0x0,
            true => Txdmaen::B0x1,
        }
    }
    #[doc = "DMA mode disabled for transmission"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txdmaen::B0x0
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txdmaen::B0x1
    }
}
#[doc = "Field `TXDMAEN` writer - DMA transmission requests enable"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Txdmaen>;
impl<'a, REG> TxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled for transmission"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::B0x0)
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::B0x1)
    }
}
#[doc = "DMA reception requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmaen {
    #[doc = "0: DMA mode disabled for reception"]
    B0x0 = 0,
    #[doc = "1: DMA mode enabled for reception"]
    B0x1 = 1,
}
impl From<Rxdmaen> for bool {
    #[inline(always)]
    fn from(variant: Rxdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - DMA reception requests enable"]
pub type RxdmaenR = crate::BitReader<Rxdmaen>;
impl RxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmaen {
        match self.bits {
            false => Rxdmaen::B0x0,
            true => Rxdmaen::B0x1,
        }
    }
    #[doc = "DMA mode disabled for reception"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxdmaen::B0x0
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxdmaen::B0x1
    }
}
#[doc = "Field `RXDMAEN` writer - DMA reception requests enable"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Rxdmaen>;
impl<'a, REG> RxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled for reception"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::B0x0)
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::B0x1)
    }
}
#[doc = "Slave byte control This bit is used to enable hardware byte control in slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbc {
    #[doc = "0: Slave byte control disabled"]
    B0x0 = 0,
    #[doc = "1: Slave byte control enabled"]
    B0x1 = 1,
}
impl From<Sbc> for bool {
    #[inline(always)]
    fn from(variant: Sbc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBC` reader - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SbcR = crate::BitReader<Sbc>;
impl SbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbc {
        match self.bits {
            false => Sbc::B0x0,
            true => Sbc::B0x1,
        }
    }
    #[doc = "Slave byte control disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sbc::B0x0
    }
    #[doc = "Slave byte control enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sbc::B0x1
    }
}
#[doc = "Field `SBC` writer - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SbcW<'a, REG> = crate::BitWriter<'a, REG, Sbc>;
impl<'a, REG> SbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave byte control disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbc::B0x0)
    }
    #[doc = "Slave byte control enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbc::B0x1)
    }
}
#[doc = "Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nostretch {
    #[doc = "0: Clock stretching enabled"]
    B0x0 = 0,
    #[doc = "1: Clock stretching disabled"]
    B0x1 = 1,
}
impl From<Nostretch> for bool {
    #[inline(always)]
    fn from(variant: Nostretch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0)."]
pub type NostretchR = crate::BitReader<Nostretch>;
impl NostretchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nostretch {
        match self.bits {
            false => Nostretch::B0x0,
            true => Nostretch::B0x1,
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Nostretch::B0x0
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Nostretch::B0x1
    }
}
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0)."]
pub type NostretchW<'a, REG> = crate::BitWriter<'a, REG, Nostretch>;
impl<'a, REG> NostretchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nostretch::B0x0)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nostretch::B0x1)
    }
}
#[doc = "Wake-up from Stop mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupen {
    #[doc = "0: Wake-up from Stop mode disable."]
    B0x0 = 0,
    #[doc = "1: Wake-up from Stop mode enable."]
    B0x1 = 1,
}
impl From<Wupen> for bool {
    #[inline(always)]
    fn from(variant: Wupen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN` reader - Wake-up from Stop mode enable"]
pub type WupenR = crate::BitReader<Wupen>;
impl WupenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupen {
        match self.bits {
            false => Wupen::B0x0,
            true => Wupen::B0x1,
        }
    }
    #[doc = "Wake-up from Stop mode disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wupen::B0x0
    }
    #[doc = "Wake-up from Stop mode enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wupen::B0x1
    }
}
#[doc = "Field `WUPEN` writer - Wake-up from Stop mode enable"]
pub type WupenW<'a, REG> = crate::BitWriter<'a, REG, Wupen>;
impl<'a, REG> WupenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up from Stop mode disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wupen::B0x0)
    }
    #[doc = "Wake-up from Stop mode enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wupen::B0x1)
    }
}
#[doc = "General call enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcen {
    #[doc = "0: General call disabled. Address 0b00000000 is NACKed."]
    B0x0 = 0,
    #[doc = "1: General call enabled. Address 0b00000000 is ACKed."]
    B0x1 = 1,
}
impl From<Gcen> for bool {
    #[inline(always)]
    fn from(variant: Gcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCEN` reader - General call enable"]
pub type GcenR = crate::BitReader<Gcen>;
impl GcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcen {
        match self.bits {
            false => Gcen::B0x0,
            true => Gcen::B0x1,
        }
    }
    #[doc = "General call disabled. Address 0b00000000 is NACKed."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gcen::B0x0
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gcen::B0x1
    }
}
#[doc = "Field `GCEN` writer - General call enable"]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG, Gcen>;
impl<'a, REG> GcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General call disabled. Address 0b00000000 is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gcen::B0x0)
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gcen::B0x1)
    }
}
#[doc = "Fast-mode Plus 20 mA drive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmp {
    #[doc = "0: 20 mA I/O drive disabled"]
    B0x0 = 0,
    #[doc = "1: 20 mA I/O drive enabled"]
    B0x1 = 1,
}
impl From<Fmp> for bool {
    #[inline(always)]
    fn from(variant: Fmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMP` reader - Fast-mode Plus 20 mA drive enable"]
pub type FmpR = crate::BitReader<Fmp>;
impl FmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmp {
        match self.bits {
            false => Fmp::B0x0,
            true => Fmp::B0x1,
        }
    }
    #[doc = "20 mA I/O drive disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fmp::B0x0
    }
    #[doc = "20 mA I/O drive enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fmp::B0x1
    }
}
#[doc = "Field `FMP` writer - Fast-mode Plus 20 mA drive enable"]
pub type FmpW<'a, REG> = crate::BitWriter<'a, REG, Fmp>;
impl<'a, REG> FmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "20 mA I/O drive disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmp::B0x0)
    }
    #[doc = "20 mA I/O drive enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmp::B0x1)
    }
}
#[doc = "Address match flag (ADDR) automatic clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addraclr {
    #[doc = "0: ADDR flag is set by hardware, cleared by software by setting ADDRCF bit."]
    B0x0 = 0,
    #[doc = "1: ADDR flag remains cleared by hardware. This mode can be used in slave mode, to avoid the ADDR clock stretching if the I2C enables only one slave address. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    B0x1 = 1,
}
impl From<Addraclr> for bool {
    #[inline(always)]
    fn from(variant: Addraclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRACLR` reader - Address match flag (ADDR) automatic clear"]
pub type AddraclrR = crate::BitReader<Addraclr>;
impl AddraclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addraclr {
        match self.bits {
            false => Addraclr::B0x0,
            true => Addraclr::B0x1,
        }
    }
    #[doc = "ADDR flag is set by hardware, cleared by software by setting ADDRCF bit."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Addraclr::B0x0
    }
    #[doc = "ADDR flag remains cleared by hardware. This mode can be used in slave mode, to avoid the ADDR clock stretching if the I2C enables only one slave address. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Addraclr::B0x1
    }
}
#[doc = "Field `ADDRACLR` writer - Address match flag (ADDR) automatic clear"]
pub type AddraclrW<'a, REG> = crate::BitWriter<'a, REG, Addraclr>;
impl<'a, REG> AddraclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADDR flag is set by hardware, cleared by software by setting ADDRCF bit."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Addraclr::B0x0)
    }
    #[doc = "ADDR flag remains cleared by hardware. This mode can be used in slave mode, to avoid the ADDR clock stretching if the I2C enables only one slave address. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Addraclr::B0x1)
    }
}
#[doc = "STOP detection flag (STOPF) automatic clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopfaclr {
    #[doc = "0: STOPF flag is set by hardware, cleared by software by setting STOPCF bit."]
    B0x0 = 0,
    #[doc = "1: STOPF flag remains cleared by hardware. This mode can be used in NOSTRETCH slave mode, to avoid the overrun error if the STOPF flag is not cleared before next data transmission. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    B0x1 = 1,
}
impl From<Stopfaclr> for bool {
    #[inline(always)]
    fn from(variant: Stopfaclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPFACLR` reader - STOP detection flag (STOPF) automatic clear"]
pub type StopfaclrR = crate::BitReader<Stopfaclr>;
impl StopfaclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopfaclr {
        match self.bits {
            false => Stopfaclr::B0x0,
            true => Stopfaclr::B0x1,
        }
    }
    #[doc = "STOPF flag is set by hardware, cleared by software by setting STOPCF bit."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Stopfaclr::B0x0
    }
    #[doc = "STOPF flag remains cleared by hardware. This mode can be used in NOSTRETCH slave mode, to avoid the overrun error if the STOPF flag is not cleared before next data transmission. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Stopfaclr::B0x1
    }
}
#[doc = "Field `STOPFACLR` writer - STOP detection flag (STOPF) automatic clear"]
pub type StopfaclrW<'a, REG> = crate::BitWriter<'a, REG, Stopfaclr>;
impl<'a, REG> StopfaclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOPF flag is set by hardware, cleared by software by setting STOPCF bit."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Stopfaclr::B0x0)
    }
    #[doc = "STOPF flag remains cleared by hardware. This mode can be used in NOSTRETCH slave mode, to avoid the overrun error if the STOPF flag is not cleared before next data transmission. This allows a slave data management by DMA only, without any interrupt from peripheral."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Stopfaclr::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX interrupt enable"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX interrupt enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match interrupt enable (slave only)"]
    #[inline(always)]
    pub fn addrie(&self) -> AddrieR {
        AddrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    pub fn nackie(&self) -> NackieR {
        NackieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection interrupt enable"]
    #[inline(always)]
    pub fn stopie(&self) -> StopieR {
        StopieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer complete interrupt enable Note: Any of these events generate an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/Underrun (OVR)"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* t&lt;sub>I2CCLK&lt;/sub> &lt;sub>...&lt;/sub> Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn dnf(&self) -> DnfR {
        DnfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn anfoff(&self) -> AnfoffR {
        AnfoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    pub fn sbc(&self) -> SbcR {
        SbcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn nostretch(&self) -> NostretchR {
        NostretchR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake-up from Stop mode enable"]
    #[inline(always)]
    pub fn wupen(&self) -> WupenR {
        WupenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GcenR {
        GcenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Fast-mode Plus 20 mA drive enable"]
    #[inline(always)]
    pub fn fmp(&self) -> FmpR {
        FmpR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Address match flag (ADDR) automatic clear"]
    #[inline(always)]
    pub fn addraclr(&self) -> AddraclrR {
        AddraclrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - STOP detection flag (STOPF) automatic clear"]
    #[inline(always)]
    pub fn stopfaclr(&self) -> StopfaclrR {
        StopfaclrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<I2cCr1Spec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 1 - TX interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TxieW<I2cCr1Spec> {
        TxieW::new(self, 1)
    }
    #[doc = "Bit 2 - RX interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RxieW<I2cCr1Spec> {
        RxieW::new(self, 2)
    }
    #[doc = "Bit 3 - Address match interrupt enable (slave only)"]
    #[inline(always)]
    #[must_use]
    pub fn addrie(&mut self) -> AddrieW<I2cCr1Spec> {
        AddrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackie(&mut self) -> NackieW<I2cCr1Spec> {
        NackieW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopie(&mut self) -> StopieW<I2cCr1Spec> {
        StopieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer complete interrupt enable Note: Any of these events generate an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<I2cCr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/Underrun (OVR)"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<I2cCr1Spec> {
        ErrieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* t&lt;sub>I2CCLK&lt;/sub> &lt;sub>...&lt;/sub> Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DnfW<I2cCr1Spec> {
        DnfW::new(self, 8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn anfoff(&mut self) -> AnfoffW<I2cCr1Spec> {
        AnfoffW::new(self, 12)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TxdmaenW<I2cCr1Spec> {
        TxdmaenW::new(self, 14)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RxdmaenW<I2cCr1Spec> {
        RxdmaenW::new(self, 15)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SbcW<I2cCr1Spec> {
        SbcW::new(self, 16)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NostretchW<I2cCr1Spec> {
        NostretchW::new(self, 17)
    }
    #[doc = "Bit 18 - Wake-up from Stop mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WupenW<I2cCr1Spec> {
        WupenW::new(self, 18)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GcenW<I2cCr1Spec> {
        GcenW::new(self, 19)
    }
    #[doc = "Bit 24 - Fast-mode Plus 20 mA drive enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmp(&mut self) -> FmpW<I2cCr1Spec> {
        FmpW::new(self, 24)
    }
    #[doc = "Bit 30 - Address match flag (ADDR) automatic clear"]
    #[inline(always)]
    #[must_use]
    pub fn addraclr(&mut self) -> AddraclrW<I2cCr1Spec> {
        AddraclrW::new(self, 30)
    }
    #[doc = "Bit 31 - STOP detection flag (STOPF) automatic clear"]
    #[inline(always)]
    #[must_use]
    pub fn stopfaclr(&mut self) -> StopfaclrW<I2cCr1Spec> {
        StopfaclrW::new(self, 31)
    }
}
#[doc = "I2C control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cCr1Spec;
impl crate::RegisterSpec for I2cCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_cr1::R`](R) reader structure"]
impl crate::Readable for I2cCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_cr1::W`](W) writer structure"]
impl crate::Writable for I2cCr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_CR1 to value 0"]
impl crate::Resettable for I2cCr1Spec {
    const RESET_VALUE: u32 = 0;
}
