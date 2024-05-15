#[doc = "Register `SYSCFG_ITLINE19` reader"]
pub type R = crate::R<SyscfgItline19Spec>;
#[doc = "Field `TIM15` reader - Timer 15 interrupt request pending"]
pub type Tim15R = crate::BitReader;
#[doc = "Field `LPTIM3` reader - Low-power timer 3 interrupt request pending"]
pub type Lptim3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 15 interrupt request pending"]
    #[inline(always)]
    pub fn tim15(&self) -> Tim15R {
        Tim15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low-power timer 3 interrupt request pending"]
    #[inline(always)]
    pub fn lptim3(&self) -> Lptim3R {
        Lptim3R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 19 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline19::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline19Spec;
impl crate::RegisterSpec for SyscfgItline19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline19::R`](R) reader structure"]
impl crate::Readable for SyscfgItline19Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE19 to value 0"]
impl crate::Resettable for SyscfgItline19Spec {
    const RESET_VALUE: u32 = 0;
}
