#[doc = "Register `SPI_CRCPR` reader"]
pub type R = crate::R<SpiCrcprSpec>;
#[doc = "Register `SPI_CRCPR` writer"]
pub type W = crate::W<SpiCrcprSpec>;
#[doc = "Field `CRCPOLY` reader - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0x0007) is the reset value of this register. Another polynomial can be configured as required."]
pub type CrcpolyR = crate::FieldReader<u16>;
#[doc = "Field `CRCPOLY` writer - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0x0007) is the reset value of this register. Another polynomial can be configured as required."]
pub type CrcpolyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0x0007) is the reset value of this register. Another polynomial can be configured as required."]
    #[inline(always)]
    pub fn crcpoly(&self) -> CrcpolyR {
        CrcpolyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0x0007) is the reset value of this register. Another polynomial can be configured as required."]
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CrcpolyW<SpiCrcprSpec> {
        CrcpolyW::new(self, 0)
    }
}
#[doc = "SPI CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_crcpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_crcpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCrcprSpec;
impl crate::RegisterSpec for SpiCrcprSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_crcpr::R`](R) reader structure"]
impl crate::Readable for SpiCrcprSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_crcpr::W`](W) writer structure"]
impl crate::Writable for SpiCrcprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_CRCPR to value 0x07"]
impl crate::Resettable for SpiCrcprSpec {
    const RESET_VALUE: u16 = 0x07;
}
