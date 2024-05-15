#[doc = "Register `EXTI_EMR1` reader"]
pub type R = crate::R<ExtiEmr1Spec>;
#[doc = "Register `EXTI_EMR1` writer"]
pub type W = crate::W<ExtiEmr1Spec>;
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em0 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em0> for bool {
    #[inline(always)]
    fn from(variant: Em0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM0` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em0R = crate::BitReader<Em0>;
impl Em0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em0 {
        match self.bits {
            false => Em0::B0x0,
            true => Em0::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em0::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em0::B0x1
    }
}
#[doc = "Field `EM0` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em0W<'a, REG> = crate::BitWriter<'a, REG, Em0>;
impl<'a, REG> Em0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em1 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em1> for bool {
    #[inline(always)]
    fn from(variant: Em1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM1` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em1R = crate::BitReader<Em1>;
impl Em1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em1 {
        match self.bits {
            false => Em1::B0x0,
            true => Em1::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em1::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em1::B0x1
    }
}
#[doc = "Field `EM1` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em1W<'a, REG> = crate::BitWriter<'a, REG, Em1>;
impl<'a, REG> Em1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em2 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em2> for bool {
    #[inline(always)]
    fn from(variant: Em2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM2` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em2R = crate::BitReader<Em2>;
impl Em2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em2 {
        match self.bits {
            false => Em2::B0x0,
            true => Em2::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em2::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em2::B0x1
    }
}
#[doc = "Field `EM2` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em2W<'a, REG> = crate::BitWriter<'a, REG, Em2>;
impl<'a, REG> Em2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em3 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em3> for bool {
    #[inline(always)]
    fn from(variant: Em3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM3` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em3R = crate::BitReader<Em3>;
impl Em3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em3 {
        match self.bits {
            false => Em3::B0x0,
            true => Em3::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em3::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em3::B0x1
    }
}
#[doc = "Field `EM3` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em3W<'a, REG> = crate::BitWriter<'a, REG, Em3>;
impl<'a, REG> Em3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em4 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em4> for bool {
    #[inline(always)]
    fn from(variant: Em4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM4` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em4R = crate::BitReader<Em4>;
impl Em4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em4 {
        match self.bits {
            false => Em4::B0x0,
            true => Em4::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em4::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em4::B0x1
    }
}
#[doc = "Field `EM4` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em4W<'a, REG> = crate::BitWriter<'a, REG, Em4>;
impl<'a, REG> Em4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em5 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em5> for bool {
    #[inline(always)]
    fn from(variant: Em5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM5` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em5R = crate::BitReader<Em5>;
impl Em5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em5 {
        match self.bits {
            false => Em5::B0x0,
            true => Em5::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em5::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em5::B0x1
    }
}
#[doc = "Field `EM5` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em5W<'a, REG> = crate::BitWriter<'a, REG, Em5>;
impl<'a, REG> Em5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em6 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em6> for bool {
    #[inline(always)]
    fn from(variant: Em6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM6` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em6R = crate::BitReader<Em6>;
impl Em6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em6 {
        match self.bits {
            false => Em6::B0x0,
            true => Em6::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em6::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em6::B0x1
    }
}
#[doc = "Field `EM6` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em6W<'a, REG> = crate::BitWriter<'a, REG, Em6>;
impl<'a, REG> Em6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em7 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em7> for bool {
    #[inline(always)]
    fn from(variant: Em7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM7` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em7R = crate::BitReader<Em7>;
impl Em7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em7 {
        match self.bits {
            false => Em7::B0x0,
            true => Em7::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em7::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em7::B0x1
    }
}
#[doc = "Field `EM7` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em7W<'a, REG> = crate::BitWriter<'a, REG, Em7>;
impl<'a, REG> Em7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em8 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em8> for bool {
    #[inline(always)]
    fn from(variant: Em8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM8` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em8R = crate::BitReader<Em8>;
impl Em8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em8 {
        match self.bits {
            false => Em8::B0x0,
            true => Em8::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em8::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em8::B0x1
    }
}
#[doc = "Field `EM8` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em8W<'a, REG> = crate::BitWriter<'a, REG, Em8>;
impl<'a, REG> Em8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em8::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em8::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em9 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em9> for bool {
    #[inline(always)]
    fn from(variant: Em9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM9` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em9R = crate::BitReader<Em9>;
impl Em9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em9 {
        match self.bits {
            false => Em9::B0x0,
            true => Em9::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em9::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em9::B0x1
    }
}
#[doc = "Field `EM9` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em9W<'a, REG> = crate::BitWriter<'a, REG, Em9>;
impl<'a, REG> Em9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em9::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em9::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em10 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em10> for bool {
    #[inline(always)]
    fn from(variant: Em10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM10` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em10R = crate::BitReader<Em10>;
impl Em10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em10 {
        match self.bits {
            false => Em10::B0x0,
            true => Em10::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em10::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em10::B0x1
    }
}
#[doc = "Field `EM10` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em10W<'a, REG> = crate::BitWriter<'a, REG, Em10>;
impl<'a, REG> Em10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em10::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em10::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em11 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em11> for bool {
    #[inline(always)]
    fn from(variant: Em11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM11` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em11R = crate::BitReader<Em11>;
impl Em11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em11 {
        match self.bits {
            false => Em11::B0x0,
            true => Em11::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em11::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em11::B0x1
    }
}
#[doc = "Field `EM11` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em11W<'a, REG> = crate::BitWriter<'a, REG, Em11>;
impl<'a, REG> Em11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em11::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em11::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em12 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em12> for bool {
    #[inline(always)]
    fn from(variant: Em12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM12` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em12R = crate::BitReader<Em12>;
impl Em12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em12 {
        match self.bits {
            false => Em12::B0x0,
            true => Em12::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em12::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em12::B0x1
    }
}
#[doc = "Field `EM12` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em12W<'a, REG> = crate::BitWriter<'a, REG, Em12>;
impl<'a, REG> Em12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em12::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em12::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em13 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em13> for bool {
    #[inline(always)]
    fn from(variant: Em13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM13` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em13R = crate::BitReader<Em13>;
impl Em13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em13 {
        match self.bits {
            false => Em13::B0x0,
            true => Em13::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em13::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em13::B0x1
    }
}
#[doc = "Field `EM13` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em13W<'a, REG> = crate::BitWriter<'a, REG, Em13>;
impl<'a, REG> Em13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em13::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em13::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em14 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em14> for bool {
    #[inline(always)]
    fn from(variant: Em14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM14` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em14R = crate::BitReader<Em14>;
impl Em14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em14 {
        match self.bits {
            false => Em14::B0x0,
            true => Em14::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em14::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em14::B0x1
    }
}
#[doc = "Field `EM14` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em14W<'a, REG> = crate::BitWriter<'a, REG, Em14>;
impl<'a, REG> Em14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em14::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em14::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em15 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em15> for bool {
    #[inline(always)]
    fn from(variant: Em15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM15` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em15R = crate::BitReader<Em15>;
impl Em15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em15 {
        match self.bits {
            false => Em15::B0x0,
            true => Em15::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em15::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em15::B0x1
    }
}
#[doc = "Field `EM15` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em15W<'a, REG> = crate::BitWriter<'a, REG, Em15>;
impl<'a, REG> Em15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em15::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em15::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em16 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em16> for bool {
    #[inline(always)]
    fn from(variant: Em16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM16` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em16R = crate::BitReader<Em16>;
impl Em16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em16 {
        match self.bits {
            false => Em16::B0x0,
            true => Em16::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em16::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em16::B0x1
    }
}
#[doc = "Field `EM16` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em16W<'a, REG> = crate::BitWriter<'a, REG, Em16>;
impl<'a, REG> Em16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em16::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em16::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em17 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em17> for bool {
    #[inline(always)]
    fn from(variant: Em17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM17` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em17R = crate::BitReader<Em17>;
impl Em17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em17 {
        match self.bits {
            false => Em17::B0x0,
            true => Em17::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em17::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em17::B0x1
    }
}
#[doc = "Field `EM17` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em17W<'a, REG> = crate::BitWriter<'a, REG, Em17>;
impl<'a, REG> Em17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em17::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em17::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em18 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em18> for bool {
    #[inline(always)]
    fn from(variant: Em18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM18` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em18R = crate::BitReader<Em18>;
impl Em18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em18 {
        match self.bits {
            false => Em18::B0x0,
            true => Em18::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em18::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em18::B0x1
    }
}
#[doc = "Field `EM18` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em18W<'a, REG> = crate::BitWriter<'a, REG, Em18>;
impl<'a, REG> Em18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em18::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em18::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em19 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em19> for bool {
    #[inline(always)]
    fn from(variant: Em19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM19` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em19R = crate::BitReader<Em19>;
impl Em19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em19 {
        match self.bits {
            false => Em19::B0x0,
            true => Em19::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em19::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em19::B0x1
    }
}
#[doc = "Field `EM19` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em19W<'a, REG> = crate::BitWriter<'a, REG, Em19>;
impl<'a, REG> Em19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em19::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em19::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em20 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em20> for bool {
    #[inline(always)]
    fn from(variant: Em20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM20` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em20R = crate::BitReader<Em20>;
impl Em20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em20 {
        match self.bits {
            false => Em20::B0x0,
            true => Em20::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em20::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em20::B0x1
    }
}
#[doc = "Field `EM20` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em20W<'a, REG> = crate::BitWriter<'a, REG, Em20>;
impl<'a, REG> Em20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em20::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em20::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em21 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em21> for bool {
    #[inline(always)]
    fn from(variant: Em21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM21` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em21R = crate::BitReader<Em21>;
impl Em21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em21 {
        match self.bits {
            false => Em21::B0x0,
            true => Em21::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em21::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em21::B0x1
    }
}
#[doc = "Field `EM21` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em21W<'a, REG> = crate::BitWriter<'a, REG, Em21>;
impl<'a, REG> Em21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em21::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em21::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em22 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em22> for bool {
    #[inline(always)]
    fn from(variant: Em22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM22` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em22R = crate::BitReader<Em22>;
impl Em22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em22 {
        match self.bits {
            false => Em22::B0x0,
            true => Em22::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em22::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em22::B0x1
    }
}
#[doc = "Field `EM22` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em22W<'a, REG> = crate::BitWriter<'a, REG, Em22>;
impl<'a, REG> Em22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em22::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em22::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em23 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em23> for bool {
    #[inline(always)]
    fn from(variant: Em23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM23` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em23R = crate::BitReader<Em23>;
impl Em23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em23 {
        match self.bits {
            false => Em23::B0x0,
            true => Em23::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em23::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em23::B0x1
    }
}
#[doc = "Field `EM23` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em23W<'a, REG> = crate::BitWriter<'a, REG, Em23>;
impl<'a, REG> Em23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em23::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em23::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em24 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em24> for bool {
    #[inline(always)]
    fn from(variant: Em24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM24` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em24R = crate::BitReader<Em24>;
impl Em24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em24 {
        match self.bits {
            false => Em24::B0x0,
            true => Em24::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em24::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em24::B0x1
    }
}
#[doc = "Field `EM24` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em24W<'a, REG> = crate::BitWriter<'a, REG, Em24>;
impl<'a, REG> Em24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em24::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em24::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em25 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em25> for bool {
    #[inline(always)]
    fn from(variant: Em25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM25` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em25R = crate::BitReader<Em25>;
impl Em25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em25 {
        match self.bits {
            false => Em25::B0x0,
            true => Em25::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em25::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em25::B0x1
    }
}
#[doc = "Field `EM25` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em25W<'a, REG> = crate::BitWriter<'a, REG, Em25>;
impl<'a, REG> Em25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em25::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em25::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em26 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em26> for bool {
    #[inline(always)]
    fn from(variant: Em26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM26` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em26R = crate::BitReader<Em26>;
impl Em26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em26 {
        match self.bits {
            false => Em26::B0x0,
            true => Em26::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em26::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em26::B0x1
    }
}
#[doc = "Field `EM26` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em26W<'a, REG> = crate::BitWriter<'a, REG, Em26>;
impl<'a, REG> Em26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em26::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em26::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em27 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em27> for bool {
    #[inline(always)]
    fn from(variant: Em27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM27` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em27R = crate::BitReader<Em27>;
impl Em27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em27 {
        match self.bits {
            false => Em27::B0x0,
            true => Em27::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em27::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em27::B0x1
    }
}
#[doc = "Field `EM27` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em27W<'a, REG> = crate::BitWriter<'a, REG, Em27>;
impl<'a, REG> Em27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em27::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em27::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em28 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em28> for bool {
    #[inline(always)]
    fn from(variant: Em28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM28` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em28R = crate::BitReader<Em28>;
impl Em28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em28 {
        match self.bits {
            false => Em28::B0x0,
            true => Em28::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em28::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em28::B0x1
    }
}
#[doc = "Field `EM28` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em28W<'a, REG> = crate::BitWriter<'a, REG, Em28>;
impl<'a, REG> Em28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em28::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em28::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em29 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em29> for bool {
    #[inline(always)]
    fn from(variant: Em29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM29` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em29R = crate::BitReader<Em29>;
impl Em29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em29 {
        match self.bits {
            false => Em29::B0x0,
            true => Em29::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em29::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em29::B0x1
    }
}
#[doc = "Field `EM29` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em29W<'a, REG> = crate::BitWriter<'a, REG, Em29>;
impl<'a, REG> Em29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em29::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em29::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em30 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em30> for bool {
    #[inline(always)]
    fn from(variant: Em30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM30` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em30R = crate::BitReader<Em30>;
impl Em30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em30 {
        match self.bits {
            false => Em30::B0x0,
            true => Em30::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em30::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em30::B0x1
    }
}
#[doc = "Field `EM30` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em30W<'a, REG> = crate::BitWriter<'a, REG, Em30>;
impl<'a, REG> Em30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em30::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em30::B0x1)
    }
}
#[doc = "CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em31 {
    #[doc = "0: wake-up with event generation masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with event generation unmasked"]
    B0x1 = 1,
}
impl From<Em31> for bool {
    #[inline(always)]
    fn from(variant: Em31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM31` reader - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em31R = crate::BitReader<Em31>;
impl Em31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em31 {
        match self.bits {
            false => Em31::B0x0,
            true => Em31::B0x1,
        }
    }
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Em31::B0x0
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Em31::B0x1
    }
}
#[doc = "Field `EM31` writer - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Em31W<'a, REG> = crate::BitWriter<'a, REG, Em31>;
impl<'a, REG> Em31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with event generation masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Em31::B0x0)
    }
    #[doc = "wake-up with event generation unmasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Em31::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em0(&self) -> Em0R {
        Em0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em1(&self) -> Em1R {
        Em1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em2(&self) -> Em2R {
        Em2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em3(&self) -> Em3R {
        Em3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em4(&self) -> Em4R {
        Em4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em5(&self) -> Em5R {
        Em5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em6(&self) -> Em6R {
        Em6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em7(&self) -> Em7R {
        Em7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em8(&self) -> Em8R {
        Em8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em9(&self) -> Em9R {
        Em9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em10(&self) -> Em10R {
        Em10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em11(&self) -> Em11R {
        Em11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em12(&self) -> Em12R {
        Em12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em13(&self) -> Em13R {
        Em13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em14(&self) -> Em14R {
        Em14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em15(&self) -> Em15R {
        Em15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em16(&self) -> Em16R {
        Em16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em17(&self) -> Em17R {
        Em17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em18(&self) -> Em18R {
        Em18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em19(&self) -> Em19R {
        Em19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em20(&self) -> Em20R {
        Em20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em21(&self) -> Em21R {
        Em21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em22(&self) -> Em22R {
        Em22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em23(&self) -> Em23R {
        Em23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em24(&self) -> Em24R {
        Em24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em25(&self) -> Em25R {
        Em25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em26(&self) -> Em26R {
        Em26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em27(&self) -> Em27R {
        Em27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em28(&self) -> Em28R {
        Em28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em29(&self) -> Em29R {
        Em29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em30(&self) -> Em30R {
        Em30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn em31(&self) -> Em31R {
        Em31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em0(&mut self) -> Em0W<ExtiEmr1Spec> {
        Em0W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em1(&mut self) -> Em1W<ExtiEmr1Spec> {
        Em1W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em2(&mut self) -> Em2W<ExtiEmr1Spec> {
        Em2W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em3(&mut self) -> Em3W<ExtiEmr1Spec> {
        Em3W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em4(&mut self) -> Em4W<ExtiEmr1Spec> {
        Em4W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em5(&mut self) -> Em5W<ExtiEmr1Spec> {
        Em5W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em6(&mut self) -> Em6W<ExtiEmr1Spec> {
        Em6W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em7(&mut self) -> Em7W<ExtiEmr1Spec> {
        Em7W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em8(&mut self) -> Em8W<ExtiEmr1Spec> {
        Em8W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em9(&mut self) -> Em9W<ExtiEmr1Spec> {
        Em9W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em10(&mut self) -> Em10W<ExtiEmr1Spec> {
        Em10W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em11(&mut self) -> Em11W<ExtiEmr1Spec> {
        Em11W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em12(&mut self) -> Em12W<ExtiEmr1Spec> {
        Em12W::new(self, 12)
    }
    #[doc = "Bit 13 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em13(&mut self) -> Em13W<ExtiEmr1Spec> {
        Em13W::new(self, 13)
    }
    #[doc = "Bit 14 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em14(&mut self) -> Em14W<ExtiEmr1Spec> {
        Em14W::new(self, 14)
    }
    #[doc = "Bit 15 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em15(&mut self) -> Em15W<ExtiEmr1Spec> {
        Em15W::new(self, 15)
    }
    #[doc = "Bit 16 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em16(&mut self) -> Em16W<ExtiEmr1Spec> {
        Em16W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em17(&mut self) -> Em17W<ExtiEmr1Spec> {
        Em17W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em18(&mut self) -> Em18W<ExtiEmr1Spec> {
        Em18W::new(self, 18)
    }
    #[doc = "Bit 19 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em19(&mut self) -> Em19W<ExtiEmr1Spec> {
        Em19W::new(self, 19)
    }
    #[doc = "Bit 20 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em20(&mut self) -> Em20W<ExtiEmr1Spec> {
        Em20W::new(self, 20)
    }
    #[doc = "Bit 21 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em21(&mut self) -> Em21W<ExtiEmr1Spec> {
        Em21W::new(self, 21)
    }
    #[doc = "Bit 22 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em22(&mut self) -> Em22W<ExtiEmr1Spec> {
        Em22W::new(self, 22)
    }
    #[doc = "Bit 23 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em23(&mut self) -> Em23W<ExtiEmr1Spec> {
        Em23W::new(self, 23)
    }
    #[doc = "Bit 24 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em24(&mut self) -> Em24W<ExtiEmr1Spec> {
        Em24W::new(self, 24)
    }
    #[doc = "Bit 25 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em25(&mut self) -> Em25W<ExtiEmr1Spec> {
        Em25W::new(self, 25)
    }
    #[doc = "Bit 26 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em26(&mut self) -> Em26W<ExtiEmr1Spec> {
        Em26W::new(self, 26)
    }
    #[doc = "Bit 27 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em27(&mut self) -> Em27W<ExtiEmr1Spec> {
        Em27W::new(self, 27)
    }
    #[doc = "Bit 28 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em28(&mut self) -> Em28W<ExtiEmr1Spec> {
        Em28W::new(self, 28)
    }
    #[doc = "Bit 29 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em29(&mut self) -> Em29W<ExtiEmr1Spec> {
        Em29W::new(self, 29)
    }
    #[doc = "Bit 30 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em30(&mut self) -> Em30W<ExtiEmr1Spec> {
        Em30W::new(self, 30)
    }
    #[doc = "Bit 31 - CPU wake-up with event generation mask on line x (x1=1311to10) Setting/clearing each bit unmasks/masks the CPU wake-up with event generation on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn em31(&mut self) -> Em31W<ExtiEmr1Spec> {
        Em31W::new(self, 31)
    }
}
#[doc = "EXTI CPU wake-up with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_emr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_emr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiEmr1Spec;
impl crate::RegisterSpec for ExtiEmr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_emr1::R`](R) reader structure"]
impl crate::Readable for ExtiEmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_emr1::W`](W) writer structure"]
impl crate::Writable for ExtiEmr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EMR1 to value 0"]
impl crate::Resettable for ExtiEmr1Spec {
    const RESET_VALUE: u32 = 0;
}
