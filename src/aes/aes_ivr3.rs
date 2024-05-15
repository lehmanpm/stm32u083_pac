#[doc = "Register `AES_IVR3` reader"]
pub type R = crate::R<AesIvr3Spec>;
#[doc = "Register `AES_IVR3` writer"]
pub type W = crate::W<AesIvr3Spec>;
#[doc = "Field `IVI` reader - Initialization vector input, bits \\[127:96\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
pub type IviR = crate::FieldReader<u32>;
#[doc = "Field `IVI` writer - Initialization vector input, bits \\[127:96\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
pub type IviW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initialization vector input, bits \\[127:96\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
    #[inline(always)]
    pub fn ivi(&self) -> IviR {
        IviR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization vector input, bits \\[127:96\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn ivi(&mut self) -> IviW<AesIvr3Spec> {
        IviW::new(self, 0)
    }
}
#[doc = "AES initialization vector register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ivr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ivr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesIvr3Spec;
impl crate::RegisterSpec for AesIvr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_ivr3::R`](R) reader structure"]
impl crate::Readable for AesIvr3Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_ivr3::W`](W) writer structure"]
impl crate::Writable for AesIvr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_IVR3 to value 0"]
impl crate::Resettable for AesIvr3Spec {
    const RESET_VALUE: u32 = 0;
}
