#[doc = "Register `AES_SUSPR7` reader"]
pub type R = crate::R<AesSuspr7Spec>;
#[doc = "Register `AES_SUSPR7` writer"]
pub type W = crate::W<AesSuspr7Spec>;
#[doc = "Field `SUSP` reader - Suspend data AES_SUSPRx registers contain the complete internal register states of the AES when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section121.4.8: AES suspend and resume operations for more details. Read to this register returns zero when EN bit is cleared in AES_SR register. AES_SUSPRx registers are not used in other chaining modes than GCM, GMAC or CCM."]
pub type SuspR = crate::FieldReader<u32>;
#[doc = "Field `SUSP` writer - Suspend data AES_SUSPRx registers contain the complete internal register states of the AES when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section121.4.8: AES suspend and resume operations for more details. Read to this register returns zero when EN bit is cleared in AES_SR register. AES_SUSPRx registers are not used in other chaining modes than GCM, GMAC or CCM."]
pub type SuspW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Suspend data AES_SUSPRx registers contain the complete internal register states of the AES when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section121.4.8: AES suspend and resume operations for more details. Read to this register returns zero when EN bit is cleared in AES_SR register. AES_SUSPRx registers are not used in other chaining modes than GCM, GMAC or CCM."]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Suspend data AES_SUSPRx registers contain the complete internal register states of the AES when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section121.4.8: AES suspend and resume operations for more details. Read to this register returns zero when EN bit is cleared in AES_SR register. AES_SUSPRx registers are not used in other chaining modes than GCM, GMAC or CCM."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SuspW<AesSuspr7Spec> {
        SuspW::new(self, 0)
    }
}
#[doc = "AES suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_suspr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_suspr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesSuspr7Spec;
impl crate::RegisterSpec for AesSuspr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_suspr7::R`](R) reader structure"]
impl crate::Readable for AesSuspr7Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_suspr7::W`](W) writer structure"]
impl crate::Writable for AesSuspr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_SUSPR7 to value 0"]
impl crate::Resettable for AesSuspr7Spec {
    const RESET_VALUE: u32 = 0;
}
