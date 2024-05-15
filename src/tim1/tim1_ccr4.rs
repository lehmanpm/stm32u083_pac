#[doc = "Register `TIM1_CCR4` reader"]
pub type R = crate::R<Tim1Ccr4Spec>;
#[doc = "Register `TIM1_CCR4` writer"]
pub type W = crate::W<Tim1Ccr4Spec>;
#[doc = "Field `CCR4` reader - Capture/Compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed."]
pub type Ccr4R = crate::FieldReader<u16>;
#[doc = "Field `CCR4` writer - Capture/Compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed."]
pub type Ccr4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub fn ccr4(&self) -> Ccr4R {
        Ccr4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ccr4(&mut self) -> Ccr4W<Tim1Ccr4Spec> {
        Ccr4W::new(self, 0)
    }
}
#[doc = "TIM1 capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccr4Spec;
impl crate::RegisterSpec for Tim1Ccr4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim1_ccr4::R`](R) reader structure"]
impl crate::Readable for Tim1Ccr4Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccr4::W`](W) writer structure"]
impl crate::Writable for Tim1Ccr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM1_CCR4 to value 0"]
impl crate::Resettable for Tim1Ccr4Spec {
    const RESET_VALUE: u16 = 0;
}
