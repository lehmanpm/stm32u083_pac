#[doc = "Register `RTC_ALRMBSSR` reader"]
pub type R = crate::R<RtcAlrmbssrSpec>;
#[doc = "Register `RTC_ALRMBSSR` writer"]
pub type W = crate::W<RtcAlrmbssrSpec>;
#[doc = "Field `SS` reader - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\]
in the RTC_ALRBBINR, and so can also be read or written through RTC_ALRBBINR."]
pub type SsR = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\]
in the RTC_ALRBBINR, and so can also be read or written through RTC_ALRBBINR."]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00)The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maskss {
    #[doc = "0: No comparison on subseconds for Alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    B0x0 = 0,
    #[doc = "1: SS\\[31:1\\]
are dont care in Alarm B comparison. Only SS\\[0\\]
is compared."]
    B0x1 = 1,
}
impl From<Maskss> for u8 {
    #[inline(always)]
    fn from(variant: Maskss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maskss {
    type Ux = u8;
}
impl crate::IsEnum for Maskss {}
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00)The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MaskssR = crate::FieldReader<Maskss>;
impl MaskssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maskss> {
        match self.bits {
            0 => Some(Maskss::B0x0),
            1 => Some(Maskss::B0x1),
            _ => None,
        }
    }
    #[doc = "No comparison on subseconds for Alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Maskss::B0x0
    }
    #[doc = "SS\\[31:1\\]
are dont care in Alarm B comparison. Only SS\\[0\\]
is compared."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Maskss::B0x1
    }
}
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00)The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MaskssW<'a, REG> = crate::FieldWriter<'a, REG, 6, Maskss>;
impl<'a, REG> MaskssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No comparison on subseconds for Alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0x0)
    }
    #[doc = "SS\\[31:1\\]
are dont care in Alarm B comparison. Only SS\\[0\\]
is compared."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Maskss::B0x1)
    }
}
#[doc = "Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssclr {
    #[doc = "0: The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is free-running."]
    B0x0 = 0,
    #[doc = "1: The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is running from 0xFFFF1FFFF to RTC_ALRBBINR.SS\\[31:0\\]
value and is automatically reloaded with 0xFFFF1FFFF one ck_apre cycle after reaching RTC_ALRBBINR.SS\\[31:0\\]."]
    B0x1 = 1,
}
impl From<Ssclr> for bool {
    #[inline(always)]
    fn from(variant: Ssclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCLR` reader - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
pub type SsclrR = crate::BitReader<Ssclr>;
impl SsclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssclr {
        match self.bits {
            false => Ssclr::B0x0,
            true => Ssclr::B0x1,
        }
    }
    #[doc = "The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is free-running."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ssclr::B0x0
    }
    #[doc = "The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is running from 0xFFFF1FFFF to RTC_ALRBBINR.SS\\[31:0\\]
value and is automatically reloaded with 0xFFFF1FFFF one ck_apre cycle after reaching RTC_ALRBBINR.SS\\[31:0\\]."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ssclr::B0x1
    }
}
#[doc = "Field `SSCLR` writer - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
pub type SsclrW<'a, REG> = crate::BitWriter<'a, REG, Ssclr>;
impl<'a, REG> SsclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is free-running."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssclr::B0x0)
    }
    #[doc = "The synchronous binary counter (SS\\[31:0\\]
in RTC_SSR) is running from 0xFFFF1FFFF to RTC_ALRBBINR.SS\\[31:0\\]
value and is automatically reloaded with 0xFFFF1FFFF one ck_apre cycle after reaching RTC_ALRBBINR.SS\\[31:0\\]."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssclr::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:14 - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\]
in the RTC_ALRBBINR, and so can also be read or written through RTC_ALRBBINR."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:29 - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00)The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn maskss(&self) -> MaskssR {
        MaskssR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
    #[inline(always)]
    pub fn ssclr(&self) -> SsclrR {
        SsclrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\]
in the RTC_ALRBBINR, and so can also be read or written through RTC_ALRBBINR."]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SsW<RtcAlrmbssrSpec> {
        SsW::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00)The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MaskssW<RtcAlrmbssrSpec> {
        MaskssW::new(self, 24)
    }
    #[doc = "Bit 31 - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
    #[inline(always)]
    #[must_use]
    pub fn ssclr(&mut self) -> SsclrW<RtcAlrmbssrSpec> {
        SsclrW::new(self, 31)
    }
}
#[doc = "RTC alarm B subsecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmbssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmbssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlrmbssrSpec;
impl crate::RegisterSpec for RtcAlrmbssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_alrmbssr::R`](R) reader structure"]
impl crate::Readable for RtcAlrmbssrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrmbssr::W`](W) writer structure"]
impl crate::Writable for RtcAlrmbssrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_ALRMBSSR to value 0"]
impl crate::Resettable for RtcAlrmbssrSpec {
    const RESET_VALUE: u32 = 0;
}
