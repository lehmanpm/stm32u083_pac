#[doc = "Register `EXTI_EXTICR4` reader"]
pub type R = crate::R<ExtiExticr4Spec>;
#[doc = "Register `EXTI_EXTICR4` writer"]
pub type W = crate::W<ExtiExticr4Spec>;
#[doc = "EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti12 {
    #[doc = "0: PA\\[12\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[12\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[12\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[12\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[12\\]
pin"]
    B0x05 = 5,
}
impl From<Exti12> for u8 {
    #[inline(always)]
    fn from(variant: Exti12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti12 {
    type Ux = u8;
}
impl crate::IsEnum for Exti12 {}
#[doc = "Field `EXTI12` reader - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved"]
pub type Exti12R = crate::FieldReader<Exti12>;
impl Exti12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti12> {
        match self.bits {
            0 => Some(Exti12::B0x00),
            1 => Some(Exti12::B0x01),
            2 => Some(Exti12::B0x02),
            3 => Some(Exti12::B0x03),
            5 => Some(Exti12::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[12\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti12::B0x00
    }
    #[doc = "PB\\[12\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti12::B0x01
    }
    #[doc = "PC\\[12\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti12::B0x02
    }
    #[doc = "PD\\[12\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti12::B0x03
    }
    #[doc = "PF\\[12\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti12::B0x05
    }
}
#[doc = "Field `EXTI12` writer - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved"]
pub type Exti12W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti12>;
impl<'a, REG> Exti12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[12\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti12::B0x00)
    }
    #[doc = "PB\\[12\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti12::B0x01)
    }
    #[doc = "PC\\[12\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti12::B0x02)
    }
    #[doc = "PD\\[12\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti12::B0x03)
    }
    #[doc = "PF\\[12\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti12::B0x05)
    }
}
#[doc = "EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti13 {
    #[doc = "0: PA\\[13\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[13\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[13\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[13\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[13\\]
pin"]
    B0x05 = 5,
}
impl From<Exti13> for u8 {
    #[inline(always)]
    fn from(variant: Exti13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti13 {
    type Ux = u8;
}
impl crate::IsEnum for Exti13 {}
#[doc = "Field `EXTI13` reader - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved"]
pub type Exti13R = crate::FieldReader<Exti13>;
impl Exti13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti13> {
        match self.bits {
            0 => Some(Exti13::B0x00),
            1 => Some(Exti13::B0x01),
            2 => Some(Exti13::B0x02),
            3 => Some(Exti13::B0x03),
            5 => Some(Exti13::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[13\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti13::B0x00
    }
    #[doc = "PB\\[13\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti13::B0x01
    }
    #[doc = "PC\\[13\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti13::B0x02
    }
    #[doc = "PD\\[13\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti13::B0x03
    }
    #[doc = "PF\\[13\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti13::B0x05
    }
}
#[doc = "Field `EXTI13` writer - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved"]
pub type Exti13W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti13>;
impl<'a, REG> Exti13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[13\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti13::B0x00)
    }
    #[doc = "PB\\[13\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti13::B0x01)
    }
    #[doc = "PC\\[13\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti13::B0x02)
    }
    #[doc = "PD\\[13\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti13::B0x03)
    }
    #[doc = "PF\\[13\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti13::B0x05)
    }
}
#[doc = "EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti14 {
    #[doc = "0: PA\\[14\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[14\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[14\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[14\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[14\\]
