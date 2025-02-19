#[doc = "Register `RCC_BDCR` reader"]
pub type R = crate::R<RccBdcrSpec>;
#[doc = "Register `RCC_BDCR` writer"]
pub type W = crate::W<RccBdcrSpec>;
#[doc = "LSE oscillator enable Set and cleared by software to enable LSE oscillator:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lseon {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lseon> for bool {
    #[inline(always)]
    fn from(variant: Lseon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enable Set and cleared by software to enable LSE oscillator:"]
pub type LseonR = crate::BitReader<Lseon>;
impl LseonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lseon {
        match self.bits {
            false => Lseon::B0x0,
            true => Lseon::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lseon::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lseon::B0x1
    }
}
#[doc = "Field `LSEON` writer - LSE oscillator enable Set and cleared by software to enable LSE oscillator:"]
pub type LseonW<'a, REG> = crate::BitWriter<'a, REG, Lseon>;
impl<'a, REG> LseonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lseon::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lseon::B0x1)
    }
}
#[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 321kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lserdy {
    #[doc = "0: Not ready"]
    B0x0 = 0,
    #[doc = "1: Ready"]
    B0x1 = 1,
}
impl From<Lserdy> for bool {
    #[inline(always)]
    fn from(variant: Lserdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDY` reader - LSE oscillator ready Set and cleared by hardware to indicate when the external 321kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
pub type LserdyR = crate::BitReader<Lserdy>;
impl LserdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lserdy {
        match self.bits {
            false => Lserdy::B0x0,
            true => Lserdy::B0x1,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lserdy::B0x0
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lserdy::B0x1
    }
}
#[doc = "LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsebyp {
    #[doc = "0: Not bypassed"]
    B0x0 = 0,
    #[doc = "1: Bypassed"]
    B0x1 = 1,
}
impl From<Lsebyp> for bool {
    #[inline(always)]
    fn from(variant: Lsebyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
pub type LsebypR = crate::BitReader<Lsebyp>;
impl LsebypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsebyp {
        match self.bits {
            false => Lsebyp::B0x0,
            true => Lsebyp::B0x1,
        }
    }
    #[doc = "Not bypassed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsebyp::B0x0
    }
    #[doc = "Bypassed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsebyp::B0x1
    }
}
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
pub type LsebypW<'a, REG> = crate::BitWriter<'a, REG, Lsebyp>;
impl<'a, REG> LsebypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not bypassed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsebyp::B0x0)
    }
    #[doc = "Bypassed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsebyp::B0x1)
    }
}
#[doc = "LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lsedrv {
    #[doc = "0: low driving capability"]
    B0x0 = 0,
    #[doc = "1: medium-low driving capability"]
    B0x1 = 1,
    #[doc = "2: medium-high driving capability"]
    B0x2 = 2,
    #[doc = "3: high driving capability"]
    B0x3 = 3,
}
impl From<Lsedrv> for u8 {
    #[inline(always)]
    fn from(variant: Lsedrv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lsedrv {
    type Ux = u8;
}
impl crate::IsEnum for Lsedrv {}
#[doc = "Field `LSEDRV` reader - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode."]
pub type LsedrvR = crate::FieldReader<Lsedrv>;
impl LsedrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsedrv {
        match self.bits {
            0 => Lsedrv::B0x0,
            1 => Lsedrv::B0x1,
            2 => Lsedrv::B0x2,
            3 => Lsedrv::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "low driving capability"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsedrv::B0x0
    }
    #[doc = "medium-low driving capability"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsedrv::B0x1
    }
    #[doc = "medium-high driving capability"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lsedrv::B0x2
    }
    #[doc = "high driving capability"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lsedrv::B0x3
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode."]
pub type LsedrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lsedrv, crate::Safe>;
impl<'a, REG> LsedrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "low driving capability"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsedrv::B0x0)
    }
    #[doc = "medium-low driving capability"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsedrv::B0x1)
    }
    #[doc = "medium-high driving capability"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lsedrv::B0x2)
    }
    #[doc = "high driving capability"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lsedrv::B0x3)
    }
}
#[doc = "CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsecsson {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lsecsson> for bool {
    #[inline(always)]
    fn from(variant: Lsecsson) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSON` reader - CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit."]
pub type LsecssonR = crate::BitReader<Lsecsson>;
impl LsecssonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsecsson {
        match self.bits {
            false => Lsecsson::B0x0,
            true => Lsecsson::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsecsson::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsecsson::B0x1
    }
}
#[doc = "Field `LSECSSON` writer - CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit."]
pub type LsecssonW<'a, REG> = crate::BitWriter<'a, REG, Lsecsson>;
impl<'a, REG> LsecssonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsecsson::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsecsson::B0x1)
    }
}
#[doc = "CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 321kHz oscillator (LSE):\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsecssd {
    #[doc = "0: No failure detected"]
    B0x0 = 0,
    #[doc = "1: Failure detected"]
    B0x1 = 1,
}
impl From<Lsecssd> for bool {
    #[inline(always)]
    fn from(variant: Lsecssd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSD` reader - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 321kHz oscillator (LSE):"]
pub type LsecssdR = crate::BitReader<Lsecssd>;
impl LsecssdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsecssd {
        match self.bits {
            false => Lsecssd::B0x0,
            true => Lsecssd::B0x1,
        }
    }
    #[doc = "No failure detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsecssd::B0x0
    }
    #[doc = "Failure detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsecssd::B0x1
    }
}
#[doc = "LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsesysen {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enabled, LSE distributed to peripherals including LSCO/MCO/SYSCLK."]
    B0x1 = 1,
}
impl From<Lsesysen> for bool {
    #[inline(always)]
    fn from(variant: Lsesysen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSESYSEN` reader - LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage."]
pub type LsesysenR = crate::BitReader<Lsesysen>;
impl LsesysenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsesysen {
        match self.bits {
            false => Lsesysen::B0x0,
            true => Lsesysen::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsesysen::B0x0
    }
    #[doc = "Enabled, LSE distributed to peripherals including LSCO/MCO/SYSCLK."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsesysen::B0x1
    }
}
#[doc = "Field `LSESYSEN` writer - LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage."]
pub type LsesysenW<'a, REG> = crate::BitWriter<'a, REG, Lsesysen>;
impl<'a, REG> LsesysenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsesysen::B0x0)
    }
    #[doc = "Enabled, LSE distributed to peripherals including LSCO/MCO/SYSCLK."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsesysen::B0x1)
    }
}
#[doc = "RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcsel {
    #[doc = "0: No clock"]
    B0x0 = 0,
    #[doc = "1: LSE"]
    B0x1 = 1,
    #[doc = "2: LSI"]
    B0x2 = 2,
    #[doc = "3: HSE divided by 32"]
    B0x3 = 3,
}
impl From<Rtcsel> for u8 {
    #[inline(always)]
    fn from(variant: Rtcsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcsel {
    type Ux = u8;
}
impl crate::IsEnum for Rtcsel {}
#[doc = "Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00."]
pub type RtcselR = crate::FieldReader<Rtcsel>;
impl RtcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcsel {
        match self.bits {
            0 => Rtcsel::B0x0,
            1 => Rtcsel::B0x1,
            2 => Rtcsel::B0x2,
            3 => Rtcsel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtcsel::B0x0
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtcsel::B0x1
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Rtcsel::B0x2
    }
    #[doc = "HSE divided by 32"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Rtcsel::B0x3
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00."]
pub type RtcselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtcsel, crate::Safe>;
impl<'a, REG> RtcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsel::B0x0)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsel::B0x1)
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsel::B0x2)
    }
    #[doc = "HSE divided by 32"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsel::B0x3)
    }
}
#[doc = "LSE clock ready for system usage This flag is set by hardware to indicate that the LSE clock is ready for being used by the system (see LSESYSEN bit). This flag is set when LSE clock is ready (LSEON1=11 and LSERDY1=11) and two LSE clock cycles after that LSESYSEN is set. Cleared by hardware to indicate that the LSE clock is not ready to be used by the system.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsesysrdy {
    #[doc = "0: LSE clock not ready for system"]
    B0x0 = 0,
    #[doc = "1: LSE clock ready for system"]
    B0x1 = 1,
}
impl From<Lsesysrdy> for bool {
    #[inline(always)]
    fn from(variant: Lsesysrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSESYSRDY` reader - LSE clock ready for system usage This flag is set by hardware to indicate that the LSE clock is ready for being used by the system (see LSESYSEN bit). This flag is set when LSE clock is ready (LSEON1=11 and LSERDY1=11) and two LSE clock cycles after that LSESYSEN is set. Cleared by hardware to indicate that the LSE clock is not ready to be used by the system."]
pub type LsesysrdyR = crate::BitReader<Lsesysrdy>;
impl LsesysrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsesysrdy {
        match self.bits {
            false => Lsesysrdy::B0x0,
            true => Lsesysrdy::B0x1,
        }
    }
    #[doc = "LSE clock not ready for system"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsesysrdy::B0x0
    }
    #[doc = "LSE clock ready for system"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsesysrdy::B0x1
    }
}
#[doc = "RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rtcen> for bool {
    #[inline(always)]
    fn from(variant: Rtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP."]
pub type RtcenR = crate::BitReader<Rtcen>;
impl RtcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcen {
        match self.bits {
            false => Rtcen::B0x0,
            true => Rtcen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtcen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtcen::B0x1
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP."]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG, Rtcen>;
impl<'a, REG> RtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcen::B0x1)
    }
}
#[doc = "RTC domain software reset Set and cleared by software to reset the RTC domain:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bdrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset"]
    B0x1 = 1,
}
impl From<Bdrst> for bool {
    #[inline(always)]
    fn from(variant: Bdrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDRST` reader - RTC domain software reset Set and cleared by software to reset the RTC domain:"]
pub type BdrstR = crate::BitReader<Bdrst>;
impl BdrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bdrst {
        match self.bits {
            false => Bdrst::B0x0,
            true => Bdrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bdrst::B0x0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bdrst::B0x1
    }
}
#[doc = "Field `BDRST` writer - RTC domain software reset Set and cleared by software to reset the RTC domain:"]
pub type BdrstW<'a, REG> = crate::BitWriter<'a, REG, Bdrst>;
impl<'a, REG> BdrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bdrst::B0x0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bdrst::B0x1)
    }
}
#[doc = "Low-speed clock output (LSCO) enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lscoen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lscoen> for bool {
    #[inline(always)]
    fn from(variant: Lscoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOEN` reader - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LscoenR = crate::BitReader<Lscoen>;
impl LscoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lscoen {
        match self.bits {
            false => Lscoen::B0x0,
            true => Lscoen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lscoen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lscoen::B0x1
    }
}
#[doc = "Field `LSCOEN` writer - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LscoenW<'a, REG> = crate::BitWriter<'a, REG, Lscoen>;
impl<'a, REG> LscoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lscoen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lscoen::B0x1)
    }
}
#[doc = "Low-speed clock output selection Set and cleared by software to select the low-speed output clock:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lscosel {
    #[doc = "0: LSI"]
    B0x0 = 0,
    #[doc = "1: LSE"]
    B0x1 = 1,
}
impl From<Lscosel> for bool {
    #[inline(always)]
    fn from(variant: Lscosel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOSEL` reader - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:"]
pub type LscoselR = crate::BitReader<Lscosel>;
impl LscoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lscosel {
        match self.bits {
            false => Lscosel::B0x0,
            true => Lscosel::B0x1,
        }
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lscosel::B0x0
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lscosel::B0x1
    }
}
#[doc = "Field `LSCOSEL` writer - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:"]
pub type LscoselW<'a, REG> = crate::BitWriter<'a, REG, Lscosel>;
impl<'a, REG> LscoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lscosel::B0x0)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lscosel::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - LSE oscillator enable Set and cleared by software to enable LSE oscillator:"]
    #[inline(always)]
    pub fn lseon(&self) -> LseonR {
        LseonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and cleared by hardware to indicate when the external 321kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles."]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    pub fn lsebyp(&self) -> LsebypR {
        LsebypR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode."]
    #[inline(always)]
    pub fn lsedrv(&self) -> LsedrvR {
        LsedrvR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit."]
    #[inline(always)]
    pub fn lsecsson(&self) -> LsecssonR {
        LsecssonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 321kHz oscillator (LSE):"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LsecssdR {
        LsecssdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage."]
    #[inline(always)]
    pub fn lsesysen(&self) -> LsesysenR {
        LsesysenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00."]
    #[inline(always)]
    pub fn rtcsel(&self) -> RtcselR {
        RtcselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - LSE clock ready for system usage This flag is set by hardware to indicate that the LSE clock is ready for being used by the system (see LSESYSEN bit). This flag is set when LSE clock is ready (LSEON1=11 and LSERDY1=11) and two LSE clock cycles after that LSESYSEN is set. Cleared by hardware to indicate that the LSE clock is not ready to be used by the system."]
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LsesysrdyR {
        LsesysrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP."]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC domain software reset Set and cleared by software to reset the RTC domain:"]
    #[inline(always)]
    pub fn bdrst(&self) -> BdrstR {
        BdrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    pub fn lscoen(&self) -> LscoenR {
        LscoenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:"]
    #[inline(always)]
    pub fn lscosel(&self) -> LscoselR {
        LscoselR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enable Set and cleared by software to enable LSE oscillator:"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LseonW<RccBdcrSpec> {
        LseonW::new(self, 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0)."]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LsebypW<RccBdcrSpec> {
        LsebypW::new(self, 2)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode."]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LsedrvW<RccBdcrSpec> {
        LsedrvW::new(self, 3)
    }
    #[doc = "Bit 5 - CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit."]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LsecssonW<RccBdcrSpec> {
        LsecssonW::new(self, 5)
    }
    #[doc = "Bit 7 - LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage."]
    #[inline(always)]
    #[must_use]
    pub fn lsesysen(&mut self) -> LsesysenW<RccBdcrSpec> {
        LsesysenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00."]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RtcselW<RccBdcrSpec> {
        RtcselW::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP."]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RtcenW<RccBdcrSpec> {
        RtcenW::new(self, 15)
    }
    #[doc = "Bit 16 - RTC domain software reset Set and cleared by software to reset the RTC domain:"]
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BdrstW<RccBdcrSpec> {
        BdrstW::new(self, 16)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LscoenW<RccBdcrSpec> {
        LscoenW::new(self, 24)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:"]
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LscoselW<RccBdcrSpec> {
        LscoselW::new(self, 25)
    }
}
#[doc = "RTC domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_bdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_bdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccBdcrSpec;
impl crate::RegisterSpec for RccBdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_bdcr::R`](R) reader structure"]
impl crate::Readable for RccBdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_bdcr::W`](W) writer structure"]
impl crate::Writable for RccBdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_BDCR to value 0"]
impl crate::Resettable for RccBdcrSpec {
    const RESET_VALUE: u32 = 0;
}
