#[doc = "Register `LPTIM3_CCR4` reader"]
pub type R = crate::R<Lptim3Ccr4Spec>;
#[doc = "Register `LPTIM3_CCR4` writer"]
pub type W = crate::W<Lptim3Ccr4Spec>;
#[doc = "Field `CCR4` reader - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM3_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM3_CCR4 register is read-only and cannot be programmed."]
pub type Ccr4R = crate::FieldReader<u16>;
#[doc = "Field `CCR4` writer - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM3_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM3_CCR4 register is read-only and cannot be programmed."]
pub type Ccr4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM3_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM3_CCR4 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub fn ccr4(&self) -> Ccr4R {
        Ccr4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM3_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM3_CCR4 register is read-only and cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ccr4(&mut self) -> Ccr4W<Lptim3Ccr4Spec> {
        Ccr4W::new(self, 0)
    }
}
#[doc = "LPTIM compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_ccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_ccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim3Ccr4Spec;
impl crate::RegisterSpec for Lptim3Ccr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim3_ccr4::R`](R) reader structure"]
impl crate::Readable for Lptim3Ccr4Spec {}
#[doc = "`write(|w| ..)` method takes [`lptim3_ccr4::W`](W) writer structure"]
impl crate::Writable for Lptim3Ccr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM3_CCR4 to value 0"]
impl crate::Resettable for Lptim3Ccr4Spec {
    const RESET_VALUE: u32 = 0;
}
