#[doc = "Register `SYSCFG_ITLINE27` reader"]
pub type R = crate::R<SyscfgItline27Spec>;
#[doc = "Field `USART1` reader - USART1 interrupt request pending, combined with EXTI line 25"]
pub type Usart1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART1 interrupt request pending, combined with EXTI line 25"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 27 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline27::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline27Spec;
impl crate::RegisterSpec for SyscfgItline27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline27::R`](R) reader structure"]
impl crate::Readable for SyscfgItline27Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE27 to value 0"]
impl crate::Resettable for SyscfgItline27Spec {
    const RESET_VALUE: u32 = 0;
}
