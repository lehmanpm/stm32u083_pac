#[doc = "Register `RCC_APBENR2` reader"]
pub type R = crate::R<RccApbenr2Spec>;
#[doc = "Register `RCC_APBENR2` writer"]
pub type W = crate::W<RccApbenr2Spec>;
#[doc = "SYSCFG, COMP and VREFBUF clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syscfgen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Syscfgen> for bool {
    #[inline(always)]
    fn from(variant: Syscfgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGEN` reader - SYSCFG, COMP and VREFBUF clock enable Set and cleared by software."]
pub type SyscfgenR = crate::BitReader<Syscfgen>;
impl SyscfgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syscfgen {
        match self.bits {
            false => Syscfgen::B0x0,
            true => Syscfgen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syscfgen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syscfgen::B0x1
    }
}
#[doc = "Field `SYSCFGEN` writer - SYSCFG, COMP and VREFBUF clock enable Set and cleared by software."]
pub type SyscfgenW<'a, REG> = crate::BitWriter<'a, REG, Syscfgen>;
impl<'a, REG> SyscfgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgen::B0x1)
    }
}
#[doc = "TIM1 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim1en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim1en> for bool {
    #[inline(always)]
    fn from(variant: Tim1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable Set and cleared by software."]
pub type Tim1enR = crate::BitReader<Tim1en>;
impl Tim1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim1en {
        match self.bits {
            false => Tim1en::B0x0,
            true => Tim1en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim1en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim1en::B0x1
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable Set and cleared by software."]
pub type Tim1enW<'a, REG> = crate::BitWriter<'a, REG, Tim1en>;
impl<'a, REG> Tim1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1en::B0x1)
    }
}
#[doc = "SPI1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi1en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Spi1en> for bool {
    #[inline(always)]
    fn from(variant: Spi1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1EN` reader - SPI1 clock enable Set and cleared by software."]
pub type Spi1enR = crate::BitReader<Spi1en>;
impl Spi1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi1en {
        match self.bits {
            false => Spi1en::B0x0,
            true => Spi1en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi1en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi1en::B0x1
    }
}
#[doc = "Field `SPI1EN` writer - SPI1 clock enable Set and cleared by software."]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG, Spi1en>;
impl<'a, REG> Spi1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1en::B0x1)
    }
}
#[doc = "USART1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart1en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usart1en> for bool {
    #[inline(always)]
    fn from(variant: Usart1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1EN` reader - USART1 clock enable Set and cleared by software."]
pub type Usart1enR = crate::BitReader<Usart1en>;
impl Usart1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1en {
        match self.bits {
            false => Usart1en::B0x0,
            true => Usart1en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart1en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart1en::B0x1
    }
}
#[doc = "Field `USART1EN` writer - USART1 clock enable Set and cleared by software."]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG, Usart1en>;
impl<'a, REG> Usart1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1en::B0x1)
    }
}
#[doc = "TIM15 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim15en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim15en> for bool {
    #[inline(always)]
    fn from(variant: Tim15en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable Set and cleared by software."]
pub type Tim15enR = crate::BitReader<Tim15en>;
impl Tim15enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim15en {
        match self.bits {
            false => Tim15en::B0x0,
            true => Tim15en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim15en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim15en::B0x1
    }
}
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable Set and cleared by software."]
pub type Tim15enW<'a, REG> = crate::BitWriter<'a, REG, Tim15en>;
impl<'a, REG> Tim15enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15en::B0x1)
    }
}
#[doc = "TIM16 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim16en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim16en> for bool {
    #[inline(always)]
    fn from(variant: Tim16en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable Set and cleared by software."]
pub type Tim16enR = crate::BitReader<Tim16en>;
impl Tim16enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim16en {
        match self.bits {
            false => Tim16en::B0x0,
            true => Tim16en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim16en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim16en::B0x1
    }
}
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable Set and cleared by software."]
pub type Tim16enW<'a, REG> = crate::BitWriter<'a, REG, Tim16en>;
impl<'a, REG> Tim16enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16en::B0x1)
    }
}
#[doc = "ADC clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Adcen> for bool {
    #[inline(always)]
    fn from(variant: Adcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - ADC clock enable Set and cleared by software."]
pub type AdcenR = crate::BitReader<Adcen>;
impl AdcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcen {
        match self.bits {
            false => Adcen::B0x0,
            true => Adcen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adcen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adcen::B0x1
    }
}
#[doc = "Field `ADCEN` writer - ADC clock enable Set and cleared by software."]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG, Adcen>;
impl<'a, REG> AdcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim1en(&self) -> Tim1enR {
        Tim1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim15en(&self) -> Tim15enR {
        Tim15enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim16en(&self) -> Tim16enR {
        Tim16enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SyscfgenW<RccApbenr2Spec> {
        SyscfgenW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> Tim1enW<RccApbenr2Spec> {
        Tim1enW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> Spi1enW<RccApbenr2Spec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> Usart1enW<RccApbenr2Spec> {
        Usart1enW::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> Tim15enW<RccApbenr2Spec> {
        Tim15enW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> Tim16enW<RccApbenr2Spec> {
        Tim16enW::new(self, 17)
    }
    #[doc = "Bit 20 - ADC clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> AdcenW<RccApbenr2Spec> {
        AdcenW::new(self, 20)
    }
}
#[doc = "APB peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApbenr2Spec;
impl crate::RegisterSpec for RccApbenr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apbenr2::R`](R) reader structure"]
impl crate::Readable for RccApbenr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apbenr2::W`](W) writer structure"]
impl crate::Writable for RccApbenr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APBENR2 to value 0"]
impl crate::Resettable for RccApbenr2Spec {
    const RESET_VALUE: u32 = 0;
}
