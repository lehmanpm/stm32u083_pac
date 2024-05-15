#[doc = "Register `LCD_RAM2` reader"]
pub type R = crate::R<LcdRam2Spec>;
#[doc = "Register `LCD_RAM2` writer"]
pub type W = crate::W<LcdRam2Spec>;
#[doc = "Each bit corresponds to one pixel of the LCD display.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SegmentData {
    #[doc = "0: Pixel inactive"]
    B0x0 = 0,
    #[doc = "1: Pixel active"]
    B0x1 = 1,
}
impl From<SegmentData> for u32 {
    #[inline(always)]
    fn from(variant: SegmentData) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SegmentData {
    type Ux = u32;
}
impl crate::IsEnum for SegmentData {}
#[doc = "Field `SEGMENT_DATA` reader - Each bit corresponds to one pixel of the LCD display."]
pub type SegmentDataR = crate::FieldReader<SegmentData>;
impl SegmentDataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SegmentData> {
        match self.bits {
            0 => Some(SegmentData::B0x0),
            1 => Some(SegmentData::B0x1),
            _ => None,
        }
    }
    #[doc = "Pixel inactive"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SegmentData::B0x0
    }
    #[doc = "Pixel active"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SegmentData::B0x1
    }
}
#[doc = "Field `SEGMENT_DATA` writer - Each bit corresponds to one pixel of the LCD display."]
pub type SegmentDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, SegmentData>;
impl<'a, REG> SegmentDataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Pixel inactive"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SegmentData::B0x0)
    }
    #[doc = "Pixel active"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SegmentData::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Each bit corresponds to one pixel of the LCD display."]
    #[inline(always)]
    pub fn segment_data(&self) -> SegmentDataR {
        SegmentDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit corresponds to one pixel of the LCD display."]
    #[inline(always)]
    #[must_use]
    pub fn segment_data(&mut self) -> SegmentDataW<LcdRam2Spec> {
        SegmentDataW::new(self, 0)
    }
}
#[doc = "LCD display memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ram2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ram2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdRam2Spec;
impl crate::RegisterSpec for LcdRam2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ram2::R`](R) reader structure"]
impl crate::Readable for LcdRam2Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_ram2::W`](W) writer structure"]
impl crate::Writable for LcdRam2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_RAM2 to value 0"]
impl crate::Resettable for LcdRam2Spec {
    const RESET_VALUE: u32 = 0;
}
