#[doc = "Register `DAC_CCR` reader"]
pub type R = crate::R<DacCcrSpec>;
#[doc = "Register `DAC_CCR` writer"]
pub type W = crate::W<DacCcrSpec>;
#[doc = "Field `OTRIM1` reader - DAC channel1 offset trimming value"]
pub type Otrim1R = crate::FieldReader;
#[doc = "Field `OTRIM1` writer - DAC channel1 offset trimming value"]
pub type Otrim1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DAC channel1 offset trimming value"]
    #[inline(always)]
    pub fn otrim1(&self) -> Otrim1R {
        Otrim1R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DAC channel1 offset trimming value"]
    #[inline(always)]
    #[must_use]
    pub fn otrim1(&mut self) -> Otrim1W<DacCcrSpec> {
        Otrim1W::new(self, 0)
    }
}
#[doc = "DAC calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCcrSpec;
impl crate::RegisterSpec for DacCcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_ccr::R`](R) reader structure"]
impl crate::Readable for DacCcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_ccr::W`](W) writer structure"]
impl crate::Writable for DacCcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_CCR to value 0"]
impl crate::Resettable for DacCcrSpec {
    const RESET_VALUE: u32 = 0;
}
