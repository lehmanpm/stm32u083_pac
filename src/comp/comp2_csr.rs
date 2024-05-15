#[doc = "Register `COMP2_CSR` reader"]
pub type R = crate::R<Comp2CsrSpec>;
#[doc = "Register `COMP2_CSR` writer"]
pub type W = crate::W<Comp2CsrSpec>;
#[doc = "Comparator 2 enable bit This bit is controlled by software (if not locked). It enables the comparator 2:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Comparator 2 enable bit This bit is controlled by software (if not locked). It enables the comparator 2:"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::B0x0,
            true => En::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == En::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == En::B0x1
    }
}
#[doc = "Field `EN` writer - Comparator 2 enable bit This bit is controlled by software (if not locked). It enables the comparator 2:"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(En::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(En::B0x1)
    }
}
#[doc = "Field `INMSEL` reader - Comparator 2 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP_INM of the comparator 2: Refer to Table178: COMP2 inverting input assignment."]
pub type InmselR = crate::FieldReader;
#[doc = "Field `INMSEL` writer - Comparator 2 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP_INM of the comparator 2: Refer to Table178: COMP2 inverting input assignment."]
pub type InmselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INPSEL` reader - Comparator 2 signal selector for noninverting input This bitfield is controlled by software (if not locked). It selects the signal for the noninverting input COMP_INP of the comparator 2 (also see the WINMODE bit): Refer to Table177: COMP2 noninverting input assignment."]
pub type InpselR = crate::FieldReader;
#[doc = "Field `INPSEL` writer - Comparator 2 signal selector for noninverting input This bitfield is controlled by software (if not locked). It selects the signal for the noninverting input COMP_INP of the comparator 2 (also see the WINMODE bit): Refer to Table177: COMP2 noninverting input assignment."]
pub type InpselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Comparator 2 noninverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP_INP input of the comparator 2:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winmode {
    #[doc = "0: Signal selected with INPSEL\\[1:0\\]
bitfield of this register"]
    B0x0 = 0,
    #[doc = "1: COMP_INP signal of the comparator 1 (required for window mode, see Figure164)"]
    B0x1 = 1,
}
impl From<Winmode> for bool {
    #[inline(always)]
    fn from(variant: Winmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINMODE` reader - Comparator 2 noninverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP_INP input of the comparator 2:"]
pub type WinmodeR = crate::BitReader<Winmode>;
impl WinmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Winmode {
        match self.bits {
            false => Winmode::B0x0,
            true => Winmode::B0x1,
        }
    }
    #[doc = "Signal selected with INPSEL\\[1:0\\]
