#[doc = "Register `RTC_ALRABINR` reader"]
pub type R = crate::R<RtcAlrabinrSpec>;
#[doc = "Register `RTC_ALRABINR` writer"]
pub type W = crate::W<RtcAlrabinrSpec>;
#[doc = "Field `SS` reader - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\]
is the mirror of SS\\[14:0\\]
in the RTC_ALRMASSRR, and so can also be read or written through RTC_ALRMASSR."]
pub type SsR = crate::FieldReader<u32>;
#[doc = "Field `SS` writer - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\]
is the mirror of SS\\[14:0\\]
in the RTC_ALRMASSRR, and so can also be read or written through RTC_ALRMASSR."]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\]
is the mirror of SS\\[14:0\\]
in the RTC_ALRMASSRR, and so can also be read or written through RTC_ALRMASSR."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\]
is the mirror of SS\\[14:0\\]
in the RTC_ALRMASSRR, and so can also be read or written through RTC_ALRMASSR."]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SsW<RtcAlrabinrSpec> {
        SsW::new(self, 0)
    }
}
#[doc = "RTC alarm A binary mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrabinr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrabinr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlrabinrSpec;
impl crate::RegisterSpec for RtcAlrabinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_alrabinr::R`](R) reader structure"]
impl crate::Readable for RtcAlrabinrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrabinr::W`](W) writer structure"]
impl crate::Writable for RtcAlrabinrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_ALRABINR to value 0"]
impl crate::Resettable for RtcAlrabinrSpec {
    const RESET_VALUE: u32 = 0;
}
