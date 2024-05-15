#[doc = "Register `RCC_CR` reader"]
pub type R = crate::R<RccCrSpec>;
#[doc = "Register `RCC_CR` writer"]
pub type W = crate::W<RccCrSpec>;
#[doc = "MSI clock enable This bit is set and cleared by software. Cleared by hardware to stop the MSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when STOPWUCK=0 when exiting from Stop modes, or in case of a failure of the HSE oscillator Set by hardware when used directly or indirectly as system clock.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msion {
    #[doc = "0: MSI oscillator OFF"]
    B0x0 = 0,
    #[doc = "1: MSI oscillator ON"]
    B0x1 = 1,
}
impl From<Msion> for bool {
    #[inline(always)]
    fn from(variant: Msion) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSION` reader - MSI clock enable This bit is set and cleared by software. Cleared by hardware to stop the MSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when STOPWUCK=0 when exiting from Stop modes, or in case of a failure of the HSE oscillator Set by hardware when used directly or indirectly as system clock."]
pub type MsionR = crate::BitReader<Msion>;
impl MsionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msion {
        match self.bits {
            false => Msion::B0x0,
            true => Msion::B0x1,
        }
    }
    #[doc = "MSI oscillator OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msion::B0x0
    }
    #[doc = "MSI oscillator ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msion::B0x1
    }
}
#[doc = "Field `MSION` writer - MSI clock enable This bit is set and cleared by software. Cleared by hardware to stop the MSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when STOPWUCK=0 when exiting from Stop modes, or in case of a failure of the HSE oscillator Set by hardware when used directly or indirectly as system clock."]
pub type MsionW<'a, REG> = crate::BitWriter<'a, REG, Msion>;
impl<'a, REG> MsionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSI oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msion::B0x0)
    }
    #[doc = "MSI oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msion::B0x1)
    }
}
#[doc = "MSI clock ready flag This bit is set by hardware to indicate that the MSI oscillator is stable. Note: Once the MSION bit is cleared, MSIRDY goes low after 6 MSI clock cycles.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msirdy {
    #[doc = "0: MSI oscillator not ready"]
    B0x0 = 0,
    #[doc = "1: MSI oscillator ready"]
    B0x1 = 1,
}
impl From<Msirdy> for bool {
    #[inline(always)]
    fn from(variant: Msirdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRDY` reader - MSI clock ready flag This bit is set by hardware to indicate that the MSI oscillator is stable. Note: Once the MSION bit is cleared, MSIRDY goes low after 6 MSI clock cycles."]
pub type MsirdyR = crate::BitReader<Msirdy>;
impl MsirdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msirdy {
        match self.bits {
            false => Msirdy::B0x0,
            true => Msirdy::B0x1,
        }
    }
    #[doc = "MSI oscillator not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msirdy::B0x0
    }
    #[doc = "MSI oscillator ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msirdy::B0x1
    }
}
#[doc = "MSI clock PLL enable Set and cleared by software to enable/ disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware).There is a hardware protection to avoid enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the Clock Security System on LSE detects a LSE failure (refer to RCC_CSR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msipllen {
    #[doc = "0: MSI PLL OFF"]
    B0x0 = 0,
    #[doc = "1: MSI PLL ON"]
    B0x1 = 1,
}
impl From<Msipllen> for bool {
    #[inline(always)]
    fn from(variant: Msipllen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIPLLEN` reader - MSI clock PLL enable Set and cleared by software to enable/ disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware).There is a hardware protection to avoid enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the Clock Security System on LSE detects a LSE failure (refer to RCC_CSR register)."]
pub type MsipllenR = crate::BitReader<Msipllen>;
impl MsipllenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msipllen {
        match self.bits {
            false => Msipllen::B0x0,
            true => Msipllen::B0x1,
        }
    }
    #[doc = "MSI PLL OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msipllen::B0x0
    }
    #[doc = "MSI PLL ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msipllen::B0x1
    }
}
#[doc = "Field `MSIPLLEN` writer - MSI clock PLL enable Set and cleared by software to enable/ disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware).There is a hardware protection to avoid enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the Clock Security System on LSE detects a LSE failure (refer to RCC_CSR register)."]
pub type MsipllenW<'a, REG> = crate::BitWriter<'a, REG, Msipllen>;
impl<'a, REG> MsipllenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSI PLL OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msipllen::B0x0)
    }
    #[doc = "MSI PLL ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msipllen::B0x1)
    }
}
#[doc = "MSI clock range selection Set by software to select the MSI clock range with MSIRANGE\\[3:0\\]. Write 0 has no effect. After a standby or a reset MSIRGSEL is at 0 and the MSI range value is provided by MSISRANGE in CSR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msirgsel {
    #[doc = "0: MSI Range is provided by MSISRANGE\\[3:0\\]
in RCC_CSR register"]
    B0x0 = 0,
    #[doc = "1: MSI Range is provided by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    B0x1 = 1,
}
impl From<Msirgsel> for bool {
    #[inline(always)]
    fn from(variant: Msirgsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRGSEL` reader - MSI clock range selection Set by software to select the MSI clock range with MSIRANGE\\[3:0\\]. Write 0 has no effect. After a standby or a reset MSIRGSEL is at 0 and the MSI range value is provided by MSISRANGE in CSR register."]
pub type MsirgselR = crate::BitReader<Msirgsel>;
impl MsirgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msirgsel {
        match self.bits {
            false => Msirgsel::B0x0,
            true => Msirgsel::B0x1,
        }
    }
    #[doc = "MSI Range is provided by MSISRANGE\\[3:0\\]
in RCC_CSR register"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msirgsel::B0x0
    }
    #[doc = "MSI Range is provided by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msirgsel::B0x1
    }
}
#[doc = "Field `MSIRGSEL` writer - MSI clock range selection Set by software to select the MSI clock range with MSIRANGE\\[3:0\\]. Write 0 has no effect. After a standby or a reset MSIRGSEL is at 0 and the MSI range value is provided by MSISRANGE in CSR register."]
pub type MsirgselW<'a, REG> = crate::BitWriter<'a, REG, Msirgsel>;
impl<'a, REG> MsirgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSI Range is provided by MSISRANGE\\[3:0\\]
in RCC_CSR register"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msirgsel::B0x0)
    }
    #[doc = "MSI Range is provided by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msirgsel::B0x1)
    }
}
#[doc = "MSI clock ranges These bits are configured by software to choose the frequency range of MSI when MSIRGSEL is set.12 frequency ranges are available: others: not allowed (hardware write protection) Note: Warning: MSIRANGE can be modified when MSI is OFF (MSION=0) or when MSI is ready (MSIRDY=1). MSIRANGE must NOT be modified when MSI is ON and NOT ready (MSION=1 and MSIRDY=0)\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msirange {
    #[doc = "0: range 0 around 1001kHz"]
    B0x0 = 0,
    #[doc = "1: range 1 around 2001kHz"]
    B0x1 = 1,
    #[doc = "2: range 2 around 4001kHz"]
    B0x2 = 2,
    #[doc = "3: range 3 around 8001kHz"]
    B0x3 = 3,
    #[doc = "4: range 4 around 1M1Hz"]
    B0x4 = 4,
    #[doc = "5: range 5 around 21MHz"]
    B0x5 = 5,
    #[doc = "6: range 6 around 41MHz (reset value)"]
    B0x6 = 6,
    #[doc = "7: range 7 around 81MHz"]
    B0x7 = 7,
    #[doc = "8: range 8 around 161MHz"]
    B0x8 = 8,
    #[doc = "9: range 9 around 241MHz"]
    B0x9 = 9,
    #[doc = "10: range 10 around 321MHz"]
    B0xA = 10,
    #[doc = "11: range 11 around 481MHz"]
    B0xB = 11,
}
impl From<Msirange> for u8 {
    #[inline(always)]
    fn from(variant: Msirange) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msirange {
    type Ux = u8;
}
impl crate::IsEnum for Msirange {}
#[doc = "Field `MSIRANGE` reader - MSI clock ranges These bits are configured by software to choose the frequency range of MSI when MSIRGSEL is set.12 frequency ranges are available: others: not allowed (hardware write protection) Note: Warning: MSIRANGE can be modified when MSI is OFF (MSION=0) or when MSI is ready (MSIRDY=1). MSIRANGE must NOT be modified when MSI is ON and NOT ready (MSION=1 and MSIRDY=0)"]
pub type MsirangeR = crate::FieldReader<Msirange>;
impl MsirangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Msirange> {
        match self.bits {
            0 => Some(Msirange::B0x0),
            1 => Some(Msirange::B0x1),
            2 => Some(Msirange::B0x2),
            3 => Some(Msirange::B0x3),
            4 => Some(Msirange::B0x4),
            5 => Some(Msirange::B0x5),
            6 => Some(Msirange::B0x6),
            7 => Some(Msirange::B0x7),
            8 => Some(Msirange::B0x8),
            9 => Some(Msirange::B0x9),
            10 => Some(Msirange::B0xA),
            11 => Some(Msirange::B0xB),
            _ => None,
        }
    }
    #[doc = "range 0 around 1001kHz"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msirange::B0x0
    }
    #[doc = "range 1 around 2001kHz"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msirange::B0x1
    }
    #[doc = "range 2 around 4001kHz"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Msirange::B0x2
    }
    #[doc = "range 3 around 8001kHz"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Msirange::B0x3
    }
    #[doc = "range 4 around 1M1Hz"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Msirange::B0x4
    }
    #[doc = "range 5 around 21MHz"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Msirange::B0x5
    }
    #[doc = "range 6 around 41MHz (reset value)"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Msirange::B0x6
    }
    #[doc = "range 7 around 81MHz"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Msirange::B0x7
    }
    #[doc = "range 8 around 161MHz"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Msirange::B0x8
    }
    #[doc = "range 9 around 241MHz"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Msirange::B0x9
    }
    #[doc = "range 10 around 321MHz"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Msirange::B0xA
    }
    #[doc = "range 11 around 481MHz"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Msirange::B0xB
    }
}
#[doc = "Field `MSIRANGE` writer - MSI clock ranges These bits are configured by software to choose the frequency range of MSI when MSIRGSEL is set.12 frequency ranges are available: others: not allowed (hardware write protection) Note: Warning: MSIRANGE can be modified when MSI is OFF (MSION=0) or when MSI is ready (MSIRDY=1). MSIRANGE must NOT be modified when MSI is ON and NOT ready (MSION=1 and MSIRDY=0)"]
pub type MsirangeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Msirange>;
impl<'a, REG> MsirangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "range 0 around 1001kHz"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x0)
    }
    #[doc = "range 1 around 2001kHz"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x1)
    }
    #[doc = "range 2 around 4001kHz"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x2)
    }
    #[doc = "range 3 around 8001kHz"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x3)
    }
    #[doc = "range 4 around 1M1Hz"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x4)
    }
    #[doc = "range 5 around 21MHz"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x5)
    }
    #[doc = "range 6 around 41MHz (reset value)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x6)
    }
    #[doc = "range 7 around 81MHz"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x7)
    }
    #[doc = "range 8 around 161MHz"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x8)
    }
    #[doc = "range 9 around 241MHz"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0x9)
    }
    #[doc = "range 10 around 321MHz"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0xA)
    }
    #[doc = "range 11 around 481MHz"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Msirange::B0xB)
    }
}
#[doc = "HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. Forced by hardware to keep the HSI16 oscillator ON when it is used directly or indirectly as system clock (also when leaving Stop, Standby, or Shutdown modes, or in case of failure of the HSE oscillator used for system clock).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsion {
    #[doc = "0: HSI16 oscillator OFF"]
    B0x0 = 0,
    #[doc = "1: HSI16 oscillator ON"]
    B0x1 = 1,
}
impl From<Hsion> for bool {
    #[inline(always)]
    fn from(variant: Hsion) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. Forced by hardware to keep the HSI16 oscillator ON when it is used directly or indirectly as system clock (also when leaving Stop, Standby, or Shutdown modes, or in case of failure of the HSE oscillator used for system clock)."]
pub type HsionR = crate::BitReader<Hsion>;
impl HsionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsion {
        match self.bits {
            false => Hsion::B0x0,
            true => Hsion::B0x1,
        }
    }
    #[doc = "HSI16 oscillator OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsion::B0x0
    }
    #[doc = "HSI16 oscillator ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsion::B0x1
    }
}
#[doc = "Field `HSION` writer - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. Forced by hardware to keep the HSI16 oscillator ON when it is used directly or indirectly as system clock (also when leaving Stop, Standby, or Shutdown modes, or in case of failure of the HSE oscillator used for system clock)."]
pub type HsionW<'a, REG> = crate::BitWriter<'a, REG, Hsion>;
impl<'a, REG> HsionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI16 oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsion::B0x0)
    }
    #[doc = "HSI16 oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsion::B0x1)
    }
}
#[doc = "HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USART1, USART2, CEC and I2C1 peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows avoiding to slow down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsikeron {
    #[doc = "0: No effect on HSI16 oscillator."]
    B0x0 = 0,
    #[doc = "1: HSI16 oscillator is forced ON even in Stop mode."]
    B0x1 = 1,
}
impl From<Hsikeron> for bool {
    #[inline(always)]
    fn from(variant: Hsikeron) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIKERON` reader - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USART1, USART2, CEC and I2C1 peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows avoiding to slow down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
pub type HsikeronR = crate::BitReader<Hsikeron>;
impl HsikeronR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsikeron {
        match self.bits {
            false => Hsikeron::B0x0,
            true => Hsikeron::B0x1,
        }
    }
    #[doc = "No effect on HSI16 oscillator."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsikeron::B0x0
    }
    #[doc = "HSI16 oscillator is forced ON even in Stop mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsikeron::B0x1
    }
}
#[doc = "Field `HSIKERON` writer - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USART1, USART2, CEC and I2C1 peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows avoiding to slow down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
pub type HsikeronW<'a, REG> = crate::BitWriter<'a, REG, Hsikeron>;
impl<'a, REG> HsikeronW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on HSI16 oscillator."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsikeron::B0x0)
    }
    #[doc = "HSI16 oscillator is forced ON even in Stop mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsikeron::B0x1)
    }
}
#[doc = "HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsirdy {
    #[doc = "0: HSI16 oscillator not ready"]
    B0x0 = 0,
    #[doc = "1: HSI16 oscillator ready"]
    B0x1 = 1,
}
impl From<Hsirdy> for bool {
    #[inline(always)]
    fn from(variant: Hsirdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles."]
pub type HsirdyR = crate::BitReader<Hsirdy>;
impl HsirdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsirdy {
        match self.bits {
            false => Hsirdy::B0x0,
            true => Hsirdy::B0x1,
        }
    }
    #[doc = "HSI16 oscillator not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsirdy::B0x0
    }
    #[doc = "HSI16 oscillator ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsirdy::B0x1
    }
}
#[doc = "HSI16 automatic start from Stop Set and cleared by software. When the system wake-up clock is MSI, this bit is used to wake up the HSI16 is parallel of the system wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsiasfs {
    #[doc = "0: HSI16 oscillator is not enabled by hardware when exiting Stop mode with MSI as wake-up clock."]
    B0x0 = 0,
    #[doc = "1: HSI16 oscillator is enabled by hardware when exiting Stop mode with MSI as wake-up clock."]
    B0x1 = 1,
}
impl From<Hsiasfs> for bool {
    #[inline(always)]
    fn from(variant: Hsiasfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIASFS` reader - HSI16 automatic start from Stop Set and cleared by software. When the system wake-up clock is MSI, this bit is used to wake up the HSI16 is parallel of the system wake-up."]
pub type HsiasfsR = crate::BitReader<Hsiasfs>;
impl HsiasfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsiasfs {
        match self.bits {
            false => Hsiasfs::B0x0,
            true => Hsiasfs::B0x1,
        }
    }
    #[doc = "HSI16 oscillator is not enabled by hardware when exiting Stop mode with MSI as wake-up clock."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsiasfs::B0x0
    }
    #[doc = "HSI16 oscillator is enabled by hardware when exiting Stop mode with MSI as wake-up clock."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsiasfs::B0x1
    }
}
#[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hseon {
    #[doc = "0: HSE oscillator OFF"]
    B0x0 = 0,
    #[doc = "1: HSE oscillator ON"]
    B0x1 = 1,
}
impl From<Hseon> for bool {
    #[inline(always)]
    fn from(variant: Hseon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEON` reader - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
pub type HseonR = crate::BitReader<Hseon>;
impl HseonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hseon {
        match self.bits {
            false => Hseon::B0x0,
            true => Hseon::B0x1,
        }
    }
    #[doc = "HSE oscillator OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hseon::B0x0
    }
    #[doc = "HSE oscillator ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hseon::B0x1
    }
}
#[doc = "Field `HSEON` writer - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG, Hseon>;
impl<'a, REG> HseonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hseon::B0x0)
    }
    #[doc = "HSE oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hseon::B0x1)
    }
}
#[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hserdy {
    #[doc = "0: HSE oscillator not ready"]
    B0x0 = 0,
    #[doc = "1: HSE oscillator ready"]
    B0x1 = 1,
}
impl From<Hserdy> for bool {
    #[inline(always)]
    fn from(variant: Hserdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDY` reader - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles."]
pub type HserdyR = crate::BitReader<Hserdy>;
impl HserdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hserdy {
        match self.bits {
            false => Hserdy::B0x0,
            true => Hserdy::B0x1,
        }
    }
    #[doc = "HSE oscillator not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hserdy::B0x0
    }
    #[doc = "HSE oscillator ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hserdy::B0x1
    }
}
#[doc = "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsebyp {
    #[doc = "0: HSE crystal oscillator not bypassed"]
    B0x0 = 0,
    #[doc = "1: HSE crystal oscillator bypassed with external clock"]
    B0x1 = 1,
}
impl From<Hsebyp> for bool {
    #[inline(always)]
    fn from(variant: Hsebyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HsebypR = crate::BitReader<Hsebyp>;
impl HsebypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsebyp {
        match self.bits {
            false => Hsebyp::B0x0,
            true => Hsebyp::B0x1,
        }
    }
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsebyp::B0x0
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsebyp::B0x1
    }
}
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HsebypW<'a, REG> = crate::BitWriter<'a, REG, Hsebyp>;
impl<'a, REG> HsebypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsebyp::B0x0)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsebyp::B0x1)
    }
}
#[doc = "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csson {
    #[doc = "0: Clock security system OFF (clock detector OFF)"]
    B0x0 = 0,
    #[doc = "1: Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)."]
    B0x1 = 1,
}
impl From<Csson> for bool {
    #[inline(always)]
    fn from(variant: Csson) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSON` reader - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
pub type CssonR = crate::BitReader<Csson>;
impl CssonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csson {
        match self.bits {
            false => Csson::B0x0,
            true => Csson::B0x1,
        }
    }
    #[doc = "Clock security system OFF (clock detector OFF)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Csson::B0x0
    }
    #[doc = "Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Csson::B0x1
    }
}
#[doc = "Field `CSSON` writer - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
pub type CssonW<'a, REG> = crate::BitWriter<'a, REG, Csson>;
impl<'a, REG> CssonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock security system OFF (clock detector OFF)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Csson::B0x0)
    }
    #[doc = "Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Csson::B0x1)
    }
}
#[doc = "PLL enable Set and cleared by software to enable the PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllon {
    #[doc = "0: PLL OFF"]
    B0x0 = 0,
    #[doc = "1: PLL ON"]
    B0x1 = 1,
}
impl From<Pllon> for bool {
    #[inline(always)]
    fn from(variant: Pllon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLON` reader - PLL enable Set and cleared by software to enable the PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
pub type PllonR = crate::BitReader<Pllon>;
impl PllonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllon {
        match self.bits {
            false => Pllon::B0x0,
            true => Pllon::B0x1,
        }
    }
    #[doc = "PLL OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllon::B0x0
    }
    #[doc = "PLL ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllon::B0x1
    }
}
#[doc = "Field `PLLON` writer - PLL enable Set and cleared by software to enable the PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
pub type PllonW<'a, REG> = crate::BitWriter<'a, REG, Pllon>;
impl<'a, REG> PllonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllon::B0x0)
    }
    #[doc = "PLL ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllon::B0x1)
    }
}
#[doc = "PLL clock ready flag Set by hardware to indicate that the PLL is locked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllrdy {
    #[doc = "0: PLL unlocked"]
    B0x0 = 0,
    #[doc = "1: PLL locked"]
    B0x1 = 1,
}
impl From<Pllrdy> for bool {
    #[inline(always)]
    fn from(variant: Pllrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDY` reader - PLL clock ready flag Set by hardware to indicate that the PLL is locked."]
pub type PllrdyR = crate::BitReader<Pllrdy>;
impl PllrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllrdy {
        match self.bits {
            false => Pllrdy::B0x0,
            true => Pllrdy::B0x1,
        }
    }
    #[doc = "PLL unlocked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllrdy::B0x0
    }
    #[doc = "PLL locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllrdy::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - MSI clock enable This bit is set and cleared by software. Cleared by hardware to stop the MSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when STOPWUCK=0 when exiting from Stop modes, or in case of a failure of the HSE oscillator Set by hardware when used directly or indirectly as system clock."]
    #[inline(always)]
    pub fn msion(&self) -> MsionR {
        MsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSI clock ready flag This bit is set by hardware to indicate that the MSI oscillator is stable. Note: Once the MSION bit is cleared, MSIRDY goes low after 6 MSI clock cycles."]
    #[inline(always)]
    pub fn msirdy(&self) -> MsirdyR {
        MsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable Set and cleared by software to enable/ disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware).There is a hardware protection to avoid enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the Clock Security System on LSE detects a LSE failure (refer to RCC_CSR register)."]
    #[inline(always)]
    pub fn msipllen(&self) -> MsipllenR {
        MsipllenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MSI clock range selection Set by software to select the MSI clock range with MSIRANGE\\[3:0\\]. Write 0 has no effect. After a standby or a reset MSIRGSEL is at 0 and the MSI range value is provided by MSISRANGE in CSR register."]
    #[inline(always)]
    pub fn msirgsel(&self) -> MsirgselR {
        MsirgselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - MSI clock ranges These bits are configured by software to choose the frequency range of MSI when MSIRGSEL is set.12 frequency ranges are available: others: not allowed (hardware write protection) Note: Warning: MSIRANGE can be modified when MSI is OFF (MSION=0) or when MSI is ready (MSIRDY=1). MSIRANGE must NOT be modified when MSI is ON and NOT ready (MSION=1 and MSIRDY=0)"]
    #[inline(always)]
    pub fn msirange(&self) -> MsirangeR {
        MsirangeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. Forced by hardware to keep the HSI16 oscillator ON when it is used directly or indirectly as system clock (also when leaving Stop, Standby, or Shutdown modes, or in case of failure of the HSE oscillator used for system clock)."]
    #[inline(always)]
    pub fn hsion(&self) -> HsionR {
        HsionR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USART1, USART2, CEC and I2C1 peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows avoiding to slow down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
    #[inline(always)]
    pub fn hsikeron(&self) -> HsikeronR {
        HsikeronR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles."]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI16 automatic start from Stop Set and cleared by software. When the system wake-up clock is MSI, this bit is used to wake up the HSI16 is parallel of the system wake-up."]
    #[inline(always)]
    pub fn hsiasfs(&self) -> HsiasfsR {
        HsiasfsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles."]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn hsebyp(&self) -> HsebypR {
        HsebypR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
    #[inline(always)]
    pub fn csson(&self) -> CssonR {
        CssonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable Set and cleared by software to enable the PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
    #[inline(always)]
    pub fn pllon(&self) -> PllonR {
        PllonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag Set by hardware to indicate that the PLL is locked."]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSI clock enable This bit is set and cleared by software. Cleared by hardware to stop the MSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSI oscillator ON when STOPWUCK=0 when exiting from Stop modes, or in case of a failure of the HSE oscillator Set by hardware when used directly or indirectly as system clock."]
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MsionW<RccCrSpec> {
        MsionW::new(self, 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable Set and cleared by software to enable/ disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware).There is a hardware protection to avoid enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the Clock Security System on LSE detects a LSE failure (refer to RCC_CSR register)."]
    #[inline(always)]
    #[must_use]
    pub fn msipllen(&mut self) -> MsipllenW<RccCrSpec> {
        MsipllenW::new(self, 2)
    }
    #[doc = "Bit 3 - MSI clock range selection Set by software to select the MSI clock range with MSIRANGE\\[3:0\\]. Write 0 has no effect. After a standby or a reset MSIRGSEL is at 0 and the MSI range value is provided by MSISRANGE in CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn msirgsel(&mut self) -> MsirgselW<RccCrSpec> {
        MsirgselW::new(self, 3)
    }
    #[doc = "Bits 4:7 - MSI clock ranges These bits are configured by software to choose the frequency range of MSI when MSIRGSEL is set.12 frequency ranges are available: others: not allowed (hardware write protection) Note: Warning: MSIRANGE can be modified when MSI is OFF (MSION=0) or when MSI is ready (MSIRDY=1). MSIRANGE must NOT be modified when MSI is ON and NOT ready (MSION=1 and MSIRDY=0)"]
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MsirangeW<RccCrSpec> {
        MsirangeW::new(self, 4)
    }
    #[doc = "Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby, or Shutdown mode. Forced by hardware to keep the HSI16 oscillator ON when it is used directly or indirectly as system clock (also when leaving Stop, Standby, or Shutdown modes, or in case of failure of the HSE oscillator used for system clock)."]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HsionW<RccCrSpec> {
        HsionW::new(self, 8)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USART1, USART2, CEC and I2C1 peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows avoiding to slow down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HsikeronW<RccCrSpec> {
        HsikeronW::new(self, 9)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby, or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HseonW<RccCrSpec> {
        HseonW::new(self, 16)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HsebypW<RccCrSpec> {
        HsebypW::new(self, 18)
    }
    #[doc = "Bit 19 - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CssonW<RccCrSpec> {
        CssonW::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable Set and cleared by software to enable the PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PllonW<RccCrSpec> {
        PllonW::new(self, 24)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCrSpec;
impl crate::RegisterSpec for RccCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_cr::R`](R) reader structure"]
impl crate::Readable for RccCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_cr::W`](W) writer structure"]
impl crate::Writable for RccCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CR to value 0x83"]
impl crate::Resettable for RccCrSpec {
    const RESET_VALUE: u32 = 0x83;
}
