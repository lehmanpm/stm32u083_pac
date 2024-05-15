#[doc = "Register `CRC_INIT` reader"]
pub type R = crate::R<CrcInitSpec>;
#[doc = "Register `CRC_INIT` writer"]
pub type W = crate::W<CrcInitSpec>;
#[doc = "Field `CRC_INIT` reader - Programmable initial CRC value This register is used to write the CRC initial value."]
pub type CrcInitR = crate::FieldReader<u32>;
#[doc = "Field `CRC_INIT` writer - Programmable initial CRC value This register is used to write the CRC initial value."]
pub type CrcInitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable initial CRC value This register is used to write the CRC initial value."]
    #[inline(always)]
    pub fn crc_init(&self) -> CrcInitR {
        CrcInitR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable initial CRC value This register is used to write the CRC initial value."]
    #[inline(always)]
    #[must_use]
    pub fn crc_init(&mut self) -> CrcInitW<CrcInitSpec> {
        CrcInitW::new(self, 0)
    }
}
#[doc = "CRC initial value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcInitSpec;
impl crate::RegisterSpec for CrcInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_init::R`](R) reader structure"]
impl crate::Readable for CrcInitSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_init::W`](W) writer structure"]
impl crate::Writable for CrcInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_INIT to value 0xffff_ffff"]
impl crate::Resettable for CrcInitSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
