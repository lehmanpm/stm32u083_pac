#[doc = "Register `SYSCFG_ITLINE22` reader"]
pub type R = crate::R<SyscfgItline22Spec>;
#[doc = "Field `LCD` reader - LCD interrupt request pending"]
pub type LcdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - LCD interrupt request pending"]
    #[inline(always)]
    pub fn lcd(&self) -> LcdR {
        LcdR::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 22 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline22::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline22Spec;
impl crate::RegisterSpec for SyscfgItline22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline22::R`](R) reader structure"]
impl crate::Readable for SyscfgItline22Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE22 to value 0"]
impl crate::Resettable for SyscfgItline22Spec {
    const RESET_VALUE: u32 = 0;
}
