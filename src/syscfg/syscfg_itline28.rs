#[doc = "Register `SYSCFG_ITLINE28` reader"]
pub type R = crate::R<SyscfgItline28Spec>;
#[doc = "Field `USART2` reader - USART2 interrupt request pending (EXTI line 35)"]
pub type Usart2R = crate::BitReader;
#[doc = "Field `LPUART2` reader - LPUART2 interrupt request pending (EXTI line 31)"]
pub type Lpuart2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART2 interrupt request pending (EXTI line 35)"]
    #[inline(always)]
    pub fn usart2(&self) -> Usart2R {
        Usart2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPUART2 interrupt request pending (EXTI line 31)"]
    #[inline(always)]
    pub fn lpuart2(&self) -> Lpuart2R {
        Lpuart2R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 28 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline28::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline28Spec;
impl crate::RegisterSpec for SyscfgItline28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline28::R`](R) reader structure"]
impl crate::Readable for SyscfgItline28Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE28 to value 0"]
impl crate::Resettable for SyscfgItline28Spec {
    const RESET_VALUE: u32 = 0;
}
