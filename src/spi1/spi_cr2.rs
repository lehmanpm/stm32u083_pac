#[doc = "Register `SPI_CR2` reader"]
pub type R = crate::R<SpiCr2Spec>;
#[doc = "Register `SPI_CR2` writer"]
pub type W = crate::W<SpiCr2Spec>;
#[doc = "Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmaen {
    #[doc = "0: Rx buffer DMA disabled"]
    B0x0 = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    B0x1 = 1,
}
impl From<Rxdmaen> for bool {
    #[inline(always)]
    fn from(variant: Rxdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set."]
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
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxdmaen::B0x0
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxdmaen::B0x1
    }
}
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set."]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Rxdmaen>;
impl<'a, REG> RxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::B0x0)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::B0x1)
    }
}
#[doc = "Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmaen {
    #[doc = "0: Tx buffer DMA disabled"]
    B0x0 = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    B0x1 = 1,
}
impl From<Txdmaen> for bool {
    #[inline(always)]
    fn from(variant: Txdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set."]
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
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txdmaen::B0x0
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txdmaen::B0x1
    }
}
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set."]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Txdmaen>;
impl<'a, REG> TxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::B0x0)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::B0x1)
    }
}
#[doc = "SS output enable Note: This bit is not used in SPI TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssoe {
    #[doc = "0: SS output is disabled in master mode and the SPI interface can work in multimaster configuration"]
    B0x0 = 0,
    #[doc = "1: SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
    B0x1 = 1,
}
impl From<Ssoe> for bool {
    #[inline(always)]
    fn from(variant: Ssoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOE` reader - SS output enable Note: This bit is not used in SPI TI mode."]
pub type SsoeR = crate::BitReader<Ssoe>;
impl SsoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssoe {
        match self.bits {
            false => Ssoe::B0x0,
            true => Ssoe::B0x1,
        }
    }
    #[doc = "SS output is disabled in master mode and the SPI interface can work in multimaster configuration"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ssoe::B0x0
    }
    #[doc = "SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ssoe::B0x1
    }
}
#[doc = "Field `SSOE` writer - SS output enable Note: This bit is not used in SPI TI mode."]
pub type SsoeW<'a, REG> = crate::BitWriter<'a, REG, Ssoe>;
impl<'a, REG> SsoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SS output is disabled in master mode and the SPI interface can work in multimaster configuration"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssoe::B0x0)
    }
    #[doc = "SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssoe::B0x1)
    }
}
#[doc = "NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1, or FRF = 1. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in SPI TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nssp {
    #[doc = "0: No NSS pulse"]
    B0x0 = 0,
    #[doc = "1: NSS pulse generated"]
    B0x1 = 1,
}
impl From<Nssp> for bool {
    #[inline(always)]
    fn from(variant: Nssp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSSP` reader - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1, or FRF = 1. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in SPI TI mode."]
pub type NsspR = crate::BitReader<Nssp>;
impl NsspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nssp {
        match self.bits {
            false => Nssp::B0x0,
            true => Nssp::B0x1,
        }
    }
    #[doc = "No NSS pulse"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Nssp::B0x0
    }
    #[doc = "NSS pulse generated"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Nssp::B0x1
    }
}
#[doc = "Field `NSSP` writer - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1, or FRF = 1. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in SPI TI mode."]
pub type NsspW<'a, REG> = crate::BitWriter<'a, REG, Nssp>;
impl<'a, REG> NsspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No NSS pulse"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nssp::B0x0)
    }
    #[doc = "NSS pulse generated"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nssp::B0x1)
    }
}
#[doc = "Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frf {
    #[doc = "0: SPI Motorola mode"]
    B0x0 = 0,
}
impl From<Frf> for bool {
    #[inline(always)]
    fn from(variant: Frf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRF` reader - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0)."]
pub type FrfR = crate::BitReader<Frf>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frf> {
        match self.bits {
            false => Some(Frf::B0x0),
            _ => None,
        }
    }
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Frf::B0x0
    }
}
#[doc = "Field `FRF` writer - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0)."]
pub type FrfW<'a, REG> = crate::BitWriter<'a, REG, Frf>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::B0x0)
    }
}
#[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Error interrupt is masked"]
    B0x0 = 0,
    #[doc = "1: Error interrupt is enabled"]
    B0x1 = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode)."]
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
    #[doc = "Error interrupt is masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Errie::B0x0
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Errie::B0x1
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode)."]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt is masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x0)
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x1)
    }
}
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxneie {
    #[doc = "0: RXNE interrupt masked"]
    B0x0 = 0,
    #[doc = "1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    B0x1 = 1,
}
impl From<Rxneie> for bool {
    #[inline(always)]
    fn from(variant: Rxneie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable"]
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
    #[doc = "RXNE interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxneie::B0x0
    }
    #[doc = "RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxneie::B0x1
    }
}
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG, Rxneie>;
impl<'a, REG> RxneieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXNE interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxneie::B0x0)
    }
    #[doc = "RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxneie::B0x1)
    }
}
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txeie {
    #[doc = "0: TXE interrupt masked"]
    B0x0 = 0,
    #[doc = "1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    B0x1 = 1,
}
impl From<Txeie> for bool {
    #[inline(always)]
    fn from(variant: Txeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable"]
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
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txeie::B0x0
    }
    #[doc = "TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txeie::B0x1
    }
}
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable"]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG, Txeie>;
impl<'a, REG> TxeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Txeie::B0x0)
    }
    #[doc = "TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Txeie::B0x1)
    }
}
#[doc = "Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ds {
    #[doc = "0: Not used"]
    B0x0 = 0,
    #[doc = "1: Not used"]
    B0x1 = 1,
    #[doc = "2: Not used"]
    B0x2 = 2,
    #[doc = "3: 4-bit"]
    B0x3 = 3,
    #[doc = "4: 5-bit"]
    B0x4 = 4,
    #[doc = "5: 6-bit"]
    B0x5 = 5,
    #[doc = "6: 7-bit"]
    B0x6 = 6,
    #[doc = "7: 8-bit"]
    B0x7 = 7,
    #[doc = "8: 9-bit"]
    B0x8 = 8,
    #[doc = "9: 10-bit"]
    B0x9 = 9,
    #[doc = "10: 11-bit"]
    B0xA = 10,
    #[doc = "11: 12-bit"]
    B0xB = 11,
    #[doc = "12: 13-bit"]
    B0xC = 12,
    #[doc = "13: 14-bit"]
    B0xD = 13,
    #[doc = "14: 15-bit"]
    B0xE = 14,
    #[doc = "15: 16-bit"]
    B0xF = 15,
}
impl From<Ds> for u8 {
    #[inline(always)]
    fn from(variant: Ds) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ds {
    type Ux = u8;
}
impl crate::IsEnum for Ds {}
#[doc = "Field `DS` reader - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit)"]
pub type DsR = crate::FieldReader<Ds>;
impl DsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds {
        match self.bits {
            0 => Ds::B0x0,
            1 => Ds::B0x1,
            2 => Ds::B0x2,
            3 => Ds::B0x3,
            4 => Ds::B0x4,
            5 => Ds::B0x5,
            6 => Ds::B0x6,
            7 => Ds::B0x7,
            8 => Ds::B0x8,
            9 => Ds::B0x9,
            10 => Ds::B0xA,
            11 => Ds::B0xB,
            12 => Ds::B0xC,
            13 => Ds::B0xD,
            14 => Ds::B0xE,
            15 => Ds::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ds::B0x0
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ds::B0x1
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ds::B0x2
    }
    #[doc = "4-bit"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ds::B0x3
    }
    #[doc = "5-bit"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Ds::B0x4
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Ds::B0x5
    }
    #[doc = "7-bit"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Ds::B0x6
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Ds::B0x7
    }
    #[doc = "9-bit"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Ds::B0x8
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Ds::B0x9
    }
    #[doc = "11-bit"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Ds::B0xA
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Ds::B0xB
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Ds::B0xC
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Ds::B0xD
    }
    #[doc = "15-bit"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Ds::B0xE
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Ds::B0xF
    }
}
#[doc = "Field `DS` writer - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit)"]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ds, crate::Safe>;
impl<'a, REG> DsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not used"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x0)
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x1)
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x2)
    }
    #[doc = "4-bit"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x3)
    }
    #[doc = "5-bit"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x4)
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x5)
    }
    #[doc = "7-bit"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x6)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x7)
    }
    #[doc = "9-bit"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x8)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0x9)
    }
    #[doc = "11-bit"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0xA)
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0xB)
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0xC)
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0xD)
    }
    #[doc = "15-bit"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0xE)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::B0xF)
    }
}
#[doc = "FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frxth {
    #[doc = "0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    B0x0 = 0,
    #[doc = "1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    B0x1 = 1,
}
impl From<Frxth> for bool {
    #[inline(always)]
    fn from(variant: Frxth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRXTH` reader - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event"]
pub type FrxthR = crate::BitReader<Frxth>;
impl FrxthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frxth {
        match self.bits {
            false => Frxth::B0x0,
            true => Frxth::B0x1,
        }
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Frxth::B0x0
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Frxth::B0x1
    }
}
#[doc = "Field `FRXTH` writer - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event"]
pub type FrxthW<'a, REG> = crate::BitWriter<'a, REG, Frxth>;
impl<'a, REG> FrxthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Frxth::B0x0)
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Frxth::B0x1)
    }
}
#[doc = "Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LdmaRx {
    #[doc = "0: Number of data to transfer is even"]
    B0x0 = 0,
    #[doc = "1: Number of data to transfer is odd"]
    B0x1 = 1,
}
impl From<LdmaRx> for bool {
    #[inline(always)]
    fn from(variant: LdmaRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDMA_RX` reader - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set."]
pub type LdmaRxR = crate::BitReader<LdmaRx>;
impl LdmaRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LdmaRx {
        match self.bits {
            false => LdmaRx::B0x0,
            true => LdmaRx::B0x1,
        }
    }
    #[doc = "Number of data to transfer is even"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LdmaRx::B0x0
    }
    #[doc = "Number of data to transfer is odd"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LdmaRx::B0x1
    }
}
#[doc = "Field `LDMA_RX` writer - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set."]
pub type LdmaRxW<'a, REG> = crate::BitWriter<'a, REG, LdmaRx>;
impl<'a, REG> LdmaRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Number of data to transfer is even"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LdmaRx::B0x0)
    }
    #[doc = "Number of data to transfer is odd"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LdmaRx::B0x1)
    }
}
#[doc = "Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LdmaTx {
    #[doc = "0: Number of data to transfer is even"]
    B0x0 = 0,
    #[doc = "1: Number of data to transfer is odd"]
    B0x1 = 1,
}
impl From<LdmaTx> for bool {
    #[inline(always)]
    fn from(variant: LdmaTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDMA_TX` reader - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set."]
pub type LdmaTxR = crate::BitReader<LdmaTx>;
impl LdmaTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LdmaTx {
        match self.bits {
            false => LdmaTx::B0x0,
            true => LdmaTx::B0x1,
        }
    }
    #[doc = "Number of data to transfer is even"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LdmaTx::B0x0
    }
    #[doc = "Number of data to transfer is odd"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LdmaTx::B0x1
    }
}
#[doc = "Field `LDMA_TX` writer - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set."]
pub type LdmaTxW<'a, REG> = crate::BitWriter<'a, REG, LdmaTx>;
impl<'a, REG> LdmaTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Number of data to transfer is even"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LdmaTx::B0x0)
    }
    #[doc = "Number of data to transfer is odd"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LdmaTx::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set."]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set."]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable Note: This bit is not used in SPI TI mode."]
    #[inline(always)]
    pub fn ssoe(&self) -> SsoeR {
        SsoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1, or FRF = 1. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in SPI TI mode."]
    #[inline(always)]
    pub fn nssp(&self) -> NsspR {
        NsspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0)."]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode)."]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit)"]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event"]
    #[inline(always)]
    pub fn frxth(&self) -> FrxthR {
        FrxthR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set."]
    #[inline(always)]
    pub fn ldma_rx(&self) -> LdmaRxR {
        LdmaRxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set."]
    #[inline(always)]
    pub fn ldma_tx(&self) -> LdmaTxR {
        LdmaTxR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RxdmaenW<SpiCr2Spec> {
        RxdmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TxdmaenW<SpiCr2Spec> {
        TxdmaenW::new(self, 1)
    }
    #[doc = "Bit 2 - SS output enable Note: This bit is not used in SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SsoeW<SpiCr2Spec> {
        SsoeW::new(self, 2)
    }
    #[doc = "Bit 3 - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1, or FRF = 1. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn nssp(&mut self) -> NsspW<SpiCr2Spec> {
        NsspW::new(self, 3)
    }
    #[doc = "Bit 4 - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FrfW<SpiCr2Spec> {
        FrfW::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode)."]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<SpiCr2Spec> {
        ErrieW::new(self, 5)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RxneieW<SpiCr2Spec> {
        RxneieW::new(self, 6)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TxeieW<SpiCr2Spec> {
        TxeieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit)"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DsW<SpiCr2Spec> {
        DsW::new(self, 8)
    }
    #[doc = "Bit 12 - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event"]
    #[inline(always)]
    #[must_use]
    pub fn frxth(&mut self) -> FrxthW<SpiCr2Spec> {
        FrxthW::new(self, 12)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn ldma_rx(&mut self) -> LdmaRxW<SpiCr2Spec> {
        LdmaRxW::new(self, 13)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to Procedure for disabling the SPI on page1954 if the CRCEN bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn ldma_tx(&mut self) -> LdmaTxW<SpiCr2Spec> {
        LdmaTxW::new(self, 14)
    }
}
#[doc = "SPI control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCr2Spec;
impl crate::RegisterSpec for SpiCr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_cr2::R`](R) reader structure"]
impl crate::Readable for SpiCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_cr2::W`](W) writer structure"]
impl crate::Writable for SpiCr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_CR2 to value 0x0700"]
impl crate::Resettable for SpiCr2Spec {
    const RESET_VALUE: u16 = 0x0700;
}
