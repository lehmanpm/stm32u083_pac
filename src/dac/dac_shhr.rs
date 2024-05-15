#[doc = "Register `DAC_SHHR` reader"]
pub type R = crate::R<DacShhrSpec>;
#[doc = "Register `DAC_SHHR` writer"]
pub type W = crate::W<DacShhrSpec>;
#[doc = "Field `THOLD1` reader - DAC channel1 hold time (only valid in Sample and hold mode) Hold time1=1(THOLD\\[9:0\\]) x LSI clock period Note: This register can be modified only when EN11=10."]
pub type Thold1R = crate::FieldReader<u16>;
#[doc = "Field `THOLD1` writer - DAC channel1 hold time (only valid in Sample and hold mode) Hold time1=1(THOLD\\[9:0\\]) x LSI clock period Note: This register can be modified only when EN11=10."]
pub type Thold1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC channel1 hold time (only valid in Sample and hold mode) Hold time1=1(THOLD\\[9:0\\]) x LSI clock period Note: This register can be modified only when EN11=10."]
    #[inline(always)]
    pub fn thold1(&self) -> Thold1R {
        Thold1R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC channel1 hold time (only valid in Sample and hold mode) Hold time1=1(THOLD\\[9:0\\]) x LSI clock period Note: This register can be modified only when EN11=10."]
    #[inline(always)]
    #[must_use]
    pub fn thold1(&mut self) -> Thold1W<DacShhrSpec> {
        Thold1W::new(self, 0)
    }
}
#[doc = "DAC sample and hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shhr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shhr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacShhrSpec;
impl crate::RegisterSpec for DacShhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_shhr::R`](R) reader structure"]
impl crate::Readable for DacShhrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_shhr::W`](W) writer structure"]
impl crate::Writable for DacShhrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SHHR to value 0x0001_0001"]
impl crate::Resettable for DacShhrSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
