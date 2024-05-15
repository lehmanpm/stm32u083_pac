#[doc = "Register `SYSCFG_ITLINE15` reader"]
pub type R = crate::R<SyscfgItline15Spec>;
#[doc = "Field `TIM2` reader - Timer 2 interrupt request pending"]
pub type Tim2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 2 interrupt request pending"]
    #[inline(always)]
    pub fn tim2(&self) -> Tim2R {
        Tim2R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 15 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline15::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline15Spec;
impl crate::RegisterSpec for SyscfgItline15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline15::R`](R) reader structure"]
impl crate::Readable for SyscfgItline15Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE15 to value 0"]
impl crate::Resettable for SyscfgItline15Spec {
    const RESET_VALUE: u32 = 0;
}
