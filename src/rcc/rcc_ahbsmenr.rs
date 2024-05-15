#[doc = "Register `RCC_AHBSMENR` reader"]
pub type R = crate::R<RccAhbsmenrSpec>;
#[doc = "Register `RCC_AHBSMENR` writer"]
pub type W = crate::W<RccAhbsmenrSpec>;
#[doc = "DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Dma1smen> for bool {
    #[inline(always)]
    fn from(variant: Dma1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1SMEN` reader - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral."]
pub type Dma1smenR = crate::BitReader<Dma1smen>;
impl Dma1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1smen {
        match self.bits {
            false => Dma1smen::B0x0,
            true => Dma1smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma1smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma1smen::B0x1
    }
}
#[doc = "Field `DMA1SMEN` writer - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral."]
pub type Dma1smenW<'a, REG> = crate::BitWriter<'a, REG, Dma1smen>;
impl<'a, REG> Dma1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1smen::B0x1)
    }
}
#[doc = "DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma2smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Dma2smen> for bool {
    #[inline(always)]
    fn from(variant: Dma2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2SMEN` reader - DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral."]
pub type Dma2smenR = crate::BitReader<Dma2smen>;
impl Dma2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma2smen {
        match self.bits {
            false => Dma2smen::B0x0,
            true => Dma2smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma2smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma2smen::B0x1
    }
}
#[doc = "Field `DMA2SMEN` writer - DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral."]
pub type Dma2smenW<'a, REG> = crate::BitWriter<'a, REG, Dma2smen>;
impl<'a, REG> Dma2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2smen::B0x1)
    }
}
#[doc = "Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Flashsmen> for bool {
    #[inline(always)]
    fn from(variant: Flashsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode."]
pub type FlashsmenR = crate::BitReader<Flashsmen>;
impl FlashsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashsmen {
        match self.bits {
            false => Flashsmen::B0x0,
            true => Flashsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Flashsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Flashsmen::B0x1
    }
}
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode."]
pub type FlashsmenW<'a, REG> = crate::BitWriter<'a, REG, Flashsmen>;
impl<'a, REG> FlashsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsmen::B0x1)
    }
}
#[doc = "SRAM clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Sramsmen> for bool {
    #[inline(always)]
    fn from(variant: Sramsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode Set and cleared by software."]
pub type SramsmenR = crate::BitReader<Sramsmen>;
impl SramsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramsmen {
        match self.bits {
            false => Sramsmen::B0x0,
            true => Sramsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sramsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sramsmen::B0x1
    }
}
#[doc = "Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode Set and cleared by software."]
pub type SramsmenW<'a, REG> = crate::BitWriter<'a, REG, Sramsmen>;
impl<'a, REG> SramsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sramsmen::B0x1)
    }
}
#[doc = "CRC clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Crcsmen> for bool {
    #[inline(always)]
    fn from(variant: Crcsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCSMEN` reader - CRC clock enable during Sleep mode Set and cleared by software."]
pub type CrcsmenR = crate::BitReader<Crcsmen>;
impl CrcsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcsmen {
        match self.bits {
            false => Crcsmen::B0x0,
            true => Crcsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcsmen::B0x1
    }
}
#[doc = "Field `CRCSMEN` writer - CRC clock enable during Sleep mode Set and cleared by software."]
pub type CrcsmenW<'a, REG> = crate::BitWriter<'a, REG, Crcsmen>;
impl<'a, REG> CrcsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsmen::B0x1)
    }
}
#[doc = "AES hardware accelerator clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aessmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Aessmen> for bool {
    #[inline(always)]
    fn from(variant: Aessmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESSMEN` reader - AES hardware accelerator clock enable during Sleep mode Set and cleared by software."]
pub type AessmenR = crate::BitReader<Aessmen>;
impl AessmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aessmen {
        match self.bits {
            false => Aessmen::B0x0,
            true => Aessmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Aessmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Aessmen::B0x1
    }
}
#[doc = "Field `AESSMEN` writer - AES hardware accelerator clock enable during Sleep mode Set and cleared by software."]
pub type AessmenW<'a, REG> = crate::BitWriter<'a, REG, Aessmen>;
impl<'a, REG> AessmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Aessmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Aessmen::B0x1)
    }
}
#[doc = "RNG clock enable during Sleep and Stop mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rngsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rngsmen> for bool {
    #[inline(always)]
    fn from(variant: Rngsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGSMEN` reader - RNG clock enable during Sleep and Stop mode Set and cleared by software."]
pub type RngsmenR = crate::BitReader<Rngsmen>;
impl RngsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rngsmen {
        match self.bits {
            false => Rngsmen::B0x0,
            true => Rngsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rngsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rngsmen::B0x1
    }
}
#[doc = "Field `RNGSMEN` writer - RNG clock enable during Sleep and Stop mode Set and cleared by software."]
pub type RngsmenW<'a, REG> = crate::BitWriter<'a, REG, Rngsmen>;
impl<'a, REG> RngsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rngsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rngsmen::B0x1)
    }
}
#[doc = "TSC clock enable during Sleep and Stop mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tscsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tscsmen> for bool {
    #[inline(always)]
    fn from(variant: Tscsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCSMEN` reader - TSC clock enable during Sleep and Stop mode Set and cleared by software."]
pub type TscsmenR = crate::BitReader<Tscsmen>;
impl TscsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tscsmen {
        match self.bits {
            false => Tscsmen::B0x0,
            true => Tscsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tscsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tscsmen::B0x1
    }
}
#[doc = "Field `TSCSMEN` writer - TSC clock enable during Sleep and Stop mode Set and cleared by software."]
pub type TscsmenW<'a, REG> = crate::BitWriter<'a, REG, Tscsmen>;
impl<'a, REG> TscsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tscsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tscsmen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral."]
    #[inline(always)]
    pub fn dma1smen(&self) -> Dma1smenR {
        Dma1smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral."]
    #[inline(always)]
    pub fn dma2smen(&self) -> Dma2smenR {
        Dma2smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode."]
    #[inline(always)]
    pub fn flashsmen(&self) -> FlashsmenR {
        FlashsmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn sramsmen(&self) -> SramsmenR {
        SramsmenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn crcsmen(&self) -> CrcsmenR {
        CrcsmenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn aessmen(&self) -> AessmenR {
        AessmenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG clock enable during Sleep and Stop mode Set and cleared by software."]
    #[inline(always)]
    pub fn rngsmen(&self) -> RngsmenR {
        RngsmenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - TSC clock enable during Sleep and Stop mode Set and cleared by software."]
    #[inline(always)]
    pub fn tscsmen(&self) -> TscsmenR {
        TscsmenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> Dma1smenW<RccAhbsmenrSpec> {
        Dma1smenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> Dma2smenW<RccAhbsmenrSpec> {
        Dma2smenW::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the flash memory is in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FlashsmenW<RccAhbsmenrSpec> {
        FlashsmenW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sramsmen(&mut self) -> SramsmenW<RccAhbsmenrSpec> {
        SramsmenW::new(self, 9)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CrcsmenW<RccAhbsmenrSpec> {
        CrcsmenW::new(self, 12)
    }
    #[doc = "Bit 16 - AES hardware accelerator clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn aessmen(&mut self) -> AessmenW<RccAhbsmenrSpec> {
        AessmenW::new(self, 16)
    }
    #[doc = "Bit 18 - RNG clock enable during Sleep and Stop mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RngsmenW<RccAhbsmenrSpec> {
        RngsmenW::new(self, 18)
    }
    #[doc = "Bit 24 - TSC clock enable during Sleep and Stop mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TscsmenW<RccAhbsmenrSpec> {
        TscsmenW::new(self, 24)
    }
}
#[doc = "AHB peripheral clock enable in Sleep/Stop mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahbsmenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahbsmenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhbsmenrSpec;
impl crate::RegisterSpec for RccAhbsmenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahbsmenr::R`](R) reader structure"]
impl crate::Readable for RccAhbsmenrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahbsmenr::W`](W) writer structure"]
impl crate::Writable for RccAhbsmenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHBSMENR to value 0x0105_1303"]
impl crate::Resettable for RccAhbsmenrSpec {
    const RESET_VALUE: u32 = 0x0105_1303;
}
