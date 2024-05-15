#[doc = "Register `LPUART_RDR` reader"]
pub type R = crate::R<LpuartRdrSpec>;
#[doc = "Field `RDR` reader - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see Figure1254). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
pub type RdrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see Figure1254). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "LPUART receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpuart_rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpuartRdrSpec;
impl crate::RegisterSpec for LpuartRdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpuart_rdr::R`](R) reader structure"]
impl crate::Readable for LpuartRdrSpec {}
#[doc = "`reset()` method sets LPUART_RDR to value 0"]
impl crate::Resettable for LpuartRdrSpec {
    const RESET_VALUE: u32 = 0;
}
