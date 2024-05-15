#[doc = "Register `LPTIM2_CCR3` reader"]
pub type R = crate::R<Lptim2Ccr3Spec>;
#[doc = "Register `LPTIM2_CCR3` writer"]
pub type W = crate::W<Lptim2Ccr3Spec>;
#[doc = "Field `CCR3` reader - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the capture/compare 3 register. Depending on the PRELOAD option, the CCR3 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 3 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC3 output. If channel CC3 is configured as input: CCR3 becomes read-only, it contains the counter value transferred by the last input capture 3 event. The LPTIM2_CCR3 register is read-only and cannot be programmed."]
pub type Ccr3R = crate::FieldReader<u16>;
#[doc = "Field `CCR3` writer - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the capture/compare 3 register. Depending on the PRELOAD option, the CCR3 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 3 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC3 output. If channel CC3 is configured as input: CCR3 becomes read-only, it contains the counter value transferred by the last input capture 3 event. The LPTIM2_CCR3 register is read-only and cannot be programmed."]
pub type Ccr3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the capture/compare 3 register. Depending on the PRELOAD option, the CCR3 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 3 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC3 output. If channel CC3 is configured as input: CCR3 becomes read-only, it contains the counter value transferred by the last input capture 3 event. The LPTIM2_CCR3 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub fn ccr3(&self) -> Ccr3R {
        Ccr3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the capture/compare 3 register. Depending on the PRELOAD option, the CCR3 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 3 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC3 output. If channel CC3 is configured as input: CCR3 becomes read-only, it contains the counter value transferred by the last input capture 3 event. The LPTIM2_CCR3 register is read-only and cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ccr3(&mut self) -> Ccr3W<Lptim2Ccr3Spec> {
        Ccr3W::new(self, 0)
    }
}
#[doc = "LPTIM compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_ccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim2_ccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim2Ccr3Spec;
impl crate::RegisterSpec for Lptim2Ccr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim2_ccr3::R`](R) reader structure"]
impl crate::Readable for Lptim2Ccr3Spec {}
#[doc = "`write(|w| ..)` method takes [`lptim2_ccr3::W`](W) writer structure"]
impl crate::Writable for Lptim2Ccr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM2_CCR3 to value 0"]
impl crate::Resettable for Lptim2Ccr3Spec {
    const RESET_VALUE: u32 = 0;
}
