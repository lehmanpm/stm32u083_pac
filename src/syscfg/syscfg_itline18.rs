#[doc = "Register `SYSCFG_ITLINE18` reader"]
pub type R = crate::R<SyscfgItline18Spec>;
#[doc = "Field `TIM7` reader - Timer 7 interrupt request pending"]
pub type Tim7R = crate::BitReader;
#[doc = "Field `LPTIM2` reader - Low-power timer 2 interrupt request pending (EXTI line 30)"]
pub type Lptim2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 7 interrupt request pending"]
    #[inline(always)]
    pub fn tim7(&self) -> Tim7R {
        Tim7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low-power timer 2 interrupt request pending (EXTI line 30)"]
    #[inline(always)]
    pub fn lptim2(&self) -> Lptim2R {
        Lptim2R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 18 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline18::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline18Spec;
impl crate::RegisterSpec for SyscfgItline18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline18::R`](R) reader structure"]
impl crate::Readable for SyscfgItline18Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE18 to value 0"]
impl crate::Resettable for SyscfgItline18Spec {
    const RESET_VALUE: u32 = 0;
}
