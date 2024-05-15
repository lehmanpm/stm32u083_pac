#[doc = "Register `RCC_PLLCFGR` reader"]
pub type R = crate::R<RccPllcfgrSpec>;
#[doc = "Register `RCC_PLLCFGR` writer"]
pub type W = crate::W<RccPllcfgrSpec>;
#[doc = "PLL input clock source This bit is controlled by software to select PLL clock source, as follows: The bitfield can be written only when the PLL is disabled. When the PLL is not used, selecting 00 allows saving power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllsrc {
    #[doc = "0: No clock"]
    B0x0 = 0,
    #[doc = "1: MSI"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
    #[doc = "3: HSE"]
    B0x3 = 3,
}
impl From<Pllsrc> for u8 {
    #[inline(always)]
    fn from(variant: Pllsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllsrc {
    type Ux = u8;
}
impl crate::IsEnum for Pllsrc {}
#[doc = "Field `PLLSRC` reader - PLL input clock source This bit is controlled by software to select PLL clock source, as follows: The bitfield can be written only when the PLL is disabled. When the PLL is not used, selecting 00 allows saving power."]
pub type PllsrcR = crate::FieldReader<Pllsrc>;
impl PllsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsrc {
        match self.bits {
            0 => Pllsrc::B0x0,
            1 => Pllsrc::B0x1,
            2 => Pllsrc::B0x2,
            3 => Pllsrc::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllsrc::B0x0
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllsrc::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pllsrc::B0x2
    }
    #[doc = "HSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pllsrc::B0x3
    }
}
#[doc = "Field `PLLSRC` writer - PLL input clock source This bit is controlled by software to select PLL clock source, as follows: The bitfield can be written only when the PLL is disabled. When the PLL is not used, selecting 00 allows saving power."]
pub type PllsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pllsrc, crate::Safe>;
impl<'a, REG> PllsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsrc::B0x0)
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsrc::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsrc::B0x2)
    }
    #[doc = "HSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsrc::B0x3)
    }
}
#[doc = "Division factor M of the PLL input clock divider This bit is controlled by software to divide the PLL input clock before the actual phase-locked loop, as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the PLL input frequency after the /M divider is between 2.66 and 161MHz.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllm {
    #[doc = "0: 1"]
    B0x0 = 0,
    #[doc = "1: 2"]
    B0x1 = 1,
    #[doc = "2: 3"]
    B0x2 = 2,
    #[doc = "3: 4"]
    B0x3 = 3,
    #[doc = "4: 5"]
    B0x4 = 4,
    #[doc = "5: 6"]
    B0x5 = 5,
    #[doc = "6: 7"]
    B0x6 = 6,
    #[doc = "7: 8"]
    B0x7 = 7,
}
impl From<Pllm> for u8 {
    #[inline(always)]
    fn from(variant: Pllm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllm {
    type Ux = u8;
}
impl crate::IsEnum for Pllm {}
#[doc = "Field `PLLM` reader - Division factor M of the PLL input clock divider This bit is controlled by software to divide the PLL input clock before the actual phase-locked loop, as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the PLL input frequency after the /M divider is between 2.66 and 161MHz."]
pub type PllmR = crate::FieldReader<Pllm>;
impl PllmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllm {
        match self.bits {
            0 => Pllm::B0x0,
            1 => Pllm::B0x1,
            2 => Pllm::B0x2,
            3 => Pllm::B0x3,
            4 => Pllm::B0x4,
            5 => Pllm::B0x5,
            6 => Pllm::B0x6,
            7 => Pllm::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllm::B0x0
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllm::B0x1
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pllm::B0x2
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pllm::B0x3
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pllm::B0x4
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pllm::B0x5
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pllm::B0x6
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pllm::B0x7
    }
}
#[doc = "Field `PLLM` writer - Division factor M of the PLL input clock divider This bit is controlled by software to divide the PLL input clock before the actual phase-locked loop, as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the PLL input frequency after the /M divider is between 2.66 and 161MHz."]
pub type PllmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pllm, crate::Safe>;
impl<'a, REG> PllmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x1)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x3)
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x4)
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x5)
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x6)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pllm::B0x7)
    }
}
#[doc = "PLL frequency multiplication factor N This bit is controlled by software to set the division factor of the f&lt;sub>VCO&lt;/sub> feedback divider (that determines the PLL multiplication ratio) as follows: ... ... The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the VCO output frequency is between 96 and 3441MHz.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plln {
    #[doc = "0: Invalid"]
    B0x0 = 0,
    #[doc = "4: 4"]
    B0x4 = 4,
    #[doc = "5: 5"]
    B0x5 = 5,
    #[doc = "126: 126"]
    B0x7e = 126,
    #[doc = "127: 127"]
    B0x7f = 127,
}
impl From<Plln> for u8 {
    #[inline(always)]
    fn from(variant: Plln) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plln {
    type Ux = u8;
}
impl crate::IsEnum for Plln {}
#[doc = "Field `PLLN` reader - PLL frequency multiplication factor N This bit is controlled by software to set the division factor of the f&lt;sub>VCO&lt;/sub> feedback divider (that determines the PLL multiplication ratio) as follows: ... ... The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the VCO output frequency is between 96 and 3441MHz."]
pub type PllnR = crate::FieldReader<Plln>;
impl PllnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Plln> {
        match self.bits {
            0 => Some(Plln::B0x0),
            4 => Some(Plln::B0x4),
            5 => Some(Plln::B0x5),
            126 => Some(Plln::B0x7e),
            127 => Some(Plln::B0x7f),
            _ => None,
        }
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Plln::B0x0
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Plln::B0x4
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Plln::B0x5
    }
    #[doc = "126"]
    #[inline(always)]
    pub fn is_b_0x7e(&self) -> bool {
        *self == Plln::B0x7e
    }
    #[doc = "127"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        *self == Plln::B0x7f
    }
}
#[doc = "Field `PLLN` writer - PLL frequency multiplication factor N This bit is controlled by software to set the division factor of the f&lt;sub>VCO&lt;/sub> feedback divider (that determines the PLL multiplication ratio) as follows: ... ... The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the VCO output frequency is between 96 and 3441MHz."]
pub type PllnW<'a, REG> = crate::FieldWriter<'a, REG, 7, Plln>;
impl<'a, REG> PllnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x0)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x4)
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x5)
    }
    #[doc = "126"]
    #[inline(always)]
    pub fn b_0x7e(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x7e)
    }
    #[doc = "127"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut crate::W<REG> {
        self.variant(Plln::B0x7f)
    }
}
#[doc = "PLLPCLK clock output enable This bit is controlled by software to enable/disable the PLLPCLK clock output of the PLL: Disabling the PLLPCLK clock output, when not used, allows saving power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllpen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Pllpen> for bool {
    #[inline(always)]
    fn from(variant: Pllpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPEN` reader - PLLPCLK clock output enable This bit is controlled by software to enable/disable the PLLPCLK clock output of the PLL: Disabling the PLLPCLK clock output, when not used, allows saving power."]
pub type PllpenR = crate::BitReader<Pllpen>;
impl PllpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllpen {
        match self.bits {
            false => Pllpen::B0x0,
            true => Pllpen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllpen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllpen::B0x1
    }
}
#[doc = "Field `PLLPEN` writer - PLLPCLK clock output enable This bit is controlled by software to enable/disable the PLLPCLK clock output of the PLL: Disabling the PLLPCLK clock output, when not used, allows saving power."]
pub type PllpenW<'a, REG> = crate::BitWriter<'a, REG, Pllpen>;
impl<'a, REG> PllpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpen::B0x1)
    }
}
#[doc = "PLL VCO division factor P for PLLPCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor P as follows: ... The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllp {
    #[doc = "1: 2"]
    B0x1 = 1,
    #[doc = "31: 32"]
    B0x1f = 31,
}
impl From<Pllp> for u8 {
    #[inline(always)]
    fn from(variant: Pllp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllp {
    type Ux = u8;
}
impl crate::IsEnum for Pllp {}
#[doc = "Field `PLLP` reader - PLL VCO division factor P for PLLPCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor P as follows: ... The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock."]
pub type PllpR = crate::FieldReader<Pllp>;
impl PllpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllp> {
        match self.bits {
            1 => Some(Pllp::B0x1),
            31 => Some(Pllp::B0x1f),
            _ => None,
        }
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllp::B0x1
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        *self == Pllp::B0x1f
    }
}
#[doc = "Field `PLLP` writer - PLL VCO division factor P for PLLPCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor P as follows: ... The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock."]
pub type PllpW<'a, REG> = crate::FieldWriter<'a, REG, 5, Pllp>;
impl<'a, REG> PllpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllp::B0x1)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut crate::W<REG> {
        self.variant(Pllp::B0x1f)
    }
}
#[doc = "PLLQCLK clock output enable This bit is controlled by software to enable/disable the PLLQCLK clock output of the PLL: Disabling the PLLQCLK clock output, when not used, allows saving power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllqen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Pllqen> for bool {
    #[inline(always)]
    fn from(variant: Pllqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLQEN` reader - PLLQCLK clock output enable This bit is controlled by software to enable/disable the PLLQCLK clock output of the PLL: Disabling the PLLQCLK clock output, when not used, allows saving power."]
pub type PllqenR = crate::BitReader<Pllqen>;
impl PllqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllqen {
        match self.bits {
            false => Pllqen::B0x0,
            true => Pllqen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllqen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllqen::B0x1
    }
}
#[doc = "Field `PLLQEN` writer - PLLQCLK clock output enable This bit is controlled by software to enable/disable the PLLQCLK clock output of the PLL: Disabling the PLLQCLK clock output, when not used, allows saving power."]
pub type PllqenW<'a, REG> = crate::BitWriter<'a, REG, Pllqen>;
impl<'a, REG> PllqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllqen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllqen::B0x1)
    }
}
#[doc = "PLL VCO division factor Q for PLLQCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor Q as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllq {
    #[doc = "1: 2"]
    B0x1 = 1,
    #[doc = "2: 3"]
    B0x2 = 2,
    #[doc = "3: 4"]
    B0x3 = 3,
    #[doc = "4: 5"]
    B0x4 = 4,
    #[doc = "5: 6"]
    B0x5 = 5,
    #[doc = "6: 7"]
    B0x6 = 6,
    #[doc = "7: 8"]
    B0x7 = 7,
}
impl From<Pllq> for u8 {
    #[inline(always)]
    fn from(variant: Pllq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllq {
    type Ux = u8;
}
impl crate::IsEnum for Pllq {}
#[doc = "Field `PLLQ` reader - PLL VCO division factor Q for PLLQCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor Q as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock."]
pub type PllqR = crate::FieldReader<Pllq>;
impl PllqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllq> {
        match self.bits {
            1 => Some(Pllq::B0x1),
            2 => Some(Pllq::B0x2),
            3 => Some(Pllq::B0x3),
            4 => Some(Pllq::B0x4),
            5 => Some(Pllq::B0x5),
            6 => Some(Pllq::B0x6),
            7 => Some(Pllq::B0x7),
            _ => None,
        }
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllq::B0x1
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pllq::B0x2
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pllq::B0x3
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pllq::B0x4
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pllq::B0x5
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pllq::B0x6
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pllq::B0x7
    }
}
#[doc = "Field `PLLQ` writer - PLL VCO division factor Q for PLLQCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor Q as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock."]
pub type PllqW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pllq>;
impl<'a, REG> PllqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x1)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x3)
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x4)
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x5)
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x6)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::B0x7)
    }
}
#[doc = "PLLRCLK clock output enable This bit is controlled by software to enable/disable the PLLRCLK clock output of the PLL: This bit cannot be written when PLLRCLK output of the PLL is selected for system clock. Disabling the PLLRCLK clock output, when not used, allows saving power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllren {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Pllren> for bool {
    #[inline(always)]
    fn from(variant: Pllren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLREN` reader - PLLRCLK clock output enable This bit is controlled by software to enable/disable the PLLRCLK clock output of the PLL: This bit cannot be written when PLLRCLK output of the PLL is selected for system clock. Disabling the PLLRCLK clock output, when not used, allows saving power."]
pub type PllrenR = crate::BitReader<Pllren>;
impl PllrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllren {
        match self.bits {
            false => Pllren::B0x0,
            true => Pllren::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllren::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllren::B0x1
    }
}
#[doc = "Field `PLLREN` writer - PLLRCLK clock output enable This bit is controlled by software to enable/disable the PLLRCLK clock output of the PLL: This bit cannot be written when PLLRCLK output of the PLL is selected for system clock. Disabling the PLLRCLK clock output, when not used, allows saving power."]
pub type PllrenW<'a, REG> = crate::BitWriter<'a, REG, Pllren>;
impl<'a, REG> PllrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllren::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllren::B0x1)
    }
}
#[doc = "PLL VCO division factor R for PLLRCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor R as follows: The bitfield can be written only when the PLL is disabled. The PLLRCLK clock can be selected as system clock. Caution: The software must set this bitfield so as not to exceed 122MHz on this clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllr {
    #[doc = "1: 2"]
    B0x1 = 1,
    #[doc = "2: 3"]
    B0x2 = 2,
    #[doc = "3: 4"]
    B0x3 = 3,
    #[doc = "4: 5"]
    B0x4 = 4,
    #[doc = "5: 6"]
    B0x5 = 5,
    #[doc = "6: 7"]
    B0x6 = 6,
    #[doc = "7: 8"]
    B0x7 = 7,
}
impl From<Pllr> for u8 {
    #[inline(always)]
    fn from(variant: Pllr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllr {
    type Ux = u8;
}
impl crate::IsEnum for Pllr {}
#[doc = "Field `PLLR` reader - PLL VCO division factor R for PLLRCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor R as follows: The bitfield can be written only when the PLL is disabled. The PLLRCLK clock can be selected as system clock. Caution: The software must set this bitfield so as not to exceed 122MHz on this clock."]
pub type PllrR = crate::FieldReader<Pllr>;
impl PllrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllr> {
        match self.bits {
            1 => Some(Pllr::B0x1),
            2 => Some(Pllr::B0x2),
            3 => Some(Pllr::B0x3),
            4 => Some(Pllr::B0x4),
            5 => Some(Pllr::B0x5),
            6 => Some(Pllr::B0x6),
            7 => Some(Pllr::B0x7),
            _ => None,
        }
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllr::B0x1
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pllr::B0x2
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pllr::B0x3
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pllr::B0x4
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pllr::B0x5
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pllr::B0x6
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pllr::B0x7
    }
}
#[doc = "Field `PLLR` writer - PLL VCO division factor R for PLLRCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor R as follows: The bitfield can be written only when the PLL is disabled. The PLLRCLK clock can be selected as system clock. Caution: The software must set this bitfield so as not to exceed 122MHz on this clock."]
pub type PllrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pllr>;
impl<'a, REG> PllrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x1)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x3)
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x4)
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x5)
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x6)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pllr::B0x7)
    }
}
impl R {
    #[doc = "Bits 0:1 - PLL input clock source This bit is controlled by software to select PLL clock source, as follows: The bitfield can be written only when the PLL is disabled. When the PLL is not used, selecting 00 allows saving power."]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Division factor M of the PLL input clock divider This bit is controlled by software to divide the PLL input clock before the actual phase-locked loop, as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the PLL input frequency after the /M divider is between 2.66 and 161MHz."]
    #[inline(always)]
    pub fn pllm(&self) -> PllmR {
        PllmR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:14 - PLL frequency multiplication factor N This bit is controlled by software to set the division factor of the f&lt;sub>VCO&lt;/sub> feedback divider (that determines the PLL multiplication ratio) as follows: ... ... The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the VCO output frequency is between 96 and 3441MHz."]
    #[inline(always)]
    pub fn plln(&self) -> PllnR {
        PllnR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - PLLPCLK clock output enable This bit is controlled by software to enable/disable the PLLPCLK clock output of the PLL: Disabling the PLLPCLK clock output, when not used, allows saving power."]
    #[inline(always)]
    pub fn pllpen(&self) -> PllpenR {
        PllpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor P as follows: ... The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock."]
    #[inline(always)]
    pub fn pllp(&self) -> PllpR {
        PllpR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - PLLQCLK clock output enable This bit is controlled by software to enable/disable the PLLQCLK clock output of the PLL: Disabling the PLLQCLK clock output, when not used, allows saving power."]
    #[inline(always)]
    pub fn pllqen(&self) -> PllqenR {
        PllqenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor Q as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock."]
    #[inline(always)]
    pub fn pllq(&self) -> PllqR {
        PllqR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - PLLRCLK clock output enable This bit is controlled by software to enable/disable the PLLRCLK clock output of the PLL: This bit cannot be written when PLLRCLK output of the PLL is selected for system clock. Disabling the PLLRCLK clock output, when not used, allows saving power."]
    #[inline(always)]
    pub fn pllren(&self) -> PllrenR {
        PllrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor R as follows: The bitfield can be written only when the PLL is disabled. The PLLRCLK clock can be selected as system clock. Caution: The software must set this bitfield so as not to exceed 122MHz on this clock."]
    #[inline(always)]
    pub fn pllr(&self) -> PllrR {
        PllrR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL input clock source This bit is controlled by software to select PLL clock source, as follows: The bitfield can be written only when the PLL is disabled. When the PLL is not used, selecting 00 allows saving power."]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PllsrcW<RccPllcfgrSpec> {
        PllsrcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Division factor M of the PLL input clock divider This bit is controlled by software to divide the PLL input clock before the actual phase-locked loop, as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the PLL input frequency after the /M divider is between 2.66 and 161MHz."]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PllmW<RccPllcfgrSpec> {
        PllmW::new(self, 4)
    }
    #[doc = "Bits 8:14 - PLL frequency multiplication factor N This bit is controlled by software to set the division factor of the f&lt;sub>VCO&lt;/sub> feedback divider (that determines the PLL multiplication ratio) as follows: ... ... The bitfield can be written only when the PLL is disabled. Caution: The software must set these bits so that the VCO output frequency is between 96 and 3441MHz."]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PllnW<RccPllcfgrSpec> {
        PllnW::new(self, 8)
    }
    #[doc = "Bit 16 - PLLPCLK clock output enable This bit is controlled by software to enable/disable the PLLPCLK clock output of the PLL: Disabling the PLLPCLK clock output, when not used, allows saving power."]
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PllpenW<RccPllcfgrSpec> {
        PllpenW::new(self, 16)
    }
    #[doc = "Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor P as follows: ... The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock."]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PllpW<RccPllcfgrSpec> {
        PllpW::new(self, 17)
    }
    #[doc = "Bit 24 - PLLQCLK clock output enable This bit is controlled by software to enable/disable the PLLQCLK clock output of the PLL: Disabling the PLLQCLK clock output, when not used, allows saving power."]
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PllqenW<RccPllcfgrSpec> {
        PllqenW::new(self, 24)
    }
    #[doc = "Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor Q as follows: The bitfield can be written only when the PLL is disabled. Caution: The software must set this bitfield so as not to exceed 541MHz on this clock."]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PllqW<RccPllcfgrSpec> {
        PllqW::new(self, 25)
    }
    #[doc = "Bit 28 - PLLRCLK clock output enable This bit is controlled by software to enable/disable the PLLRCLK clock output of the PLL: This bit cannot be written when PLLRCLK output of the PLL is selected for system clock. Disabling the PLLRCLK clock output, when not used, allows saving power."]
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PllrenW<RccPllcfgrSpec> {
        PllrenW::new(self, 28)
    }
    #[doc = "Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output This bitfield is controlled by software. It sets the PLL VCO division factor R as follows: The bitfield can be written only when the PLL is disabled. The PLLRCLK clock can be selected as system clock. Caution: The software must set this bitfield so as not to exceed 122MHz on this clock."]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PllrW<RccPllcfgrSpec> {
        PllrW::new(self, 29)
    }
}
#[doc = "PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pllcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pllcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccPllcfgrSpec;
impl crate::RegisterSpec for RccPllcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_pllcfgr::R`](R) reader structure"]
impl crate::Readable for RccPllcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_pllcfgr::W`](W) writer structure"]
impl crate::Writable for RccPllcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_PLLCFGR to value 0x1000"]
impl crate::Resettable for RccPllcfgrSpec {
    const RESET_VALUE: u32 = 0x1000;
}
