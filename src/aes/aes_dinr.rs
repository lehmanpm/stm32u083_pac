#[doc = "Register `AES_DINR` writer"]
pub type W = crate::W<AesDinrSpec>;
#[doc = "Field `DIN` writer - Data input A four-fold sequential write to this bitfield during the Input phase results in writing a complete 16-bytes block of input data to the AES peripheral. From the first to the fourth write, the corresponding data weights are \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield, then written into the AES core 16-bytes input buffer. Reads return zero."]
pub type DinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Data input A four-fold sequential write to this bitfield during the Input phase results in writing a complete 16-bytes block of input data to the AES peripheral. From the first to the fourth write, the corresponding data weights are \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield, then written into the AES core 16-bytes input buffer. Reads return zero."]
    #[inline(always)]
    #[must_use]
    pub fn din(&mut self) -> DinW<AesDinrSpec> {
        DinW::new(self, 0)
    }
}
#[doc = "AES data input register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_dinr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDinrSpec;
impl crate::RegisterSpec for AesDinrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_dinr::W`](W) writer structure"]
impl crate::Writable for AesDinrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_DINR to value 0"]
impl crate::Resettable for AesDinrSpec {
    const RESET_VALUE: u32 = 0;
}
