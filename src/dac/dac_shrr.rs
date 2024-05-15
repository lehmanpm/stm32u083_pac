#[doc = "Register `DAC_SHRR` reader"]
pub type R = crate::R<DacShrrSpec>;
#[doc = "Register `DAC_SHRR` writer"]
pub type W = crate::W<DacShrrSpec>;
#[doc = "Field `TREFRESH1` reader - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time1=1(TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN11=10."]
pub type Trefresh1R = crate::FieldReader;
#[doc = "Field `TREFRESH1` writer - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time1=1(TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN11=10."]
pub type Trefresh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time1=1(TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN11=10."]
    #[inline(always)]
    pub fn trefresh1(&self) -> Trefresh1R {
        Trefresh1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time1=1(TREFRESH\\[7:0\\]) x LSI clock period Note: This register can be modified only when EN11=10."]
    #[inline(always)]
    #[must_use]
    pub fn trefresh1(&mut self) -> Trefresh1W<DacShrrSpec> {
        Trefresh1W::new(self, 0)
    }
}
#[doc = "DAC sample and hold refresh time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacShrrSpec;
impl crate::RegisterSpec for DacShrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_shrr::R`](R) reader structure"]
impl crate::Readable for DacShrrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_shrr::W`](W) writer structure"]
impl crate::Writable for DacShrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SHRR to value 0x0001_0001"]
impl crate::Resettable for DacShrrSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
