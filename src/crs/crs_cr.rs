#[doc = "Register `CRS_CR` reader"]
pub type R = crate::R<CrsCrSpec>;
#[doc = "Register `CRS_CR` writer"]
pub type W = crate::W<CrsCrSpec>;
#[doc = "SYNC event OK interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncokie {
    #[doc = "0: SYNC event OK (SYNCOKF) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: SYNC event OK (SYNCOKF) interrupt enabled"]
    B0x1 = 1,
}
impl From<Syncokie> for bool {
    #[inline(always)]
    fn from(variant: Syncokie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCOKIE` reader - SYNC event OK interrupt enable"]
pub type SyncokieR = crate::BitReader<Syncokie>;
impl SyncokieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncokie {
        match self.bits {
            false => Syncokie::B0x0,
            true => Syncokie::B0x1,
        }
    }
    #[doc = "SYNC event OK (SYNCOKF) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncokie::B0x0
    }
    #[doc = "SYNC event OK (SYNCOKF) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncokie::B0x1
    }
}
#[doc = "Field `SYNCOKIE` writer - SYNC event OK interrupt enable"]
pub type SyncokieW<'a, REG> = crate::BitWriter<'a, REG, Syncokie>;
impl<'a, REG> SyncokieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYNC event OK (SYNCOKF) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syncokie::B0x0)
    }
    #[doc = "SYNC event OK (SYNCOKF) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syncokie::B0x1)
    }
}
#[doc = "SYNC warning interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncwarnie {
    #[doc = "0: SYNC warning (SYNCWARNF) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: SYNC warning (SYNCWARNF) interrupt enabled"]
    B0x1 = 1,
}
impl From<Syncwarnie> for bool {
    #[inline(always)]
    fn from(variant: Syncwarnie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCWARNIE` reader - SYNC warning interrupt enable"]
pub type SyncwarnieR = crate::BitReader<Syncwarnie>;
impl SyncwarnieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncwarnie {
        match self.bits {
            false => Syncwarnie::B0x0,
            true => Syncwarnie::B0x1,
        }
    }
    #[doc = "SYNC warning (SYNCWARNF) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncwarnie::B0x0
    }
    #[doc = "SYNC warning (SYNCWARNF) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncwarnie::B0x1
    }
}
#[doc = "Field `SYNCWARNIE` writer - SYNC warning interrupt enable"]
pub type SyncwarnieW<'a, REG> = crate::BitWriter<'a, REG, Syncwarnie>;
impl<'a, REG> SyncwarnieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYNC warning (SYNCWARNF) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syncwarnie::B0x0)
    }
    #[doc = "SYNC warning (SYNCWARNF) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syncwarnie::B0x1)
    }
}
#[doc = "Synchronization or trimming error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Synchronization or trimming error (ERRF) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Synchronization or trimming error (ERRF) interrupt enabled"]
    B0x1 = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Synchronization or trimming error interrupt enable"]
