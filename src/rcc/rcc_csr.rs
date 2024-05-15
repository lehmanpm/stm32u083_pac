#[doc = "Register `RCC_CSR` reader"]
pub type R = crate::R<RccCsrSpec>;
#[doc = "Register `RCC_CSR` writer"]
pub type W = crate::W<RccCsrSpec>;
#[doc = "LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsion {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lsion> for bool {
    #[inline(always)]
    fn from(variant: Lsion) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSION` reader - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:"]
pub type LsionR = crate::BitReader<Lsion>;
impl LsionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsion {
        match self.bits {
            false => Lsion::B0x0,
            true => Lsion::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsion::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsion::B0x1
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:"]
pub type LsionW<'a, REG> = crate::BitWriter<'a, REG, Lsion>;
impl<'a, REG> LsionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsion::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsion::B0x1)
    }
}
#[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsirdy {
    #[doc = "0: Not ready"]
    B0x0 = 0,
    #[doc = "1: Ready"]
    B0x1 = 1,
}
impl From<Lsirdy> for bool {
    #[inline(always)]
    fn from(variant: Lsirdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC."]
pub type LsirdyR = crate::BitReader<Lsirdy>;
impl LsirdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsirdy {
        match self.bits {
            false => Lsirdy::B0x0,
            true => Lsirdy::B0x1,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsirdy::B0x0
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsirdy::B0x1
    }
}
#[doc = "Internal low-speed oscillator pre-divided by 128 Set and reset by hardware to indicate when the low-speed internal RC oscillator has to be divided by 128. The software has to switch off the LSI before changing this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsiprediv {
    #[doc = "0: LSI RC oscillator is not divided"]
    B0x0 = 0,
    #[doc = "1: LSI RC oscillator is divided by 128"]
    B0x1 = 1,
}
impl From<Lsiprediv> for bool {
    #[inline(always)]
    fn from(variant: Lsiprediv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIPREDIV` reader - Internal low-speed oscillator pre-divided by 128 Set and reset by hardware to indicate when the low-speed internal RC oscillator has to be divided by 128. The software has to switch off the LSI before changing this bit."]
