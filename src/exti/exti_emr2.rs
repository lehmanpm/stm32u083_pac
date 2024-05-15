#[doc = "Register `EXTI_EMR2` reader"]
pub type R = crate::R<ExtiEmr2Spec>;
#[doc = "Register `EXTI_EMR2` writer"]
pub type W = crate::W<ExtiEmr2Spec>;
#[doc = "CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em32 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em32> for bool {
    #[inline(always)]
    fn from(variant: Em32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM32` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em32R = crate::BitReader<Em32>;
impl Em32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em32 {
        match self.bits {
            false => Em32::B0x0,
            true => Em32::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em32::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em32::B0x1
    }
}
#[doc = "Field `EM32` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em32W<'a, REG> = crate::BitWriter<'a, REG, Em32>;
impl<'a, REG> Em32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em32::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em32::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em33 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em33> for bool {
    #[inline(always)]
    fn from(variant: Em33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM33` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em33R = crate::BitReader<Em33>;
impl Em33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em33 {
        match self.bits {
            false => Em33::B0x0,
            true => Em33::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em33::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em33::B0x1
    }
}
#[doc = "Field `EM33` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em33W<'a, REG> = crate::BitWriter<'a, REG, Em33>;
impl<'a, REG> Em33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em33::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em33::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em34 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em34> for bool {
    #[inline(always)]
    fn from(variant: Em34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM34` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em34R = crate::BitReader<Em34>;
impl Em34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em34 {
        match self.bits {
            false => Em34::B0x0,
            true => Em34::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em34::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em34::B0x1
    }
}
#[doc = "Field `EM34` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em34W<'a, REG> = crate::BitWriter<'a, REG, Em34>;
impl<'a, REG> Em34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em34::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em34::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em35 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em35> for bool {
    #[inline(always)]
    fn from(variant: Em35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM35` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em35R = crate::BitReader<Em35>;
impl Em35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em35 {
        match self.bits {
            false => Em35::B0x0,
            true => Em35::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em35::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em35::B0x1
    }
}
#[doc = "Field `EM35` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em35W<'a, REG> = crate::BitWriter<'a, REG, Em35>;
impl<'a, REG> Em35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em35::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em35::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em36 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em36> for bool {
    #[inline(always)]
    fn from(variant: Em36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM36` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em36R = crate::BitReader<Em36>;
impl Em36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em36 {
        match self.bits {
            false => Em36::B0x0,
            true => Em36::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em36::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em36::B0x1
    }
}
#[doc = "Field `EM36` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em36W<'a, REG> = crate::BitWriter<'a, REG, Em36>;
impl<'a, REG> Em36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em36::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em36::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em37 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em37> for bool {
    #[inline(always)]
    fn from(variant: Em37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM37` reader - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em37R = crate::BitReader<Em37>;
impl Em37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em37 {
        match self.bits {
            false => Em37::B0x0,
            true => Em37::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em37::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em37::B0x1
    }
}
#[doc = "Field `EM37` writer - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Em37W<'a, REG> = crate::BitWriter<'a, REG, Em37>;
impl<'a, REG> Em37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em37::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em37::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em32(&self) -> Em32R {
        Em32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em33(&self) -> Em33R {
        Em33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em34(&self) -> Em34R {
        Em34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em35(&self) -> Em35R {
        Em35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em36(&self) -> Em36R {
        Em36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em37(&self) -> Em37R {
        Em37R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em32(&mut self) -> Em32W<ExtiEmr2Spec> {
        Em32W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em33(&mut self) -> Em33W<ExtiEmr2Spec> {
        Em33W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em34(&mut self) -> Em34W<ExtiEmr2Spec> {
        Em34W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em35(&mut self) -> Em35W<ExtiEmr2Spec> {
        Em35W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em36(&mut self) -> Em36W<ExtiEmr2Spec> {
        Em36W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wake-up with event generation mask on line x, (x1=1371to132) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em37(&mut self) -> Em37W<ExtiEmr2Spec> {
        Em37W::new(self, 5)
    }
}
#[doc = "EXTI CPU wake-up with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_emr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_emr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiEmr2Spec;
impl crate::RegisterSpec for ExtiEmr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_emr2::R`](R) reader structure"]
impl crate::Readable for ExtiEmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_emr2::W`](W) writer structure"]
impl crate::Writable for ExtiEmr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EMR2 to value 0"]
impl crate::Resettable for ExtiEmr2Spec {
    const RESET_VALUE: u32 = 0;
}
