#[doc = "Register `RNG_CR` reader"]
pub type R = crate::R<RngCrSpec>;
#[doc = "Register `RNG_CR` writer"]
pub type W = crate::W<RngCrSpec>;
#[doc = "True random number generator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rngen {
    #[doc = "0: True random number generator is disabled. Analog noise sources are powered off and logic clocked by the RNG clock is gated."]
    B0x0 = 0,
    #[doc = "1: True random number generator is enabled."]
    B0x1 = 1,
}
impl From<Rngen> for bool {
    #[inline(always)]
    fn from(variant: Rngen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - True random number generator enable"]
pub type RngenR = crate::BitReader<Rngen>;
impl RngenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rngen {
        match self.bits {
            false => Rngen::B0x0,
            true => Rngen::B0x1,
        }
    }
    #[doc = "True random number generator is disabled. Analog noise sources are powered off and logic clocked by the RNG clock is gated."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rngen::B0x0
    }
    #[doc = "True random number generator is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rngen::B0x1
    }
}
#[doc = "Field `RNGEN` writer - True random number generator enable"]
pub type RngenW<'a, REG> = crate::BitWriter<'a, REG, Rngen>;
impl<'a, REG> RngenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "True random number generator is disabled. Analog noise sources are powered off and logic clocked by the RNG clock is gated."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rngen::B0x0)
    }
    #[doc = "True random number generator is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rngen::B0x1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ie {
    #[doc = "0: RNG interrupt is disabled"]
    B0x0 = 0,
    #[doc = "1: RNG interrupt is enabled. An interrupt is pending as soon as DRDY1=11, SEIS1=11 or CEIS1=11 in the RNG_SR register."]
    B0x1 = 1,
}
impl From<Ie> for bool {
    #[inline(always)]
    fn from(variant: Ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt enable"]
pub type IeR = crate::BitReader<Ie>;
impl IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ie {
        match self.bits {
            false => Ie::B0x0,
            true => Ie::B0x1,
        }
    }
    #[doc = "RNG interrupt is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ie::B0x0
    }
    #[doc = "RNG interrupt is enabled. An interrupt is pending as soon as DRDY1=11, SEIS1=11 or CEIS1=11 in the RNG_SR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ie::B0x1
    }
}
#[doc = "Field `IE` writer - Interrupt enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG, Ie>;
impl<'a, REG> IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RNG interrupt is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::B0x0)
    }
    #[doc = "RNG interrupt is enabled. An interrupt is pending as soon as DRDY1=11, SEIS1=11 or CEIS1=11 in the RNG_SR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::B0x1)
    }
}
#[doc = "Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ced {
    #[doc = "0: Clock error detection enabled"]
    B0x0 = 0,
    #[doc = "1: Clock error detection is disabled"]
    B0x1 = 1,
}
impl From<Ced> for bool {
    #[inline(always)]
    fn from(variant: Ced) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CED` reader - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type CedR = crate::BitReader<Ced>;
impl CedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ced {
        match self.bits {
            false => Ced::B0x0,
            true => Ced::B0x1,
        }
    }
    #[doc = "Clock error detection enabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ced::B0x0
    }
    #[doc = "Clock error detection is disabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ced::B0x1
    }
}
#[doc = "Field `CED` writer - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type CedW<'a, REG> = crate::BitWriter<'a, REG, Ced>;
impl<'a, REG> CedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock error detection enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ced::B0x0)
    }
    #[doc = "Clock error detection is disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ced::B0x1)
    }
}
#[doc = "Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ardis {
    #[doc = "0: When a noise source error occurs RNG performs an automatic reset to clear the SECS bit."]
    B0x0 = 0,
    #[doc = "1: When a noise source error occurs the application must reset RNG by writing CONDRST to 1 then to 0, in order to restart random number generation."]
    B0x1 = 1,
}
impl From<Ardis> for bool {
    #[inline(always)]
    fn from(variant: Ardis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARDIS` reader - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type ArdisR = crate::BitReader<Ardis>;
impl ArdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ardis {
        match self.bits {
            false => Ardis::B0x0,
            true => Ardis::B0x1,
        }
    }
    #[doc = "When a noise source error occurs RNG performs an automatic reset to clear the SECS bit."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ardis::B0x0
    }
    #[doc = "When a noise source error occurs the application must reset RNG by writing CONDRST to 1 then to 0, in order to restart random number generation."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ardis::B0x1
    }
}
#[doc = "Field `ARDIS` writer - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type ArdisW<'a, REG> = crate::BitWriter<'a, REG, Ardis>;
impl<'a, REG> ArdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When a noise source error occurs RNG performs an automatic reset to clear the SECS bit."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ardis::B0x0)
    }
    #[doc = "When a noise source error occurs the application must reset RNG by writing CONDRST to 1 then to 0, in order to restart random number generation."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ardis::B0x1)
    }
}
#[doc = "Field `RNG_CONFIG3` reader - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
pub type RngConfig3R = crate::FieldReader;
#[doc = "Field `RNG_CONFIG3` writer - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
pub type RngConfig3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nistc {
    #[doc = "0: Hardware default values for NIST compliant RNG. In this configuration per 128-bit output"]
    B0x0 = 0,
    #[doc = "1: Custom values for NIST compliant RNG. See Section120.6: RNG entropy source validation for proposed configuration."]
    B0x1 = 1,
}
impl From<Nistc> for bool {
    #[inline(always)]
    fn from(variant: Nistc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NISTC` reader - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type NistcR = crate::BitReader<Nistc>;
impl NistcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nistc {
        match self.bits {
            false => Nistc::B0x0,
            true => Nistc::B0x1,
        }
    }
    #[doc = "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Nistc::B0x0
    }
    #[doc = "Custom values for NIST compliant RNG. See Section120.6: RNG entropy source validation for proposed configuration."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Nistc::B0x1
    }
}
#[doc = "Field `NISTC` writer - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type NistcW<'a, REG> = crate::BitWriter<'a, REG, Nistc>;
impl<'a, REG> NistcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nistc::B0x0)
    }
    #[doc = "Custom values for NIST compliant RNG. See Section120.6: RNG entropy source validation for proposed configuration."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nistc::B0x1)
    }
}
#[doc = "Field `RNG_CONFIG2` reader - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details."]
pub type RngConfig2R = crate::FieldReader;
#[doc = "Field `RNG_CONFIG2` writer - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details."]
pub type RngConfig2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkdiv {
    #[doc = "0: internal RNG clock after divider is similar to incoming RNG clock."]
    B0x0 = 0,
    #[doc = "1: two RNG clock cycles per internal RNG clock."]
    B0x1 = 1,
    #[doc = "2: 2&lt;sup>2&lt;/sup> (= 4) RNG clock cycles per internal RNG clock."]
    B0x2 = 2,
    #[doc = "15: 2&lt;sup>15&lt;/sup> RNG clock cycles per internal clock (for example. an incoming 481MHz RNG clock becomes a 1.51kHz internal RNG clock)"]
    B0xF = 15,
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Clkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Clkdiv {}
#[doc = "Field `CLKDIV` reader - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type ClkdivR = crate::FieldReader<Clkdiv>;
impl ClkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkdiv> {
        match self.bits {
            0 => Some(Clkdiv::B0x0),
            1 => Some(Clkdiv::B0x1),
            2 => Some(Clkdiv::B0x2),
            15 => Some(Clkdiv::B0xF),
            _ => None,
        }
    }
    #[doc = "internal RNG clock after divider is similar to incoming RNG clock."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Clkdiv::B0x0
    }
    #[doc = "two RNG clock cycles per internal RNG clock."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Clkdiv::B0x1
    }
    #[doc = "2&lt;sup>2&lt;/sup> (= 4) RNG clock cycles per internal RNG clock."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Clkdiv::B0x2
    }
    #[doc = "2&lt;sup>15&lt;/sup> RNG clock cycles per internal clock (for example. an incoming 481MHz RNG clock becomes a 1.51kHz internal RNG clock)"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Clkdiv::B0xF
    }
}
#[doc = "Field `CLKDIV` writer - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Clkdiv>;
impl<'a, REG> ClkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "internal RNG clock after divider is similar to incoming RNG clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::B0x0)
    }
    #[doc = "two RNG clock cycles per internal RNG clock."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::B0x1)
    }
    #[doc = "2&lt;sup>2&lt;/sup> (= 4) RNG clock cycles per internal RNG clock."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::B0x2)
    }
    #[doc = "2&lt;sup>15&lt;/sup> RNG clock cycles per internal clock (for example. an incoming 481MHz RNG clock becomes a 1.51kHz internal RNG clock)"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::B0xF)
    }
}
#[doc = "Field `RNG_CONFIG1` reader - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type RngConfig1R = crate::FieldReader;
#[doc = "Field `RNG_CONFIG1` writer - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
pub type RngConfig1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CONDRST` reader - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\]
must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
pub type CondrstR = crate::BitReader;
#[doc = "Field `CONDRST` writer - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\]
must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
pub type CondrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Configlock {
    #[doc = "0: Writes to the RNG_HTCR and RNG_CR configuration bits \\[29:4\\]
are allowed."]
    B0x0 = 0,
    #[doc = "1: Writes to the RNG_HTCR and RNG_CR configuration bits \\[29:4\\]
are ignored until the next RNG reset."]
    B0x1 = 1,
}
impl From<Configlock> for bool {
    #[inline(always)]
    fn from(variant: Configlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONFIGLOCK` reader - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
pub type ConfiglockR = crate::BitReader<Configlock>;
impl ConfiglockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Configlock {
        match self.bits {
            false => Configlock::B0x0,
            true => Configlock::B0x1,
        }
    }
    #[doc = "Writes to the RNG_HTCR and RNG_CR configuration bits \\[29:4\\]
are allowed."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Configlock::B0x0
    }
    #[doc = "Writes to the RNG_HTCR and RNG_CR configuration bits \\[29:4\\]
are ignored until the next RNG reset."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Configlock::B0x1
    }
}
#[doc = "Field `CONFIGLOCK` writer - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
pub type ConfiglockW<'a, REG> = crate::BitWriter<'a, REG, Configlock>;
impl<'a, REG> ConfiglockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes to the RNG_HTCR and RNG_CR configuration bits \\[29:4\\]
are allowed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Configlock::B0x0)
    }
    #[doc = "Writes to the RNG_HTCR and RNG_CR configuration bits \\[29:4\\]
are ignored until the next RNG reset."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Configlock::B0x1)
    }
}
impl R {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RngenR {
        RngenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    pub fn ced(&self) -> CedR {
        CedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    pub fn ardis(&self) -> ArdisR {
        ArdisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
    #[inline(always)]
    pub fn rng_config3(&self) -> RngConfig3R {
        RngConfig3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    pub fn nistc(&self) -> NistcR {
        NistcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details."]
    #[inline(always)]
    pub fn rng_config2(&self) -> RngConfig2R {
        RngConfig2R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    pub fn rng_config1(&self) -> RngConfig1R {
        RngConfig1R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\]
must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
    #[inline(always)]
    pub fn condrst(&self) -> CondrstR {
        CondrstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
    #[inline(always)]
    pub fn configlock(&self) -> ConfiglockR {
        ConfiglockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RngenW<RngCrSpec> {
        RngenW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<RngCrSpec> {
        IeW::new(self, 3)
    }
    #[doc = "Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, that is to enable or disable CED, the RNG must be disabled. Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CedW<RngCrSpec> {
        CedW::new(self, 5)
    }
    #[doc = "Bit 7 - Auto reset disable When auto-reset is enabled the application still need to clear the SEIS bit after a noise source error. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    #[must_use]
    pub fn ardis(&mut self) -> ArdisW<RngCrSpec> {
        ArdisW::new(self, 7)
    }
    #[doc = "Bits 8:11 - RNG configuration 3 Reserved to the RNG configuration (bitfield 3). Refer to RNG_CONFIG1 bitfield for details. If the NISTC bit is cleared in this register RNG_CONFIG3 bitfield values are ignored by RNG."]
    #[inline(always)]
    #[must_use]
    pub fn rng_config3(&mut self) -> RngConfig3W<RngCrSpec> {
        RngConfig3W::new(self, 8)
    }
    #[doc = "Bit 12 - NIST custom two conditioning loops are performed and 256 bits of noise source are used. Writing this bit is taken into account only if CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    #[must_use]
    pub fn nistc(&mut self) -> NistcW<RngCrSpec> {
        NistcW::new(self, 12)
    }
    #[doc = "Bits 13:15 - RNG configuration 2 Reserved to the RNG configuration (bitfield 2). Bit 13 can be set when RNG power consumption is critical. See Section120.3.8: RNG low-power use. Refer to the RNG_CONFIG1 bitfield for details."]
    #[inline(always)]
    #[must_use]
    pub fn rng_config2(&mut self) -> RngConfig2W<RngCrSpec> {
        RngConfig2W::new(self, 13)
    }
    #[doc = "Bits 16:19 - Clock divider factor This value used to configure an internal programmable divider (from 1 to 16) acting on the incoming RNG clock. These bits can be written only when the core is disabled (RNGEN1=10). ... Writing these bits is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<RngCrSpec> {
        ClkdivW::new(self, 16)
    }
    #[doc = "Bits 20:25 - RNG configuration 1 Reserved to the RNG configuration (bitfield 1). Must be initialized using the recommended value documented in Section120.6: RNG entropy source validation. Writing any bit of RNG_CONFIG1 is taken into account only if the CONDRST bit is set to 1 in the same access, while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK1=11."]
    #[inline(always)]
    #[must_use]
    pub fn rng_config1(&mut self) -> RngConfig1W<RngCrSpec> {
        RngConfig1W::new(self, 20)
    }
    #[doc = "Bit 30 - Conditioning soft reset Write 1 and then write 0 to reset the conditioning logic, clear all the FIFOs and start a new RNG initialization process, with RNG_SR cleared. Registers RNG_CR and RNG_HTCR are not changed by CONDRST. This bit must be set to 1 in the same access that set any configuration bits \\[29:4\\]. In other words, when CONDRST bit is set to 1 correct configuration in bits \\[29:4\\]
must also be written. When CONDRST is set to 0 by the software, its value goes to 0 when the reset process is done. It takes about 2 AHB clock cycles + 2 RNG clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn condrst(&mut self) -> CondrstW<RngCrSpec> {
        CondrstW::new(self, 30)
    }
    #[doc = "Bit 31 - RNG Config lock This bitfield is set once: if this bit is set it can only be reset to 0 if RNG is reset."]
    #[inline(always)]
    #[must_use]
    pub fn configlock(&mut self) -> ConfiglockW<RngCrSpec> {
        ConfiglockW::new(self, 31)
    }
}
#[doc = "RNG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngCrSpec;
impl crate::RegisterSpec for RngCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_cr::R`](R) reader structure"]
impl crate::Readable for RngCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_cr::W`](W) writer structure"]
impl crate::Writable for RngCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_CR to value 0x0080_0d00"]
impl crate::Resettable for RngCrSpec {
    const RESET_VALUE: u32 = 0x0080_0d00;
}
