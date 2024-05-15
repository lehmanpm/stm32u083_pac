#[doc = "Register `SYSCFG_ITLINE2` reader"]
pub type R = crate::R<SyscfgItline2Spec>;
#[doc = "Field `TAMP` reader - Tamper interrupt request pending (EXTI line 21)"]
pub type TampR = crate::BitReader;
#[doc = "Field `RTC` reader - RTC interrupt request pending (EXTI line 19)"]
pub type RtcR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Tamper interrupt request pending (EXTI line 21)"]
    #[inline(always)]
    pub fn tamp(&self) -> TampR {
        TampR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC interrupt request pending (EXTI line 19)"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline2Spec;
impl crate::RegisterSpec for SyscfgItline2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline2::R`](R) reader structure"]
impl crate::Readable for SyscfgItline2Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE2 to value 0"]
impl crate::Resettable for SyscfgItline2Spec {
    const RESET_VALUE: u32 = 0;
}
