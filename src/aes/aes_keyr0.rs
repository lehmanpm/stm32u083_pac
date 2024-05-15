#[doc = "Register `AES_KEYR0` writer"]
pub type W = crate::W<AesKeyr0Spec>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[31:0\\]
These are bits \\[31:0\\]
of the write-only bitfield KEY\\[255:0\\]
AES encryption or decryption key, depending on the MODE\\[1:0\\]
bitfield of the AES_CR register. Writes to AES_KEYRx registers are ignored when AES is enabled (EN bit set). A special writing sequence is required. In this sequence, any valid write to AES_KEYRx register clears the KEYVALID flag except for the sequence-completing write that sets it. Also refer to the description of the KEYVALID flag in the AES_SR register."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[31:0\\]
These are bits \\[31:0\\]
of the write-only bitfield KEY\\[255:0\\]
AES encryption or decryption key, depending on the MODE\\[1:0\\]
bitfield of the AES_CR register. Writes to AES_KEYRx registers are ignored when AES is enabled (EN bit set). A special writing sequence is required. In this sequence, any valid write to AES_KEYRx register clears the KEYVALID flag except for the sequence-completing write that sets it. Also refer to the description of the KEYVALID flag in the AES_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<AesKeyr0Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "AES key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKeyr0Spec;
impl crate::RegisterSpec for AesKeyr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_keyr0::W`](W) writer structure"]
impl crate::Writable for AesKeyr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_KEYR0 to value 0"]
impl crate::Resettable for AesKeyr0Spec {
    const RESET_VALUE: u32 = 0;
}
