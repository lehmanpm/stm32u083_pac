#[doc = "Register `DAC_DHR12L1` reader"]
pub type R = crate::R<DacDhr12l1Spec>;
#[doc = "Register `DAC_DHR12L1` writer"]
pub type W = crate::W<DacDhr12l1Spec>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit left-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
pub type Dacc1dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit left-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
pub type Dacc1dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> Dacc1dhrR {
        Dacc1dhrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> Dacc1dhrW<DacDhr12l1Spec> {
        Dacc1dhrW::new(self, 4)
    }
}
#[doc = "DAC channel1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12l1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12l1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDhr12l1Spec;
impl crate::RegisterSpec for DacDhr12l1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr12l1::R`](R) reader structure"]
impl crate::Readable for DacDhr12l1Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr12l1::W`](W) writer structure"]
impl crate::Writable for DacDhr12l1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_DHR12L1 to value 0"]
impl crate::Resettable for DacDhr12l1Spec {
    const RESET_VALUE: u32 = 0;
}
