#[doc = "Register `LPTIM3_CCR2` reader"]
pub type R = crate::R<Lptim3Ccr2Spec>;
#[doc = "Register `LPTIM3_CCR2` writer"]
pub type W = crate::W<Lptim3Ccr2Spec>;
#[doc = "Field `CCR2` reader - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the capture/compare 2 register. Depending on the PRELOAD option, the CCR2 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 2 contains the value to be compared to the counter LPTIM3_CNT and signaled on OC2 output. If channel CC2 is configured as input: CCR2 becomes read-only, it contains the counter value transferred by the last input capture 2 event. The LPTIM3_CCR2 register is read-only and cannot be programmed."]
pub type Ccr2R = crate::FieldReader<u16>;
#[doc = "Field `CCR2` writer - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the capture/compare 2 register. Depending on the PRELOAD option, the CCR2 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 2 contains the value to be compared to the counter LPTIM3_CNT and signaled on OC2 output. If channel CC2 is configured as input: CCR2 becomes read-only, it contains the counter value transferred by the last input capture 2 event. The LPTIM3_CCR2 register is read-only and cannot be programmed."]
pub type Ccr2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the capture/compare 2 register. Depending on the PRELOAD option, the CCR2 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 2 contains the value to be compared to the counter LPTIM3_CNT and signaled on OC2 output. If channel CC2 is configured as input: CCR2 becomes read-only, it contains the counter value transferred by the last input capture 2 event. The LPTIM3_CCR2 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub fn ccr2(&self) -> Ccr2R {
        Ccr2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the capture/compare 2 register. Depending on the PRELOAD option, the CCR2 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 2 contains the value to be compared to the counter LPTIM3_CNT and signaled on OC2 output. If channel CC2 is configured as input: CCR2 becomes read-only, it contains the counter value transferred by the last input capture 2 event. The LPTIM3_CCR2 register is read-only and cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ccr2(&mut self) -> Ccr2W<Lptim3Ccr2Spec> {
        Ccr2W::new(self, 0)
    }
}
#[doc = "LPTIM compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_ccr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_ccr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim3Ccr2Spec;
impl crate::RegisterSpec for Lptim3Ccr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim3_ccr2::R`](R) reader structure"]
impl crate::Readable for Lptim3Ccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`lptim3_ccr2::W`](W) writer structure"]
impl crate::Writable for Lptim3Ccr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM3_CCR2 to value 0"]
impl crate::Resettable for Lptim3Ccr2Spec {
    const RESET_VALUE: u32 = 0;
}
