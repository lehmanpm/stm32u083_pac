#[doc = "Register `EXTI_EXTICR3` reader"]
pub type R = crate::R<ExtiExticr3Spec>;
#[doc = "Register `EXTI_EXTICR3` writer"]
pub type W = crate::W<ExtiExticr3Spec>;
#[doc = "EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti8 {
    #[doc = "0: PA\\[8\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[8\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[8\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[8\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[8\\]
pin"]
    B0x05 = 5,
}
impl From<Exti8> for u8 {
    #[inline(always)]
    fn from(variant: Exti8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti8 {
    type Ux = u8;
}
impl crate::IsEnum for Exti8 {}
#[doc = "Field `EXTI8` reader - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved"]
pub type Exti8R = crate::FieldReader<Exti8>;
impl Exti8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti8> {
        match self.bits {
            0 => Some(Exti8::B0x00),
            1 => Some(Exti8::B0x01),
            2 => Some(Exti8::B0x02),
            3 => Some(Exti8::B0x03),
            5 => Some(Exti8::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[8\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti8::B0x00
    }
    #[doc = "PB\\[8\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti8::B0x01
    }
    #[doc = "PC\\[8\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti8::B0x02
    }
    #[doc = "PD\\[8\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti8::B0x03
    }
    #[doc = "PF\\[8\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti8::B0x05
    }
}
#[doc = "Field `EXTI8` writer - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved"]
pub type Exti8W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti8>;
impl<'a, REG> Exti8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[8\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti8::B0x00)
    }
    #[doc = "PB\\[8\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti8::B0x01)
    }
    #[doc = "PC\\[8\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti8::B0x02)
    }
    #[doc = "PD\\[8\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti8::B0x03)
    }
    #[doc = "PF\\[8\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti8::B0x05)
    }
}
#[doc = "EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti9 {
    #[doc = "0: PA\\[9\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[9\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[9\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[9\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[9\\]
pin"]
    B0x05 = 5,
}
impl From<Exti9> for u8 {
    #[inline(always)]
    fn from(variant: Exti9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti9 {
    type Ux = u8;
}
impl crate::IsEnum for Exti9 {}
#[doc = "Field `EXTI9` reader - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved"]
pub type Exti9R = crate::FieldReader<Exti9>;
impl Exti9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti9> {
        match self.bits {
            0 => Some(Exti9::B0x00),
            1 => Some(Exti9::B0x01),
            2 => Some(Exti9::B0x02),
            3 => Some(Exti9::B0x03),
            5 => Some(Exti9::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[9\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti9::B0x00
    }
    #[doc = "PB\\[9\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti9::B0x01
    }
    #[doc = "PC\\[9\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti9::B0x02
    }
    #[doc = "PD\\[9\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti9::B0x03
    }
    #[doc = "PF\\[9\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti9::B0x05
    }
}
#[doc = "Field `EXTI9` writer - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved"]
pub type Exti9W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti9>;
impl<'a, REG> Exti9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[9\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti9::B0x00)
    }
    #[doc = "PB\\[9\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti9::B0x01)
    }
    #[doc = "PC\\[9\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti9::B0x02)
    }
    #[doc = "PD\\[9\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti9::B0x03)
    }
    #[doc = "PF\\[9\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti9::B0x05)
    }
}
#[doc = "EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti10 {
    #[doc = "0: PA\\[10\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[10\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[10\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[10\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[10\\]
pin"]
    B0x05 = 5,
}
impl From<Exti10> for u8 {
    #[inline(always)]
    fn from(variant: Exti10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti10 {
    type Ux = u8;
}
impl crate::IsEnum for Exti10 {}
#[doc = "Field `EXTI10` reader - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved"]
pub type Exti10R = crate::FieldReader<Exti10>;
impl Exti10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti10> {
        match self.bits {
            0 => Some(Exti10::B0x00),
            1 => Some(Exti10::B0x01),
            2 => Some(Exti10::B0x02),
            3 => Some(Exti10::B0x03),
            5 => Some(Exti10::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[10\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti10::B0x00
    }
    #[doc = "PB\\[10\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti10::B0x01
    }
    #[doc = "PC\\[10\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti10::B0x02
    }
    #[doc = "PD\\[10\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti10::B0x03
    }
    #[doc = "PF\\[10\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti10::B0x05
    }
}
#[doc = "Field `EXTI10` writer - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved"]
pub type Exti10W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti10>;
impl<'a, REG> Exti10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[10\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti10::B0x00)
    }
    #[doc = "PB\\[10\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti10::B0x01)
    }
    #[doc = "PC\\[10\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti10::B0x02)
    }
    #[doc = "PD\\[10\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti10::B0x03)
    }
    #[doc = "PF\\[10\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti10::B0x05)
    }
}
#[doc = "EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti11 {
    #[doc = "0: PA\\[11\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[11\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[11\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[11\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[11\\]
pin"]
    B0x05 = 5,
}
impl From<Exti11> for u8 {
    #[inline(always)]
    fn from(variant: Exti11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti11 {
    type Ux = u8;
}
impl crate::IsEnum for Exti11 {}
#[doc = "Field `EXTI11` reader - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved"]
pub type Exti11R = crate::FieldReader<Exti11>;
impl Exti11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti11> {
        match self.bits {
            0 => Some(Exti11::B0x00),
            1 => Some(Exti11::B0x01),
            2 => Some(Exti11::B0x02),
            3 => Some(Exti11::B0x03),
            5 => Some(Exti11::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[11\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti11::B0x00
    }
    #[doc = "PB\\[11\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti11::B0x01
    }
    #[doc = "PC\\[11\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti11::B0x02
    }
    #[doc = "PD\\[11\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti11::B0x03
    }
    #[doc = "PF\\[11\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti11::B0x05
    }
}
#[doc = "Field `EXTI11` writer - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved"]
pub type Exti11W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti11>;
impl<'a, REG> Exti11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[11\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti11::B0x00)
    }
    #[doc = "PB\\[11\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti11::B0x01)
    }
    #[doc = "PC\\[11\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti11::B0x02)
    }
    #[doc = "PD\\[11\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti11::B0x03)
    }
    #[doc = "PF\\[11\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti11::B0x05)
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti8(&self) -> Exti8R {
        Exti8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti9(&self) -> Exti9R {
        Exti9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti10(&self) -> Exti10R {
        Exti10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti11(&self) -> Exti11R {
        Exti11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti8(&mut self) -> Exti8W<ExtiExticr3Spec> {
        Exti8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti9(&mut self) -> Exti9W<ExtiExticr3Spec> {
        Exti9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti10(&mut self) -> Exti10W<ExtiExticr3Spec> {
        Exti10W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti11(&mut self) -> Exti11W<ExtiExticr3Spec> {
        Exti11W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiExticr3Spec;
impl crate::RegisterSpec for ExtiExticr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_exticr3::R`](R) reader structure"]
impl crate::Readable for ExtiExticr3Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_exticr3::W`](W) writer structure"]
impl crate::Writable for ExtiExticr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EXTICR3 to value 0"]
impl crate::Resettable for ExtiExticr3Spec {
    const RESET_VALUE: u32 = 0;
}