pub type LsipredivR = crate::BitReader<Lsiprediv>;
impl LsipredivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsiprediv {
        match self.bits {
            false => Lsiprediv::B0x0,
            true => Lsiprediv::B0x1,
        }
    }
    #[doc = "LSI RC oscillator is not divided"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lsiprediv::B0x0
    }
    #[doc = "LSI RC oscillator is divided by 128"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lsiprediv::B0x1
    }
}
#[doc = "Field `LSIPREDIV` writer - Internal low-speed oscillator pre-divided by 128 Set and reset by hardware to indicate when the low-speed internal RC oscillator has to be divided by 128. The software has to switch off the LSI before changing this bit."]
pub type LsipredivW<'a, REG> = crate::BitWriter<'a, REG, Lsiprediv>;
impl<'a, REG> LsipredivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI RC oscillator is not divided"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsiprediv::B0x0)
    }
    #[doc = "LSI RC oscillator is divided by 128"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsiprediv::B0x1)
    }
}
#[doc = "MSI range after Standby mode Set by software to chose the MSI frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a pad or a power-on reset, the range is always 41MHz. MSISRANGE\\[3:0\\]
can be written only when MSIRGSEL1=11. Others: Reserved Note: Changing the MSISRANGE\\[3:0\\]
does not change the current MSI frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msisrange {
    #[doc = "4: Range 7 around 81MHz"]
    B0x4 = 4,
}
impl From<Msisrange> for u8 {
    #[inline(always)]
    fn from(variant: Msisrange) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msisrange {
    type Ux = u8;
}
impl crate::IsEnum for Msisrange {}
#[doc = "Field `MSISRANGE` reader - MSI range after Standby mode Set by software to chose the MSI frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a pad or a power-on reset, the range is always 41MHz. MSISRANGE\\[3:0\\]
can be written only when MSIRGSEL1=11. Others: Reserved Note: Changing the MSISRANGE\\[3:0\\]
does not change the current MSI frequency."]
pub type MsisrangeR = crate::FieldReader<Msisrange>;
impl MsisrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Msisrange> {
        match self.bits {
            4 => Some(Msisrange::B0x4),
            _ => None,
        }
    }
    #[doc = "Range 7 around 81MHz"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Msisrange::B0x4
    }
}
#[doc = "Field `MSISRANGE` writer - MSI range after Standby mode Set by software to chose the MSI frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a pad or a power-on reset, the range is always 41MHz. MSISRANGE\\[3:0\\]
can be written only when MSIRGSEL1=11. Others: Reserved Note: Changing the MSISRANGE\\[3:0\\]
does not change the current MSI frequency."]
pub type MsisrangeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Msisrange>;
impl<'a, REG> MsisrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Range 7 around 81MHz"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Msisrange::B0x4)
    }
}
#[doc = "Remove reset flags Set by software to clear the reset flags.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmvf {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clear reset flags"]
    B0x1 = 1,
}
impl From<Rmvf> for bool {
    #[inline(always)]
    fn from(variant: Rmvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMVF` reader - Remove reset flags Set by software to clear the reset flags."]
pub type RmvfR = crate::BitReader<Rmvf>;
impl RmvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rmvf {
        match self.bits {
            false => Rmvf::B0x0,
            true => Rmvf::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rmvf::B0x0
    }
    #[doc = "Clear reset flags"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rmvf::B0x1
    }
}
#[doc = "Field `RMVF` writer - Remove reset flags Set by software to clear the reset flags."]
pub type RmvfW<'a, REG> = crate::BitWriter<'a, REG, Rmvf>;
impl<'a, REG> RmvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmvf::B0x0)
    }
    #[doc = "Clear reset flags"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmvf::B0x1)
    }
}
#[doc = "Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oblrstf {
    #[doc = "0: No reset from Option byte loading occurred"]
    B0x0 = 0,
    #[doc = "1: Reset from Option byte loading occurred"]
    B0x1 = 1,
}
impl From<Oblrstf> for bool {
    #[inline(always)]
    fn from(variant: Oblrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit."]
pub type OblrstfR = crate::BitReader<Oblrstf>;
impl OblrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oblrstf {
        match self.bits {
            false => Oblrstf::B0x0,
            true => Oblrstf::B0x1,
        }
    }
    #[doc = "No reset from Option byte loading occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oblrstf::B0x0
    }
    #[doc = "Reset from Option byte loading occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oblrstf::B0x1
    }
}
#[doc = "Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pinrstf {
    #[doc = "0: No reset from NRST pin occurred"]
    B0x0 = 0,
    #[doc = "1: Reset from NRST pin occurred"]
    B0x1 = 1,
}
impl From<Pinrstf> for bool {
    #[inline(always)]
    fn from(variant: Pinrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINRSTF` reader - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit."]
pub type PinrstfR = crate::BitReader<Pinrstf>;
impl PinrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pinrstf {
        match self.bits {
            false => Pinrstf::B0x0,
            true => Pinrstf::B0x1,
        }
    }
    #[doc = "No reset from NRST pin occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pinrstf::B0x0
    }
    #[doc = "Reset from NRST pin occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pinrstf::B0x1
    }
}
#[doc = "BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrrstf {
    #[doc = "0: No BOR or POR occurred"]
    B0x0 = 0,
    #[doc = "1: BOR or POR occurred"]
    B0x1 = 1,
}
impl From<Pwrrstf> for bool {
    #[inline(always)]
    fn from(variant: Pwrrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRRSTF` reader - BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit."]
pub type PwrrstfR = crate::BitReader<Pwrrstf>;
impl PwrrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrrstf {
        match self.bits {
            false => Pwrrstf::B0x0,
            true => Pwrrstf::B0x1,
        }
    }
    #[doc = "No BOR or POR occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pwrrstf::B0x0
    }
    #[doc = "BOR or POR occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pwrrstf::B0x1
    }
}
#[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sftrstf {
    #[doc = "0: No software reset occurred"]
    B0x0 = 0,
    #[doc = "1: Software reset occurred"]
    B0x1 = 1,
}
impl From<Sftrstf> for bool {
    #[inline(always)]
    fn from(variant: Sftrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFTRSTF` reader - Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit."]
pub type SftrstfR = crate::BitReader<Sftrstf>;
impl SftrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sftrstf {
        match self.bits {
            false => Sftrstf::B0x0,
            true => Sftrstf::B0x1,
        }
    }
    #[doc = "No software reset occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sftrstf::B0x0
    }
    #[doc = "Software reset occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sftrstf::B0x1
    }
}
#[doc = "Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdgrstf {
    #[doc = "0: No independent watchdog reset occurred"]
    B0x0 = 0,
    #[doc = "1: Independent watchdog reset occurred"]
    B0x1 = 1,
}
impl From<Iwdgrstf> for bool {
    #[inline(always)]
    fn from(variant: Iwdgrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit."]
pub type IwdgrstfR = crate::BitReader<Iwdgrstf>;
impl IwdgrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdgrstf {
        match self.bits {
            false => Iwdgrstf::B0x0,
            true => Iwdgrstf::B0x1,
        }
    }
    #[doc = "No independent watchdog reset occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Iwdgrstf::B0x0
    }
    #[doc = "Independent watchdog reset occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Iwdgrstf::B0x1
    }
}
#[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdgrstf {
    #[doc = "0: No window watchdog reset occurred"]
    B0x0 = 0,
    #[doc = "1: Window watchdog reset occurred"]
    B0x1 = 1,
}
impl From<Wwdgrstf> for bool {
    #[inline(always)]
    fn from(variant: Wwdgrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit."]
pub type WwdgrstfR = crate::BitReader<Wwdgrstf>;
impl WwdgrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdgrstf {
        match self.bits {
            false => Wwdgrstf::B0x0,
            true => Wwdgrstf::B0x1,
        }
    }
    #[doc = "No window watchdog reset occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wwdgrstf::B0x0
    }
    #[doc = "Window watchdog reset occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wwdgrstf::B0x1
    }
}
#[doc = "Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, nRST_STDBY or nRST_SHDW option bits are cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpwrrstf {
    #[doc = "0: No illegal mode reset occurred"]
    B0x0 = 0,
    #[doc = "1: Illegal mode reset occurred"]
    B0x1 = 1,
}
impl From<Lpwrrstf> for bool {
    #[inline(always)]
    fn from(variant: Lpwrrstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, nRST_STDBY or nRST_SHDW option bits are cleared."]
pub type LpwrrstfR = crate::BitReader<Lpwrrstf>;
impl LpwrrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpwrrstf {
        match self.bits {
            false => Lpwrrstf::B0x0,
            true => Lpwrrstf::B0x1,
        }
    }
    #[doc = "No illegal mode reset occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpwrrstf::B0x0
    }
    #[doc = "Illegal mode reset occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpwrrstf::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:"]
    #[inline(always)]
    pub fn lsion(&self) -> LsionR {
        LsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC."]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal low-speed oscillator pre-divided by 128 Set and reset by hardware to indicate when the low-speed internal RC oscillator has to be divided by 128. The software has to switch off the LSI before changing this bit."]
    #[inline(always)]
    pub fn lsiprediv(&self) -> LsipredivR {
        LsipredivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - MSI range after Standby mode Set by software to chose the MSI frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a pad or a power-on reset, the range is always 41MHz. MSISRANGE\\[3:0\\]
can be written only when MSIRGSEL1=11. Others: Reserved Note: Changing the MSISRANGE\\[3:0\\]
does not change the current MSI frequency."]
    #[inline(always)]
    pub fn msisrange(&self) -> MsisrangeR {
        MsisrangeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Remove reset flags Set by software to clear the reset flags."]
    #[inline(always)]
    pub fn rmvf(&self) -> RmvfR {
        RmvfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit."]
    #[inline(always)]
    pub fn oblrstf(&self) -> OblrstfR {
        OblrstfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit."]
    #[inline(always)]
    pub fn pinrstf(&self) -> PinrstfR {
        PinrstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit."]
    #[inline(always)]
    pub fn pwrrstf(&self) -> PwrrstfR {
        PwrrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit."]
    #[inline(always)]
    pub fn sftrstf(&self) -> SftrstfR {
        SftrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit."]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IwdgrstfR {
        IwdgrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit."]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WwdgrstfR {
        WwdgrstfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, nRST_STDBY or nRST_SHDW option bits are cleared."]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LpwrrstfR {
        LpwrrstfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LsionW<RccCsrSpec> {
        LsionW::new(self, 0)
    }
    #[doc = "Bit 2 - Internal low-speed oscillator pre-divided by 128 Set and reset by hardware to indicate when the low-speed internal RC oscillator has to be divided by 128. The software has to switch off the LSI before changing this bit."]
    #[inline(always)]
    #[must_use]
    pub fn lsiprediv(&mut self) -> LsipredivW<RccCsrSpec> {
        LsipredivW::new(self, 2)
    }
    #[doc = "Bits 8:11 - MSI range after Standby mode Set by software to chose the MSI frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a pad or a power-on reset, the range is always 41MHz. MSISRANGE\\[3:0\\]
can be written only when MSIRGSEL1=11. Others: Reserved Note: Changing the MSISRANGE\\[3:0\\]
does not change the current MSI frequency."]
    #[inline(always)]
    #[must_use]
    pub fn msisrange(&mut self) -> MsisrangeW<RccCsrSpec> {
        MsisrangeW::new(self, 8)
    }
    #[doc = "Bit 23 - Remove reset flags Set by software to clear the reset flags."]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RmvfW<RccCsrSpec> {
        RmvfW::new(self, 23)
    }
}
#[doc = "Control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCsrSpec;
impl crate::RegisterSpec for RccCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_csr::R`](R) reader structure"]
impl crate::Readable for RccCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_csr::W`](W) writer structure"]
impl crate::Writable for RccCsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CSR to value 0"]
impl crate::Resettable for RccCsrSpec {
    const RESET_VALUE: u32 = 0;
}
