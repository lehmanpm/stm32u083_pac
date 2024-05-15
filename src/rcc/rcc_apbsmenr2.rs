#[doc = "Register `RCC_APBSMENR2` reader"]
pub type R = crate::R<RccApbsmenr2Spec>;
#[doc = "Register `RCC_APBSMENR2` writer"]
pub type W = crate::W<RccApbsmenr2Spec>;
#[doc = "SYSCFG, COMP and VREFBUF clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syscfgsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Syscfgsmen> for bool {
    #[inline(always)]
    fn from(variant: Syscfgsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG, COMP and VREFBUF clock enable during Sleep and Stop modes Set and cleared by software."]
pub type SyscfgsmenR = crate::BitReader<Syscfgsmen>;
impl SyscfgsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syscfgsmen {
        match self.bits {
            false => Syscfgsmen::B0x0,
            true => Syscfgsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syscfgsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syscfgsmen::B0x1
    }
}
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG, COMP and VREFBUF clock enable during Sleep and Stop modes Set and cleared by software."]
pub type SyscfgsmenW<'a, REG> = crate::BitWriter<'a, REG, Syscfgsmen>;
impl<'a, REG> SyscfgsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgsmen::B0x1)
    }
}
#[doc = "TIM1 timer clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim1smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim1smen> for bool {
    #[inline(always)]
    fn from(variant: Tim1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim1smenR = crate::BitReader<Tim1smen>;
impl Tim1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim1smen {
        match self.bits {
            false => Tim1smen::B0x0,
            true => Tim1smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim1smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim1smen::B0x1
    }
}
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim1smenW<'a, REG> = crate::BitWriter<'a, REG, Tim1smen>;
impl<'a, REG> Tim1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1smen::B0x1)
    }
}
#[doc = "SPI1 clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi1smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Spi1smen> for bool {
    #[inline(always)]
    fn from(variant: Spi1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during Sleep mode Set and cleared by software."]
pub type Spi1smenR = crate::BitReader<Spi1smen>;
impl Spi1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi1smen {
        match self.bits {
            false => Spi1smen::B0x0,
            true => Spi1smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi1smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi1smen::B0x1
    }
}
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during Sleep mode Set and cleared by software."]
pub type Spi1smenW<'a, REG> = crate::BitWriter<'a, REG, Spi1smen>;
impl<'a, REG> Spi1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1smen::B0x1)
    }
}
#[doc = "USART1 clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart1smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usart1smen> for bool {
    #[inline(always)]
    fn from(variant: Usart1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart1smenR = crate::BitReader<Usart1smen>;
impl Usart1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1smen {
        match self.bits {
            false => Usart1smen::B0x0,
            true => Usart1smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart1smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart1smen::B0x1
    }
}
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart1smenW<'a, REG> = crate::BitWriter<'a, REG, Usart1smen>;
impl<'a, REG> Usart1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1smen::B0x1)
    }
}
#[doc = "TIM15 timer clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim15smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim15smen> for bool {
    #[inline(always)]
    fn from(variant: Tim15smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim15smenR = crate::BitReader<Tim15smen>;
impl Tim15smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim15smen {
        match self.bits {
            false => Tim15smen::B0x0,
            true => Tim15smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim15smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim15smen::B0x1
    }
}
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim15smenW<'a, REG> = crate::BitWriter<'a, REG, Tim15smen>;
impl<'a, REG> Tim15smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15smen::B0x1)
    }
}
#[doc = "TIM16 timer clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim16smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim16smen> for bool {
    #[inline(always)]
    fn from(variant: Tim16smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim16smenR = crate::BitReader<Tim16smen>;
impl Tim16smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim16smen {
        match self.bits {
            false => Tim16smen::B0x0,
            true => Tim16smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim16smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim16smen::B0x1
    }
}
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim16smenW<'a, REG> = crate::BitWriter<'a, REG, Tim16smen>;
impl<'a, REG> Tim16smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16smen::B0x1)
    }
}
#[doc = "ADC clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Adcsmen> for bool {
    #[inline(always)]
    fn from(variant: Adcsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCSMEN` reader - ADC clock enable during Sleep mode Set and cleared by software."]
pub type AdcsmenR = crate::BitReader<Adcsmen>;
impl AdcsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsmen {
        match self.bits {
            false => Adcsmen::B0x0,
            true => Adcsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adcsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adcsmen::B0x1
    }
}
#[doc = "Field `ADCSMEN` writer - ADC clock enable during Sleep mode Set and cleared by software."]
pub type AdcsmenW<'a, REG> = crate::BitWriter<'a, REG, Adcsmen>;
impl<'a, REG> AdcsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsmen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SyscfgsmenR {
        SyscfgsmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim1smen(&self) -> Tim1smenR {
        Tim1smenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn spi1smen(&self) -> Spi1smenR {
        Spi1smenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart1smen(&self) -> Usart1smenR {
        Usart1smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim15smen(&self) -> Tim15smenR {
        Tim15smenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim16smen(&self) -> Tim16smenR {
        Tim16smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn adcsmen(&self) -> AdcsmenR {
        AdcsmenR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SyscfgsmenW<RccApbsmenr2Spec> {
        SyscfgsmenW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> Tim1smenW<RccApbsmenr2Spec> {
        Tim1smenW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> Spi1smenW<RccApbsmenr2Spec> {
        Spi1smenW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> Usart1smenW<RccApbsmenr2Spec> {
        Usart1smenW::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15smen(&mut self) -> Tim15smenW<RccApbsmenr2Spec> {
        Tim15smenW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> Tim16smenW<RccApbsmenr2Spec> {
        Tim16smenW::new(self, 17)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adcsmen(&mut self) -> AdcsmenW<RccApbsmenr2Spec> {
        AdcsmenW::new(self, 20)
    }
}
#[doc = "APB peripheral clock enable in Sleep/Stop mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbsmenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbsmenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApbsmenr2Spec;
impl crate::RegisterSpec for RccApbsmenr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apbsmenr2::R`](R) reader structure"]
impl crate::Readable for RccApbsmenr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apbsmenr2::W`](W) writer structure"]
impl crate::Writable for RccApbsmenr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APBSMENR2 to value 0x0017_d801"]
impl crate::Resettable for RccApbsmenr2Spec {
    const RESET_VALUE: u32 = 0x0017_d801;
}
