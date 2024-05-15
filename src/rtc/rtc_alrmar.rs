#[doc = "Register `RTC_ALRMAR` reader"]
pub type R = crate::R<RtcAlrmarSpec>;
#[doc = "Register `RTC_ALRMAR` writer"]
pub type W = crate::W<RtcAlrmarSpec>;
#[doc = "Field `SU` reader - Second units in BCD format."]
pub type SuR = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format."]
pub type SuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens in BCD format."]
pub type StR = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format."]
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm A seconds mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk1 {
    #[doc = "0: Alarm A set if the seconds match"]
    B0x0 = 0,
    #[doc = "1: Seconds dont care in alarm A comparison"]
    B0x1 = 1,
}
impl From<Msk1> for bool {
    #[inline(always)]
    fn from(variant: Msk1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK1` reader - Alarm A seconds mask"]
pub type Msk1R = crate::BitReader<Msk1>;
impl Msk1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk1 {
        match self.bits {
            false => Msk1::B0x0,
            true => Msk1::B0x1,
        }
    }
    #[doc = "Alarm A set if the seconds match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msk1::B0x0
    }
    #[doc = "Seconds dont care in alarm A comparison"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msk1::B0x1
    }
}
#[doc = "Field `MSK1` writer - Alarm A seconds mask"]
pub type Msk1W<'a, REG> = crate::BitWriter<'a, REG, Msk1>;
impl<'a, REG> Msk1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A set if the seconds match"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msk1::B0x0)
    }
    #[doc = "Seconds dont care in alarm A comparison"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msk1::B0x1)
    }
}
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MnuR = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MnuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MntR = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm A minutes mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk2 {
    #[doc = "0: Alarm A set if the minutes match"]
    B0x0 = 0,
    #[doc = "1: Minutes dont care in alarm A comparison"]
    B0x1 = 1,
}
impl From<Msk2> for bool {
    #[inline(always)]
    fn from(variant: Msk2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK2` reader - Alarm A minutes mask"]
pub type Msk2R = crate::BitReader<Msk2>;
impl Msk2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk2 {
        match self.bits {
            false => Msk2::B0x0,
            true => Msk2::B0x1,
        }
    }
    #[doc = "Alarm A set if the minutes match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msk2::B0x0
    }
    #[doc = "Minutes dont care in alarm A comparison"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msk2::B0x1
    }
}
#[doc = "Field `MSK2` writer - Alarm A minutes mask"]
pub type Msk2W<'a, REG> = crate::BitWriter<'a, REG, Msk2>;
impl<'a, REG> Msk2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A set if the minutes match"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msk2::B0x0)
    }
    #[doc = "Minutes dont care in alarm A comparison"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msk2::B0x1)
    }
}
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub type HuR = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units in BCD format"]
pub type HuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub type HtR = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens in BCD format"]
pub type HtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "AM/PM notation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    #[doc = "0: AM or 24-hour format"]
    B0x0 = 0,
    #[doc = "1: PM"]
    B0x1 = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::B0x0,
            true => Pm::B0x1,
        }
    }
    #[doc = "AM or 24-hour format"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pm::B0x0
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pm::B0x1
    }
}
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AM or 24-hour format"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::B0x0)
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::B0x1)
    }
}
#[doc = "Alarm A hours mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk3 {
    #[doc = "0: Alarm A set if the hours match"]
    B0x0 = 0,
    #[doc = "1: Hours dont care in alarm A comparison"]
    B0x1 = 1,
}
impl From<Msk3> for bool {
    #[inline(always)]
    fn from(variant: Msk3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK3` reader - Alarm A hours mask"]
pub type Msk3R = crate::BitReader<Msk3>;
impl Msk3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk3 {
        match self.bits {
            false => Msk3::B0x0,
            true => Msk3::B0x1,
        }
    }
    #[doc = "Alarm A set if the hours match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msk3::B0x0
    }
    #[doc = "Hours dont care in alarm A comparison"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msk3::B0x1
    }
}
#[doc = "Field `MSK3` writer - Alarm A hours mask"]
pub type Msk3W<'a, REG> = crate::BitWriter<'a, REG, Msk3>;
impl<'a, REG> Msk3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A set if the hours match"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msk3::B0x0)
    }
    #[doc = "Hours dont care in alarm A comparison"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msk3::B0x1)
    }
}
#[doc = "Field `DU` reader - Date units or day in BCD format"]
pub type DuR = crate::FieldReader;
#[doc = "Field `DU` writer - Date units or day in BCD format"]
pub type DuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Week day selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdsel {
    #[doc = "0: DU\\[3:0\\]
represents the date units"]
    B0x0 = 0,
    #[doc = "1: DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is dont care."]
    B0x1 = 1,
}
impl From<Wdsel> for bool {
    #[inline(always)]
    fn from(variant: Wdsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDSEL` reader - Week day selection"]
pub type WdselR = crate::BitReader<Wdsel>;
impl WdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdsel {
        match self.bits {
            false => Wdsel::B0x0,
            true => Wdsel::B0x1,
        }
    }
    #[doc = "DU\\[3:0\\]
represents the date units"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wdsel::B0x0
    }
    #[doc = "DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is dont care."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wdsel::B0x1
    }
}
#[doc = "Field `WDSEL` writer - Week day selection"]
pub type WdselW<'a, REG> = crate::BitWriter<'a, REG, Wdsel>;
impl<'a, REG> WdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DU\\[3:0\\]
represents the date units"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdsel::B0x0)
    }
    #[doc = "DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is dont care."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdsel::B0x1)
    }
}
#[doc = "Alarm A date mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk4 {
    #[doc = "0: Alarm A set if the date/day match"]
    B0x0 = 0,
    #[doc = "1: Date/day dont care in alarm A comparison"]
    B0x1 = 1,
}
impl From<Msk4> for bool {
    #[inline(always)]
    fn from(variant: Msk4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK4` reader - Alarm A date mask"]
pub type Msk4R = crate::BitReader<Msk4>;
impl Msk4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk4 {
        match self.bits {
            false => Msk4::B0x0,
            true => Msk4::B0x1,
        }
    }
    #[doc = "Alarm A set if the date/day match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msk4::B0x0
    }
    #[doc = "Date/day dont care in alarm A comparison"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msk4::B0x1
    }
}
#[doc = "Field `MSK4` writer - Alarm A date mask"]
pub type Msk4W<'a, REG> = crate::BitWriter<'a, REG, Msk4>;
impl<'a, REG> Msk4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A set if the date/day match"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msk4::B0x0)
    }
    #[doc = "Date/day dont care in alarm A comparison"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msk4::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn su(&self) -> SuR {
        SuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm A seconds mask"]
    #[inline(always)]
    pub fn msk1(&self) -> Msk1R {
        Msk1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MnuR {
        MnuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MntR {
        MntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm A minutes mask"]
    #[inline(always)]
    pub fn msk2(&self) -> Msk2R {
        Msk2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        HuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm A hours mask"]
    #[inline(always)]
    pub fn msk3(&self) -> Msk3R {
        Msk3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&self) -> WdselR {
        WdselR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm A date mask"]
    #[inline(always)]
    pub fn msk4(&self) -> Msk4R {
        Msk4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SuW<RtcAlrmarSpec> {
        SuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> StW<RtcAlrmarSpec> {
        StW::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm A seconds mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk1(&mut self) -> Msk1W<RtcAlrmarSpec> {
        Msk1W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MnuW<RtcAlrmarSpec> {
        MnuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MntW<RtcAlrmarSpec> {
        MntW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm A minutes mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk2(&mut self) -> Msk2W<RtcAlrmarSpec> {
        Msk2W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HuW<RtcAlrmarSpec> {
        HuW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HtW<RtcAlrmarSpec> {
        HtW::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PmW<RtcAlrmarSpec> {
        PmW::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm A hours mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk3(&mut self) -> Msk3W<RtcAlrmarSpec> {
        Msk3W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DuW<RtcAlrmarSpec> {
        DuW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<RtcAlrmarSpec> {
        DtW::new(self, 28)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    #[must_use]
    pub fn wdsel(&mut self) -> WdselW<RtcAlrmarSpec> {
        WdselW::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm A date mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk4(&mut self) -> Msk4W<RtcAlrmarSpec> {
        Msk4W::new(self, 31)
    }
}
#[doc = "RTC alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlrmarSpec;
impl crate::RegisterSpec for RtcAlrmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_alrmar::R`](R) reader structure"]
impl crate::Readable for RtcAlrmarSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrmar::W`](W) writer structure"]
impl crate::Writable for RtcAlrmarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_ALRMAR to value 0"]
impl crate::Resettable for RtcAlrmarSpec {
    const RESET_VALUE: u32 = 0;
}
