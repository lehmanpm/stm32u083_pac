#[doc = "Register `DAC_DOR1` reader"]
pub type R = crate::R<DacDor1Spec>;
#[doc = "Field `DACC1DOR` reader - DAC channel1 data output These bits are read-only, they contain data output for DAC channel1."]
pub type Dacc1dorR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 data output These bits are read-only, they contain data output for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dor(&self) -> Dacc1dorR {
        Dacc1dorR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC channel1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dor1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDor1Spec;
impl crate::RegisterSpec for DacDor1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dor1::R`](R) reader structure"]
impl crate::Readable for DacDor1Spec {}
#[doc = "`reset()` method sets DAC_DOR1 to value 0"]
impl crate::Resettable for DacDor1Spec {
    const RESET_VALUE: u32 = 0;
}
