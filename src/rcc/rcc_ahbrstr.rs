#[doc = "Register `RCC_AHBRSTR` reader"]
pub type R = crate::R<RccAhbrstrSpec>;
#[doc = "Register `RCC_AHBRSTR` writer"]
pub type W = crate::W<RccAhbrstrSpec>;
#[doc = "DMA1 and DMAMUX reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DMA1 and DMAMUX"]
    B0x1 = 1,
}
impl From<Dma1rst> for bool {
    #[inline(always)]
    fn from(variant: Dma1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1RST` reader - DMA1 and DMAMUX reset Set and cleared by software."]
pub type Dma1rstR = crate::BitReader<Dma1rst>;
impl Dma1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1rst {
        match self.bits {
            false => Dma1rst::B0x0,
            true => Dma1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma1rst::B0x0
    }
    #[doc = "Reset DMA1 and DMAMUX"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma1rst::B0x1
    }
}
#[doc = "Field `DMA1RST` writer - DMA1 and DMAMUX reset Set and cleared by software."]
pub type Dma1rstW<'a, REG> = crate::BitWriter<'a, REG, Dma1rst>;
impl<'a, REG> Dma1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1rst::B0x0)
    }
    #[doc = "Reset DMA1 and DMAMUX"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1rst::B0x1)
    }
}
#[doc = "DMA2 and DMAMUX reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DMA2 and DMAMUX"]
    B0x1 = 1,
}
impl From<Dma2rst> for bool {
    #[inline(always)]
    fn from(variant: Dma2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2RST` reader - DMA2 and DMAMUX reset Set and cleared by software."]
pub type Dma2rstR = crate::BitReader<Dma2rst>;
impl Dma2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma2rst {
        match self.bits {
            false => Dma2rst::B0x0,
            true => Dma2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma2rst::B0x0
    }
    #[doc = "Reset DMA2 and DMAMUX"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma2rst::B0x1
    }
}
#[doc = "Field `DMA2RST` writer - DMA2 and DMAMUX reset Set and cleared by software."]
pub type Dma2rstW<'a, REG> = crate::BitWriter<'a, REG, Dma2rst>;
impl<'a, REG> Dma2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2rst::B0x0)
    }
    #[doc = "Reset DMA2 and DMAMUX"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2rst::B0x1)
    }
}
#[doc = "Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset flash memory interface"]
    B0x1 = 1,
}
impl From<Flashrst> for bool {
    #[inline(always)]
    fn from(variant: Flashrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHRST` reader - Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode."]
pub type FlashrstR = crate::BitReader<Flashrst>;
impl FlashrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashrst {
        match self.bits {
            false => Flashrst::B0x0,
            true => Flashrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Flashrst::B0x0
    }
    #[doc = "Reset flash memory interface"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Flashrst::B0x1
    }
}
#[doc = "Field `FLASHRST` writer - Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode."]
pub type FlashrstW<'a, REG> = crate::BitWriter<'a, REG, Flashrst>;
impl<'a, REG> FlashrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashrst::B0x0)
    }
    #[doc = "Reset flash memory interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashrst::B0x1)
    }
}
#[doc = "CRC reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset CRC"]
    B0x1 = 1,
}
impl From<Crcrst> for bool {
    #[inline(always)]
    fn from(variant: Crcrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCRST` reader - CRC reset Set and cleared by software."]
pub type CrcrstR = crate::BitReader<Crcrst>;
impl CrcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcrst {
        match self.bits {
            false => Crcrst::B0x0,
            true => Crcrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcrst::B0x0
    }
    #[doc = "Reset CRC"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcrst::B0x1
    }
}
#[doc = "Field `CRCRST` writer - CRC reset Set and cleared by software."]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG, Crcrst>;
impl<'a, REG> CrcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcrst::B0x0)
    }
    #[doc = "Reset CRC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcrst::B0x1)
    }
}
#[doc = "AES hardware accelerator reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset AES"]
    B0x1 = 1,
}
impl From<Aesrst> for bool {
    #[inline(always)]
    fn from(variant: Aesrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESRST` reader - AES hardware accelerator reset Set and cleared by software."]
pub type AesrstR = crate::BitReader<Aesrst>;
impl AesrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesrst {
        match self.bits {
            false => Aesrst::B0x0,
            true => Aesrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Aesrst::B0x0
    }
    #[doc = "Reset AES"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Aesrst::B0x1
    }
}
#[doc = "Field `AESRST` writer - AES hardware accelerator reset Set and cleared by software."]
pub type AesrstW<'a, REG> = crate::BitWriter<'a, REG, Aesrst>;
impl<'a, REG> AesrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrst::B0x0)
    }
    #[doc = "Reset AES"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrst::B0x1)
    }
}
#[doc = "Random number generator reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rngrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset RNG"]
    B0x1 = 1,
}
impl From<Rngrst> for bool {
    #[inline(always)]
    fn from(variant: Rngrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGRST` reader - Random number generator reset Set and cleared by software."]
pub type RngrstR = crate::BitReader<Rngrst>;
impl RngrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rngrst {
        match self.bits {
            false => Rngrst::B0x0,
            true => Rngrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rngrst::B0x0
    }
    #[doc = "Reset RNG"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rngrst::B0x1
    }
}
#[doc = "Field `RNGRST` writer - Random number generator reset Set and cleared by software."]
pub type RngrstW<'a, REG> = crate::BitWriter<'a, REG, Rngrst>;
impl<'a, REG> RngrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rngrst::B0x0)
    }
    #[doc = "Reset RNG"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rngrst::B0x1)
    }
}
#[doc = "Touch sensing controller reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tscrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TSC"]
    B0x1 = 1,
}
impl From<Tscrst> for bool {
    #[inline(always)]
    fn from(variant: Tscrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCRST` reader - Touch sensing controller reset Set and cleared by software."]
pub type TscrstR = crate::BitReader<Tscrst>;
impl TscrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tscrst {
        match self.bits {
            false => Tscrst::B0x0,
            true => Tscrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tscrst::B0x0
    }
    #[doc = "Reset TSC"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tscrst::B0x1
    }
}
#[doc = "Field `TSCRST` writer - Touch sensing controller reset Set and cleared by software."]
pub type TscrstW<'a, REG> = crate::BitWriter<'a, REG, Tscrst>;
impl<'a, REG> TscrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tscrst::B0x0)
    }
    #[doc = "Reset TSC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tscrst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 and DMAMUX reset Set and cleared by software."]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 and DMAMUX reset Set and cleared by software."]
    #[inline(always)]
    pub fn dma2rst(&self) -> Dma2rstR {
        Dma2rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode."]
    #[inline(always)]
    pub fn flashrst(&self) -> FlashrstR {
        FlashrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset Set and cleared by software."]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset Set and cleared by software."]
    #[inline(always)]
    pub fn aesrst(&self) -> AesrstR {
        AesrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Random number generator reset Set and cleared by software."]
    #[inline(always)]
    pub fn rngrst(&self) -> RngrstR {
        RngrstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller reset Set and cleared by software."]
    #[inline(always)]
    pub fn tscrst(&self) -> TscrstR {
        TscrstR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 and DMAMUX reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> Dma1rstW<RccAhbrstrSpec> {
        Dma1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 and DMAMUX reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> Dma2rstW<RccAhbrstrSpec> {
        Dma2rstW::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the flash memory is in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FlashrstW<RccAhbrstrSpec> {
        FlashrstW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CrcrstW<RccAhbrstrSpec> {
        CrcrstW::new(self, 12)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AesrstW<RccAhbrstrSpec> {
        AesrstW::new(self, 16)
    }
    #[doc = "Bit 18 - Random number generator reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RngrstW<RccAhbrstrSpec> {
        RngrstW::new(self, 18)
    }
    #[doc = "Bit 24 - Touch sensing controller reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TscrstW<RccAhbrstrSpec> {
        TscrstW::new(self, 24)
    }
}
#[doc = "AHB peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahbrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahbrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhbrstrSpec;
impl crate::RegisterSpec for RccAhbrstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahbrstr::R`](R) reader structure"]
impl crate::Readable for RccAhbrstrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahbrstr::W`](W) writer structure"]
impl crate::Writable for RccAhbrstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHBRSTR to value 0"]
impl crate::Resettable for RccAhbrstrSpec {
    const RESET_VALUE: u32 = 0;
}
