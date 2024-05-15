#[doc = "Register `SYSCFG_ITLINE5` reader"]
pub type R = crate::R<SyscfgItline5Spec>;
#[doc = "Field `EXTI0` reader - EXTI line 0 interrupt request pending"]
pub type Exti0R = crate::BitReader;
#[doc = "Field `EXTI1` reader - EXTI line 1 interrupt request pending"]
pub type Exti1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EXTI line 0 interrupt request pending"]
    #[inline(always)]
    pub fn exti0(&self) -> Exti0R {
        Exti0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI line 1 interrupt request pending"]
    #[inline(always)]
    pub fn exti1(&self) -> Exti1R {
        Exti1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 5 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline5Spec;
impl crate::RegisterSpec for SyscfgItline5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline5::R`](R) reader structure"]
impl crate::Readable for SyscfgItline5Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE5 to value 0"]
impl crate::Resettable for SyscfgItline5Spec {
    const RESET_VALUE: u32 = 0;
}
