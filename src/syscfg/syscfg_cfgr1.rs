#[doc = "Register `SYSCFG_CFGR1` reader"]
pub type R = crate::R<SyscfgCfgr1Spec>;
#[doc = "Register `SYSCFG_CFGR1` writer"]
pub type W = crate::W<SyscfgCfgr1Spec>;
#[doc = "Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MemMode {
    #[doc = "1: System flash memory mapped at 0x000010000"]
    B0x1 = 1,
    #[doc = "3: Embedded SRAM mapped at 0x000010000"]
    B0x3 = 3,
}
impl From<MemMode> for u8 {
    #[inline(always)]
    fn from(variant: MemMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MemMode {
    type Ux = u8;
}
impl crate::IsEnum for MemMode {}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000"]
pub type MemModeR = crate::FieldReader<MemMode>;
impl MemModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MemMode> {
        match self.bits {
            1 => Some(MemMode::B0x1),
            3 => Some(MemMode::B0x3),
            _ => None,
        }
    }
    #[doc = "System flash memory mapped at 0x000010000"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MemMode::B0x1
    }
    #[doc = "Embedded SRAM mapped at 0x000010000"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MemMode::B0x3
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000"]
pub type MemModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, MemMode>;
impl<'a, REG> MemModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System flash memory mapped at 0x000010000"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MemMode::B0x1)
    }
    #[doc = "Embedded SRAM mapped at 0x000010000"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MemMode::B0x3)
    }
}
#[doc = "PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pa11Rmp {
    #[doc = "0: No remap (PA11)"]
    B0x0 = 0,
    #[doc = "1: Remap (PA9)"]
    B0x1 = 1,
}
impl From<Pa11Rmp> for bool {
    #[inline(always)]
    fn from(variant: Pa11Rmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PA11_RMP` reader - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
pub type Pa11RmpR = crate::BitReader<Pa11Rmp>;
impl Pa11RmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pa11Rmp {
        match self.bits {
            false => Pa11Rmp::B0x0,
            true => Pa11Rmp::B0x1,
        }
    }
    #[doc = "No remap (PA11)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pa11Rmp::B0x0
    }
    #[doc = "Remap (PA9)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pa11Rmp::B0x1
    }
}
#[doc = "Field `PA11_RMP` writer - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
pub type Pa11RmpW<'a, REG> = crate::BitWriter<'a, REG, Pa11Rmp>;
impl<'a, REG> Pa11RmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No remap (PA11)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pa11Rmp::B0x0)
    }
    #[doc = "Remap (PA9)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pa11Rmp::B0x1)
    }
}
#[doc = "PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pa12Rmp {
    #[doc = "0: No remap (PA12)"]
    B0x0 = 0,
    #[doc = "1: Remap (PA10)"]
    B0x1 = 1,
}
impl From<Pa12Rmp> for bool {
    #[inline(always)]
    fn from(variant: Pa12Rmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PA12_RMP` reader - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
pub type Pa12RmpR = crate::BitReader<Pa12Rmp>;
impl Pa12RmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pa12Rmp {
        match self.bits {
            false => Pa12Rmp::B0x0,
            true => Pa12Rmp::B0x1,
        }
    }
    #[doc = "No remap (PA12)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pa12Rmp::B0x0
    }
    #[doc = "Remap (PA10)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pa12Rmp::B0x1
    }
}
#[doc = "Field `PA12_RMP` writer - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
pub type Pa12RmpW<'a, REG> = crate::BitWriter<'a, REG, Pa12Rmp>;
impl<'a, REG> Pa12RmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No remap (PA12)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pa12Rmp::B0x0)
    }
    #[doc = "Remap (PA10)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pa12Rmp::B0x1)
    }
}
#[doc = "IR output polarity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrPol {
    #[doc = "0: Output of IRTIM (IR_OUT) is not inverted"]
    B0x0 = 0,
    #[doc = "1: Output of IRTIM (IR_OUT) is inverted"]
    B0x1 = 1,
}
impl From<IrPol> for bool {
    #[inline(always)]
    fn from(variant: IrPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IR_POL` reader - IR output polarity selection"]
pub type IrPolR = crate::BitReader<IrPol>;
impl IrPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrPol {
        match self.bits {
            false => IrPol::B0x0,
            true => IrPol::B0x1,
        }
    }
    #[doc = "Output of IRTIM (IR_OUT) is not inverted"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IrPol::B0x0
    }
    #[doc = "Output of IRTIM (IR_OUT) is inverted"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IrPol::B0x1
    }
}
#[doc = "Field `IR_POL` writer - IR output polarity selection"]
pub type IrPolW<'a, REG> = crate::BitWriter<'a, REG, IrPol>;
impl<'a, REG> IrPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output of IRTIM (IR_OUT) is not inverted"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IrPol::B0x0)
    }
    #[doc = "Output of IRTIM (IR_OUT) is inverted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IrPol::B0x1)
    }
}
#[doc = "IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IrMod {
    #[doc = "0: TIM16"]
    B0x0 = 0,
    #[doc = "1: USART1"]
    B0x1 = 1,
    #[doc = "2: USART2"]
    B0x2 = 2,
}
impl From<IrMod> for u8 {
    #[inline(always)]
    fn from(variant: IrMod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IrMod {
    type Ux = u8;
}
impl crate::IsEnum for IrMod {}
#[doc = "Field `IR_MOD` reader - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
pub type IrModR = crate::FieldReader<IrMod>;
impl IrModR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IrMod> {
        match self.bits {
            0 => Some(IrMod::B0x0),
            1 => Some(IrMod::B0x1),
            2 => Some(IrMod::B0x2),
            _ => None,
        }
    }
    #[doc = "TIM16"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IrMod::B0x0
    }
    #[doc = "USART1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IrMod::B0x1
    }
    #[doc = "USART2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == IrMod::B0x2
    }
}
#[doc = "Field `IR_MOD` writer - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
pub type IrModW<'a, REG> = crate::FieldWriter<'a, REG, 2, IrMod>;
impl<'a, REG> IrModW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM16"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IrMod::B0x0)
    }
    #[doc = "USART1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IrMod::B0x1)
    }
    #[doc = "USART2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IrMod::B0x2)
    }
}
#[doc = "I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V&lt;sub>DD&lt;/sub>, setting to 1 for low V&lt;sub>DD&lt;/sub> (less than 2.4 V).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boosten {
    #[doc = "0: V&lt;sub>DD&lt;/sub>"]
    B0x0 = 0,
    #[doc = "1: Dedicated voltage booster (supplied by V&lt;sub>DD&lt;/sub>)"]
    B0x1 = 1,
}
impl From<Boosten> for bool {
    #[inline(always)]
    fn from(variant: Boosten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOSTEN` reader - I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V&lt;sub>DD&lt;/sub>, setting to 1 for low V&lt;sub>DD&lt;/sub> (less than 2.4 V)."]
pub type BoostenR = crate::BitReader<Boosten>;
impl BoostenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boosten {
        match self.bits {
            false => Boosten::B0x0,
            true => Boosten::B0x1,
        }
    }
    #[doc = "V&lt;sub>DD&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Boosten::B0x0
    }
    #[doc = "Dedicated voltage booster (supplied by V&lt;sub>DD&lt;/sub>)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Boosten::B0x1
    }
}
#[doc = "Field `BOOSTEN` writer - I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V&lt;sub>DD&lt;/sub>, setting to 1 for low V&lt;sub>DD&lt;/sub> (less than 2.4 V)."]
pub type BoostenW<'a, REG> = crate::BitWriter<'a, REG, Boosten>;
impl<'a, REG> BoostenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V&lt;sub>DD&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Boosten::B0x0)
    }
    #[doc = "Dedicated voltage booster (supplied by V&lt;sub>DD&lt;/sub>)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Boosten::B0x1)
    }
}
#[doc = "Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cPb6Fmp {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2cPb6Fmp> for bool {
    #[inline(always)]
    fn from(variant: I2cPb6Fmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPb6FmpR = crate::BitReader<I2cPb6Fmp>;
impl I2cPb6FmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cPb6Fmp {
        match self.bits {
            false => I2cPb6Fmp::B0x0,
            true => I2cPb6Fmp::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2cPb6Fmp::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2cPb6Fmp::B0x1
    }
}
#[doc = "Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPb6FmpW<'a, REG> = crate::BitWriter<'a, REG, I2cPb6Fmp>;
impl<'a, REG> I2cPb6FmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPb6Fmp::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPb6Fmp::B0x1)
    }
}
#[doc = "Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cPb7Fmp {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2cPb7Fmp> for bool {
    #[inline(always)]
    fn from(variant: I2cPb7Fmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPb7FmpR = crate::BitReader<I2cPb7Fmp>;
impl I2cPb7FmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cPb7Fmp {
        match self.bits {
            false => I2cPb7Fmp::B0x0,
            true => I2cPb7Fmp::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2cPb7Fmp::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2cPb7Fmp::B0x1
    }
}
#[doc = "Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPb7FmpW<'a, REG> = crate::BitWriter<'a, REG, I2cPb7Fmp>;
impl<'a, REG> I2cPb7FmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPb7Fmp::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPb7Fmp::B0x1)
    }
}
#[doc = "Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cPb8Fmp {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2cPb8Fmp> for bool {
    #[inline(always)]
    fn from(variant: I2cPb8Fmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPb8FmpR = crate::BitReader<I2cPb8Fmp>;
impl I2cPb8FmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cPb8Fmp {
        match self.bits {
            false => I2cPb8Fmp::B0x0,
            true => I2cPb8Fmp::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2cPb8Fmp::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2cPb8Fmp::B0x1
    }
}
#[doc = "Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPb8FmpW<'a, REG> = crate::BitWriter<'a, REG, I2cPb8Fmp>;
impl<'a, REG> I2cPb8FmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPb8Fmp::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPb8Fmp::B0x1)
    }
}
#[doc = "Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cPb9Fmp {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2cPb9Fmp> for bool {
    #[inline(always)]
    fn from(variant: I2cPb9Fmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPb9FmpR = crate::BitReader<I2cPb9Fmp>;
impl I2cPb9FmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cPb9Fmp {
        match self.bits {
            false => I2cPb9Fmp::B0x0,
            true => I2cPb9Fmp::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2cPb9Fmp::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2cPb9Fmp::B0x1
    }
}
#[doc = "Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPb9FmpW<'a, REG> = crate::BitWriter<'a, REG, I2cPb9Fmp>;
impl<'a, REG> I2cPb9FmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPb9Fmp::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPb9Fmp::B0x1)
    }
}
#[doc = "Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cPa9Fmp {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2cPa9Fmp> for bool {
    #[inline(always)]
    fn from(variant: I2cPa9Fmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PA9_FMP` reader - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPa9FmpR = crate::BitReader<I2cPa9Fmp>;
impl I2cPa9FmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cPa9Fmp {
        match self.bits {
            false => I2cPa9Fmp::B0x0,
            true => I2cPa9Fmp::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2cPa9Fmp::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2cPa9Fmp::B0x1
    }
}
#[doc = "Field `I2C_PA9_FMP` writer - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPa9FmpW<'a, REG> = crate::BitWriter<'a, REG, I2cPa9Fmp>;
impl<'a, REG> I2cPa9FmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPa9Fmp::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPa9Fmp::B0x1)
    }
}
#[doc = "Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cPa10Fmp {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2cPa10Fmp> for bool {
    #[inline(always)]
    fn from(variant: I2cPa10Fmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PA10_FMP` reader - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPa10FmpR = crate::BitReader<I2cPa10Fmp>;
impl I2cPa10FmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cPa10Fmp {
        match self.bits {
            false => I2cPa10Fmp::B0x0,
            true => I2cPa10Fmp::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2cPa10Fmp::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2cPa10Fmp::B0x1
    }
}
#[doc = "Field `I2C_PA10_FMP` writer - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2cPa10FmpW<'a, REG> = crate::BitWriter<'a, REG, I2cPa10Fmp>;
impl<'a, REG> I2cPa10FmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPa10Fmp::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2cPa10Fmp::B0x1)
    }
}
#[doc = "Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c3Fmp {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2c3Fmp> for bool {
    #[inline(always)]
    fn from(variant: I2c3Fmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3_FMP` reader - Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2c3FmpR = crate::BitReader<I2c3Fmp>;
impl I2c3FmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c3Fmp {
        match self.bits {
            false => I2c3Fmp::B0x0,
            true => I2c3Fmp::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c3Fmp::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c3Fmp::B0x1
    }
}
#[doc = "Field `I2C3_FMP` writer - Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
pub type I2c3FmpW<'a, REG> = crate::BitWriter<'a, REG, I2c3Fmp>;
impl<'a, REG> I2c3FmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3Fmp::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3Fmp::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MemModeR {
        MemModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
    #[inline(always)]
    pub fn pa11_rmp(&self) -> Pa11RmpR {
        Pa11RmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
    #[inline(always)]
    pub fn pa12_rmp(&self) -> Pa12RmpR {
        Pa12RmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IrPolR {
        IrPolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
    #[inline(always)]
    pub fn ir_mod(&self) -> IrModR {
        IrModR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V&lt;sub>DD&lt;/sub>, setting to 1 for low V&lt;sub>DD&lt;/sub> (less than 2.4 V)."]
    #[inline(always)]
    pub fn boosten(&self) -> BoostenR {
        BoostenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2cPb6FmpR {
        I2cPb6FmpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2cPb7FmpR {
        I2cPb7FmpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2cPb8FmpR {
        I2cPb8FmpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2cPb9FmpR {
        I2cPb9FmpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2cPa9FmpR {
        I2cPa9FmpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2cPa10FmpR {
        I2cPa10FmpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2c3FmpR {
        I2c3FmpR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MemModeW<SyscfgCfgr1Spec> {
        MemModeW::new(self, 0)
    }
    #[doc = "Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port."]
    #[inline(always)]
    #[must_use]
    pub fn pa11_rmp(&mut self) -> Pa11RmpW<SyscfgCfgr1Spec> {
        Pa11RmpW::new(self, 3)
    }
    #[doc = "Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port."]
    #[inline(always)]
    #[must_use]
    pub fn pa12_rmp(&mut self) -> Pa12RmpW<SyscfgCfgr1Spec> {
        Pa12RmpW::new(self, 4)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn ir_pol(&mut self) -> IrPolW<SyscfgCfgr1Spec> {
        IrPolW::new(self, 5)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
    #[inline(always)]
    #[must_use]
    pub fn ir_mod(&mut self) -> IrModW<SyscfgCfgr1Spec> {
        IrModW::new(self, 6)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V&lt;sub>DD&lt;/sub>, setting to 1 for low V&lt;sub>DD&lt;/sub> (less than 2.4 V)."]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BoostenW<SyscfgCfgr1Spec> {
        BoostenW::new(self, 8)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2cPb6FmpW<SyscfgCfgr1Spec> {
        I2cPb6FmpW::new(self, 16)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2cPb7FmpW<SyscfgCfgr1Spec> {
        I2cPb7FmpW::new(self, 17)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2cPb8FmpW<SyscfgCfgr1Spec> {
        I2cPb8FmpW::new(self, 18)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2cPb9FmpW<SyscfgCfgr1Spec> {
        I2cPb9FmpW::new(self, 19)
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa9_fmp(&mut self) -> I2cPa9FmpW<SyscfgCfgr1Spec> {
        I2cPa9FmpW::new(self, 22)
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa10_fmp(&mut self) -> I2cPa10FmpW<SyscfgCfgr1Spec> {
        I2cPa10FmpW::new(self, 23)
    }
    #[doc = "Bit 24 - Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I&lt;sup>2&lt;/sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I&lt;sup>2&lt;/sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2c3FmpW<SyscfgCfgr1Spec> {
        I2c3FmpW::new(self, 24)
    }
}
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgCfgr1Spec;
impl crate::RegisterSpec for SyscfgCfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cfgr1::R`](R) reader structure"]
impl crate::Readable for SyscfgCfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cfgr1::W`](W) writer structure"]
impl crate::Writable for SyscfgCfgr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_CFGR1 to value 0"]
impl crate::Resettable for SyscfgCfgr1Spec {
    const RESET_VALUE: u32 = 0;
}
