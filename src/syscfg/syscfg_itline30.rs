#[doc = "Register `SYSCFG_ITLINE30` reader"]
pub type R = crate::R<SyscfgItline30Spec>;
#[doc = "Field `USART4` reader - USART4 interrupt request pending"]
pub type Usart4R = crate::BitReader;
#[doc = "Field `LPUART3` reader - LPUART3 interrupt request pending (EXTI line 32)"]
pub type Lpuart3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART4 interrupt request pending"]
    #[inline(always)]
    pub fn usart4(&self) -> Usart4R {
        Usart4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPUART3 interrupt request pending (EXTI line 32)"]
    #[inline(always)]
    pub fn lpuart3(&self) -> Lpuart3R {
        Lpuart3R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 30 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline30::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline30Spec;
impl crate::RegisterSpec for SyscfgItline30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline30::R`](R) reader structure"]
impl crate::Readable for SyscfgItline30Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE30 to value 0"]
impl crate::Resettable for SyscfgItline30Spec {
    const RESET_VALUE: u32 = 0;
}
