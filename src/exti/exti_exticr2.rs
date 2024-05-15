#[doc = "Register `EXTI_EXTICR2` reader"]
pub type R = crate::R<ExtiExticr2Spec>;
#[doc = "Register `EXTI_EXTICR2` writer"]
pub type W = crate::W<ExtiExticr2Spec>;
#[doc = "EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti4 {
    #[doc = "0: PA\\[4\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[4\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[4\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[4\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[4\\]
pin"]
    B0x05 = 5,
}
impl From<Exti4> for u8 {
    #[inline(always)]
    fn from(variant: Exti4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti4 {
    type Ux = u8;
}
impl crate::IsEnum for Exti4 {}
#[doc = "Field `EXTI4` reader - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved"]
pub type Exti4R = crate::FieldReader<Exti4>;
impl Exti4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti4> {
        match self.bits {
            0 => Some(Exti4::B0x00),
            1 => Some(Exti4::B0x01),
            2 => Some(Exti4::B0x02),
            3 => Some(Exti4::B0x03),
            5 => Some(Exti4::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[4\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti4::B0x00
    }
    #[doc = "PB\\[4\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti4::B0x01
    }
    #[doc = "PC\\[4\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti4::B0x02
    }
    #[doc = "PD\\[4\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti4::B0x03
    }
    #[doc = "PF\\[4\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti4::B0x05
    }
}
#[doc = "Field `EXTI4` writer - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved"]
pub type Exti4W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti4>;
impl<'a, REG> Exti4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[4\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti4::B0x00)
    }
    #[doc = "PB\\[4\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti4::B0x01)
    }
    #[doc = "PC\\[4\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti4::B0x02)
    }
    #[doc = "PD\\[4\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti4::B0x03)
    }
    #[doc = "PF\\[4\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti4::B0x05)
    }
}
#[doc = "EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti5 {
    #[doc = "0: PA\\[5\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[5\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[5\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[5\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[5\\]
pin"]
    B0x05 = 5,
}
impl From<Exti5> for u8 {
    #[inline(always)]
    fn from(variant: Exti5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti5 {
    type Ux = u8;
}
impl crate::IsEnum for Exti5 {}
#[doc = "Field `EXTI5` reader - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved"]
pub type Exti5R = crate::FieldReader<Exti5>;
impl Exti5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti5> {
        match self.bits {
            0 => Some(Exti5::B0x00),
            1 => Some(Exti5::B0x01),
            2 => Some(Exti5::B0x02),
            3 => Some(Exti5::B0x03),
            5 => Some(Exti5::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[5\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti5::B0x00
    }
    #[doc = "PB\\[5\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti5::B0x01
    }
    #[doc = "PC\\[5\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti5::B0x02
    }
    #[doc = "PD\\[5\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti5::B0x03
    }
    #[doc = "PF\\[5\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti5::B0x05
    }
}
#[doc = "Field `EXTI5` writer - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved"]
pub type Exti5W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti5>;
impl<'a, REG> Exti5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[5\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti5::B0x00)
    }
    #[doc = "PB\\[5\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti5::B0x01)
    }
    #[doc = "PC\\[5\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti5::B0x02)
    }
    #[doc = "PD\\[5\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti5::B0x03)
    }
    #[doc = "PF\\[5\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti5::B0x05)
    }
}
#[doc = "EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti6 {
    #[doc = "0: PA\\[6\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[6\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[6\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[6\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[6\\]
pin"]
    B0x05 = 5,
}
impl From<Exti6> for u8 {
    #[inline(always)]
    fn from(variant: Exti6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti6 {
    type Ux = u8;
}
impl crate::IsEnum for Exti6 {}
#[doc = "Field `EXTI6` reader - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved"]
pub type Exti6R = crate::FieldReader<Exti6>;
impl Exti6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti6> {
        match self.bits {
            0 => Some(Exti6::B0x00),
            1 => Some(Exti6::B0x01),
            2 => Some(Exti6::B0x02),
            3 => Some(Exti6::B0x03),
            5 => Some(Exti6::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[6\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti6::B0x00
    }
    #[doc = "PB\\[6\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti6::B0x01
    }
    #[doc = "PC\\[6\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti6::B0x02
    }
    #[doc = "PD\\[6\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti6::B0x03
    }
    #[doc = "PF\\[6\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti6::B0x05
    }
}
#[doc = "Field `EXTI6` writer - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved"]
pub type Exti6W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti6>;
impl<'a, REG> Exti6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[6\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti6::B0x00)
    }
    #[doc = "PB\\[6\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti6::B0x01)
    }
    #[doc = "PC\\[6\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti6::B0x02)
    }
    #[doc = "PD\\[6\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti6::B0x03)
    }
    #[doc = "PF\\[6\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti6::B0x05)
    }
}
#[doc = "EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti7 {
    #[doc = "0: PA\\[7\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[7\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[7\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[7\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[7\\]
pin"]
    B0x05 = 5,
}
impl From<Exti7> for u8 {
    #[inline(always)]
    fn from(variant: Exti7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti7 {
    type Ux = u8;
}
impl crate::IsEnum for Exti7 {}
#[doc = "Field `EXTI7` reader - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved"]
pub type Exti7R = crate::FieldReader<Exti7>;
impl Exti7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti7> {
        match self.bits {
            0 => Some(Exti7::B0x00),
            1 => Some(Exti7::B0x01),
            2 => Some(Exti7::B0x02),
            3 => Some(Exti7::B0x03),
            5 => Some(Exti7::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[7\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti7::B0x00
    }
    #[doc = "PB\\[7\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti7::B0x01
    }
    #[doc = "PC\\[7\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti7::B0x02
    }
    #[doc = "PD\\[7\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti7::B0x03
    }
    #[doc = "PF\\[7\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti7::B0x05
    }
}
#[doc = "Field `EXTI7` writer - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved"]
pub type Exti7W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti7>;
impl<'a, REG> Exti7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[7\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti7::B0x00)
    }
    #[doc = "PB\\[7\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti7::B0x01)
    }
    #[doc = "PC\\[7\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti7::B0x02)
    }
    #[doc = "PD\\[7\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti7::B0x03)
    }
    #[doc = "PF\\[7\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti7::B0x05)
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti4(&self) -> Exti4R {
        Exti4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti5(&self) -> Exti5R {
        Exti5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti6(&self) -> Exti6R {
        Exti6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti7(&self) -> Exti7R {
        Exti7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> Exti4W<ExtiExticr2Spec> {
        Exti4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> Exti5W<ExtiExticr2Spec> {
        Exti5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> Exti6W<ExtiExticr2Spec> {
        Exti6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> Exti7W<ExtiExticr2Spec> {
        Exti7W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiExticr2Spec;
impl crate::RegisterSpec for ExtiExticr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_exticr2::R`](R) reader structure"]
impl crate::Readable for ExtiExticr2Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_exticr2::W`](W) writer structure"]
impl crate::Writable for ExtiExticr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EXTICR2 to value 0"]
impl crate::Resettable for ExtiExticr2Spec {
    const RESET_VALUE: u32 = 0;
}
