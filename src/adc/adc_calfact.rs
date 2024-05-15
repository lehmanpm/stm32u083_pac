#[doc = "Register `ADC_CALFACT` reader"]
pub type R = crate::R<AdcCalfactSpec>;
#[doc = "Register `ADC_CALFACT` writer"]
pub type W = crate::W<AdcCalfactSpec>;
#[doc = "Field `CALFACT` reader - Calibration factor These bits are written by hardware or by software. Once a calibration is complete,1they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new conversion is launched. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
pub type CalfactR = crate::FieldReader;
#[doc = "Field `CALFACT` writer - Calibration factor These bits are written by hardware or by software. Once a calibration is complete,1they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new conversion is launched. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
pub type CalfactW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Calibration factor These bits are written by hardware or by software. Once a calibration is complete,1they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new conversion is launched. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
    #[inline(always)]
    pub fn calfact(&self) -> CalfactR {
        CalfactR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration factor These bits are written by hardware or by software. Once a calibration is complete,1they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new conversion is launched. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn calfact(&mut self) -> CalfactW<AdcCalfactSpec> {
        CalfactW::new(self, 0)
    }
}
#[doc = "ADC calibration factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_calfact::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_calfact::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCalfactSpec;
impl crate::RegisterSpec for AdcCalfactSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_calfact::R`](R) reader structure"]
impl crate::Readable for AdcCalfactSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_calfact::W`](W) writer structure"]
impl crate::Writable for AdcCalfactSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CALFACT to value 0"]
impl crate::Resettable for AdcCalfactSpec {
    const RESET_VALUE: u32 = 0;
}
