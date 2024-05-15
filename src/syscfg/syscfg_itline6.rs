#[doc = "Register `SYSCFG_ITLINE6` reader"]
pub type R = crate::R<SyscfgItline6Spec>;
#[doc = "Field `EXTI2` reader - EXTI line 2 interrupt request pending"]
pub type Exti2R = crate::BitReader;
#[doc = "Field `EXTI3` reader - EXTI line 3 interrupt request pending"]
pub type Exti3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EXTI line 2 interrupt request pending"]
    #[inline(always)]
    pub fn exti2(&self) -> Exti2R {
        Exti2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI line 3 interrupt request pending"]
    #[inline(always)]
    pub fn exti3(&self) -> Exti3R {
        Exti3R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 6 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline6Spec;
impl crate::RegisterSpec for SyscfgItline6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline6::R`](R) reader structure"]
impl crate::Readable for SyscfgItline6Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE6 to value 0"]
impl crate::Resettable for SyscfgItline6Spec {
    const RESET_VALUE: u32 = 0;
}
