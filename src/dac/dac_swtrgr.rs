#[doc = "Register `DAC_SWTRGR` writer"]
pub type W = crate::W<DacSwtrgrSpec>;
#[doc = "DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swtrig1 {
    #[doc = "0: No trigger"]
    B0x0 = 0,
    #[doc = "1: Trigger"]
    B0x1 = 1,
}
impl From<Swtrig1> for bool {
    #[inline(always)]
    fn from(variant: Swtrig1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
pub type Swtrig1W<'a, REG> = crate::BitWriter<'a, REG, Swtrig1>;
impl<'a, REG> Swtrig1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swtrig1::B0x0)
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swtrig1::B0x1)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> Swtrig1W<DacSwtrgrSpec> {
        Swtrig1W::new(self, 0)
    }
}
#[doc = "DAC software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacSwtrgrSpec;
impl crate::RegisterSpec for DacSwtrgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dac_swtrgr::W`](W) writer structure"]
impl crate::Writable for DacSwtrgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SWTRGR to value 0"]
impl crate::Resettable for DacSwtrgrSpec {
    const RESET_VALUE: u32 = 0;
}
