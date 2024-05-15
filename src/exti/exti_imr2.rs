#[doc = "Register `EXTI_IMR2` reader"]
pub type R = crate::R<ExtiImr2Spec>;
#[doc = "Register `EXTI_IMR2` writer"]
pub type W = crate::W<ExtiImr2Spec>;
#[doc = "CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im32 {
    #[doc = "0: wake-up with interrupt request from Line x is masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt request from Line x is unmasked"]
    B0x1 = 1,
}
impl From<Im32> for bool {
    #[inline(always)]
    fn from(variant: Im32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM32` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im32R = crate::BitReader<Im32>;
impl Im32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im32 {
        match self.bits {
            false => Im32::B0x0,
            true => Im32::B0x1,
        }
    }
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im32::B0x0
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im32::B0x1
    }
}
#[doc = "Field `IM32` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im32W<'a, REG> = crate::BitWriter<'a, REG, Im32>;
impl<'a, REG> Im32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im32::B0x0)
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im32::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im33 {
    #[doc = "0: wake-up with interrupt request from Line x is masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt request from Line x is unmasked"]
    B0x1 = 1,
}
impl From<Im33> for bool {
    #[inline(always)]
    fn from(variant: Im33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM33` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im33R = crate::BitReader<Im33>;
impl Im33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im33 {
        match self.bits {
            false => Im33::B0x0,
            true => Im33::B0x1,
        }
    }
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im33::B0x0
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im33::B0x1
    }
}
#[doc = "Field `IM33` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im33W<'a, REG> = crate::BitWriter<'a, REG, Im33>;
impl<'a, REG> Im33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im33::B0x0)
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im33::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im34 {
    #[doc = "0: wake-up with interrupt request from Line x is masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt request from Line x is unmasked"]
    B0x1 = 1,
}
impl From<Im34> for bool {
    #[inline(always)]
    fn from(variant: Im34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM34` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im34R = crate::BitReader<Im34>;
impl Im34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im34 {
        match self.bits {
            false => Im34::B0x0,
            true => Im34::B0x1,
        }
    }
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im34::B0x0
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im34::B0x1
    }
}
#[doc = "Field `IM34` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im34W<'a, REG> = crate::BitWriter<'a, REG, Im34>;
impl<'a, REG> Im34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im34::B0x0)
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im34::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im35 {
    #[doc = "0: wake-up with interrupt request from Line x is masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt request from Line x is unmasked"]
    B0x1 = 1,
}
impl From<Im35> for bool {
    #[inline(always)]
    fn from(variant: Im35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM35` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im35R = crate::BitReader<Im35>;
impl Im35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im35 {
        match self.bits {
            false => Im35::B0x0,
            true => Im35::B0x1,
        }
    }
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im35::B0x0
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im35::B0x1
    }
}
#[doc = "Field `IM35` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im35W<'a, REG> = crate::BitWriter<'a, REG, Im35>;
impl<'a, REG> Im35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im35::B0x0)
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im35::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im36 {
    #[doc = "0: wake-up with interrupt request from Line x is masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt request from Line x is unmasked"]
    B0x1 = 1,
}
impl From<Im36> for bool {
    #[inline(always)]
    fn from(variant: Im36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM36` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im36R = crate::BitReader<Im36>;
impl Im36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im36 {
        match self.bits {
            false => Im36::B0x0,
            true => Im36::B0x1,
        }
    }
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im36::B0x0
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im36::B0x1
    }
}
#[doc = "Field `IM36` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im36W<'a, REG> = crate::BitWriter<'a, REG, Im36>;
impl<'a, REG> Im36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im36::B0x0)
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im36::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im37 {
    #[doc = "0: wake-up with interrupt request from Line x is masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt request from Line x is unmasked"]
    B0x1 = 1,
}
impl From<Im37> for bool {
    #[inline(always)]
    fn from(variant: Im37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM37` reader - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im37R = crate::BitReader<Im37>;
impl Im37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im37 {
        match self.bits {
            false => Im37::B0x0,
            true => Im37::B0x1,
        }
    }
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im37::B0x0
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im37::B0x1
    }
}
#[doc = "Field `IM37` writer - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
pub type Im37W<'a, REG> = crate::BitWriter<'a, REG, Im37>;
impl<'a, REG> Im37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt request from Line x is masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im37::B0x0)
    }
    #[doc = "wake-up with interrupt request from Line x is unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im37::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im32(&self) -> Im32R {
        Im32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im33(&self) -> Im33R {
        Im33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im34(&self) -> Im34R {
        Im34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im35(&self) -> Im35R {
        Im35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im36(&self) -> Im36R {
        Im36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im37(&self) -> Im37R {
        Im37R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im32(&mut self) -> Im32W<ExtiImr2Spec> {
        Im32W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im33(&mut self) -> Im33W<ExtiImr2Spec> {
        Im33W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im34(&mut self) -> Im34W<ExtiImr2Spec> {
        Im34W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im35(&mut self) -> Im35W<ExtiImr2Spec> {
        Im35W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im36(&mut self) -> Im36W<ExtiImr2Spec> {
        Im36W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wake-up with interrupt mask on line x (x1=1371to132) Setting/clearing this bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bit IM36 is available only on STM32U0x3xx devices, it is reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im37(&mut self) -> Im37W<ExtiImr2Spec> {
        Im37W::new(self, 5)
    }
}
#[doc = "EXTI CPU wake-up with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_imr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_imr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiImr2Spec;
impl crate::RegisterSpec for ExtiImr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_imr2::R`](R) reader structure"]
impl crate::Readable for ExtiImr2Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_imr2::W`](W) writer structure"]
impl crate::Writable for ExtiImr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_IMR2 to value 0xffff_ffff"]
impl crate::Resettable for ExtiImr2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
