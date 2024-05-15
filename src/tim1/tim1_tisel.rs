#[doc = "Register `TIM1_TISEL` reader"]
pub type R = crate::R<Tim1TiselSpec>;
#[doc = "Register `TIM1_TISEL` writer"]
pub type W = crate::W<Tim1TiselSpec>;
#[doc = "selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti1sel {
    #[doc = "0: TIM1_CH1 input"]
    B0x0 = 0,
    #[doc = "1: COMP1 output"]
    B0x1 = 1,
}
impl From<Ti1sel> for u8 {
    #[inline(always)]
    fn from(variant: Ti1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti1sel {
    type Ux = u8;
}
impl crate::IsEnum for Ti1sel {}
#[doc = "Field `TI1SEL` reader - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
pub type Ti1selR = crate::FieldReader<Ti1sel>;
impl Ti1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti1sel> {
        match self.bits {
            0 => Some(Ti1sel::B0x0),
            1 => Some(Ti1sel::B0x1),
            _ => None,
        }
    }
    #[doc = "TIM1_CH1 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ti1sel::B0x0
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ti1sel::B0x1
    }
}
#[doc = "Field `TI1SEL` writer - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
pub type Ti1selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ti1sel>;
impl<'a, REG> Ti1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CH1 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1sel::B0x0)
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti1sel::B0x1)
    }
}
#[doc = "selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti2sel {
    #[doc = "0: TIM1_CH2 input"]
    B0x0 = 0,
    #[doc = "1: COMP2 output"]
    B0x1 = 1,
}
impl From<Ti2sel> for u8 {
    #[inline(always)]
    fn from(variant: Ti2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti2sel {
    type Ux = u8;
}
impl crate::IsEnum for Ti2sel {}
#[doc = "Field `TI2SEL` reader - selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved"]
pub type Ti2selR = crate::FieldReader<Ti2sel>;
impl Ti2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti2sel> {
        match self.bits {
            0 => Some(Ti2sel::B0x0),
            1 => Some(Ti2sel::B0x1),
            _ => None,
        }
    }
    #[doc = "TIM1_CH2 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ti2sel::B0x0
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ti2sel::B0x1
    }
}
#[doc = "Field `TI2SEL` writer - selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved"]
pub type Ti2selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ti2sel>;
impl<'a, REG> Ti2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CH2 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti2sel::B0x0)
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti2sel::B0x1)
    }
}
#[doc = "selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti3sel {
    #[doc = "0: TIM1_CH3 input"]
    B0x0 = 0,
}
impl From<Ti3sel> for u8 {
    #[inline(always)]
    fn from(variant: Ti3sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti3sel {
    type Ux = u8;
}
impl crate::IsEnum for Ti3sel {}
#[doc = "Field `TI3SEL` reader - selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved"]
pub type Ti3selR = crate::FieldReader<Ti3sel>;
impl Ti3selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti3sel> {
        match self.bits {
            0 => Some(Ti3sel::B0x0),
            _ => None,
        }
    }
    #[doc = "TIM1_CH3 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ti3sel::B0x0
    }
}
#[doc = "Field `TI3SEL` writer - selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved"]
pub type Ti3selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ti3sel>;
impl<'a, REG> Ti3selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CH3 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti3sel::B0x0)
    }
}
#[doc = "selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti4sel {
    #[doc = "0: TIM1_CH4 input"]
    B0x0 = 0,
}
impl From<Ti4sel> for u8 {
    #[inline(always)]
    fn from(variant: Ti4sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti4sel {
    type Ux = u8;
}
impl crate::IsEnum for Ti4sel {}
#[doc = "Field `TI4SEL` reader - selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved"]
pub type Ti4selR = crate::FieldReader<Ti4sel>;
impl Ti4selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti4sel> {
        match self.bits {
            0 => Some(Ti4sel::B0x0),
            _ => None,
        }
    }
    #[doc = "TIM1_CH4 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ti4sel::B0x0
    }
}
#[doc = "Field `TI4SEL` writer - selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved"]
pub type Ti4selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ti4sel>;
impl<'a, REG> Ti4selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CH4 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ti4sel::B0x0)
    }
}
impl R {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    pub fn ti1sel(&self) -> Ti1selR {
        Ti1selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    pub fn ti2sel(&self) -> Ti2selR {
        Ti2selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    pub fn ti3sel(&self) -> Ti3selR {
        Ti3selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    pub fn ti4sel(&self) -> Ti4selR {
        Ti4selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> Ti1selW<Tim1TiselSpec> {
        Ti1selW::new(self, 0)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> Ti2selW<Tim1TiselSpec> {
        Ti2selW::new(self, 8)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti3sel(&mut self) -> Ti3selW<Tim1TiselSpec> {
        Ti3selW::new(self, 16)
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti4sel(&mut self) -> Ti4selW<Tim1TiselSpec> {
        Ti4selW::new(self, 24)
    }
}
#[doc = "TIM1 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1TiselSpec;
impl crate::RegisterSpec for Tim1TiselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_tisel::R`](R) reader structure"]
impl crate::Readable for Tim1TiselSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_tisel::W`](W) writer structure"]
impl crate::Writable for Tim1TiselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_TISEL to value 0"]
impl crate::Resettable for Tim1TiselSpec {
    const RESET_VALUE: u32 = 0;
}
