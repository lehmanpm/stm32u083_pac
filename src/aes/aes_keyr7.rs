#[doc = "Register `AES_KEYR7` writer"]
pub type W = crate::W<AesKeyr7Spec>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[255:224\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield and for information relative to writing AES_KEYRx registers."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[255:224\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield and for information relative to writing AES_KEYRx registers."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<AesKeyr7Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "AES key register 7\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKeyr7Spec;
impl crate::RegisterSpec for AesKeyr7Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_keyr7::W`](W) writer structure"]
impl crate::Writable for AesKeyr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_KEYR7 to value 0"]
impl crate::Resettable for AesKeyr7Spec {
    const RESET_VALUE: u32 = 0;
}