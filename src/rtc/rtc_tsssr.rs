#[doc = "Register `RTC_TSSSR` reader"]
pub type R = crate::R<RtcTsssrSpec>;
#[doc = "Field `SS` reader - Subsecond value/synchronous binary counter values SS\\[31:0\\]
is the value of the synchronous prescaler counter when the timestamp event occurred."]
pub type SsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Subsecond value/synchronous binary counter values SS\\[31:0\\]
is the value of the synchronous prescaler counter when the timestamp event occurred."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new(self.bits)
    }
}
#[doc = "RTC timestamp subsecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tsssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTsssrSpec;
impl crate::RegisterSpec for RtcTsssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tsssr::R`](R) reader structure"]
impl crate::Readable for RtcTsssrSpec {}
#[doc = "`reset()` method sets RTC_TSSSR to value 0"]
impl crate::Resettable for RtcTsssrSpec {
    const RESET_VALUE: u32 = 0;
}
