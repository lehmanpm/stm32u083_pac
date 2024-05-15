#[doc = "Register `SYSCFG_ITLINE0` reader"]
pub type R = crate::R<SyscfgItline0Spec>;
#[doc = "Field `WWDG` reader - Window watchdog interrupt pending flag"]
pub type WwdgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Window watchdog interrupt pending flag"]
    #[inline(always)]
    pub fn wwdg(&self) -> WwdgR {
        WwdgR::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline0Spec;
impl crate::RegisterSpec for SyscfgItline0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline0::R`](R) reader structure"]
impl crate::Readable for SyscfgItline0Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE0 to value 0"]
impl crate::Resettable for SyscfgItline0Spec {
    const RESET_VALUE: u32 = 0;
}
