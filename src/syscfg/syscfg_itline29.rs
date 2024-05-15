#[doc = "Register `SYSCFG_ITLINE29` reader"]
pub type R = crate::R<SyscfgItline29Spec>;
#[doc = "Field `USART3` reader - USART3 interrupt request pending"]
pub type Usart3R = crate::BitReader;
#[doc = "Field `LPUART1` reader - LPUART1 interrupt request pending (EXTI line 30)"]
pub type Lpuart1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART3 interrupt request pending"]
    #[inline(always)]
    pub fn usart3(&self) -> Usart3R {
        Usart3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPUART1 interrupt request pending (EXTI line 30)"]
    #[inline(always)]
    pub fn lpuart1(&self) -> Lpuart1R {
        Lpuart1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 29 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline29::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline29Spec;
impl crate::RegisterSpec for SyscfgItline29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline29::R`](R) reader structure"]
impl crate::Readable for SyscfgItline29Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE29 to value 0"]
impl crate::Resettable for SyscfgItline29Spec {
    const RESET_VALUE: u32 = 0;
}
