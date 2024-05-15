#[doc = "Register `TSC_IOG1CR` reader"]
pub type R = crate::R<TscIog1crSpec>;
#[doc = "Field `CNT` reader - Counter value These bits represent the number of charge transfer cycles generated on the analog I/O group x to complete its acquisition (voltage across C&lt;sub>S&lt;/sub> has reached the threshold)."]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Counter value These bits represent the number of charge transfer cycles generated on the analog I/O group x to complete its acquisition (voltage across C&lt;sub>S&lt;/sub> has reached the threshold)."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "TSC I/O group 1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iog1cr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscIog1crSpec;
impl crate::RegisterSpec for TscIog1crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsc_iog1cr::R`](R) reader structure"]
impl crate::Readable for TscIog1crSpec {}
#[doc = "`reset()` method sets TSC_IOG1CR to value 0"]
impl crate::Resettable for TscIog1crSpec {
    const RESET_VALUE: u32 = 0;
}
