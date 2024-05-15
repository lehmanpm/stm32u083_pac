#[doc = "Register `USART_TDR` reader"]
pub type R = crate::R<UsartTdrSpec>;
#[doc = "Register `USART_TDR` writer"]
pub type W = crate::W<UsartTdrSpec>;
#[doc = "Field `TDR` reader - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see Figure1227). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
pub type TdrR = crate::FieldReader<u16>;
#[doc = "Field `TDR` writer - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see Figure1227). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
pub type TdrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see Figure1227). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
    #[inline(always)]
    pub fn tdr(&self) -> TdrR {
        TdrR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see Figure1227). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
    #[inline(always)]
    #[must_use]
    pub fn tdr(&mut self) -> TdrW<UsartTdrSpec> {
        TdrW::new(self, 0)
    }
}
#[doc = "USART transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_tdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_tdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartTdrSpec;
impl crate::RegisterSpec for UsartTdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_tdr::R`](R) reader structure"]
impl crate::Readable for UsartTdrSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_tdr::W`](W) writer structure"]
impl crate::Writable for UsartTdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_TDR to value 0"]
impl crate::Resettable for UsartTdrSpec {
    const RESET_VALUE: u32 = 0;
}
