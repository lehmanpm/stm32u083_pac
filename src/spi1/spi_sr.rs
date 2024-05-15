#[doc = "Register `SPI_SR` reader"]
pub type R = crate::R<SpiSrSpec>;
#[doc = "Register `SPI_SR` writer"]
pub type W = crate::W<SpiSrSpec>;
#[doc = "Receive buffer not empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxne {
    #[doc = "0: Rx buffer empty"]
    B0x0 = 0,
    #[doc = "1: Rx buffer not empty"]
    B0x1 = 1,
}
impl From<Rxne> for bool {
    #[inline(always)]
    fn from(variant: Rxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNE` reader - Receive buffer not empty"]
pub type RxneR = crate::BitReader<Rxne>;
impl RxneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxne {
        match self.bits {
            false => Rxne::B0x0,
            true => Rxne::B0x1,
        }
    }
    #[doc = "Rx buffer empty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rxne::B0x0
    }
    #[doc = "Rx buffer not empty"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rxne::B0x1
    }
}
#[doc = "Transmit buffer empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txe {
    #[doc = "0: Tx buffer not empty"]
    B0x0 = 0,
    #[doc = "1: Tx buffer empty"]
    B0x1 = 1,
}
impl From<Txe> for bool {
    #[inline(always)]
    fn from(variant: Txe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - Transmit buffer empty"]
pub type TxeR = crate::BitReader<Txe>;
impl TxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txe {
        match self.bits {
            false => Txe::B0x0,
            true => Txe::B0x1,
        }
    }
    #[doc = "Tx buffer not empty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Txe::B0x0
    }
    #[doc = "Tx buffer empty"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Txe::B0x1
    }
}
#[doc = "CRC error flag Note: This flag is set by hardware and cleared by software writing 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcerr {
    #[doc = "0: CRC value received matches the SPI_RXCRCR value"]
    B0x0 = 0,
    #[doc = "1: CRC value received does not match the SPI_RXCRCR value"]
    B0x1 = 1,
}
impl From<Crcerr> for bool {
    #[inline(always)]
    fn from(variant: Crcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` reader - CRC error flag Note: This flag is set by hardware and cleared by software writing 0."]
pub type CrcerrR = crate::BitReader<Crcerr>;
impl CrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcerr {
        match self.bits {
            false => Crcerr::B0x0,
            true => Crcerr::B0x1,
        }
    }
    #[doc = "CRC value received matches the SPI_RXCRCR value"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcerr::B0x0
    }
    #[doc = "CRC value received does not match the SPI_RXCRCR value"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcerr::B0x1
    }
}
#[doc = "Field `CRCERR` writer - CRC error flag Note: This flag is set by hardware and cleared by software writing 0."]
pub type CrcerrW<'a, REG> = crate::BitWriter<'a, REG, Crcerr>;
impl<'a, REG> CrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC value received matches the SPI_RXCRCR value"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcerr::B0x0)
    }
    #[doc = "CRC value received does not match the SPI_RXCRCR value"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcerr::B0x1)
    }
}
#[doc = "Mode fault This flag is set by hardware and reset by a software sequence. Refer to Section1: Mode fault (MODF) on page1964 for the software sequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modf {
    #[doc = "0: No mode fault occurred"]
    B0x0 = 0,
    #[doc = "1: Mode fault occurred"]
    B0x1 = 1,
}
impl From<Modf> for bool {
    #[inline(always)]
    fn from(variant: Modf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` reader - Mode fault This flag is set by hardware and reset by a software sequence. Refer to Section1: Mode fault (MODF) on page1964 for the software sequence."]
pub type ModfR = crate::BitReader<Modf>;
impl ModfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modf {
        match self.bits {
            false => Modf::B0x0,
            true => Modf::B0x1,
        }
    }
    #[doc = "No mode fault occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Modf::B0x0
    }
    #[doc = "Mode fault occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Modf::B0x1
    }
}
#[doc = "Overrun flag This flag is set by hardware and reset by a software sequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovr {
    #[doc = "0: No overrun occurred"]
    B0x0 = 0,
    #[doc = "1: Overrun occurred"]
    B0x1 = 1,
}
impl From<Ovr> for bool {
    #[inline(always)]
    fn from(variant: Ovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - Overrun flag This flag is set by hardware and reset by a software sequence."]
pub type OvrR = crate::BitReader<Ovr>;
impl OvrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovr {
        match self.bits {
            false => Ovr::B0x0,
            true => Ovr::B0x1,
        }
    }
    #[doc = "No overrun occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ovr::B0x0
    }
    #[doc = "Overrun occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ovr::B0x1
    }
}
#[doc = "Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to Section133.4.10: SPI status flags and Procedure for disabling the SPI on page1954.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsy {
    #[doc = "0: SPI not busy"]
    B0x0 = 0,
    #[doc = "1: SPI is busy in communication or Tx buffer is not empty"]
    B0x1 = 1,
}
impl From<Bsy> for bool {
    #[inline(always)]
    fn from(variant: Bsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to Section133.4.10: SPI status flags and Procedure for disabling the SPI on page1954."]
pub type BsyR = crate::BitReader<Bsy>;
impl BsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsy {
        match self.bits {
            false => Bsy::B0x0,
            true => Bsy::B0x1,
        }
    }
    #[doc = "SPI not busy"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bsy::B0x0
    }
    #[doc = "SPI is busy in communication or Tx buffer is not empty"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bsy::B0x1
    }
}
#[doc = "Frame format error This flag is used for SPI in TI slave mode. Refer to Section133.4.11: SPI error flags. This flag is set by hardware and reset when SPI_SR is read by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fre {
    #[doc = "0: No frame format error"]
    B0x0 = 0,
    #[doc = "1: A frame format error occurred"]
    B0x1 = 1,
}
impl From<Fre> for bool {
    #[inline(always)]
    fn from(variant: Fre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRE` reader - Frame format error This flag is used for SPI in TI slave mode. Refer to Section133.4.11: SPI error flags. This flag is set by hardware and reset when SPI_SR is read by software."]
pub type FreR = crate::BitReader<Fre>;
impl FreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fre {
        match self.bits {
            false => Fre::B0x0,
            true => Fre::B0x1,
        }
    }
    #[doc = "No frame format error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fre::B0x0
    }
    #[doc = "A frame format error occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fre::B0x1
    }
}
#[doc = "FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in SPI receive-only mode while CRC calculation is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frlvl {
    #[doc = "0: FIFO empty"]
    B0x0 = 0,
    #[doc = "1: 1/4 FIFO"]
    B0x1 = 1,
    #[doc = "2: 1/2 FIFO"]
    B0x2 = 2,
    #[doc = "3: FIFO full"]
    B0x3 = 3,
}
impl From<Frlvl> for u8 {
    #[inline(always)]
    fn from(variant: Frlvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frlvl {
    type Ux = u8;
}
impl crate::IsEnum for Frlvl {}
#[doc = "Field `FRLVL` reader - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in SPI receive-only mode while CRC calculation is enabled."]
pub type FrlvlR = crate::FieldReader<Frlvl>;
impl FrlvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frlvl {
        match self.bits {
            0 => Frlvl::B0x0,
            1 => Frlvl::B0x1,
            2 => Frlvl::B0x2,
            3 => Frlvl::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Frlvl::B0x0
    }
    #[doc = "1/4 FIFO"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Frlvl::B0x1
    }
    #[doc = "1/2 FIFO"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Frlvl::B0x2
    }
    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Frlvl::B0x3
    }
}
#[doc = "FIFO transmission level These bits are set and cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ftlvl {
    #[doc = "0: FIFO empty"]
    B0x0 = 0,
    #[doc = "1: 1/4 FIFO"]
    B0x1 = 1,
    #[doc = "2: 1/2 FIFO"]
    B0x2 = 2,
    #[doc = "3: FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)"]
    B0x3 = 3,
}
impl From<Ftlvl> for u8 {
    #[inline(always)]
    fn from(variant: Ftlvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ftlvl {
    type Ux = u8;
}
impl crate::IsEnum for Ftlvl {}
#[doc = "Field `FTLVL` reader - FIFO transmission level These bits are set and cleared by hardware."]
pub type FtlvlR = crate::FieldReader<Ftlvl>;
impl FtlvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ftlvl {
        match self.bits {
            0 => Ftlvl::B0x0,
            1 => Ftlvl::B0x1,
            2 => Ftlvl::B0x2,
            3 => Ftlvl::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ftlvl::B0x0
    }
    #[doc = "1/4 FIFO"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ftlvl::B0x1
    }
    #[doc = "1/2 FIFO"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ftlvl::B0x2
    }
    #[doc = "FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ftlvl::B0x3
    }
}
impl R {
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0."]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault This flag is set by hardware and reset by a software sequence. Refer to Section1: Mode fault (MODF) on page1964 for the software sequence."]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag This flag is set by hardware and reset by a software sequence."]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to Section133.4.10: SPI status flags and Procedure for disabling the SPI on page1954."]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame format error This flag is used for SPI in TI slave mode. Refer to Section133.4.11: SPI error flags. This flag is set by hardware and reset when SPI_SR is read by software."]
    #[inline(always)]
    pub fn fre(&self) -> FreR {
        FreR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in SPI receive-only mode while CRC calculation is enabled."]
    #[inline(always)]
    pub fn frlvl(&self) -> FrlvlR {
        FrlvlR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - FIFO transmission level These bits are set and cleared by hardware."]
    #[inline(always)]
    pub fn ftlvl(&self) -> FtlvlR {
        FtlvlR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0."]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CrcerrW<SpiSrSpec> {
        CrcerrW::new(self, 4)
    }
}
#[doc = "SPI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSrSpec;
impl crate::RegisterSpec for SpiSrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_sr::R`](R) reader structure"]
impl crate::Readable for SpiSrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_sr::W`](W) writer structure"]
impl crate::Writable for SpiSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_SR to value 0x02"]
impl crate::Resettable for SpiSrSpec {
    const RESET_VALUE: u16 = 0x02;
}
