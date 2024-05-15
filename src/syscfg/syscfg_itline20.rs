#[doc = "Register `SYSCFG_ITLINE20` reader"]
pub type R = crate::R<SyscfgItline20Spec>;
#[doc = "Field `TIM16` reader - Timer 16 interrupt request pending"]
pub type Tim16R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 16 interrupt request pending"]
    #[inline(always)]
    pub fn tim16(&self) -> Tim16R {
        Tim16R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 20 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline20::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline20Spec;
impl crate::RegisterSpec for SyscfgItline20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline20::R`](R) reader structure"]
impl crate::Readable for SyscfgItline20Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE20 to value 0"]
impl crate::Resettable for SyscfgItline20Spec {
    const RESET_VALUE: u32 = 0;
}
