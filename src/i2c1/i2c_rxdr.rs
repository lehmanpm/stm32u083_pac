#[doc = "Register `I2C_RXDR` reader"]
pub type R = crate::R<I2cRxdrSpec>;
#[doc = "Field `RXDATA` reader - 8-bit receive data Data byte received from the I&lt;sup>2&lt;/sup>C bus"]
pub type RxdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 8-bit receive data Data byte received from the I&lt;sup>2&lt;/sup>C bus"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cRxdrSpec;
impl crate::RegisterSpec for I2cRxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_rxdr::R`](R) reader structure"]
impl crate::Readable for I2cRxdrSpec {}
#[doc = "`reset()` method sets I2C_RXDR to value 0"]
impl crate::Resettable for I2cRxdrSpec {
    const RESET_VALUE: u32 = 0;
}
