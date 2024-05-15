#[doc = "Register `ADC_CR` reader"]
pub type R = crate::R<AdcCrSpec>;
#[doc = "Register `ADC_CR` writer"]
pub type W = crate::W<AdcCrSpec>;
#[doc = "ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aden {
    #[doc = "0: ADC is disabled (OFF state)"]
    B0x0 = 0,
    #[doc = "1: Write 1 to enable the ADC."]
    B0x1 = 1,
}
impl From<Aden> for bool {
    #[inline(always)]
    fn from(variant: Aden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` reader - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command."]
pub type AdenR = crate::BitReader<Aden>;
impl AdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aden {
        match self.bits {
            false => Aden::B0x0,
            true => Aden::B0x1,
        }
    }
    #[doc = "ADC is disabled (OFF state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Aden::B0x0
    }
    #[doc = "Write 1 to enable the ADC."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Aden::B0x1
    }
}
#[doc = "Field `ADEN` writer - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command."]
pub type AdenW<'a, REG> = crate::BitWriter<'a, REG, Aden>;
impl<'a, REG> AdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC is disabled (OFF state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Aden::B0x0)
    }
    #[doc = "Write 1 to enable the ADC."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Aden::B0x1)
    }
}
#[doc = "ADC disable command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addis {
    #[doc = "0: No ADDIS command ongoing"]
    B0x0 = 0,
    #[doc = "1: Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress."]
    B0x1 = 1,
}
impl From<Addis> for bool {
    #[inline(always)]
    fn from(variant: Addis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` reader - ADC disable command"]
pub type AddisR = crate::BitReader<Addis>;
impl AddisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addis {
        match self.bits {
            false => Addis::B0x0,
            true => Addis::B0x1,
        }
    }
    #[doc = "No ADDIS command ongoing"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Addis::B0x0
    }
    #[doc = "Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Addis::B0x1
    }
}
#[doc = "Field `ADDIS` writer - ADC disable command"]
pub type AddisW<'a, REG> = crate::BitWriter<'a, REG, Addis>;
impl<'a, REG> AddisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ADDIS command ongoing"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Addis::B0x0)
    }
    #[doc = "Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Addis::B0x1)
    }
}
#[doc = "ADC start conversion command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adstart {
    #[doc = "0: No ADC conversion is ongoing."]
    B0x0 = 0,
    #[doc = "1: Write 1 to start the ADC. Read 1 means that the ADC is operating and may be converting."]
    B0x1 = 1,
}
impl From<Adstart> for bool {
    #[inline(always)]
    fn from(variant: Adstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` reader - ADC start conversion command"]
pub type AdstartR = crate::BitReader<Adstart>;
impl AdstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adstart {
        match self.bits {
            false => Adstart::B0x0,
            true => Adstart::B0x1,
        }
    }
    #[doc = "No ADC conversion is ongoing."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adstart::B0x0
    }
    #[doc = "Write 1 to start the ADC. Read 1 means that the ADC is operating and may be converting."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adstart::B0x1
    }
}
#[doc = "Field `ADSTART` writer - ADC start conversion command"]
pub type AdstartW<'a, REG> = crate::BitWriter<'a, REG, Adstart>;
impl<'a, REG> AdstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ADC conversion is ongoing."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adstart::B0x0)
    }
    #[doc = "Write 1 to start the ADC. Read 1 means that the ADC is operating and may be converting."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adstart::B0x1)
    }
}
#[doc = "ADC stop conversion command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adstp {
    #[doc = "0: No ADC stop conversion command ongoing"]
    B0x0 = 0,
    #[doc = "1: Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress."]
    B0x1 = 1,
}
impl From<Adstp> for bool {
    #[inline(always)]
    fn from(variant: Adstp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` reader - ADC stop conversion command"]
pub type AdstpR = crate::BitReader<Adstp>;
impl AdstpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adstp {
        match self.bits {
            false => Adstp::B0x0,
            true => Adstp::B0x1,
        }
    }
    #[doc = "No ADC stop conversion command ongoing"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adstp::B0x0
    }
    #[doc = "Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adstp::B0x1
    }
}
#[doc = "Field `ADSTP` writer - ADC stop conversion command"]
pub type AdstpW<'a, REG> = crate::BitWriter<'a, REG, Adstp>;
impl<'a, REG> AdstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ADC stop conversion command ongoing"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adstp::B0x0)
    }
    #[doc = "Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adstp::B0x1)
    }
}
#[doc = "ADC Voltage Regulator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Advregen {
    #[doc = "0: ADC voltage regulator disabled"]
    B0x0 = 0,
    #[doc = "1: ADC voltage regulator enabled"]
    B0x1 = 1,
}
impl From<Advregen> for bool {
    #[inline(always)]
    fn from(variant: Advregen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADVREGEN` reader - ADC Voltage Regulator Enable"]
pub type AdvregenR = crate::BitReader<Advregen>;
impl AdvregenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Advregen {
        match self.bits {
            false => Advregen::B0x0,
            true => Advregen::B0x1,
        }
    }
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Advregen::B0x0
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Advregen::B0x1
    }
}
#[doc = "Field `ADVREGEN` writer - ADC Voltage Regulator Enable"]
pub type AdvregenW<'a, REG> = crate::BitWriter<'a, REG, Advregen>;
impl<'a, REG> AdvregenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Advregen::B0x0)
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Advregen::B0x1)
    }
}
#[doc = "ADC calibration This bit is set by software to start the calibration of the ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcal {
    #[doc = "0: Calibration complete"]
    B0x0 = 0,
    #[doc = "1: Write 1 to calibrate the ADC. Read at 1 means that a calibration is in progress."]
    B0x1 = 1,
}
impl From<Adcal> for bool {
    #[inline(always)]
    fn from(variant: Adcal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` reader - ADC calibration This bit is set by software to start the calibration of the ADC."]
pub type AdcalR = crate::BitReader<Adcal>;
impl AdcalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcal {
        match self.bits {
            false => Adcal::B0x0,
            true => Adcal::B0x1,
        }
    }
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adcal::B0x0
    }
    #[doc = "Write 1 to calibrate the ADC. Read at 1 means that a calibration is in progress."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adcal::B0x1
    }
}
#[doc = "Field `ADCAL` writer - ADC calibration This bit is set by software to start the calibration of the ADC."]
pub type AdcalW<'a, REG> = crate::BitWriter<'a, REG, Adcal>;
impl<'a, REG> AdcalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcal::B0x0)
    }
    #[doc = "Write 1 to calibrate the ADC. Read at 1 means that a calibration is in progress."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcal::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command."]
    #[inline(always)]
    pub fn aden(&self) -> AdenR {
        AdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC disable command"]
    #[inline(always)]
    pub fn addis(&self) -> AddisR {
        AddisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC start conversion command"]
    #[inline(always)]
    pub fn adstart(&self) -> AdstartR {
        AdstartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC stop conversion command"]
    #[inline(always)]
    pub fn adstp(&self) -> AdstpR {
        AdstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC Voltage Regulator Enable"]
    #[inline(always)]
    pub fn advregen(&self) -> AdvregenR {
        AdvregenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC."]
    #[inline(always)]
    pub fn adcal(&self) -> AdcalR {
        AdcalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable command This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the ADRDY flag has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command."]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> AdenW<AdcCrSpec> {
        AdenW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC disable command"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> AddisW<AdcCrSpec> {
        AddisW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC start conversion command"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> AdstartW<AdcCrSpec> {
        AdstartW::new(self, 2)
    }
    #[doc = "Bit 4 - ADC stop conversion command"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> AdstpW<AdcCrSpec> {
        AdstpW::new(self, 4)
    }
    #[doc = "Bit 28 - ADC Voltage Regulator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> AdvregenW<AdcCrSpec> {
        AdvregenW::new(self, 28)
    }
    #[doc = "Bit 31 - ADC calibration This bit is set by software to start the calibration of the ADC."]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> AdcalW<AdcCrSpec> {
        AdcalW::new(self, 31)
    }
}
#[doc = "ADC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCrSpec;
impl crate::RegisterSpec for AdcCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cr::R`](R) reader structure"]
impl crate::Readable for AdcCrSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_cr::W`](W) writer structure"]
impl crate::Writable for AdcCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CR to value 0"]
impl crate::Resettable for AdcCrSpec {
    const RESET_VALUE: u32 = 0;
}