bitfield of this register"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Winmode::B0x0
    }
    #[doc = "COMP_INP signal of the comparator 1 (required for window mode, see Figure164)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Winmode::B0x1
    }
}
#[doc = "Field `WINMODE` writer - Comparator 2 noninverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP_INP input of the comparator 2:"]
pub type WinmodeW<'a, REG> = crate::BitWriter<'a, REG, Winmode>;
impl<'a, REG> WinmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Signal selected with INPSEL\\[1:0\\]
bitfield of this register"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Winmode::B0x0)
    }
    #[doc = "COMP_INP signal of the comparator 1 (required for window mode, see Figure164)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Winmode::B0x1)
    }
}
#[doc = "Comparator 2 output selector This bit is controlled by software (if not locked). It selects the comparator 2 output:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winout {
    #[doc = "0: COMP_VALUE"]
    B0x0 = 0,
    #[doc = "1: COMP_VALUE XOR COMP_VALUE (required for window mode, see Figure164)"]
    B0x1 = 1,
}
impl From<Winout> for bool {
    #[inline(always)]
    fn from(variant: Winout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINOUT` reader - Comparator 2 output selector This bit is controlled by software (if not locked). It selects the comparator 2 output:"]
pub type WinoutR = crate::BitReader<Winout>;
impl WinoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Winout {
        match self.bits {
            false => Winout::B0x0,
            true => Winout::B0x1,
        }
    }
    #[doc = "COMP_VALUE"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Winout::B0x0
    }
    #[doc = "COMP_VALUE XOR COMP_VALUE (required for window mode, see Figure164)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Winout::B0x1
    }
}
#[doc = "Field `WINOUT` writer - Comparator 2 output selector This bit is controlled by software (if not locked). It selects the comparator 2 output:"]
pub type WinoutW<'a, REG> = crate::BitWriter<'a, REG, Winout>;
impl<'a, REG> WinoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP_VALUE"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Winout::B0x0)
    }
    #[doc = "COMP_VALUE XOR COMP_VALUE (required for window mode, see Figure164)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Winout::B0x1)
    }
}
#[doc = "Comparator 2 polarity selector This bit is controlled by software (if not locked). It selects the comparator 2 output polarity:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    #[doc = "0: Non-inverted"]
    B0x0 = 0,
    #[doc = "1: Inverted"]
    B0x1 = 1,
}
impl From<Polarity> for bool {
    #[inline(always)]
    fn from(variant: Polarity) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLARITY` reader - Comparator 2 polarity selector This bit is controlled by software (if not locked). It selects the comparator 2 output polarity:"]
pub type PolarityR = crate::BitReader<Polarity>;
impl PolarityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity {
        match self.bits {
            false => Polarity::B0x0,
            true => Polarity::B0x1,
        }
    }
    #[doc = "Non-inverted"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Polarity::B0x0
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Polarity::B0x1
    }
}
#[doc = "Field `POLARITY` writer - Comparator 2 polarity selector This bit is controlled by software (if not locked). It selects the comparator 2 output polarity:"]
pub type PolarityW<'a, REG> = crate::BitWriter<'a, REG, Polarity>;
impl<'a, REG> PolarityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-inverted"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::B0x0)
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::B0x1)
    }
}
#[doc = "Comparator 2 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 2:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hyst {
    #[doc = "0: No hysteresis"]
    B0x0 = 0,
    #[doc = "1: Low hysteresis"]
    B0x1 = 1,
    #[doc = "2: Medium hysteresis"]
    B0x2 = 2,
    #[doc = "3: High hysteresis"]
    B0x3 = 3,
}
impl From<Hyst> for u8 {
    #[inline(always)]
    fn from(variant: Hyst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hyst {
    type Ux = u8;
}
impl crate::IsEnum for Hyst {}
#[doc = "Field `HYST` reader - Comparator 2 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 2:"]
pub type HystR = crate::FieldReader<Hyst>;
impl HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hyst {
        match self.bits {
            0 => Hyst::B0x0,
            1 => Hyst::B0x1,
            2 => Hyst::B0x2,
            3 => Hyst::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hyst::B0x0
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hyst::B0x1
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Hyst::B0x2
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Hyst::B0x3
    }
}
#[doc = "Field `HYST` writer - Comparator 2 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 2:"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hyst, crate::Safe>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::B0x0)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::B0x1)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::B0x2)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::B0x3)
    }
}
#[doc = "Comparator 2 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 2:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pwrmode {
    #[doc = "0: High speed/high power"]
    B0x0 = 0,
    #[doc = "1: Medium speed/medium power"]
    B0x1 = 1,
    #[doc = "2: Medium speed/medium power"]
    B0x2 = 2,
    #[doc = "3: Low speed/low power"]
    B0x3 = 3,
}
impl From<Pwrmode> for u8 {
    #[inline(always)]
    fn from(variant: Pwrmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pwrmode {
    type Ux = u8;
}
impl crate::IsEnum for Pwrmode {}
#[doc = "Field `PWRMODE` reader - Comparator 2 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 2:"]
pub type PwrmodeR = crate::FieldReader<Pwrmode>;
impl PwrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrmode {
        match self.bits {
            0 => Pwrmode::B0x0,
            1 => Pwrmode::B0x1,
            2 => Pwrmode::B0x2,
            3 => Pwrmode::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "High speed/high power"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pwrmode::B0x0
    }
    #[doc = "Medium speed/medium power"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pwrmode::B0x1
    }
    #[doc = "Medium speed/medium power"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pwrmode::B0x2
    }
    #[doc = "Low speed/low power"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pwrmode::B0x3
    }
}
#[doc = "Field `PWRMODE` writer - Comparator 2 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 2:"]
pub type PwrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pwrmode, crate::Safe>;
impl<'a, REG> PwrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed/high power"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrmode::B0x0)
    }
    #[doc = "Medium speed/medium power"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrmode::B0x1)
    }
    #[doc = "Medium speed/medium power"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrmode::B0x2)
    }
    #[doc = "Low speed/low power"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrmode::B0x3)
    }
}
#[doc = "Comparator 2 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: Others: Reserved, must not be used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blanksel {
    #[doc = "0: No blanking"]
    B0x0 = 0,
    #[doc = "1: TIM1 OC4 enabled as blanking source"]
    B0x1 = 1,
    #[doc = "2: TIM1 OC5 enabled as blanking source"]
    B0x2 = 2,
    #[doc = "4: TIM2 OC3 enabled as blanking source"]
    B0x4 = 4,
    #[doc = "8: TIM3 OC3 enabled as blanking source"]
    B0x8 = 8,
    #[doc = "16: TIM15 OC2 enabled as blanking source"]
    B0x10 = 16,
}
impl From<Blanksel> for u8 {
    #[inline(always)]
    fn from(variant: Blanksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blanksel {
    type Ux = u8;
}
impl crate::IsEnum for Blanksel {}
#[doc = "Field `BLANKSEL` reader - Comparator 2 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: Others: Reserved, must not be used"]
pub type BlankselR = crate::FieldReader<Blanksel>;
impl BlankselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Blanksel> {
        match self.bits {
            0 => Some(Blanksel::B0x0),
            1 => Some(Blanksel::B0x1),
            2 => Some(Blanksel::B0x2),
            4 => Some(Blanksel::B0x4),
            8 => Some(Blanksel::B0x8),
            16 => Some(Blanksel::B0x10),
            _ => None,
        }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Blanksel::B0x0
    }
    #[doc = "TIM1 OC4 enabled as blanking source"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Blanksel::B0x1
    }
    #[doc = "TIM1 OC5 enabled as blanking source"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Blanksel::B0x2
    }
    #[doc = "TIM2 OC3 enabled as blanking source"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Blanksel::B0x4
    }
    #[doc = "TIM3 OC3 enabled as blanking source"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Blanksel::B0x8
    }
    #[doc = "TIM15 OC2 enabled as blanking source"]
    #[inline(always)]
    pub fn is_b_0x10(&self) -> bool {
        *self == Blanksel::B0x10
    }
}
#[doc = "Field `BLANKSEL` writer - Comparator 2 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: Others: Reserved, must not be used"]
pub type BlankselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Blanksel>;
impl<'a, REG> BlankselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksel::B0x0)
    }
    #[doc = "TIM1 OC4 enabled as blanking source"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksel::B0x1)
    }
    #[doc = "TIM1 OC5 enabled as blanking source"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksel::B0x2)
    }
    #[doc = "TIM2 OC3 enabled as blanking source"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksel::B0x4)
    }
    #[doc = "TIM3 OC3 enabled as blanking source"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksel::B0x8)
    }
    #[doc = "TIM15 OC2 enabled as blanking source"]
    #[inline(always)]
    pub fn b_0x10(self) -> &'a mut crate::W<REG> {
        self.variant(Blanksel::B0x10)
    }
}
#[doc = "Field `VALUE` reader - Comparator 2 output status This bit is read-only. It reflects the level of the comparator 2 output after the polarity selector and blanking, as indicated in Figure163."]
pub type ValueR = crate::BitReader;
#[doc = "COMP_CSR register lock This bit is set by software and cleared by a system reset. It locks the comparator 3 control bits. When locked, all register bits are read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Not locked"]
    B0x0 = 0,
    #[doc = "1: Locked"]
    B0x1 = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - COMP_CSR register lock This bit is set by software and cleared by a system reset. It locks the comparator 3 control bits. When locked, all register bits are read-only."]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::B0x0,
            true => Lock::B0x1,
        }
    }
    #[doc = "Not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lock::B0x0
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lock::B0x1
    }
}
#[doc = "Field `LOCK` writer - COMP_CSR register lock This bit is set by software and cleared by a system reset. It locks the comparator 3 control bits. When locked, all register bits are read-only."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::B0x0)
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit This bit is controlled by software (if not locked). It enables the comparator 2:"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparator 2 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP_INM of the comparator 2: Refer to Table178: COMP2 inverting input assignment."]
    #[inline(always)]
    pub fn inmsel(&self) -> InmselR {
        InmselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Comparator 2 signal selector for noninverting input This bitfield is controlled by software (if not locked). It selects the signal for the noninverting input COMP_INP of the comparator 2 (also see the WINMODE bit): Refer to Table177: COMP2 noninverting input assignment."]
    #[inline(always)]
    pub fn inpsel(&self) -> InpselR {
        InpselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Comparator 2 noninverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP_INP input of the comparator 2:"]
    #[inline(always)]
    pub fn winmode(&self) -> WinmodeR {
        WinmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator 2 output selector This bit is controlled by software (if not locked). It selects the comparator 2 output:"]
    #[inline(always)]
    pub fn winout(&self) -> WinoutR {
        WinoutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selector This bit is controlled by software (if not locked). It selects the comparator 2 output polarity:"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 2:"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Comparator 2 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 2:"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PwrmodeR {
        PwrmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:24 - Comparator 2 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: Others: Reserved, must not be used"]
    #[inline(always)]
    pub fn blanksel(&self) -> BlankselR {
        BlankselR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output status This bit is read-only. It reflects the level of the comparator 2 output after the polarity selector and blanking, as indicated in Figure163."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP_CSR register lock This bit is set by software and cleared by a system reset. It locks the comparator 3 control bits. When locked, all register bits are read-only."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit This bit is controlled by software (if not locked). It enables the comparator 2:"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Comp2CsrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Comparator 2 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP_INM of the comparator 2: Refer to Table178: COMP2 inverting input assignment."]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> InmselW<Comp2CsrSpec> {
        InmselW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Comparator 2 signal selector for noninverting input This bitfield is controlled by software (if not locked). It selects the signal for the noninverting input COMP_INP of the comparator 2 (also see the WINMODE bit): Refer to Table177: COMP2 noninverting input assignment."]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> InpselW<Comp2CsrSpec> {
        InpselW::new(self, 8)
    }
    #[doc = "Bit 11 - Comparator 2 noninverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP_INP input of the comparator 2:"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WinmodeW<Comp2CsrSpec> {
        WinmodeW::new(self, 11)
    }
    #[doc = "Bit 14 - Comparator 2 output selector This bit is controlled by software (if not locked). It selects the comparator 2 output:"]
    #[inline(always)]
    #[must_use]
    pub fn winout(&mut self) -> WinoutW<Comp2CsrSpec> {
        WinoutW::new(self, 14)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selector This bit is controlled by software (if not locked). It selects the comparator 2 output polarity:"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> PolarityW<Comp2CsrSpec> {
        PolarityW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 2:"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HystW<Comp2CsrSpec> {
        HystW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Comparator 2 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 2:"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PwrmodeW<Comp2CsrSpec> {
        PwrmodeW::new(self, 18)
    }
    #[doc = "Bits 20:24 - Comparator 2 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: Others: Reserved, must not be used"]
    #[inline(always)]
    #[must_use]
    pub fn blanksel(&mut self) -> BlankselW<Comp2CsrSpec> {
        BlankselW::new(self, 20)
    }
    #[doc = "Bit 31 - COMP_CSR register lock This bit is set by software and cleared by a system reset. It locks the comparator 3 control bits. When locked, all register bits are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<Comp2CsrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "Comparator 2 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp2CsrSpec;
impl crate::RegisterSpec for Comp2CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2_csr::R`](R) reader structure"]
impl crate::Readable for Comp2CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure"]
impl crate::Writable for Comp2CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for Comp2CsrSpec {
    const RESET_VALUE: u32 = 0;
}
