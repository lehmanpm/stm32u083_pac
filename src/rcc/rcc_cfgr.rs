#[doc = "Register `RCC_CFGR` reader"]
pub type R = crate::R<RccCfgrSpec>;
#[doc = "Register `RCC_CFGR` writer"]
pub type W = crate::W<RccCfgrSpec>;
#[doc = "System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sw {
    #[doc = "0: MSI"]
    B0x0 = 0,
    #[doc = "1: HSI16"]
    B0x1 = 1,
    #[doc = "2: HSE"]
    B0x2 = 2,
    #[doc = "3: PLLRCLK"]
    B0x3 = 3,
    #[doc = "4: LSI"]
    B0x4 = 4,
    #[doc = "5: LSE"]
    B0x5 = 5,
}
impl From<Sw> for u8 {
    #[inline(always)]
    fn from(variant: Sw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sw {
    type Ux = u8;
}
impl crate::IsEnum for Sw {}
#[doc = "Field `SW` reader - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected."]
pub type SwR = crate::FieldReader<Sw>;
impl SwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sw> {
        match self.bits {
            0 => Some(Sw::B0x0),
            1 => Some(Sw::B0x1),
            2 => Some(Sw::B0x2),
            3 => Some(Sw::B0x3),
            4 => Some(Sw::B0x4),
            5 => Some(Sw::B0x5),
            _ => None,
        }
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sw::B0x0
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sw::B0x1
    }
    #[doc = "HSE"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Sw::B0x2
    }
    #[doc = "PLLRCLK"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Sw::B0x3
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Sw::B0x4
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Sw::B0x5
    }
}
#[doc = "Field `SW` writer - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected."]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sw>;
impl<'a, REG> SwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MSI"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x0)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x1)
    }
    #[doc = "HSE"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x2)
    }
    #[doc = "PLLRCLK"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x3)
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x4)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x5)
    }
}
#[doc = "System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sws {
    #[doc = "0: MSI"]
    B0x0 = 0,
    #[doc = "1: HSI16"]
    B0x1 = 1,
    #[doc = "2: HSE"]
    B0x2 = 2,
    #[doc = "3: PLLRCLK"]
    B0x3 = 3,
    #[doc = "4: LSI"]
    B0x4 = 4,
    #[doc = "5: LSE"]
    B0x5 = 5,
}
impl From<Sws> for u8 {
    #[inline(always)]
    fn from(variant: Sws) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sws {
    type Ux = u8;
}
impl crate::IsEnum for Sws {}
#[doc = "Field `SWS` reader - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved"]
pub type SwsR = crate::FieldReader<Sws>;
impl SwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sws> {
        match self.bits {
            0 => Some(Sws::B0x0),
            1 => Some(Sws::B0x1),
            2 => Some(Sws::B0x2),
            3 => Some(Sws::B0x3),
            4 => Some(Sws::B0x4),
            5 => Some(Sws::B0x5),
            _ => None,
        }
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sws::B0x0
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sws::B0x1
    }
    #[doc = "HSE"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Sws::B0x2
    }
    #[doc = "PLLRCLK"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Sws::B0x3
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Sws::B0x4
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Sws::B0x5
    }
}
#[doc = "AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hpre {
    #[doc = "8: 2"]
    B0x8 = 8,
    #[doc = "9: 4"]
    B0x9 = 9,
    #[doc = "10: 8"]
    B0xA = 10,
    #[doc = "11: 16"]
    B0xB = 11,
    #[doc = "12: 64"]
    B0xC = 12,
    #[doc = "13: 128"]
    B0xD = 13,
    #[doc = "14: 256"]
    B0xE = 14,
    #[doc = "15: 512"]
    B0xF = 15,
}
impl From<Hpre> for u8 {
    #[inline(always)]
    fn from(variant: Hpre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hpre {
    type Ux = u8;
}
impl crate::IsEnum for Hpre {}
#[doc = "Field `HPRE` reader - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account."]
pub type HpreR = crate::FieldReader<Hpre>;
impl HpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hpre> {
        match self.bits {
            8 => Some(Hpre::B0x8),
            9 => Some(Hpre::B0x9),
            10 => Some(Hpre::B0xA),
            11 => Some(Hpre::B0xB),
            12 => Some(Hpre::B0xC),
            13 => Some(Hpre::B0xD),
            14 => Some(Hpre::B0xE),
            15 => Some(Hpre::B0xF),
            _ => None,
        }
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Hpre::B0x8
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Hpre::B0x9
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Hpre::B0xA
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Hpre::B0xB
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Hpre::B0xC
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Hpre::B0xD
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Hpre::B0xE
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Hpre::B0xF
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account."]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hpre>;
impl<'a, REG> HpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0x8)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0x9)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xA)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xB)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xC)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xD)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xE)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xF)
    }
}
#[doc = "APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ppre {
    #[doc = "4: 2"]
    B0x4 = 4,
    #[doc = "5: 4"]
    B0x5 = 5,
    #[doc = "6: 8"]
    B0x6 = 6,
    #[doc = "7: 16"]
    B0x7 = 7,
}
impl From<Ppre> for u8 {
    #[inline(always)]
    fn from(variant: Ppre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ppre {
    type Ux = u8;
}
impl crate::IsEnum for Ppre {}
#[doc = "Field `PPRE` reader - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1"]
pub type PpreR = crate::FieldReader<Ppre>;
impl PpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ppre> {
        match self.bits {
            4 => Some(Ppre::B0x4),
            5 => Some(Ppre::B0x5),
            6 => Some(Ppre::B0x6),
            7 => Some(Ppre::B0x7),
            _ => None,
        }
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Ppre::B0x4
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Ppre::B0x5
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Ppre::B0x6
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Ppre::B0x7
    }
}
#[doc = "Field `PPRE` writer - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1"]
pub type PpreW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ppre>;
impl<'a, REG> PpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre::B0x4)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre::B0x5)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre::B0x6)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre::B0x7)
    }
}
#[doc = "Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopwuck {
    #[doc = "0: MSI oscillator selected as wake-up from stop clock and CSS backup clock."]
    B0x0 = 0,
    #[doc = "1: HSI16 oscillator selected as wake-up from stop clock and CSS backup clock"]
    B0x1 = 1,
}
impl From<Stopwuck> for bool {
    #[inline(always)]
    fn from(variant: Stopwuck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPWUCK` reader - Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10)."]
pub type StopwuckR = crate::BitReader<Stopwuck>;
impl StopwuckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopwuck {
        match self.bits {
            false => Stopwuck::B0x0,
            true => Stopwuck::B0x1,
        }
    }
    #[doc = "MSI oscillator selected as wake-up from stop clock and CSS backup clock."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Stopwuck::B0x0
    }
    #[doc = "HSI16 oscillator selected as wake-up from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Stopwuck::B0x1
    }
}
#[doc = "Field `STOPWUCK` writer - Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10)."]
pub type StopwuckW<'a, REG> = crate::BitWriter<'a, REG, Stopwuck>;
impl<'a, REG> StopwuckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSI oscillator selected as wake-up from stop clock and CSS backup clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Stopwuck::B0x0)
    }
    #[doc = "HSI16 oscillator selected as wake-up from stop clock and CSS backup clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Stopwuck::B0x1)
    }
}
#[doc = "Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mco2sel {
    #[doc = "0: no clock, MCO2 output disabled"]
    B0x0 = 0,
    #[doc = "1: SYSCLK"]
    B0x1 = 1,
    #[doc = "2: MSI"]
    B0x2 = 2,
    #[doc = "3: HSI16"]
    B0x3 = 3,
    #[doc = "4: HSE"]
    B0x4 = 4,
    #[doc = "5: PLLRCLK"]
    B0x5 = 5,
    #[doc = "6: LSI"]
    B0x6 = 6,
    #[doc = "7: LSE"]
    B0x7 = 7,
    #[doc = "8: HSI48"]
    B0x8 = 8,
    #[doc = "9: RTCCLK"]
    B0x9 = 9,
    #[doc = "10: RTC WAKEUP"]
    B0xA = 10,
}
impl From<Mco2sel> for u8 {
    #[inline(always)]
    fn from(variant: Mco2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mco2sel {
    type Ux = u8;
}
impl crate::IsEnum for Mco2sel {}
#[doc = "Field `MCO2SEL` reader - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching."]
pub type Mco2selR = crate::FieldReader<Mco2sel>;
impl Mco2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mco2sel> {
        match self.bits {
            0 => Some(Mco2sel::B0x0),
            1 => Some(Mco2sel::B0x1),
            2 => Some(Mco2sel::B0x2),
            3 => Some(Mco2sel::B0x3),
            4 => Some(Mco2sel::B0x4),
            5 => Some(Mco2sel::B0x5),
            6 => Some(Mco2sel::B0x6),
            7 => Some(Mco2sel::B0x7),
            8 => Some(Mco2sel::B0x8),
            9 => Some(Mco2sel::B0x9),
            10 => Some(Mco2sel::B0xA),
            _ => None,
        }
    }
    #[doc = "no clock, MCO2 output disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mco2sel::B0x0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mco2sel::B0x1
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mco2sel::B0x2
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mco2sel::B0x3
    }
    #[doc = "HSE"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mco2sel::B0x4
    }
    #[doc = "PLLRCLK"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Mco2sel::B0x5
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Mco2sel::B0x6
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Mco2sel::B0x7
    }
    #[doc = "HSI48"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Mco2sel::B0x8
    }
    #[doc = "RTCCLK"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Mco2sel::B0x9
    }
    #[doc = "RTC WAKEUP"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Mco2sel::B0xA
    }
}
#[doc = "Field `MCO2SEL` writer - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching."]
pub type Mco2selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mco2sel>;
impl<'a, REG> Mco2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no clock, MCO2 output disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x1)
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x2)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x3)
    }
    #[doc = "HSE"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x4)
    }
    #[doc = "PLLRCLK"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x5)
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x6)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x7)
    }
    #[doc = "HSI48"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x8)
    }
    #[doc = "RTCCLK"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0x9)
    }
    #[doc = "RTC WAKEUP"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2sel::B0xA)
    }
}
#[doc = "Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mco2pre {
    #[doc = "0: 1"]
    B0x0 = 0,
    #[doc = "1: 2"]
    B0x1 = 1,
    #[doc = "2: 4"]
    B0x2 = 2,
    #[doc = "7: 128"]
    B0x7 = 7,
    #[doc = "8: 256"]
    B0x8 = 8,
    #[doc = "9: 512"]
    B0x9 = 9,
    #[doc = "10: 1024"]
    B0xA = 10,
}
impl From<Mco2pre> for u8 {
    #[inline(always)]
    fn from(variant: Mco2pre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mco2pre {
    type Ux = u8;
}
impl crate::IsEnum for Mco2pre {}
#[doc = "Field `MCO2PRE` reader - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled."]
pub type Mco2preR = crate::FieldReader<Mco2pre>;
impl Mco2preR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mco2pre> {
        match self.bits {
            0 => Some(Mco2pre::B0x0),
            1 => Some(Mco2pre::B0x1),
            2 => Some(Mco2pre::B0x2),
            7 => Some(Mco2pre::B0x7),
            8 => Some(Mco2pre::B0x8),
            9 => Some(Mco2pre::B0x9),
            10 => Some(Mco2pre::B0xA),
            _ => None,
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mco2pre::B0x0
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mco2pre::B0x1
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mco2pre::B0x2
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Mco2pre::B0x7
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Mco2pre::B0x8
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Mco2pre::B0x9
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Mco2pre::B0xA
    }
}
#[doc = "Field `MCO2PRE` writer - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled."]
pub type Mco2preW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mco2pre>;
impl<'a, REG> Mco2preW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2pre::B0x0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2pre::B0x1)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2pre::B0x2)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2pre::B0x7)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2pre::B0x8)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2pre::B0x9)
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Mco2pre::B0xA)
    }
}
#[doc = "Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mcosel {
    #[doc = "0: no clock, MCO output disabled"]
    B0x0 = 0,
    #[doc = "1: SYSCLK"]
    B0x1 = 1,
    #[doc = "2: MSI"]
    B0x2 = 2,
    #[doc = "3: HSI16"]
    B0x3 = 3,
    #[doc = "4: HSE"]
    B0x4 = 4,
    #[doc = "5: PLLRCLK"]
    B0x5 = 5,
    #[doc = "6: LSI"]
    B0x6 = 6,
    #[doc = "7: LSE"]
    B0x7 = 7,
    #[doc = "8: HSI48"]
    B0x8 = 8,
    #[doc = "9: RTCCLK"]
    B0x9 = 9,
    #[doc = "10: RTC WAKEUP"]
    B0xA = 10,
}
impl From<Mcosel> for u8 {
    #[inline(always)]
    fn from(variant: Mcosel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mcosel {
    type Ux = u8;
}
impl crate::IsEnum for Mcosel {}
#[doc = "Field `MCOSEL` reader - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
pub type McoselR = crate::FieldReader<Mcosel>;
impl McoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mcosel> {
        match self.bits {
            0 => Some(Mcosel::B0x0),
            1 => Some(Mcosel::B0x1),
            2 => Some(Mcosel::B0x2),
            3 => Some(Mcosel::B0x3),
            4 => Some(Mcosel::B0x4),
            5 => Some(Mcosel::B0x5),
            6 => Some(Mcosel::B0x6),
            7 => Some(Mcosel::B0x7),
            8 => Some(Mcosel::B0x8),
            9 => Some(Mcosel::B0x9),
            10 => Some(Mcosel::B0xA),
            _ => None,
        }
    }
    #[doc = "no clock, MCO output disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mcosel::B0x0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mcosel::B0x1
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mcosel::B0x2
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mcosel::B0x3
    }
    #[doc = "HSE"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mcosel::B0x4
    }
    #[doc = "PLLRCLK"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Mcosel::B0x5
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Mcosel::B0x6
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Mcosel::B0x7
    }
    #[doc = "HSI48"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Mcosel::B0x8
    }
    #[doc = "RTCCLK"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Mcosel::B0x9
    }
    #[doc = "RTC WAKEUP"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Mcosel::B0xA
    }
}
#[doc = "Field `MCOSEL` writer - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
pub type McoselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mcosel>;
impl<'a, REG> McoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no clock, MCO output disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x1)
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x2)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x3)
    }
    #[doc = "HSE"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x4)
    }
    #[doc = "PLLRCLK"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x5)
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x6)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x7)
    }
    #[doc = "HSI48"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x8)
    }
    #[doc = "RTCCLK"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x9)
    }
    #[doc = "RTC WAKEUP"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0xA)
    }
}
#[doc = "Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mcopre {
    #[doc = "0: 1"]
    B0x0 = 0,
    #[doc = "1: 2"]
    B0x1 = 1,
    #[doc = "2: 4"]
    B0x2 = 2,
    #[doc = "7: 128"]
    B0x7 = 7,
    #[doc = "8: 256"]
    B0x8 = 8,
    #[doc = "9: 512"]
    B0x9 = 9,
    #[doc = "10: 1024"]
    B0xA = 10,
}
impl From<Mcopre> for u8 {
    #[inline(always)]
    fn from(variant: Mcopre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mcopre {
    type Ux = u8;
}
impl crate::IsEnum for Mcopre {}
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled."]
pub type McopreR = crate::FieldReader<Mcopre>;
impl McopreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mcopre> {
        match self.bits {
            0 => Some(Mcopre::B0x0),
            1 => Some(Mcopre::B0x1),
            2 => Some(Mcopre::B0x2),
            7 => Some(Mcopre::B0x7),
            8 => Some(Mcopre::B0x8),
            9 => Some(Mcopre::B0x9),
            10 => Some(Mcopre::B0xA),
            _ => None,
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mcopre::B0x0
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mcopre::B0x1
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mcopre::B0x2
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Mcopre::B0x7
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Mcopre::B0x8
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Mcopre::B0x9
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Mcopre::B0xA
    }
}
#[doc = "Field `MCOPRE` writer - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled."]
pub type McopreW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mcopre>;
impl<'a, REG> McopreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x1)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x2)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x7)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x8)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x9)
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0xA)
    }
}
impl R {
    #[doc = "Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected."]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account."]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1"]
    #[inline(always)]
    pub fn ppre(&self) -> PpreR {
        PpreR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10)."]
    #[inline(always)]
    pub fn stopwuck(&self) -> StopwuckR {
        StopwuckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching."]
    #[inline(always)]
    pub fn mco2sel(&self) -> Mco2selR {
        Mco2selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled."]
    #[inline(always)]
    pub fn mco2pre(&self) -> Mco2preR {
        Mco2preR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
    #[inline(always)]
    pub fn mcosel(&self) -> McoselR {
        McoselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled."]
    #[inline(always)]
    pub fn mcopre(&self) -> McopreR {
        McopreR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected."]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SwW<RccCfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1 Caution: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to Section14.1.4: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account."]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HpreW<RccCfgrSpec> {
        HpreW::new(self, 8)
    }
    #[doc = "Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1"]
    #[inline(always)]
    #[must_use]
    pub fn ppre(&mut self) -> PpreW<RccCfgrSpec> {
        PpreW::new(self, 12)
    }
    #[doc = "Bit 15 - Wake-up from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the Clock Security System on HSE. Warning: STOPWUCK must not be modified when the Clock Security System is enabled by HSECSSON in RCC_CR register and the system clock is HSE (SWS=10) or a switch on HSE is requested (SW=10)."]
    #[inline(always)]
    #[must_use]
    pub fn stopwuck(&mut self) -> StopwuckW<RccCfgrSpec> {
        StopwuckW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching."]
    #[inline(always)]
    #[must_use]
    pub fn mco2sel(&mut self) -> Mco2selW<RccCfgrSpec> {
        Mco2selW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Others: reserved It is highly recommended to set this field before the MCO2 output is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn mco2pre(&mut self) -> Mco2preW<RccCfgrSpec> {
        Mco2preW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> McoselW<RccCfgrSpec> {
        McoselW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Others: reserved It is highly recommended to set this field before the MCO output is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> McopreW<RccCfgrSpec> {
        McopreW::new(self, 28)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCfgrSpec;
impl crate::RegisterSpec for RccCfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_cfgr::R`](R) reader structure"]
impl crate::Readable for RccCfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_cfgr::W`](W) writer structure"]
impl crate::Writable for RccCfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CFGR to value 0"]
impl crate::Resettable for RccCfgrSpec {
    const RESET_VALUE: u32 = 0;
}
