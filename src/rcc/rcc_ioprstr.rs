#[doc = "Register `RCC_IOPRSTR` reader"]
pub type R = crate::R<RccIoprstrSpec>;
#[doc = "Register `RCC_IOPRSTR` writer"]
pub type W = crate::W<RccIoprstrSpec>;
#[doc = "I/O port A reset This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioarst {
    #[doc = "0: no effect"]
    B0x0 = 0,
    #[doc = "1: Reset I/O port A"]
    B0x1 = 1,
}
impl From<Gpioarst> for bool {
    #[inline(always)]
    fn from(variant: Gpioarst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOARST` reader - I/O port A reset This bit is set and cleared by software."]
pub type GpioarstR = crate::BitReader<Gpioarst>;
impl GpioarstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioarst {
        match self.bits {
            false => Gpioarst::B0x0,
            true => Gpioarst::B0x1,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioarst::B0x0
    }
    #[doc = "Reset I/O port A"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioarst::B0x1
    }
}
#[doc = "Field `GPIOARST` writer - I/O port A reset This bit is set and cleared by software."]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG, Gpioarst>;
impl<'a, REG> GpioarstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioarst::B0x0)
    }
    #[doc = "Reset I/O port A"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioarst::B0x1)
    }
}
#[doc = "I/O port B reset This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiobrst {
    #[doc = "0: no effect"]
    B0x0 = 0,
    #[doc = "1: Reset I/O port B"]
    B0x1 = 1,
}
impl From<Gpiobrst> for bool {
    #[inline(always)]
    fn from(variant: Gpiobrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOBRST` reader - I/O port B reset This bit is set and cleared by software."]
pub type GpiobrstR = crate::BitReader<Gpiobrst>;
impl GpiobrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiobrst {
        match self.bits {
            false => Gpiobrst::B0x0,
            true => Gpiobrst::B0x1,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiobrst::B0x0
    }
    #[doc = "Reset I/O port B"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiobrst::B0x1
    }
}
#[doc = "Field `GPIOBRST` writer - I/O port B reset This bit is set and cleared by software."]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG, Gpiobrst>;
impl<'a, REG> GpiobrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiobrst::B0x0)
    }
    #[doc = "Reset I/O port B"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiobrst::B0x1)
    }
}
#[doc = "I/O port C reset This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiocrst {
    #[doc = "0: no effect"]
    B0x0 = 0,
    #[doc = "1: Reset I/O port C"]
    B0x1 = 1,
}
impl From<Gpiocrst> for bool {
    #[inline(always)]
    fn from(variant: Gpiocrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOCRST` reader - I/O port C reset This bit is set and cleared by software."]
pub type GpiocrstR = crate::BitReader<Gpiocrst>;
impl GpiocrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiocrst {
        match self.bits {
            false => Gpiocrst::B0x0,
            true => Gpiocrst::B0x1,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiocrst::B0x0
    }
    #[doc = "Reset I/O port C"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiocrst::B0x1
    }
}
#[doc = "Field `GPIOCRST` writer - I/O port C reset This bit is set and cleared by software."]
pub type GpiocrstW<'a, REG> = crate::BitWriter<'a, REG, Gpiocrst>;
impl<'a, REG> GpiocrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocrst::B0x0)
    }
    #[doc = "Reset I/O port C"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocrst::B0x1)
    }
}
#[doc = "I/O port D reset This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiodrst {
    #[doc = "0: no effect"]
    B0x0 = 0,
    #[doc = "1: Reset I/O port D"]
    B0x1 = 1,
}
impl From<Gpiodrst> for bool {
    #[inline(always)]
    fn from(variant: Gpiodrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIODRST` reader - I/O port D reset This bit is set and cleared by software."]
pub type GpiodrstR = crate::BitReader<Gpiodrst>;
impl GpiodrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiodrst {
        match self.bits {
            false => Gpiodrst::B0x0,
            true => Gpiodrst::B0x1,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiodrst::B0x0
    }
    #[doc = "Reset I/O port D"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiodrst::B0x1
    }
}
#[doc = "Field `GPIODRST` writer - I/O port D reset This bit is set and cleared by software."]
pub type GpiodrstW<'a, REG> = crate::BitWriter<'a, REG, Gpiodrst>;
impl<'a, REG> GpiodrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiodrst::B0x0)
    }
    #[doc = "Reset I/O port D"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiodrst::B0x1)
    }
}
#[doc = "I/O port E reset This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioerst {
    #[doc = "0: no effect"]
    B0x0 = 0,
    #[doc = "1: Reset I/O port E"]
    B0x1 = 1,
}
impl From<Gpioerst> for bool {
    #[inline(always)]
    fn from(variant: Gpioerst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOERST` reader - I/O port E reset This bit is set and cleared by software."]
pub type GpioerstR = crate::BitReader<Gpioerst>;
impl GpioerstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioerst {
        match self.bits {
            false => Gpioerst::B0x0,
            true => Gpioerst::B0x1,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioerst::B0x0
    }
    #[doc = "Reset I/O port E"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioerst::B0x1
    }
}
#[doc = "Field `GPIOERST` writer - I/O port E reset This bit is set and cleared by software."]
pub type GpioerstW<'a, REG> = crate::BitWriter<'a, REG, Gpioerst>;
impl<'a, REG> GpioerstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioerst::B0x0)
    }
    #[doc = "Reset I/O port E"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioerst::B0x1)
    }
}
#[doc = "I/O port F reset This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiofrst {
    #[doc = "0: no effect"]
    B0x0 = 0,
    #[doc = "1: Reset I/O port F"]
    B0x1 = 1,
}
impl From<Gpiofrst> for bool {
    #[inline(always)]
    fn from(variant: Gpiofrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOFRST` reader - I/O port F reset This bit is set and cleared by software."]
pub type GpiofrstR = crate::BitReader<Gpiofrst>;
impl GpiofrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiofrst {
        match self.bits {
            false => Gpiofrst::B0x0,
            true => Gpiofrst::B0x1,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiofrst::B0x0
    }
    #[doc = "Reset I/O port F"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiofrst::B0x1
    }
}
#[doc = "Field `GPIOFRST` writer - I/O port F reset This bit is set and cleared by software."]
pub type GpiofrstW<'a, REG> = crate::BitWriter<'a, REG, Gpiofrst>;
impl<'a, REG> GpiofrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofrst::B0x0)
    }
    #[doc = "Reset I/O port F"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofrst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - I/O port A reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GpiodrstR {
        GpiodrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioerst(&self) -> GpioerstR {
        GpioerstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F reset This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GpiofrstR {
        GpiofrstR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GpioarstW<RccIoprstrSpec> {
        GpioarstW::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GpiobrstW<RccIoprstrSpec> {
        GpiobrstW::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GpiocrstW<RccIoprstrSpec> {
        GpiocrstW::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GpiodrstW<RccIoprstrSpec> {
        GpiodrstW::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port E reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GpioerstW<RccIoprstrSpec> {
        GpioerstW::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port F reset This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GpiofrstW<RccIoprstrSpec> {
        GpiofrstW::new(self, 5)
    }
}
#[doc = "I/O port reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ioprstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ioprstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccIoprstrSpec;
impl crate::RegisterSpec for RccIoprstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ioprstr::R`](R) reader structure"]
impl crate::Readable for RccIoprstrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ioprstr::W`](W) writer structure"]
impl crate::Writable for RccIoprstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_IOPRSTR to value 0"]
impl crate::Resettable for RccIoprstrSpec {
    const RESET_VALUE: u32 = 0;
}
