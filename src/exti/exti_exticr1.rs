#[doc = "Register `EXTI_EXTICR1` reader"]
pub type R = crate::R<ExtiExticr1Spec>;
#[doc = "Register `EXTI_EXTICR1` writer"]
pub type W = crate::W<ExtiExticr1Spec>;
#[doc = "EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti0 {
    #[doc = "0: PA\\[0\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[0\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[0\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[0\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[0\\]
pin"]
    B0x05 = 5,
}
impl From<Exti0> for u8 {
    #[inline(always)]
    fn from(variant: Exti0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti0 {
    type Ux = u8;
}
impl crate::IsEnum for Exti0 {}
#[doc = "Field `EXTI0` reader - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved"]
pub type Exti0R = crate::FieldReader<Exti0>;
impl Exti0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti0> {
        match self.bits {
            0 => Some(Exti0::B0x00),
            1 => Some(Exti0::B0x01),
            2 => Some(Exti0::B0x02),
            3 => Some(Exti0::B0x03),
            5 => Some(Exti0::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[0\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti0::B0x00
    }
    #[doc = "PB\\[0\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti0::B0x01
    }
    #[doc = "PC\\[0\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti0::B0x02
    }
    #[doc = "PD\\[0\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti0::B0x03
    }
    #[doc = "PF\\[0\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti0::B0x05
    }
}
#[doc = "Field `EXTI0` writer - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved"]
pub type Exti0W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti0>;
impl<'a, REG> Exti0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[0\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti0::B0x00)
    }
    #[doc = "PB\\[0\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti0::B0x01)
    }
    #[doc = "PC\\[0\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti0::B0x02)
    }
    #[doc = "PD\\[0\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti0::B0x03)
    }
    #[doc = "PF\\[0\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti0::B0x05)
    }
}
#[doc = "EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti1 {
    #[doc = "0: PA\\[1\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[1\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[1\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[1\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[1\\]
pin"]
    B0x05 = 5,
}
impl From<Exti1> for u8 {
    #[inline(always)]
    fn from(variant: Exti1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti1 {
    type Ux = u8;
}
impl crate::IsEnum for Exti1 {}
#[doc = "Field `EXTI1` reader - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved"]
pub type Exti1R = crate::FieldReader<Exti1>;
impl Exti1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti1> {
        match self.bits {
            0 => Some(Exti1::B0x00),
            1 => Some(Exti1::B0x01),
            2 => Some(Exti1::B0x02),
            3 => Some(Exti1::B0x03),
            5 => Some(Exti1::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[1\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti1::B0x00
    }
    #[doc = "PB\\[1\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti1::B0x01
    }
    #[doc = "PC\\[1\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti1::B0x02
    }
    #[doc = "PD\\[1\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti1::B0x03
    }
    #[doc = "PF\\[1\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti1::B0x05
    }
}
#[doc = "Field `EXTI1` writer - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved"]
pub type Exti1W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti1>;
impl<'a, REG> Exti1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[1\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti1::B0x00)
    }
    #[doc = "PB\\[1\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti1::B0x01)
    }
    #[doc = "PC\\[1\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti1::B0x02)
    }
    #[doc = "PD\\[1\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti1::B0x03)
    }
    #[doc = "PF\\[1\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti1::B0x05)
    }
}
#[doc = "EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti2 {
    #[doc = "0: PA\\[2\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[2\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[2\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[2\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[2\\]
pin"]
    B0x05 = 5,
}
impl From<Exti2> for u8 {
    #[inline(always)]
    fn from(variant: Exti2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti2 {
    type Ux = u8;
}
impl crate::IsEnum for Exti2 {}
#[doc = "Field `EXTI2` reader - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved"]
pub type Exti2R = crate::FieldReader<Exti2>;
impl Exti2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti2> {
        match self.bits {
            0 => Some(Exti2::B0x00),
            1 => Some(Exti2::B0x01),
            2 => Some(Exti2::B0x02),
            3 => Some(Exti2::B0x03),
            5 => Some(Exti2::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[2\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti2::B0x00
    }
    #[doc = "PB\\[2\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti2::B0x01
    }
    #[doc = "PC\\[2\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti2::B0x02
    }
    #[doc = "PD\\[2\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti2::B0x03
    }
    #[doc = "PF\\[2\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti2::B0x05
    }
}
#[doc = "Field `EXTI2` writer - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved"]
pub type Exti2W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti2>;
impl<'a, REG> Exti2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[2\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti2::B0x00)
    }
    #[doc = "PB\\[2\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti2::B0x01)
    }
    #[doc = "PC\\[2\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti2::B0x02)
    }
    #[doc = "PD\\[2\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti2::B0x03)
    }
    #[doc = "PF\\[2\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti2::B0x05)
    }
}
#[doc = "EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti3 {
    #[doc = "0: PA\\[3\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[3\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[3\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[3\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[3\\]
pin"]
    B0x05 = 5,
}
impl From<Exti3> for u8 {
    #[inline(always)]
    fn from(variant: Exti3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti3 {
    type Ux = u8;
}
impl crate::IsEnum for Exti3 {}
#[doc = "Field `EXTI3` reader - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved"]
pub type Exti3R = crate::FieldReader<Exti3>;
impl Exti3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti3> {
        match self.bits {
            0 => Some(Exti3::B0x00),
            1 => Some(Exti3::B0x01),
            2 => Some(Exti3::B0x02),
            3 => Some(Exti3::B0x03),
            5 => Some(Exti3::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[3\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti3::B0x00
    }
    #[doc = "PB\\[3\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti3::B0x01
    }
    #[doc = "PC\\[3\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti3::B0x02
    }
    #[doc = "PD\\[3\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti3::B0x03
    }
    #[doc = "PF\\[3\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti3::B0x05
    }
}
#[doc = "Field `EXTI3` writer - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved"]
pub type Exti3W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti3>;
impl<'a, REG> Exti3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[3\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti3::B0x00)
    }
    #[doc = "PB\\[3\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti3::B0x01)
    }
    #[doc = "PC\\[3\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti3::B0x02)
    }
    #[doc = "PD\\[3\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti3::B0x03)
    }
    #[doc = "PF\\[3\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti3::B0x05)
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti0(&self) -> Exti0R {
        Exti0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti1(&self) -> Exti1R {
        Exti1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti2(&self) -> Exti2R {
        Exti2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti3(&self) -> Exti3R {
        Exti3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> Exti0W<ExtiExticr1Spec> {
        Exti0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> Exti1W<ExtiExticr1Spec> {
        Exti1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> Exti2W<ExtiExticr1Spec> {
        Exti2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> Exti3W<ExtiExticr1Spec> {
        Exti3W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiExticr1Spec;
impl crate::RegisterSpec for ExtiExticr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_exticr1::R`](R) reader structure"]
impl crate::Readable for ExtiExticr1Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_exticr1::W`](W) writer structure"]
impl crate::Writable for ExtiExticr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EXTICR1 to value 0"]
impl crate::Resettable for ExtiExticr1Spec {
    const RESET_VALUE: u32 = 0;
}
