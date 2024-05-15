#[doc = "Register `OPAMP_CSR` reader"]
pub type R = crate::R<OpampCsrSpec>;
#[doc = "Register `OPAMP_CSR` writer"]
pub type W = crate::W<OpampCsrSpec>;
#[doc = "Operational amplifier Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opaen {
    #[doc = "0: operational amplifier disabled"]
    B0x0 = 0,
    #[doc = "1: operational amplifier enabled"]
    B0x1 = 1,
}
impl From<Opaen> for bool {
    #[inline(always)]
    fn from(variant: Opaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub type OpaenR = crate::BitReader<Opaen>;
impl OpaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opaen {
        match self.bits {
            false => Opaen::B0x0,
            true => Opaen::B0x1,
        }
    }
    #[doc = "operational amplifier disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Opaen::B0x0
    }
    #[doc = "operational amplifier enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Opaen::B0x1
    }
}
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub type OpaenW<'a, REG> = crate::BitWriter<'a, REG, Opaen>;
impl<'a, REG> OpaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "operational amplifier disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Opaen::B0x0)
    }
    #[doc = "operational amplifier enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Opaen::B0x1)
    }
}
#[doc = "Operational amplifier Low Power Mode The operational amplifier must be disable to change this configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opalpm {
    #[doc = "0: operational amplifier in normal mode"]
    B0x0 = 0,
    #[doc = "1: operational amplifier in low-power mode"]
    B0x1 = 1,
}
impl From<Opalpm> for bool {
    #[inline(always)]
    fn from(variant: Opalpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPALPM` reader - Operational amplifier Low Power Mode The operational amplifier must be disable to change this configuration."]
pub type OpalpmR = crate::BitReader<Opalpm>;
impl OpalpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opalpm {
        match self.bits {
            false => Opalpm::B0x0,
            true => Opalpm::B0x1,
        }
    }
    #[doc = "operational amplifier in normal mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Opalpm::B0x0
    }
    #[doc = "operational amplifier in low-power mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Opalpm::B0x1
    }
}
#[doc = "Field `OPALPM` writer - Operational amplifier Low Power Mode The operational amplifier must be disable to change this configuration."]
pub type OpalpmW<'a, REG> = crate::BitWriter<'a, REG, Opalpm>;
impl<'a, REG> OpalpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "operational amplifier in normal mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Opalpm::B0x0)
    }
    #[doc = "operational amplifier in low-power mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Opalpm::B0x1)
    }
}
#[doc = "Operational amplifier PGA mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opamode {
    #[doc = "0: internal PGA disable"]
    B0x0 = 0,
    #[doc = "1: internal PGA disable"]
    B0x1 = 1,
    #[doc = "2: internal PGA enable, gain programmed in PGA_GAIN"]
    B0x2 = 2,
    #[doc = "3: internal follower"]
    B0x3 = 3,
}
impl From<Opamode> for u8 {
    #[inline(always)]
    fn from(variant: Opamode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opamode {
    type Ux = u8;
}
impl crate::IsEnum for Opamode {}
#[doc = "Field `OPAMODE` reader - Operational amplifier PGA mode"]
pub type OpamodeR = crate::FieldReader<Opamode>;
impl OpamodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opamode {
        match self.bits {
            0 => Opamode::B0x0,
            1 => Opamode::B0x1,
            2 => Opamode::B0x2,
            3 => Opamode::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "internal PGA disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Opamode::B0x0
    }
    #[doc = "internal PGA disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Opamode::B0x1
    }
    #[doc = "internal PGA enable, gain programmed in PGA_GAIN"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Opamode::B0x2
    }
    #[doc = "internal follower"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Opamode::B0x3
    }
}
#[doc = "Field `OPAMODE` writer - Operational amplifier PGA mode"]
pub type OpamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Opamode, crate::Safe>;
impl<'a, REG> OpamodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "internal PGA disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Opamode::B0x0)
    }
    #[doc = "internal PGA disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Opamode::B0x1)
    }
    #[doc = "internal PGA enable, gain programmed in PGA_GAIN"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Opamode::B0x2)
    }
    #[doc = "internal follower"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Opamode::B0x3)
    }
}
#[doc = "Operational amplifier Programmable amplifier gain value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PgaGain {
    #[doc = "0: internal PGA Gain 2"]
    B0x0 = 0,
    #[doc = "1: internal PGA Gain 4"]
    B0x1 = 1,
    #[doc = "2: internal PGA Gain 8"]
    B0x2 = 2,
    #[doc = "3: internal PGA Gain 16"]
    B0x3 = 3,
}
impl From<PgaGain> for u8 {
    #[inline(always)]
    fn from(variant: PgaGain) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PgaGain {
    type Ux = u8;
}
impl crate::IsEnum for PgaGain {}
#[doc = "Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value"]
pub type PgaGainR = crate::FieldReader<PgaGain>;
impl PgaGainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PgaGain {
        match self.bits {
            0 => PgaGain::B0x0,
            1 => PgaGain::B0x1,
            2 => PgaGain::B0x2,
            3 => PgaGain::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "internal PGA Gain 2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PgaGain::B0x0
    }
    #[doc = "internal PGA Gain 4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PgaGain::B0x1
    }
    #[doc = "internal PGA Gain 8"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PgaGain::B0x2
    }
    #[doc = "internal PGA Gain 16"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PgaGain::B0x3
    }
}
#[doc = "Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value"]
pub type PgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 2, PgaGain, crate::Safe>;
impl<'a, REG> PgaGainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "internal PGA Gain 2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PgaGain::B0x0)
    }
    #[doc = "internal PGA Gain 4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PgaGain::B0x1)
    }
    #[doc = "internal PGA Gain 8"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PgaGain::B0x2)
    }
    #[doc = "internal PGA Gain 16"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PgaGain::B0x3)
    }
}
#[doc = "Inverting input selection These bits are used only when OPAMODE = 00, 01 or 10. 1x: Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VmSel {
    #[doc = "0: GPIO connected to VINM (valid also in PGA mode for filtering)"]
    B0x0 = 0,
}
impl From<VmSel> for u8 {
    #[inline(always)]
    fn from(variant: VmSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VmSel {
    type Ux = u8;
}
impl crate::IsEnum for VmSel {}
#[doc = "Field `VM_SEL` reader - Inverting input selection These bits are used only when OPAMODE = 00, 01 or 10. 1x: Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode)"]
pub type VmSelR = crate::FieldReader<VmSel>;
impl VmSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VmSel> {
        match self.bits {
            0 => Some(VmSel::B0x0),
            _ => None,
        }
    }
    #[doc = "GPIO connected to VINM (valid also in PGA mode for filtering)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VmSel::B0x0
    }
}
#[doc = "Field `VM_SEL` writer - Inverting input selection These bits are used only when OPAMODE = 00, 01 or 10. 1x: Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode)"]
pub type VmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, VmSel>;
impl<'a, REG> VmSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO connected to VINM (valid also in PGA mode for filtering)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VmSel::B0x0)
    }
}
#[doc = "Non inverted input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpSel {
    #[doc = "0: GPIO connected to VINP"]
    B0x0 = 0,
    #[doc = "1: DAC connected to VINP"]
    B0x1 = 1,
}
impl From<VpSel> for bool {
    #[inline(always)]
    fn from(variant: VpSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_SEL` reader - Non inverted input selection"]
pub type VpSelR = crate::BitReader<VpSel>;
impl VpSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpSel {
        match self.bits {
            false => VpSel::B0x0,
            true => VpSel::B0x1,
        }
    }
    #[doc = "GPIO connected to VINP"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VpSel::B0x0
    }
    #[doc = "DAC connected to VINP"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VpSel::B0x1
    }
}
#[doc = "Field `VP_SEL` writer - Non inverted input selection"]
pub type VpSelW<'a, REG> = crate::BitWriter<'a, REG, VpSel>;
impl<'a, REG> VpSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO connected to VINP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VpSel::B0x0)
    }
    #[doc = "DAC connected to VINP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VpSel::B0x1)
    }
}
#[doc = "Calibration mode enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calon {
    #[doc = "0: Normal mode"]
    B0x0 = 0,
    #[doc = "1: Calibration mode (all switches opened by HW)"]
    B0x1 = 1,
}
impl From<Calon> for bool {
    #[inline(always)]
    fn from(variant: Calon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALON` reader - Calibration mode enabled"]
pub type CalonR = crate::BitReader<Calon>;
impl CalonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calon {
        match self.bits {
            false => Calon::B0x0,
            true => Calon::B0x1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Calon::B0x0
    }
    #[doc = "Calibration mode (all switches opened by HW)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Calon::B0x1
    }
}
#[doc = "Field `CALON` writer - Calibration mode enabled"]
pub type CalonW<'a, REG> = crate::BitWriter<'a, REG, Calon>;
impl<'a, REG> CalonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Calon::B0x0)
    }
    #[doc = "Calibration mode (all switches opened by HW)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Calon::B0x1)
    }
}
#[doc = "Calibration selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calsel {
    #[doc = "0: NMOS calibration (200mV applied on OPAMP inputs)"]
    B0x0 = 0,
    #[doc = "1: PMOS calibration (VDDA-200mV applied on OPAMP inputs)"]
    B0x1 = 1,
}
impl From<Calsel> for bool {
    #[inline(always)]
    fn from(variant: Calsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub type CalselR = crate::BitReader<Calsel>;
impl CalselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calsel {
        match self.bits {
            false => Calsel::B0x0,
            true => Calsel::B0x1,
        }
    }
    #[doc = "NMOS calibration (200mV applied on OPAMP inputs)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Calsel::B0x0
    }
    #[doc = "PMOS calibration (VDDA-200mV applied on OPAMP inputs)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Calsel::B0x1
    }
}
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CalselW<'a, REG> = crate::BitWriter<'a, REG, Calsel>;
impl<'a, REG> CalselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NMOS calibration (200mV applied on OPAMP inputs)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Calsel::B0x0)
    }
    #[doc = "PMOS calibration (VDDA-200mV applied on OPAMP inputs)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Calsel::B0x1)
    }
}
#[doc = "allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values This bit is active for both mode normal and low-power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usertrim {
    #[doc = "0: factory trim code used"]
    B0x0 = 0,
    #[doc = "1: user trim code used"]
    B0x1 = 1,
}
impl From<Usertrim> for bool {
    #[inline(always)]
    fn from(variant: Usertrim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USERTRIM` reader - allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values This bit is active for both mode normal and low-power."]
pub type UsertrimR = crate::BitReader<Usertrim>;
impl UsertrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usertrim {
        match self.bits {
            false => Usertrim::B0x0,
            true => Usertrim::B0x1,
        }
    }
    #[doc = "factory trim code used"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usertrim::B0x0
    }
    #[doc = "user trim code used"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usertrim::B0x1
    }
}
#[doc = "Field `USERTRIM` writer - allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values This bit is active for both mode normal and low-power."]
pub type UsertrimW<'a, REG> = crate::BitWriter<'a, REG, Usertrim>;
impl<'a, REG> UsertrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "factory trim code used"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usertrim::B0x0)
    }
    #[doc = "user trim code used"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usertrim::B0x1)
    }
}
#[doc = "Field `CALOUT` reader - Operational amplifier calibration output During calibration mode offset is trimmed when this signal toggle."]
pub type CaloutR = crate::BitReader;
#[doc = "Operational amplifier power supply range for stability All AOP must be in power down to allow AOP-RANGE bit write. It applies to all AOP embedded in the product.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpaRange {
    #[doc = "0: Low range (VDDA &lt; 2.4V)"]
    B0x0 = 0,
    #[doc = "1: High range (VDDA > 2.4V)"]
    B0x1 = 1,
}
impl From<OpaRange> for bool {
    #[inline(always)]
    fn from(variant: OpaRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPA_RANGE` reader - Operational amplifier power supply range for stability All AOP must be in power down to allow AOP-RANGE bit write. It applies to all AOP embedded in the product."]
pub type OpaRangeR = crate::BitReader<OpaRange>;
impl OpaRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OpaRange {
        match self.bits {
            false => OpaRange::B0x0,
            true => OpaRange::B0x1,
        }
    }
    #[doc = "Low range (VDDA &lt; 2.4V)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OpaRange::B0x0
    }
    #[doc = "High range (VDDA > 2.4V)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OpaRange::B0x1
    }
}
#[doc = "Field `OPA_RANGE` writer - Operational amplifier power supply range for stability All AOP must be in power down to allow AOP-RANGE bit write. It applies to all AOP embedded in the product."]
pub type OpaRangeW<'a, REG> = crate::BitWriter<'a, REG, OpaRange>;
impl<'a, REG> OpaRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low range (VDDA &lt; 2.4V)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OpaRange::B0x0)
    }
    #[doc = "High range (VDDA > 2.4V)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OpaRange::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OpaenR {
        OpaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode The operational amplifier must be disable to change this configuration."]
    #[inline(always)]
    pub fn opalpm(&self) -> OpalpmR {
        OpalpmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    pub fn opamode(&self) -> OpamodeR {
        OpamodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PgaGainR {
        PgaGainR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Inverting input selection These bits are used only when OPAMODE = 00, 01 or 10. 1x: Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode)"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VmSelR {
        VmSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VpSelR {
        VpSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&self) -> CalonR {
        CalonR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CalselR {
        CalselR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values This bit is active for both mode normal and low-power."]
    #[inline(always)]
    pub fn usertrim(&self) -> UsertrimR {
        UsertrimR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output During calibration mode offset is trimmed when this signal toggle."]
    #[inline(always)]
    pub fn calout(&self) -> CaloutR {
        CaloutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Operational amplifier power supply range for stability All AOP must be in power down to allow AOP-RANGE bit write. It applies to all AOP embedded in the product."]
    #[inline(always)]
    pub fn opa_range(&self) -> OpaRangeR {
        OpaRangeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OpaenW<OpampCsrSpec> {
        OpaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode The operational amplifier must be disable to change this configuration."]
    #[inline(always)]
    #[must_use]
    pub fn opalpm(&mut self) -> OpalpmW<OpampCsrSpec> {
        OpalpmW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    #[must_use]
    pub fn opamode(&mut self) -> OpamodeW<OpampCsrSpec> {
        OpamodeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PgaGainW<OpampCsrSpec> {
        PgaGainW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Inverting input selection These bits are used only when OPAMODE = 00, 01 or 10. 1x: Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode)"]
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VmSelW<OpampCsrSpec> {
        VmSelW::new(self, 8)
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VpSelW<OpampCsrSpec> {
        VpSelW::new(self, 10)
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CalonW<OpampCsrSpec> {
        CalonW::new(self, 12)
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CalselW<OpampCsrSpec> {
        CalselW::new(self, 13)
    }
    #[doc = "Bit 14 - allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values This bit is active for both mode normal and low-power."]
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> UsertrimW<OpampCsrSpec> {
        UsertrimW::new(self, 14)
    }
    #[doc = "Bit 31 - Operational amplifier power supply range for stability All AOP must be in power down to allow AOP-RANGE bit write. It applies to all AOP embedded in the product."]
    #[inline(always)]
    #[must_use]
    pub fn opa_range(&mut self) -> OpaRangeW<OpampCsrSpec> {
        OpaRangeW::new(self, 31)
    }
}
#[doc = "OPAMP control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpampCsrSpec;
impl crate::RegisterSpec for OpampCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp_csr::R`](R) reader structure"]
impl crate::Readable for OpampCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp_csr::W`](W) writer structure"]
impl crate::Writable for OpampCsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP_CSR to value 0"]
impl crate::Resettable for OpampCsrSpec {
    const RESET_VALUE: u32 = 0;
}
