#[doc = "Register `RCC_AHBENR` reader"]
pub type R = crate::R<RccAhbenrSpec>;
#[doc = "Register `RCC_AHBENR` writer"]
pub type W = crate::W<RccAhbenrSpec>;
#[doc = "DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Dma1en> for bool {
    #[inline(always)]
    fn from(variant: Dma1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1EN` reader - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled."]
pub type Dma1enR = crate::BitReader<Dma1en>;
impl Dma1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1en {
        match self.bits {
            false => Dma1en::B0x0,
            true => Dma1en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma1en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma1en::B0x1
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled."]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG, Dma1en>;
impl<'a, REG> Dma1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1en::B0x1)
    }
}
#[doc = "DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma2en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Dma2en> for bool {
    #[inline(always)]
    fn from(variant: Dma2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2EN` reader - DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled."]
pub type Dma2enR = crate::BitReader<Dma2en>;
impl Dma2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma2en {
        match self.bits {
            false => Dma2en::B0x0,
            true => Dma2en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma2en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma2en::B0x1
    }
}
#[doc = "Field `DMA2EN` writer - DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled."]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG, Dma2en>;
impl<'a, REG> Dma2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2en::B0x1)
    }
}
#[doc = "Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Flashen> for bool {
    #[inline(always)]
    fn from(variant: Flashen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHEN` reader - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode."]
pub type FlashenR = crate::BitReader<Flashen>;
impl FlashenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashen {
        match self.bits {
            false => Flashen::B0x0,
            true => Flashen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Flashen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Flashen::B0x1
    }
}
#[doc = "Field `FLASHEN` writer - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode."]
pub type FlashenW<'a, REG> = crate::BitWriter<'a, REG, Flashen>;
impl<'a, REG> FlashenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashen::B0x1)
    }
}
#[doc = "CRC clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Crcen> for bool {
    #[inline(always)]
    fn from(variant: Crcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - CRC clock enable Set and cleared by software."]
pub type CrcenR = crate::BitReader<Crcen>;
impl CrcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcen {
        match self.bits {
            false => Crcen::B0x0,
            true => Crcen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcen::B0x1
    }
}
#[doc = "Field `CRCEN` writer - CRC clock enable Set and cleared by software."]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG, Crcen>;
impl<'a, REG> CrcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcen::B0x1)
    }
}
#[doc = "AES hardware accelerator Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Aesen> for bool {
    #[inline(always)]
    fn from(variant: Aesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESEN` reader - AES hardware accelerator Set and cleared by software."]
pub type AesenR = crate::BitReader<Aesen>;
impl AesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesen {
        match self.bits {
            false => Aesen::B0x0,
            true => Aesen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Aesen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Aesen::B0x1
    }
}
#[doc = "Field `AESEN` writer - AES hardware accelerator Set and cleared by software."]
pub type AesenW<'a, REG> = crate::BitWriter<'a, REG, Aesen>;
impl<'a, REG> AesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesen::B0x1)
    }
}
#[doc = "Random number generator clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rngen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rngen> for bool {
    #[inline(always)]
    fn from(variant: Rngen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - Random number generator clock enable Set and cleared by software."]
pub type RngenR = crate::BitReader<Rngen>;
impl RngenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rngen {
        match self.bits {
            false => Rngen::B0x0,
            true => Rngen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rngen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rngen::B0x1
    }
}
#[doc = "Field `RNGEN` writer - Random number generator clock enable Set and cleared by software."]
pub type RngenW<'a, REG> = crate::BitWriter<'a, REG, Rngen>;
impl<'a, REG> RngenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rngen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rngen::B0x1)
    }
}
#[doc = "Touch sensing controller clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tscen {
    #[doc = "0: TSC clock disable"]
    B0x0 = 0,
    #[doc = "1: TSC clock enable"]
    B0x1 = 1,
}
impl From<Tscen> for bool {
    #[inline(always)]
    fn from(variant: Tscen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCEN` reader - Touch sensing controller clock enable Set and cleared by software."]
pub type TscenR = crate::BitReader<Tscen>;
impl TscenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tscen {
        match self.bits {
            false => Tscen::B0x0,
            true => Tscen::B0x1,
        }
    }
    #[doc = "TSC clock disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tscen::B0x0
    }
    #[doc = "TSC clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tscen::B0x1
    }
}
#[doc = "Field `TSCEN` writer - Touch sensing controller clock enable Set and cleared by software."]
pub type TscenW<'a, REG> = crate::BitWriter<'a, REG, Tscen>;
impl<'a, REG> TscenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSC clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tscen::B0x0)
    }
    #[doc = "TSC clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tscen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled."]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled."]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode."]
    #[inline(always)]
    pub fn flashen(&self) -> FlashenR {
        FlashenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator Set and cleared by software."]
    #[inline(always)]
    pub fn aesen(&self) -> AesenR {
        AesenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Random number generator clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn rngen(&self) -> RngenR {
        RngenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tscen(&self) -> TscenR {
        TscenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> Dma1enW<RccAhbenrSpec> {
        Dma1enW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> Dma2enW<RccAhbenrSpec> {
        Dma2enW::new(self, 1)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the flash memory is in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FlashenW<RccAhbenrSpec> {
        FlashenW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<RccAhbenrSpec> {
        CrcenW::new(self, 12)
    }
    #[doc = "Bit 16 - AES hardware accelerator Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AesenW<RccAhbenrSpec> {
        AesenW::new(self, 16)
    }
    #[doc = "Bit 18 - Random number generator clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RngenW<RccAhbenrSpec> {
        RngenW::new(self, 18)
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tscen(&mut self) -> TscenW<RccAhbenrSpec> {
        TscenW::new(self, 24)
    }
}
#[doc = "AHB peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahbenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahbenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhbenrSpec;
impl crate::RegisterSpec for RccAhbenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahbenr::R`](R) reader structure"]
impl crate::Readable for RccAhbenrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahbenr::W`](W) writer structure"]
impl crate::Writable for RccAhbenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHBENR to value 0x0100"]
impl crate::Resettable for RccAhbenrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
