#[doc = "Register `LPTIM2_CNT` reader"]
pub type R = crate::R<Lptim2CntSpec>;
#[doc = "Field `CNT` reader - Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM2_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM2_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "LPTIM counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim2CntSpec;
impl crate::RegisterSpec for Lptim2CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim2_cnt::R`](R) reader structure"]
impl crate::Readable for Lptim2CntSpec {}
#[doc = "`reset()` method sets LPTIM2_CNT to value 0"]
impl crate::Resettable for Lptim2CntSpec {
    const RESET_VALUE: u32 = 0;
}
