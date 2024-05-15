#[doc = "Register `TIM1_CCR6` reader"]
pub type R = crate::R<Tim1Ccr6Spec>;
#[doc = "Register `TIM1_CCR6` writer"]
pub type W = crate::W<Tim1Ccr6Spec>;
#[doc = "Field `CCR6` reader - Capture/Compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC6 output."]
pub type Ccr6R = crate::FieldReader<u16>;
#[doc = "Field `CCR6` writer - Capture/Compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC6 output."]
pub type Ccr6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC6 output."]
    #[inline(always)]
    pub fn ccr6(&self) -> Ccr6R {
        Ccr6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC6 output."]
    #[inline(always)]
    #[must_use]
    pub fn ccr6(&mut self) -> Ccr6W<Tim1Ccr6Spec> {
        Ccr6W::new(self, 0)
    }
}
#[doc = "TIM1 capture/compare register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccr6Spec;
impl crate::RegisterSpec for Tim1Ccr6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim1_ccr6::R`](R) reader structure"]
impl crate::Readable for Tim1Ccr6Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccr6::W`](W) writer structure"]
impl crate::Writable for Tim1Ccr6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM1_CCR6 to value 0"]
impl crate::Resettable for Tim1Ccr6Spec {
    const RESET_VALUE: u16 = 0;
}
