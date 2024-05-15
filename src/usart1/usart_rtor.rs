#[doc = "Register `USART_RTOR` reader"]
pub type R = crate::R<UsartRtorSpec>;
#[doc = "Register `USART_RTOR` writer"]
pub type W = crate::W<UsartRtorSpec>;
#[doc = "Field `RTO` reader - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bit duration. In Standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character."]
pub type RtoR = crate::FieldReader<u32>;
#[doc = "Field `RTO` writer - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bit duration. In Standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character."]
pub type RtoW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BLEN` reader - Block Length This bitfield gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 -> 0 information characters + LEC BLEN = 1 -> 0 information characters + CRC BLEN = 255 -> 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block."]
pub type BlenR = crate::FieldReader;
#[doc = "Field `BLEN` writer - Block Length This bitfield gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 -> 0 information characters + LEC BLEN = 1 -> 0 information characters + CRC BLEN = 255 -> 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block."]
pub type BlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bit duration. In Standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character."]
    #[inline(always)]
    pub fn rto(&self) -> RtoR {
        RtoR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Block Length This bitfield gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 -> 0 information characters + LEC BLEN = 1 -> 0 information characters + CRC BLEN = 255 -> 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block."]
    #[inline(always)]
    pub fn blen(&self) -> BlenR {
        BlenR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bit duration. In Standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character."]
    #[inline(always)]
    #[must_use]
    pub fn rto(&mut self) -> RtoW<UsartRtorSpec> {
        RtoW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Block Length This bitfield gives the Block length in Smartcard T=1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0 -> 0 information characters + LEC BLEN = 1 -> 0 information characters + CRC BLEN = 255 -> 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE=0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE=0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block."]
    #[inline(always)]
    #[must_use]
    pub fn blen(&mut self) -> BlenW<UsartRtorSpec> {
        BlenW::new(self, 24)
    }
}
#[doc = "USART receiver timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_rtor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_rtor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsartRtorSpec;
impl crate::RegisterSpec for UsartRtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart_rtor::R`](R) reader structure"]
impl crate::Readable for UsartRtorSpec {}
#[doc = "`write(|w| ..)` method takes [`usart_rtor::W`](W) writer structure"]
impl crate::Writable for UsartRtorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART_RTOR to value 0"]
impl crate::Resettable for UsartRtorSpec {
    const RESET_VALUE: u32 = 0;
}
