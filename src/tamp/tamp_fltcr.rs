#[doc = "Register `TAMP_FLTCR` reader"]
pub type R = crate::R<TampFltcrSpec>;
#[doc = "Register `TAMP_FLTCR` writer"]
pub type W = crate::W<TampFltcrSpec>;
#[doc = "Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tampfreq {
    #[doc = "0: RTCCLK / 32768 (11Hz when RTCCLK = 327681Hz)"]
    B0x0 = 0,
    #[doc = "1: RTCCLK / 16384 (21Hz when RTCCLK = 327681Hz)"]
    B0x1 = 1,
    #[doc = "2: RTCCLK / 8192 (41Hz when RTCCLK = 327681Hz)"]
    B0x2 = 2,
    #[doc = "3: RTCCLK / 4096 (81Hz when RTCCLK = 327681Hz)"]
    B0x3 = 3,
    #[doc = "4: RTCCLK / 2048 (161Hz when RTCCLK = 327681Hz)"]
    B0x4 = 4,
    #[doc = "5: RTCCLK / 1024 (321Hz when RTCCLK = 327681Hz)"]
    B0x5 = 5,
    #[doc = "6: RTCCLK / 512 (641Hz when RTCCLK = 327681Hz)"]
    B0x6 = 6,
    #[doc = "7: RTCCLK / 256 (1281Hz when RTCCLK = 327681Hz)"]
    B0x7 = 7,
}
impl From<Tampfreq> for u8 {
    #[inline(always)]
    fn from(variant: Tampfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tampfreq {
    type Ux = u8;
}
impl crate::IsEnum for Tampfreq {}
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
pub type TampfreqR = crate::FieldReader<Tampfreq>;
impl TampfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tampfreq {
        match self.bits {
            0 => Tampfreq::B0x0,
            1 => Tampfreq::B0x1,
            2 => Tampfreq::B0x2,
            3 => Tampfreq::B0x3,
            4 => Tampfreq::B0x4,
            5 => Tampfreq::B0x5,
            6 => Tampfreq::B0x6,
            7 => Tampfreq::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "RTCCLK / 32768 (11Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tampfreq::B0x0
    }
    #[doc = "RTCCLK / 16384 (21Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tampfreq::B0x1
    }
    #[doc = "RTCCLK / 8192 (41Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Tampfreq::B0x2
    }
    #[doc = "RTCCLK / 4096 (81Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Tampfreq::B0x3
    }
    #[doc = "RTCCLK / 2048 (161Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Tampfreq::B0x4
    }
    #[doc = "RTCCLK / 1024 (321Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Tampfreq::B0x5
    }
    #[doc = "RTCCLK / 512 (641Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Tampfreq::B0x6
    }
    #[doc = "RTCCLK / 256 (1281Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Tampfreq::B0x7
    }
}
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
pub type TampfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tampfreq, crate::Safe>;
impl<'a, REG> TampfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTCCLK / 32768 (11Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tampfreq::B0x0)
    }
    #[doc = "RTCCLK / 16384 (21Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tampfreq::B0x1)
    }
    #[doc = "RTCCLK / 8192 (41Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Tampfreq::B0x2)
    }
    #[doc = "RTCCLK / 4096 (81Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Tampfreq::B0x3)
    }
    #[doc = "RTCCLK / 2048 (161Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Tampfreq::B0x4)
    }
    #[doc = "RTCCLK / 1024 (321Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Tampfreq::B0x5)
    }
    #[doc = "RTCCLK / 512 (641Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Tampfreq::B0x6)
    }
    #[doc = "RTCCLK / 256 (1281Hz when RTCCLK = 327681Hz)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Tampfreq::B0x7)
    }
}
#[doc = "TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tampflt {
    #[doc = "0: Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)."]
    B0x0 = 0,
    #[doc = "1: Tamper event is activated after 2 consecutive samples at the active level."]
    B0x1 = 1,
    #[doc = "2: Tamper event is activated after 4 consecutive samples at the active level."]
    B0x2 = 2,
    #[doc = "3: Tamper event is activated after 8 consecutive samples at the active level."]
    B0x3 = 3,
}
impl From<Tampflt> for u8 {
    #[inline(always)]
    fn from(variant: Tampflt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tampflt {
    type Ux = u8;
}
impl crate::IsEnum for Tampflt {}
#[doc = "Field `TAMPFLT` reader - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
pub type TampfltR = crate::FieldReader<Tampflt>;
impl TampfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tampflt {
        match self.bits {
            0 => Tampflt::B0x0,
            1 => Tampflt::B0x1,
            2 => Tampflt::B0x2,
            3 => Tampflt::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tampflt::B0x0
    }
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tampflt::B0x1
    }
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Tampflt::B0x2
    }
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Tampflt::B0x3
    }
}
#[doc = "Field `TAMPFLT` writer - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
pub type TampfltW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tampflt, crate::Safe>;
impl<'a, REG> TampfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tampflt::B0x0)
    }
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tampflt::B0x1)
    }
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Tampflt::B0x2)
    }
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Tampflt::B0x3)
    }
}
#[doc = "TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tampprch {
    #[doc = "0: 1 RTCCLK cycle"]
    B0x0 = 0,
    #[doc = "1: 2 RTCCLK cycles"]
    B0x1 = 1,
    #[doc = "2: 4 RTCCLK cycles"]
    B0x2 = 2,
    #[doc = "3: 8 RTCCLK cycles"]
    B0x3 = 3,
}
impl From<Tampprch> for u8 {
    #[inline(always)]
    fn from(variant: Tampprch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tampprch {
    type Ux = u8;
}
impl crate::IsEnum for Tampprch {}
#[doc = "Field `TAMPPRCH` reader - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
pub type TampprchR = crate::FieldReader<Tampprch>;
impl TampprchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tampprch {
        match self.bits {
            0 => Tampprch::B0x0,
            1 => Tampprch::B0x1,
            2 => Tampprch::B0x2,
            3 => Tampprch::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "1 RTCCLK cycle"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tampprch::B0x0
    }
    #[doc = "2 RTCCLK cycles"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tampprch::B0x1
    }
    #[doc = "4 RTCCLK cycles"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Tampprch::B0x2
    }
    #[doc = "8 RTCCLK cycles"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Tampprch::B0x3
    }
}
#[doc = "Field `TAMPPRCH` writer - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
pub type TampprchW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tampprch, crate::Safe>;
impl<'a, REG> TampprchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 RTCCLK cycle"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tampprch::B0x0)
    }
    #[doc = "2 RTCCLK cycles"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tampprch::B0x1)
    }
    #[doc = "4 RTCCLK cycles"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Tampprch::B0x2)
    }
    #[doc = "8 RTCCLK cycles"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Tampprch::B0x3)
    }
}
#[doc = "TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamppudis {
    #[doc = "0: Precharge TAMP_INx pins before sampling (enable internal pull-up)"]
    B0x0 = 0,
    #[doc = "1: Disable precharge of TAMP_INx pins."]
    B0x1 = 1,
}
impl From<Tamppudis> for bool {
    #[inline(always)]
    fn from(variant: Tamppudis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPPUDIS` reader - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
pub type TamppudisR = crate::BitReader<Tamppudis>;
impl TamppudisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamppudis {
        match self.bits {
            false => Tamppudis::B0x0,
            true => Tamppudis::B0x1,
        }
    }
    #[doc = "Precharge TAMP_INx pins before sampling (enable internal pull-up)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamppudis::B0x0
    }
    #[doc = "Disable precharge of TAMP_INx pins."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamppudis::B0x1
    }
}
#[doc = "Field `TAMPPUDIS` writer - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
pub type TamppudisW<'a, REG> = crate::BitWriter<'a, REG, Tamppudis>;
impl<'a, REG> TamppudisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Precharge TAMP_INx pins before sampling (enable internal pull-up)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamppudis::B0x0)
    }
    #[doc = "Disable precharge of TAMP_INx pins."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamppudis::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
    #[inline(always)]
    pub fn tampfreq(&self) -> TampfreqR {
        TampfreqR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    pub fn tampflt(&self) -> TampfltR {
        TampfltR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    pub fn tampprch(&self) -> TampprchR {
        TampprchR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
    #[inline(always)]
    pub fn tamppudis(&self) -> TamppudisR {
        TamppudisR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TampfreqW<TampFltcrSpec> {
        TampfreqW::new(self, 0)
    }
    #[doc = "Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TampfltW<TampFltcrSpec> {
        TampfltW::new(self, 3)
    }
    #[doc = "Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TampprchW<TampFltcrSpec> {
        TampprchW::new(self, 5)
    }
    #[doc = "Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TamppudisW<TampFltcrSpec> {
        TamppudisW::new(self, 7)
    }
}
#[doc = "TAMP filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_fltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_fltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampFltcrSpec;
impl crate::RegisterSpec for TampFltcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_fltcr::R`](R) reader structure"]
impl crate::Readable for TampFltcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tamp_fltcr::W`](W) writer structure"]
impl crate::Writable for TampFltcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_FLTCR to value 0"]
impl crate::Resettable for TampFltcrSpec {
    const RESET_VALUE: u32 = 0;
}
