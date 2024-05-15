#[doc = "Register `TIM1_CCR1` reader"]
pub type R = crate::R<Tim1Ccr1Spec>;
#[doc = "Register `TIM1_CCR1` writer"]
pub type W = crate::W<Tim1Ccr1Spec>;
#[doc = "Field `CCR1` reader - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1 is configured as input: CR1 is the counter value transferred by the last input capture 1 event (IC1). The TIMx_CCR1 register is read-only and cannot be programmed."]
pub type Ccr1R = crate::FieldReader<u16>;
#[doc = "Field `CCR1` writer - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1 is configured as input: CR1 is the counter value transferred by the last input capture 1 event (IC1). The TIMx_CCR1 register is read-only and cannot be programmed."]
pub type Ccr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1 is configured as input: CR1 is the counter value transferred by the last input capture 1 event (IC1). The TIMx_CCR1 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub fn ccr1(&self) -> Ccr1R {
        Ccr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1 is configured as input: CR1 is the counter value transferred by the last input capture 1 event (IC1). The TIMx_CCR1 register is read-only and cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ccr1(&mut self) -> Ccr1W<Tim1Ccr1Spec> {
        Ccr1W::new(self, 0)
    }
}
#[doc = "TIM1 capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccr1Spec;
impl crate::RegisterSpec for Tim1Ccr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim1_ccr1::R`](R) reader structure"]
impl crate::Readable for Tim1Ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccr1::W`](W) writer structure"]
impl crate::Writable for Tim1Ccr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM1_CCR1 to value 0"]
impl crate::Resettable for Tim1Ccr1Spec {
    const RESET_VALUE: u16 = 0;
}
