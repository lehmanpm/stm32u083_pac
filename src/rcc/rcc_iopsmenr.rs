#[doc = "Register `RCC_IOPSMENR` reader"]
pub type R = crate::R<RccIopsmenrSpec>;
#[doc = "Register `RCC_IOPSMENR` writer"]
pub type W = crate::W<RccIopsmenrSpec>;
#[doc = "I/O port A clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioasmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpioasmen> for bool {
    #[inline(always)]
    fn from(variant: Gpioasmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOASMEN` reader - I/O port A clock enable during Sleep mode Set and cleared by software."]
pub type GpioasmenR = crate::BitReader<Gpioasmen>;
impl GpioasmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioasmen {
        match self.bits {
            false => Gpioasmen::B0x0,
            true => Gpioasmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioasmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioasmen::B0x1
    }
}
#[doc = "Field `GPIOASMEN` writer - I/O port A clock enable during Sleep mode Set and cleared by software."]
pub type GpioasmenW<'a, REG> = crate::BitWriter<'a, REG, Gpioasmen>;
impl<'a, REG> GpioasmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioasmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioasmen::B0x1)
    }
}
#[doc = "I/O port B clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiobsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpiobsmen> for bool {
    #[inline(always)]
    fn from(variant: Gpiobsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOBSMEN` reader - I/O port B clock enable during Sleep mode Set and cleared by software."]
pub type GpiobsmenR = crate::BitReader<Gpiobsmen>;
impl GpiobsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiobsmen {
        match self.bits {
            false => Gpiobsmen::B0x0,
            true => Gpiobsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiobsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiobsmen::B0x1
    }
}
#[doc = "Field `GPIOBSMEN` writer - I/O port B clock enable during Sleep mode Set and cleared by software."]
pub type GpiobsmenW<'a, REG> = crate::BitWriter<'a, REG, Gpiobsmen>;
impl<'a, REG> GpiobsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiobsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiobsmen::B0x1)
    }
}
#[doc = "I/O port C clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiocsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpiocsmen> for bool {
    #[inline(always)]
    fn from(variant: Gpiocsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOCSMEN` reader - I/O port C clock enable during Sleep mode Set and cleared by software."]
pub type GpiocsmenR = crate::BitReader<Gpiocsmen>;
impl GpiocsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiocsmen {
        match self.bits {
            false => Gpiocsmen::B0x0,
            true => Gpiocsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiocsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiocsmen::B0x1
    }
}
#[doc = "Field `GPIOCSMEN` writer - I/O port C clock enable during Sleep mode Set and cleared by software."]
pub type GpiocsmenW<'a, REG> = crate::BitWriter<'a, REG, Gpiocsmen>;
impl<'a, REG> GpiocsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocsmen::B0x1)
    }
}
#[doc = "I/O port D clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiodsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpiodsmen> for bool {
    #[inline(always)]
    fn from(variant: Gpiodsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIODSMEN` reader - I/O port D clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type GpiodsmenR = crate::BitReader<Gpiodsmen>;
impl GpiodsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiodsmen {
        match self.bits {
            false => Gpiodsmen::B0x0,
            true => Gpiodsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiodsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiodsmen::B0x1
    }
}
#[doc = "Field `GPIODSMEN` writer - I/O port D clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type GpiodsmenW<'a, REG> = crate::BitWriter<'a, REG, Gpiodsmen>;
impl<'a, REG> GpiodsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiodsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiodsmen::B0x1)
    }
}
#[doc = "I/O port E clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioesmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpioesmen> for bool {
    #[inline(always)]
    fn from(variant: Gpioesmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOESMEN` reader - I/O port E clock enable during Sleep mode Set and cleared by software."]
pub type GpioesmenR = crate::BitReader<Gpioesmen>;
impl GpioesmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioesmen {
        match self.bits {
            false => Gpioesmen::B0x0,
            true => Gpioesmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioesmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioesmen::B0x1
    }
}
#[doc = "Field `GPIOESMEN` writer - I/O port E clock enable during Sleep mode Set and cleared by software."]
pub type GpioesmenW<'a, REG> = crate::BitWriter<'a, REG, Gpioesmen>;
impl<'a, REG> GpioesmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioesmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioesmen::B0x1)
    }
}
#[doc = "I/O port F clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiofsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpiofsmen> for bool {
    #[inline(always)]
    fn from(variant: Gpiofsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOFSMEN` reader - I/O port F clock enable during Sleep mode Set and cleared by software."]
pub type GpiofsmenR = crate::BitReader<Gpiofsmen>;
impl GpiofsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiofsmen {
        match self.bits {
            false => Gpiofsmen::B0x0,
            true => Gpiofsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiofsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiofsmen::B0x1
    }
}
#[doc = "Field `GPIOFSMEN` writer - I/O port F clock enable during Sleep mode Set and cleared by software."]
pub type GpiofsmenW<'a, REG> = crate::BitWriter<'a, REG, Gpiofsmen>;
impl<'a, REG> GpiofsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofsmen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GpioasmenR {
        GpioasmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GpiobsmenR {
        GpiobsmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GpiocsmenR {
        GpiocsmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GpiodsmenR {
        GpiodsmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GpioesmenR {
        GpioesmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GpiofsmenR {
        GpiofsmenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GpioasmenW<RccIopsmenrSpec> {
        GpioasmenW::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GpiobsmenW<RccIopsmenrSpec> {
        GpiobsmenW::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GpiocsmenW<RccIopsmenrSpec> {
        GpiocsmenW::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GpiodsmenW<RccIopsmenrSpec> {
        GpiodsmenW::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GpioesmenW<RccIopsmenrSpec> {
        GpioesmenW::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GpiofsmenW<RccIopsmenrSpec> {
        GpiofsmenW::new(self, 5)
    }
}
#[doc = "I/O port in Sleep mode clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_iopsmenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_iopsmenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccIopsmenrSpec;
impl crate::RegisterSpec for RccIopsmenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_iopsmenr::R`](R) reader structure"]
impl crate::Readable for RccIopsmenrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_iopsmenr::W`](W) writer structure"]
impl crate::Writable for RccIopsmenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_IOPSMENR to value 0x3f"]
impl crate::Resettable for RccIopsmenrSpec {
    const RESET_VALUE: u32 = 0x3f;
}