pub type ErrieR = crate::BitReader<Errie>;
impl ErrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errie {
        match self.bits {
            false => Errie::B0x0,
            true => Errie::B0x1,
        }
    }
    #[doc = "Synchronization or trimming error (ERRF) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Errie::B0x0
    }
    #[doc = "Synchronization or trimming error (ERRF) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Errie::B0x1
    }
}
#[doc = "Field `ERRIE` writer - Synchronization or trimming error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization or trimming error (ERRF) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x0)
    }
    #[doc = "Synchronization or trimming error (ERRF) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::B0x1)
    }
}
#[doc = "Expected SYNC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esyncie {
    #[doc = "0: Expected SYNC (ESYNCF) interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Expected SYNC (ESYNCF) interrupt enabled"]
    B0x1 = 1,
}
impl From<Esyncie> for bool {
    #[inline(always)]
    fn from(variant: Esyncie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESYNCIE` reader - Expected SYNC interrupt enable"]
pub type EsyncieR = crate::BitReader<Esyncie>;
impl EsyncieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esyncie {
        match self.bits {
            false => Esyncie::B0x0,
            true => Esyncie::B0x1,
        }
    }
    #[doc = "Expected SYNC (ESYNCF) interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Esyncie::B0x0
    }
    #[doc = "Expected SYNC (ESYNCF) interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Esyncie::B0x1
    }
}
#[doc = "Field `ESYNCIE` writer - Expected SYNC interrupt enable"]
pub type EsyncieW<'a, REG> = crate::BitWriter<'a, REG, Esyncie>;
impl<'a, REG> EsyncieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Expected SYNC (ESYNCF) interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Esyncie::B0x0)
    }
    #[doc = "Expected SYNC (ESYNCF) interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Esyncie::B0x1)
    }
}
#[doc = "Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cen {
    #[doc = "0: Frequency error counter disabled"]
    B0x0 = 0,
    #[doc = "1: Frequency error counter enabled"]
    B0x1 = 1,
}
impl From<Cen> for bool {
    #[inline(always)]
    fn from(variant: Cen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CenR = crate::BitReader<Cen>;
impl CenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cen {
        match self.bits {
            false => Cen::B0x0,
            true => Cen::B0x1,
        }
    }
    #[doc = "Frequency error counter disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cen::B0x0
    }
    #[doc = "Frequency error counter enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cen::B0x1
    }
}
#[doc = "Field `CEN` writer - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG, Cen>;
impl<'a, REG> CenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frequency error counter disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::B0x0)
    }
    #[doc = "Frequency error counter enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::B0x1)
    }
}
#[doc = "Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section15.4.4 for more details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autotrimen {
    #[doc = "0: Automatic trimming disabled, TRIM bits can be adjusted by the user."]
    B0x0 = 0,
    #[doc = "1: Automatic trimming enabled, TRIM bits are read-only and under hardware control."]
    B0x1 = 1,
}
impl From<Autotrimen> for bool {
    #[inline(always)]
    fn from(variant: Autotrimen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOTRIMEN` reader - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section15.4.4 for more details."]
pub type AutotrimenR = crate::BitReader<Autotrimen>;
impl AutotrimenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autotrimen {
        match self.bits {
            false => Autotrimen::B0x0,
            true => Autotrimen::B0x1,
        }
    }
    #[doc = "Automatic trimming disabled, TRIM bits can be adjusted by the user."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Autotrimen::B0x0
    }
    #[doc = "Automatic trimming enabled, TRIM bits are read-only and under hardware control."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Autotrimen::B0x1
    }
}
#[doc = "Field `AUTOTRIMEN` writer - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section15.4.4 for more details."]
pub type AutotrimenW<'a, REG> = crate::BitWriter<'a, REG, Autotrimen>;
impl<'a, REG> AutotrimenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic trimming disabled, TRIM bits can be adjusted by the user."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Autotrimen::B0x0)
    }
    #[doc = "Automatic trimming enabled, TRIM bits are read-only and under hardware control."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Autotrimen::B0x1)
    }
}
#[doc = "Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swsync {
    #[doc = "0: No action"]
    B0x0 = 0,
    #[doc = "1: A software SYNC event is generated."]
    B0x1 = 1,
}
impl From<Swsync> for bool {
    #[inline(always)]
    fn from(variant: Swsync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSYNC` reader - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SwsyncR = crate::BitReader<Swsync>;
impl SwsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swsync {
        match self.bits {
            false => Swsync::B0x0,
            true => Swsync::B0x1,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swsync::B0x0
    }
    #[doc = "A software SYNC event is generated."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swsync::B0x1
    }
}
#[doc = "Field `SWSYNC` writer - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SwsyncW<'a, REG> = crate::BitWriter<'a, REG, Swsync>;
impl<'a, REG> SwsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swsync::B0x0)
    }
    #[doc = "A software SYNC event is generated."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swsync::B0x1)
    }
}
#[doc = "Field `TRIM` reader - HSI48 oscillator smooth trimming The default value of the HSI48 oscillator smooth trimming is 64, which corresponds to the middle of the trimming interval."]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - HSI48 oscillator smooth trimming The default value of the HSI48 oscillator smooth trimming is 64, which corresponds to the middle of the trimming interval."]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    pub fn syncokie(&self) -> SyncokieR {
        SyncokieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    pub fn syncwarnie(&self) -> SyncwarnieR {
        SyncwarnieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    pub fn esyncie(&self) -> EsyncieR {
        EsyncieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section15.4.4 for more details."]
    #[inline(always)]
    pub fn autotrimen(&self) -> AutotrimenR {
        AutotrimenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn swsync(&self) -> SwsyncR {
        SwsyncR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - HSI48 oscillator smooth trimming The default value of the HSI48 oscillator smooth trimming is 64, which corresponds to the middle of the trimming interval."]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncokie(&mut self) -> SyncokieW<CrsCrSpec> {
        SyncokieW::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncwarnie(&mut self) -> SyncwarnieW<CrsCrSpec> {
        SyncwarnieW::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<CrsCrSpec> {
        ErrieW::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn esyncie(&mut self) -> EsyncieW<CrsCrSpec> {
        EsyncieW::new(self, 3)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CenW<CrsCrSpec> {
        CenW::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section15.4.4 for more details."]
    #[inline(always)]
    #[must_use]
    pub fn autotrimen(&mut self) -> AutotrimenW<CrsCrSpec> {
        AutotrimenW::new(self, 6)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SwsyncW<CrsCrSpec> {
        SwsyncW::new(self, 7)
    }
    #[doc = "Bits 8:14 - HSI48 oscillator smooth trimming The default value of the HSI48 oscillator smooth trimming is 64, which corresponds to the middle of the trimming interval."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<CrsCrSpec> {
        TrimW::new(self, 8)
    }
}
#[doc = "CRS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crs_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsCrSpec;
impl crate::RegisterSpec for CrsCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crs_cr::R`](R) reader structure"]
impl crate::Readable for CrsCrSpec {}
#[doc = "`write(|w| ..)` method takes [`crs_cr::W`](W) writer structure"]
impl crate::Writable for CrsCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRS_CR to value 0x4000"]
impl crate::Resettable for CrsCrSpec {
    const RESET_VALUE: u32 = 0x4000;
}
