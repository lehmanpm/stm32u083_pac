#[doc = "Register `SYSCFG_ITLINE16` reader"]
pub type R = crate::R<SyscfgItline16Spec>;
#[doc = "Field `TIM3` reader - Timer 3 interrupt request pending"]
pub type Tim3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 3 interrupt request pending"]
    #[inline(always)]
    pub fn tim3(&self) -> Tim3R {
        Tim3R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 16 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline16Spec;
impl crate::RegisterSpec for SyscfgItline16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline16::R`](R) reader structure"]
impl crate::Readable for SyscfgItline16Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE16 to value 0"]
impl crate::Resettable for SyscfgItline16Spec {
    const RESET_VALUE: u32 = 0;
}
