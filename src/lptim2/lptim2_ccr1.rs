#[doc = "Register `LPTIM2_CCR1` reader"]
pub type R = crate::R<Lptim2Ccr1Spec>;
#[doc = "Register `LPTIM2_CCR1` writer"]
pub type W = crate::W<Lptim2Ccr1Spec>;
#[doc = "Field `CCR1` reader - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 becomes read-only, it contains the counter value transferred by the last input capture 1 event. The LPTIM2_CCR1 register is read-only and cannot be programmed."]
pub type Ccr1R = crate::FieldReader<u16>;
#[doc = "Field `CCR1` writer - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 becomes read-only, it contains the counter value transferred by the last input capture 1 event. The LPTIM2_CCR1 register is read-only and cannot be programmed."]
pub type Ccr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 becomes read-only, it contains the counter value transferred by the last input capture 1 event. The LPTIM2_CCR1 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub fn ccr1(&self) -> Ccr1R {
        Ccr1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 becomes read-only, it contains the counter value transferred by the last input capture 1 event. The LPTIM2_CCR1 register is read-only and cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ccr1(&mut self) -> Ccr1W<Lptim2Ccr1Spec> {
        Ccr1W::new(self, 0)
    }
}
#[doc = "LPTIM compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_ccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_ccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim2Ccr1Spec;
impl crate::RegisterSpec for Lptim2Ccr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim2_ccr1::R`](R) reader structure"]
impl crate::Readable for Lptim2Ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`lptim2_ccr1::W`](W) writer structure"]
impl crate::Writable for Lptim2Ccr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM2_CCR1 to value 0"]
impl crate::Resettable for Lptim2Ccr1Spec {
    const RESET_VALUE: u32 = 0;
}
