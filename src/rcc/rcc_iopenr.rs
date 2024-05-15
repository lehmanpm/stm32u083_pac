#[doc = "Register `RCC_IOPENR` reader"]
pub type R = crate::R<RccIopenrSpec>;
#[doc = "Register `RCC_IOPENR` writer"]
pub type W = crate::W<RccIopenrSpec>;
#[doc = "I/O port A clock enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioaen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpioaen> for bool {
    #[inline(always)]
    fn from(variant: Gpioaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - I/O port A clock enable This bit is set and cleared by software."]
pub type GpioaenR = crate::BitReader<Gpioaen>;
impl GpioaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioaen {
        match self.bits {
            false => Gpioaen::B0x0,
            true => Gpioaen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioaen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioaen::B0x1
    }
}
#[doc = "Field `GPIOAEN` writer - I/O port A clock enable This bit is set and cleared by software."]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG, Gpioaen>;
impl<'a, REG> GpioaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioaen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioaen::B0x1)
    }
}
#[doc = "I/O port B clock enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioben {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpioben> for bool {
    #[inline(always)]
    fn from(variant: Gpioben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOBEN` reader - I/O port B clock enable This bit is set and cleared by software."]
pub type GpiobenR = crate::BitReader<Gpioben>;
impl GpiobenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioben {
        match self.bits {
            false => Gpioben::B0x0,
            true => Gpioben::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioben::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioben::B0x1
    }
}
#[doc = "Field `GPIOBEN` writer - I/O port B clock enable This bit is set and cleared by software."]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG, Gpioben>;
impl<'a, REG> GpiobenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioben::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioben::B0x1)
    }
}
#[doc = "I/O port C clock enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiocen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpiocen> for bool {
    #[inline(always)]
    fn from(variant: Gpiocen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOCEN` reader - I/O port C clock enable This bit is set and cleared by software."]
pub type GpiocenR = crate::BitReader<Gpiocen>;
impl GpiocenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiocen {
        match self.bits {
            false => Gpiocen::B0x0,
            true => Gpiocen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiocen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiocen::B0x1
    }
}
#[doc = "Field `GPIOCEN` writer - I/O port C clock enable This bit is set and cleared by software."]
pub type GpiocenW<'a, REG> = crate::BitWriter<'a, REG, Gpiocen>;
impl<'a, REG> GpiocenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocen::B0x1)
    }
}
#[doc = "I/O port D clock enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioden {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpioden> for bool {
    #[inline(always)]
    fn from(variant: Gpioden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIODEN` reader - I/O port D clock enable This bit is set and cleared by software."]
pub type GpiodenR = crate::BitReader<Gpioden>;
impl GpiodenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioden {
        match self.bits {
            false => Gpioden::B0x0,
            true => Gpioden::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioden::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioden::B0x1
    }
}
#[doc = "Field `GPIODEN` writer - I/O port D clock enable This bit is set and cleared by software."]
pub type GpiodenW<'a, REG> = crate::BitWriter<'a, REG, Gpioden>;
impl<'a, REG> GpiodenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioden::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioden::B0x1)
    }
}
#[doc = "I/O port E clock enable&lt;sup>(1)&lt;/sup> This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioeen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpioeen> for bool {
    #[inline(always)]
    fn from(variant: Gpioeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOEEN` reader - I/O port E clock enable&lt;sup>(1)&lt;/sup> This bit is set and cleared by software."]
pub type GpioeenR = crate::BitReader<Gpioeen>;
impl GpioeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioeen {
        match self.bits {
            false => Gpioeen::B0x0,
            true => Gpioeen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioeen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioeen::B0x1
    }
}
#[doc = "Field `GPIOEEN` writer - I/O port E clock enable&lt;sup>(1)&lt;/sup> This bit is set and cleared by software."]
pub type GpioeenW<'a, REG> = crate::BitWriter<'a, REG, Gpioeen>;
impl<'a, REG> GpioeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioeen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioeen::B0x1)
    }
}
#[doc = "I/O port F clock enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiofen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Gpiofen> for bool {
    #[inline(always)]
    fn from(variant: Gpiofen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOFEN` reader - I/O port F clock enable This bit is set and cleared by software."]
pub type GpiofenR = crate::BitReader<Gpiofen>;
impl GpiofenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiofen {
        match self.bits {
            false => Gpiofen::B0x0,
            true => Gpiofen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiofen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiofen::B0x1
    }
}
#[doc = "Field `GPIOFEN` writer - I/O port F clock enable This bit is set and cleared by software."]
pub type GpiofenW<'a, REG> = crate::BitWriter<'a, REG, Gpiofen>;
impl<'a, REG> GpiofenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - I/O port A clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioden(&self) -> GpiodenR {
        GpiodenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E clock enable&lt;sup>(1)&lt;/sup> This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpioeen(&self) -> GpioeenR {
        GpioeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GpioaenW<RccIopenrSpec> {
        GpioaenW::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GpiobenW<RccIopenrSpec> {
        GpiobenW::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GpiocenW<RccIopenrSpec> {
        GpiocenW::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GpiodenW<RccIopenrSpec> {
        GpiodenW::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port E clock enable&lt;sup>(1)&lt;/sup> This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GpioeenW<RccIopenrSpec> {
        GpioeenW::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port F clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GpiofenW<RccIopenrSpec> {
        GpiofenW::new(self, 5)
    }
}
#[doc = "I/O port clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_iopenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_iopenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccIopenrSpec;
impl crate::RegisterSpec for RccIopenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_iopenr::R`](R) reader structure"]
impl crate::Readable for RccIopenrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_iopenr::W`](W) writer structure"]
impl crate::Writable for RccIopenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_IOPENR to value 0"]
impl crate::Resettable for RccIopenrSpec {
    const RESET_VALUE: u32 = 0;
}
