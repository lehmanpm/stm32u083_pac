#[doc = "Register `AES_DOUTR` reader"]
pub type R = crate::R<AesDoutrSpec>;
#[doc = "Field `DOUT` reader - Data output This read-only bitfield fetches a 32-bit output buffer. A four-fold sequential read of this bitfield, upon the computation completion (CCF flag set), virtually reads a complete 16-byte block of output data from the AES peripheral. Before reaching the output buffer, the data produced by the AES core are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield. Data weights from the first to the fourth read operation are: \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]."]
pub type DoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data output This read-only bitfield fetches a 32-bit output buffer. A four-fold sequential read of this bitfield, upon the computation completion (CCF flag set), virtually reads a complete 16-byte block of output data from the AES peripheral. Before reaching the output buffer, the data produced by the AES core are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield. Data weights from the first to the fourth read operation are: \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]."]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new(self.bits)
    }
}
#[doc = "AES data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_doutr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesDoutrSpec;
impl crate::RegisterSpec for AesDoutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_doutr::R`](R) reader structure"]
impl crate::Readable for AesDoutrSpec {}
#[doc = "`reset()` method sets AES_DOUTR to value 0"]
impl crate::Resettable for AesDoutrSpec {
    const RESET_VALUE: u32 = 0;
}
