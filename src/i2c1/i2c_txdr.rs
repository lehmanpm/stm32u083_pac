#[doc = "Register `I2C_TXDR` reader"]
pub type R = crate::R<I2cTxdrSpec>;
#[doc = "Register `I2C_TXDR` writer"]
pub type W = crate::W<I2cTxdrSpec>;
#[doc = "Field `TXDATA` reader - 8-bit transmit data Data byte to be transmitted to the I&lt;sup>2&lt;/sup>C bus Note: These bits can be written only when TXE = 1."]
pub type TxdataR = crate::FieldReader;
#[doc = "Field `TXDATA` writer - 8-bit transmit data Data byte to be transmitted to the I&lt;sup>2&lt;/sup>C bus Note: These bits can be written only when TXE = 1."]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit transmit data Data byte to be transmitted to the I&lt;sup>2&lt;/sup>C bus Note: These bits can be written only when TXE = 1."]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit transmit data Data byte to be transmitted to the I&lt;sup>2&lt;/sup>C bus Note: These bits can be written only when TXE = 1."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<I2cTxdrSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "I2C transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_txdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_txdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cTxdrSpec;
impl crate::RegisterSpec for I2cTxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_txdr::R`](R) reader structure"]
impl crate::Readable for I2cTxdrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_txdr::W`](W) writer structure"]
impl crate::Writable for I2cTxdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_TXDR to value 0"]
impl crate::Resettable for I2cTxdrSpec {
    const RESET_VALUE: u32 = 0;
}