pin"]
    B0x05 = 5,
}
impl From<Exti14> for u8 {
    #[inline(always)]
    fn from(variant: Exti14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti14 {
    type Ux = u8;
}
impl crate::IsEnum for Exti14 {}
#[doc = "Field `EXTI14` reader - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved"]
pub type Exti14R = crate::FieldReader<Exti14>;
impl Exti14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti14> {
        match self.bits {
            0 => Some(Exti14::B0x00),
            1 => Some(Exti14::B0x01),
            2 => Some(Exti14::B0x02),
            3 => Some(Exti14::B0x03),
            5 => Some(Exti14::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[14\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti14::B0x00
    }
    #[doc = "PB\\[14\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti14::B0x01
    }
    #[doc = "PC\\[14\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti14::B0x02
    }
    #[doc = "PD\\[14\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti14::B0x03
    }
    #[doc = "PF\\[14\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti14::B0x05
    }
}
#[doc = "Field `EXTI14` writer - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved"]
pub type Exti14W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti14>;
impl<'a, REG> Exti14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[14\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti14::B0x00)
    }
    #[doc = "PB\\[14\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti14::B0x01)
    }
    #[doc = "PC\\[14\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti14::B0x02)
    }
    #[doc = "PD\\[14\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti14::B0x03)
    }
    #[doc = "PF\\[14\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti14::B0x05)
    }
}
#[doc = "EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exti15 {
    #[doc = "0: PA\\[15\\]
pin"]
    B0x00 = 0,
    #[doc = "1: PB\\[15\\]
pin"]
    B0x01 = 1,
    #[doc = "2: PC\\[15\\]
pin"]
    B0x02 = 2,
    #[doc = "3: PD\\[15\\]
pin"]
    B0x03 = 3,
    #[doc = "5: PF\\[15\\]
pin"]
    B0x05 = 5,
}
impl From<Exti15> for u8 {
    #[inline(always)]
    fn from(variant: Exti15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exti15 {
    type Ux = u8;
}
impl crate::IsEnum for Exti15 {}
#[doc = "Field `EXTI15` reader - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved"]
pub type Exti15R = crate::FieldReader<Exti15>;
impl Exti15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exti15> {
        match self.bits {
            0 => Some(Exti15::B0x00),
            1 => Some(Exti15::B0x01),
            2 => Some(Exti15::B0x02),
            3 => Some(Exti15::B0x03),
            5 => Some(Exti15::B0x05),
            _ => None,
        }
    }
    #[doc = "PA\\[15\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == Exti15::B0x00
    }
    #[doc = "PB\\[15\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == Exti15::B0x01
    }
    #[doc = "PC\\[15\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == Exti15::B0x02
    }
    #[doc = "PD\\[15\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == Exti15::B0x03
    }
    #[doc = "PF\\[15\\]
pin"]
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == Exti15::B0x05
    }
}
#[doc = "Field `EXTI15` writer - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved"]
pub type Exti15W<'a, REG> = crate::FieldWriter<'a, REG, 8, Exti15>;
impl<'a, REG> Exti15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[15\\]
pin"]
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Exti15::B0x00)
    }
    #[doc = "PB\\[15\\]
pin"]
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Exti15::B0x01)
    }
    #[doc = "PC\\[15\\]
pin"]
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Exti15::B0x02)
    }
    #[doc = "PD\\[15\\]
pin"]
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Exti15::B0x03)
    }
    #[doc = "PF\\[15\\]
pin"]
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Exti15::B0x05)
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti12(&self) -> Exti12R {
        Exti12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti13(&self) -> Exti13R {
        Exti13R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti14(&self) -> Exti14R {
        Exti14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved"]
    #[inline(always)]
    pub fn exti15(&self) -> Exti15R {
        Exti15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti12(&mut self) -> Exti12W<ExtiExticr4Spec> {
        Exti12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti13(&mut self) -> Exti13W<ExtiExticr4Spec> {
        Exti13W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti14(&mut self) -> Exti14W<ExtiExticr4Spec> {
        Exti14W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti15(&mut self) -> Exti15W<ExtiExticr4Spec> {
        Exti15W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiExticr4Spec;
impl crate::RegisterSpec for ExtiExticr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_exticr4::R`](R) reader structure"]
impl crate::Readable for ExtiExticr4Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_exticr4::W`](W) writer structure"]
impl crate::Writable for ExtiExticr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EXTICR4 to value 0"]
impl crate::Resettable for ExtiExticr4Spec {
    const RESET_VALUE: u32 = 0;
}
