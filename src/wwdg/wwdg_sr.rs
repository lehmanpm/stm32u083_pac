#[doc = "Register `WWDG_SR` reader"]
pub type R = crate::R<WwdgSrSpec>;
#[doc = "Register `WWDG_SR` writer"]
pub type W = crate::W<WwdgSrSpec>;
#[doc = "Field `EWIF` reader - Early wake-up interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. Writing 1 has no effect. This bit is also set if the interrupt is not enabled."]
pub type EwifR = crate::BitReader;
#[doc = "Field `EWIF` writer - Early wake-up interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. Writing 1 has no effect. This bit is also set if the interrupt is not enabled."]
pub type EwifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Early wake-up interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. Writing 1 has no effect. This bit is also set if the interrupt is not enabled."]
    #[inline(always)]
    pub fn ewif(&self) -> EwifR {
        EwifR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Early wake-up interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. Writing 1 has no effect. This bit is also set if the interrupt is not enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ewif(&mut self) -> EwifW<WwdgSrSpec> {
        EwifW::new(self, 0)
    }
}
#[doc = "WWDG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WwdgSrSpec;
impl crate::RegisterSpec for WwdgSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_sr::R`](R) reader structure"]
impl crate::Readable for WwdgSrSpec {}
#[doc = "`write(|w| ..)` method takes [`wwdg_sr::W`](W) writer structure"]
impl crate::Writable for WwdgSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDG_SR to value 0"]
impl crate::Resettable for WwdgSrSpec {
    const RESET_VALUE: u32 = 0;
}
