#[doc = "Register `DAC_SR` reader"]
pub type R = crate::R<DacSrSpec>;
#[doc = "Register `DAC_SR` writer"]
pub type W = crate::W<DacSrSpec>;
#[doc = "DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaudr1 {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel1"]
    B0x0 = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    B0x1 = 1,
}
impl From<Dmaudr1> for bool {
    #[inline(always)]
    fn from(variant: Dmaudr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type Dmaudr1R = crate::BitReader<Dmaudr1>;
impl Dmaudr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaudr1 {
        match self.bits {
            false => Dmaudr1::B0x0,
            true => Dmaudr1::B0x1,
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmaudr1::B0x0
    }
    #[doc = "DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmaudr1::B0x1
    }
}
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub type Dmaudr1W<'a, REG> = crate::BitWriter<'a, REG, Dmaudr1>;
impl<'a, REG> Dmaudr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA underrun error condition occurred for DAC channel1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaudr1::B0x0)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaudr1::B0x1)
    }
}
#[doc = "DAC channel1 calibration offset status This bit is set and cleared by hardware\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalFlag1 {
    #[doc = "0: calibration trimming value is lower than the offset correction value"]
    B0x0 = 0,
    #[doc = "1: calibration trimming value is equal or greater than the offset correction value"]
    B0x1 = 1,
}
impl From<CalFlag1> for bool {
    #[inline(always)]
    fn from(variant: CalFlag1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL_FLAG1` reader - DAC channel1 calibration offset status This bit is set and cleared by hardware"]
pub type CalFlag1R = crate::BitReader<CalFlag1>;
impl CalFlag1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalFlag1 {
        match self.bits {
            false => CalFlag1::B0x0,
            true => CalFlag1::B0x1,
        }
    }
    #[doc = "calibration trimming value is lower than the offset correction value"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CalFlag1::B0x0
    }
    #[doc = "calibration trimming value is equal or greater than the offset correction value"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CalFlag1::B0x1
    }
}
#[doc = "DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI periods of synchronization).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwst1 {
    #[doc = "0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written"]
    B0x0 = 0,
    #[doc = "1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written"]
    B0x1 = 1,
}
impl From<Bwst1> for bool {
    #[inline(always)]
    fn from(variant: Bwst1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWST1` reader - DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI periods of synchronization)."]
pub type Bwst1R = crate::BitReader<Bwst1>;
impl Bwst1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwst1 {
        match self.bits {
            false => Bwst1::B0x0,
            true => Bwst1::B0x1,
        }
    }
    #[doc = "There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bwst1::B0x0
    }
    #[doc = "There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bwst1::B0x1
    }
}
impl R {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr1(&self) -> Dmaudr1R {
        Dmaudr1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC channel1 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CalFlag1R {
        CalFlag1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI periods of synchronization)."]
    #[inline(always)]
    pub fn bwst1(&self) -> Bwst1R {
        Bwst1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> Dmaudr1W<DacSrSpec> {
        Dmaudr1W::new(self, 13)
    }
}
#[doc = "DAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacSrSpec;
impl crate::RegisterSpec for DacSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_sr::R`](R) reader structure"]
impl crate::Readable for DacSrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_sr::W`](W) writer structure"]
impl crate::Writable for DacSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SR to value 0"]
impl crate::Resettable for DacSrSpec {
    const RESET_VALUE: u32 = 0;
}
