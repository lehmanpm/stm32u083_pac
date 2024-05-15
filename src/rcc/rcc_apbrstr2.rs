#[doc = "Register `RCC_APBRSTR2` reader"]
pub type R = crate::R<RccApbrstr2Spec>;
#[doc = "Register `RCC_APBRSTR2` writer"]
pub type W = crate::W<RccApbrstr2Spec>;
#[doc = "SYSCFG, COMP and VREFBUF reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syscfgrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SYSCFG + COMP + VREFBUF"]
    B0x1 = 1,
}
impl From<Syscfgrst> for bool {
    #[inline(always)]
    fn from(variant: Syscfgrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGRST` reader - SYSCFG, COMP and VREFBUF reset Set and cleared by software."]
pub type SyscfgrstR = crate::BitReader<Syscfgrst>;
impl SyscfgrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syscfgrst {
        match self.bits {
            false => Syscfgrst::B0x0,
            true => Syscfgrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syscfgrst::B0x0
    }
    #[doc = "Reset SYSCFG + COMP + VREFBUF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syscfgrst::B0x1
    }
}
#[doc = "Field `SYSCFGRST` writer - SYSCFG, COMP and VREFBUF reset Set and cleared by software."]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG, Syscfgrst>;
impl<'a, REG> SyscfgrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgrst::B0x0)
    }
    #[doc = "Reset SYSCFG + COMP + VREFBUF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgrst::B0x1)
    }
}
#[doc = "TIM1 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM1 timer"]
    B0x1 = 1,
}
impl From<Tim1rst> for bool {
    #[inline(always)]
    fn from(variant: Tim1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1RST` reader - TIM1 timer reset Set and cleared by software."]
pub type Tim1rstR = crate::BitReader<Tim1rst>;
impl Tim1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim1rst {
        match self.bits {
            false => Tim1rst::B0x0,
            true => Tim1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim1rst::B0x0
    }
    #[doc = "Reset TIM1 timer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim1rst::B0x1
    }
}
#[doc = "Field `TIM1RST` writer - TIM1 timer reset Set and cleared by software."]
pub type Tim1rstW<'a, REG> = crate::BitWriter<'a, REG, Tim1rst>;
impl<'a, REG> Tim1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1rst::B0x0)
    }
    #[doc = "Reset TIM1 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1rst::B0x1)
    }
}
#[doc = "SPI1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SPI1"]
    B0x1 = 1,
}
impl From<Spi1rst> for bool {
    #[inline(always)]
    fn from(variant: Spi1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1RST` reader - SPI1 reset Set and cleared by software."]
pub type Spi1rstR = crate::BitReader<Spi1rst>;
impl Spi1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi1rst {
        match self.bits {
            false => Spi1rst::B0x0,
            true => Spi1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi1rst::B0x0
    }
    #[doc = "Reset SPI1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi1rst::B0x1
    }
}
#[doc = "Field `SPI1RST` writer - SPI1 reset Set and cleared by software."]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG, Spi1rst>;
impl<'a, REG> Spi1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1rst::B0x0)
    }
    #[doc = "Reset SPI1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1rst::B0x1)
    }
}
#[doc = "USART1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset USART1"]
    B0x1 = 1,
}
impl From<Usart1rst> for bool {
    #[inline(always)]
    fn from(variant: Usart1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1RST` reader - USART1 reset Set and cleared by software."]
pub type Usart1rstR = crate::BitReader<Usart1rst>;
impl Usart1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1rst {
        match self.bits {
            false => Usart1rst::B0x0,
            true => Usart1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart1rst::B0x0
    }
    #[doc = "Reset USART1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart1rst::B0x1
    }
}
#[doc = "Field `USART1RST` writer - USART1 reset Set and cleared by software."]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG, Usart1rst>;
impl<'a, REG> Usart1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1rst::B0x0)
    }
    #[doc = "Reset USART1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1rst::B0x1)
    }
}
#[doc = "TIM15 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim15rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM15 timer"]
    B0x1 = 1,
}
impl From<Tim15rst> for bool {
    #[inline(always)]
    fn from(variant: Tim15rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM15RST` reader - TIM15 timer reset Set and cleared by software."]
pub type Tim15rstR = crate::BitReader<Tim15rst>;
impl Tim15rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim15rst {
        match self.bits {
            false => Tim15rst::B0x0,
            true => Tim15rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim15rst::B0x0
    }
    #[doc = "Reset TIM15 timer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim15rst::B0x1
    }
}
#[doc = "Field `TIM15RST` writer - TIM15 timer reset Set and cleared by software."]
pub type Tim15rstW<'a, REG> = crate::BitWriter<'a, REG, Tim15rst>;
impl<'a, REG> Tim15rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15rst::B0x0)
    }
    #[doc = "Reset TIM15 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15rst::B0x1)
    }
}
#[doc = "TIM16 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim16rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM16 timer"]
    B0x1 = 1,
}
impl From<Tim16rst> for bool {
    #[inline(always)]
    fn from(variant: Tim16rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM16RST` reader - TIM16 timer reset Set and cleared by software."]
pub type Tim16rstR = crate::BitReader<Tim16rst>;
impl Tim16rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim16rst {
        match self.bits {
            false => Tim16rst::B0x0,
            true => Tim16rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim16rst::B0x0
    }
    #[doc = "Reset TIM16 timer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim16rst::B0x1
    }
}
#[doc = "Field `TIM16RST` writer - TIM16 timer reset Set and cleared by software."]
pub type Tim16rstW<'a, REG> = crate::BitWriter<'a, REG, Tim16rst>;
impl<'a, REG> Tim16rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16rst::B0x0)
    }
    #[doc = "Reset TIM16 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16rst::B0x1)
    }
}
#[doc = "ADC reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset ADC"]
    B0x1 = 1,
}
impl From<Adcrst> for bool {
    #[inline(always)]
    fn from(variant: Adcrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCRST` reader - ADC reset Set and cleared by software."]
pub type AdcrstR = crate::BitReader<Adcrst>;
impl AdcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcrst {
        match self.bits {
            false => Adcrst::B0x0,
            true => Adcrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adcrst::B0x0
    }
    #[doc = "Reset ADC"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adcrst::B0x1
    }
}
#[doc = "Field `ADCRST` writer - ADC reset Set and cleared by software."]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG, Adcrst>;
impl<'a, REG> AdcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcrst::B0x0)
    }
    #[doc = "Reset ADC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcrst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF reset Set and cleared by software."]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim1rst(&self) -> Tim1rstR {
        Tim1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim15rst(&self) -> Tim15rstR {
        Tim15rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim16rst(&self) -> Tim16rstR {
        Tim16rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC reset Set and cleared by software."]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<RccApbrstr2Spec> {
        SyscfgrstW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> Tim1rstW<RccApbrstr2Spec> {
        Tim1rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> Spi1rstW<RccApbrstr2Spec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> Usart1rstW<RccApbrstr2Spec> {
        Usart1rstW::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> Tim15rstW<RccApbrstr2Spec> {
        Tim15rstW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> Tim16rstW<RccApbrstr2Spec> {
        Tim16rstW::new(self, 17)
    }
    #[doc = "Bit 20 - ADC reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> AdcrstW<RccApbrstr2Spec> {
        AdcrstW::new(self, 20)
    }
}
#[doc = "APB peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbrstr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbrstr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApbrstr2Spec;
impl crate::RegisterSpec for RccApbrstr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apbrstr2::R`](R) reader structure"]
impl crate::Readable for RccApbrstr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apbrstr2::W`](W) writer structure"]
impl crate::Writable for RccApbrstr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APBRSTR2 to value 0"]
impl crate::Resettable for RccApbrstr2Spec {
    const RESET_VALUE: u32 = 0;
}
