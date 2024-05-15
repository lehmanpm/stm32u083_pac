#[doc = "Register `CRS_CFGR` reader"]
pub type R = crate::R<CrsCfgrSpec>;
#[doc = "Register `CRS_CFGR` writer"]
pub type W = crate::W<CrsCfgrSpec>;
#[doc = "Field `RELOAD` reader - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section15.4.3 for more details about counter behavior."]
pub type ReloadR = crate::FieldReader<u16>;
#[doc = "Field `RELOAD` writer - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section15.4.3 for more details about counter behavior."]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FELIM` reader - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section15.4.4 for more details about FECAP evaluation."]
pub type FelimR = crate::FieldReader;
#[doc = "Field `FELIM` writer - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section15.4.4 for more details about FECAP evaluation."]
pub type FelimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syncdiv {
    #[doc = "0: SYNC not divided (default)"]
    B0x0 = 0,
    #[doc = "1: SYNC divided by 2"]
    B0x1 = 1,
    #[doc = "2: SYNC divided by 4"]
    B0x2 = 2,
    #[doc = "3: SYNC divided by 8"]
    B0x3 = 3,
    #[doc = "4: SYNC divided by 16"]
    B0x4 = 4,
    #[doc = "5: SYNC divided by 32"]
    B0x5 = 5,
    #[doc = "6: SYNC divided by 64"]
    B0x6 = 6,
    #[doc = "7: SYNC divided by 128"]
    B0x7 = 7,
}
impl From<Syncdiv> for u8 {
    #[inline(always)]
    fn from(variant: Syncdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syncdiv {
    type Ux = u8;
}
impl crate::IsEnum for Syncdiv {}
#[doc = "Field `SYNCDIV` reader - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SyncdivR = crate::FieldReader<Syncdiv>;
impl SyncdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncdiv {
        match self.bits {
            0 => Syncdiv::B0x0,
            1 => Syncdiv::B0x1,
            2 => Syncdiv::B0x2,
            3 => Syncdiv::B0x3,
            4 => Syncdiv::B0x4,
            5 => Syncdiv::B0x5,
            6 => Syncdiv::B0x6,
            7 => Syncdiv::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "SYNC not divided (default)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncdiv::B0x0
    }
    #[doc = "SYNC divided by 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncdiv::B0x1
    }
    #[doc = "SYNC divided by 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Syncdiv::B0x2
    }
    #[doc = "SYNC divided by 8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Syncdiv::B0x3
    }
    #[doc = "SYNC divided by 16"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Syncdiv::B0x4
    }
    #[doc = "SYNC divided by 32"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Syncdiv::B0x5
    }
    #[doc = "SYNC divided by 64"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Syncdiv::B0x6
    }
    #[doc = "SYNC divided by 128"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Syncdiv::B0x7
    }
}
#[doc = "Field `SYNCDIV` writer - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SyncdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Syncdiv, crate::Safe>;
impl<'a, REG> SyncdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYNC not divided (default)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syncdiv::B0x0)
    }
    #[doc = "SYNC divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syncdiv::B0x1)
    }
    #[doc = "SYNC divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Syncdiv::B0x2)
    }
    #[doc = "SYNC divided by 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Syncdiv::B0x3)
    }
    #[doc = "SYNC divided by 16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Syncdiv::B0x4)
    }
    #[doc = "SYNC divided by 32"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Syncdiv::B0x5)
    }
    #[doc = "SYNC divided by 64"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Syncdiv::B0x6)
    }
    #[doc = "SYNC divided by 128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Syncdiv::B0x7)
    }
}
#[doc = "SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table122): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syncsrc {
    #[doc = "0: crs_sync_in_1 selected as SYNC signal source"]
    B0x0 = 0,
    #[doc = "1: crs_sync_in_2 selected as SYNC signal source"]
    B0x1 = 1,
    #[doc = "2: crs_sync_in_3 selected as SYNC signal source"]
    B0x2 = 2,
    #[doc = "3: crs_sync_in_4 selected as SYNC signal source"]
    B0x3 = 3,
}
impl From<Syncsrc> for u8 {
    #[inline(always)]
    fn from(variant: Syncsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syncsrc {
    type Ux = u8;
}
impl crate::IsEnum for Syncsrc {}
#[doc = "Field `SYNCSRC` reader - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table122): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
pub type SyncsrcR = crate::FieldReader<Syncsrc>;
impl SyncsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncsrc {
        match self.bits {
            0 => Syncsrc::B0x0,
            1 => Syncsrc::B0x1,
            2 => Syncsrc::B0x2,
            3 => Syncsrc::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "crs_sync_in_1 selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncsrc::B0x0
    }
    #[doc = "crs_sync_in_2 selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncsrc::B0x1
    }
    #[doc = "crs_sync_in_3 selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Syncsrc::B0x2
    }
    #[doc = "crs_sync_in_4 selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Syncsrc::B0x3
    }
}
#[doc = "Field `SYNCSRC` writer - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table122): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
pub type SyncsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Syncsrc, crate::Safe>;
impl<'a, REG> SyncsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "crs_sync_in_1 selected as SYNC signal source"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsrc::B0x0)
    }
    #[doc = "crs_sync_in_2 selected as SYNC signal source"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsrc::B0x1)
    }
    #[doc = "crs_sync_in_3 selected as SYNC signal source"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsrc::B0x2)
    }
    #[doc = "crs_sync_in_4 selected as SYNC signal source"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsrc::B0x3)
    }
}
#[doc = "SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncpol {
    #[doc = "0: SYNC active on rising edge (default)"]
    B0x0 = 0,
    #[doc = "1: SYNC active on falling edge"]
    B0x1 = 1,
}
impl From<Syncpol> for bool {
    #[inline(always)]
    fn from(variant: Syncpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCPOL` reader - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SyncpolR = crate::BitReader<Syncpol>;
impl SyncpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncpol {
        match self.bits {
            false => Syncpol::B0x0,
            true => Syncpol::B0x1,
        }
    }
    #[doc = "SYNC active on rising edge (default)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncpol::B0x0
    }
    #[doc = "SYNC active on falling edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncpol::B0x1
    }
}
#[doc = "Field `SYNCPOL` writer - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SyncpolW<'a, REG> = crate::BitWriter<'a, REG, Syncpol>;
impl<'a, REG> SyncpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYNC active on rising edge (default)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syncpol::B0x0)
    }
    #[doc = "SYNC active on falling edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syncpol::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section15.4.3 for more details about counter behavior."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section15.4.4 for more details about FECAP evaluation."]
    #[inline(always)]
    pub fn felim(&self) -> FelimR {
        FelimR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    pub fn syncdiv(&self) -> SyncdivR {
        SyncdivR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table122): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
    #[inline(always)]
    pub fn syncsrc(&self) -> SyncsrcR {
        SyncsrcR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    pub fn syncpol(&self) -> SyncpolR {
        SyncpolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section15.4.3 for more details about counter behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<CrsCfgrSpec> {
        ReloadW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section15.4.4 for more details about FECAP evaluation."]
    #[inline(always)]
    #[must_use]
    pub fn felim(&mut self) -> FelimW<CrsCfgrSpec> {
        FelimW::new(self, 16)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    #[must_use]
    pub fn syncdiv(&mut self) -> SyncdivW<CrsCfgrSpec> {
        SyncdivW::new(self, 24)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table122): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
    #[inline(always)]
    #[must_use]
    pub fn syncsrc(&mut self) -> SyncsrcW<CrsCfgrSpec> {
        SyncsrcW::new(self, 28)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SyncpolW<CrsCfgrSpec> {
        SyncpolW::new(self, 31)
    }
}
#[doc = "CRS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs_cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crs_cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsCfgrSpec;
impl crate::RegisterSpec for CrsCfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crs_cfgr::R`](R) reader structure"]
impl crate::Readable for CrsCfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`crs_cfgr::W`](W) writer structure"]
impl crate::Writable for CrsCfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRS_CFGR to value 0x2022_bb7f"]
impl crate::Resettable for CrsCfgrSpec {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}
