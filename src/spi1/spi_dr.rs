#[doc = "Register `SPI_DR` reader"]
pub type R = crate::R<SpiDrSpec>;
#[doc = "Register `SPI_DR` writer"]
pub type W = crate::W<SpiDrSpec>;
#[doc = "Field `DR` reader - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section133.4.9: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
pub type DrR = crate::FieldReader<u16>;
#[doc = "Field `DR` writer - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section133.4.9: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
pub type DrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section133.4.9: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See Section133.4.9: Data transmission and reception procedures). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DrW<SpiDrSpec> {
        DrW::new(self, 0)
    }
}
#[doc = "SPI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiDrSpec;
impl crate::RegisterSpec for SpiDrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_dr::R`](R) reader structure"]
impl crate::Readable for SpiDrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_dr::W`](W) writer structure"]
impl crate::Writable for SpiDrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_DR to value 0"]
impl crate::Resettable for SpiDrSpec {
    const RESET_VALUE: u16 = 0;
}
