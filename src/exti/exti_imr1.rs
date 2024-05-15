#[doc = "Register `EXTI_IMR1` reader"]
pub type R = crate::R<ExtiImr1Spec>;
#[doc = "Register `EXTI_IMR1` writer"]
pub type W = crate::W<ExtiImr1Spec>;
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im0 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im0> for bool {
    #[inline(always)]
    fn from(variant: Im0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM0` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im0R = crate::BitReader<Im0>;
impl Im0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im0 {
        match self.bits {
            false => Im0::B0x0,
            true => Im0::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im0::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im0::B0x1
    }
}
#[doc = "Field `IM0` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im0W<'a, REG> = crate::BitWriter<'a, REG, Im0>;
impl<'a, REG> Im0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im0::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im0::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im1 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im1> for bool {
    #[inline(always)]
    fn from(variant: Im1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM1` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im1R = crate::BitReader<Im1>;
impl Im1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im1 {
        match self.bits {
            false => Im1::B0x0,
            true => Im1::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im1::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im1::B0x1
    }
}
#[doc = "Field `IM1` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im1W<'a, REG> = crate::BitWriter<'a, REG, Im1>;
impl<'a, REG> Im1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im1::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im1::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im2 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im2> for bool {
    #[inline(always)]
    fn from(variant: Im2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM2` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im2R = crate::BitReader<Im2>;
impl Im2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im2 {
        match self.bits {
            false => Im2::B0x0,
            true => Im2::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im2::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im2::B0x1
    }
}
#[doc = "Field `IM2` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im2W<'a, REG> = crate::BitWriter<'a, REG, Im2>;
impl<'a, REG> Im2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im2::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im2::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im3 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im3> for bool {
    #[inline(always)]
    fn from(variant: Im3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM3` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im3R = crate::BitReader<Im3>;
impl Im3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im3 {
        match self.bits {
            false => Im3::B0x0,
            true => Im3::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im3::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im3::B0x1
    }
}
#[doc = "Field `IM3` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im3W<'a, REG> = crate::BitWriter<'a, REG, Im3>;
impl<'a, REG> Im3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im3::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im3::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im4 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im4> for bool {
    #[inline(always)]
    fn from(variant: Im4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM4` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im4R = crate::BitReader<Im4>;
impl Im4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im4 {
        match self.bits {
            false => Im4::B0x0,
            true => Im4::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im4::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im4::B0x1
    }
}
#[doc = "Field `IM4` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im4W<'a, REG> = crate::BitWriter<'a, REG, Im4>;
impl<'a, REG> Im4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im4::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im4::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im5 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im5> for bool {
    #[inline(always)]
    fn from(variant: Im5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM5` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im5R = crate::BitReader<Im5>;
impl Im5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im5 {
        match self.bits {
            false => Im5::B0x0,
            true => Im5::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im5::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im5::B0x1
    }
}
#[doc = "Field `IM5` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im5W<'a, REG> = crate::BitWriter<'a, REG, Im5>;
impl<'a, REG> Im5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im5::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im5::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im6 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im6> for bool {
    #[inline(always)]
    fn from(variant: Im6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM6` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im6R = crate::BitReader<Im6>;
impl Im6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im6 {
        match self.bits {
            false => Im6::B0x0,
            true => Im6::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im6::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im6::B0x1
    }
}
#[doc = "Field `IM6` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im6W<'a, REG> = crate::BitWriter<'a, REG, Im6>;
impl<'a, REG> Im6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im6::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im6::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im7 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im7> for bool {
    #[inline(always)]
    fn from(variant: Im7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM7` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im7R = crate::BitReader<Im7>;
impl Im7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im7 {
        match self.bits {
            false => Im7::B0x0,
            true => Im7::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im7::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im7::B0x1
    }
}
#[doc = "Field `IM7` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im7W<'a, REG> = crate::BitWriter<'a, REG, Im7>;
impl<'a, REG> Im7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im7::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im7::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im8 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im8> for bool {
    #[inline(always)]
    fn from(variant: Im8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM8` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im8R = crate::BitReader<Im8>;
impl Im8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im8 {
        match self.bits {
            false => Im8::B0x0,
            true => Im8::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im8::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im8::B0x1
    }
}
#[doc = "Field `IM8` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im8W<'a, REG> = crate::BitWriter<'a, REG, Im8>;
impl<'a, REG> Im8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im8::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im8::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im9 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im9> for bool {
    #[inline(always)]
    fn from(variant: Im9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM9` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im9R = crate::BitReader<Im9>;
impl Im9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im9 {
        match self.bits {
            false => Im9::B0x0,
            true => Im9::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im9::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im9::B0x1
    }
}
#[doc = "Field `IM9` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im9W<'a, REG> = crate::BitWriter<'a, REG, Im9>;
impl<'a, REG> Im9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im9::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im9::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im10 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im10> for bool {
    #[inline(always)]
    fn from(variant: Im10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM10` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im10R = crate::BitReader<Im10>;
impl Im10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im10 {
        match self.bits {
            false => Im10::B0x0,
            true => Im10::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im10::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im10::B0x1
    }
}
#[doc = "Field `IM10` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im10W<'a, REG> = crate::BitWriter<'a, REG, Im10>;
impl<'a, REG> Im10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im10::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im10::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im11 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im11> for bool {
    #[inline(always)]
    fn from(variant: Im11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM11` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im11R = crate::BitReader<Im11>;
impl Im11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im11 {
        match self.bits {
            false => Im11::B0x0,
            true => Im11::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im11::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im11::B0x1
    }
}
#[doc = "Field `IM11` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im11W<'a, REG> = crate::BitWriter<'a, REG, Im11>;
impl<'a, REG> Im11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im11::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im11::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im12 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im12> for bool {
    #[inline(always)]
    fn from(variant: Im12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM12` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im12R = crate::BitReader<Im12>;
impl Im12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im12 {
        match self.bits {
            false => Im12::B0x0,
            true => Im12::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im12::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im12::B0x1
    }
}
#[doc = "Field `IM12` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im12W<'a, REG> = crate::BitWriter<'a, REG, Im12>;
impl<'a, REG> Im12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im12::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im12::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im13 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im13> for bool {
    #[inline(always)]
    fn from(variant: Im13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM13` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im13R = crate::BitReader<Im13>;
impl Im13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im13 {
        match self.bits {
            false => Im13::B0x0,
            true => Im13::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im13::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im13::B0x1
    }
}
#[doc = "Field `IM13` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im13W<'a, REG> = crate::BitWriter<'a, REG, Im13>;
impl<'a, REG> Im13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im13::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im13::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im14 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im14> for bool {
    #[inline(always)]
    fn from(variant: Im14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM14` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im14R = crate::BitReader<Im14>;
impl Im14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im14 {
        match self.bits {
            false => Im14::B0x0,
            true => Im14::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im14::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im14::B0x1
    }
}
#[doc = "Field `IM14` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im14W<'a, REG> = crate::BitWriter<'a, REG, Im14>;
impl<'a, REG> Im14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im14::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im14::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im15 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im15> for bool {
    #[inline(always)]
    fn from(variant: Im15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM15` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im15R = crate::BitReader<Im15>;
impl Im15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im15 {
        match self.bits {
            false => Im15::B0x0,
            true => Im15::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im15::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im15::B0x1
    }
}
#[doc = "Field `IM15` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im15W<'a, REG> = crate::BitWriter<'a, REG, Im15>;
impl<'a, REG> Im15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im15::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im15::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im16 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im16> for bool {
    #[inline(always)]
    fn from(variant: Im16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM16` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im16R = crate::BitReader<Im16>;
impl Im16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im16 {
        match self.bits {
            false => Im16::B0x0,
            true => Im16::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im16::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im16::B0x1
    }
}
#[doc = "Field `IM16` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im16W<'a, REG> = crate::BitWriter<'a, REG, Im16>;
impl<'a, REG> Im16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im16::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im16::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im17 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im17> for bool {
    #[inline(always)]
    fn from(variant: Im17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM17` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im17R = crate::BitReader<Im17>;
impl Im17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im17 {
        match self.bits {
            false => Im17::B0x0,
            true => Im17::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im17::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im17::B0x1
    }
}
#[doc = "Field `IM17` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im17W<'a, REG> = crate::BitWriter<'a, REG, Im17>;
impl<'a, REG> Im17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im17::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im17::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im18 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im18> for bool {
    #[inline(always)]
    fn from(variant: Im18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM18` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im18R = crate::BitReader<Im18>;
impl Im18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im18 {
        match self.bits {
            false => Im18::B0x0,
            true => Im18::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im18::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im18::B0x1
    }
}
#[doc = "Field `IM18` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im18W<'a, REG> = crate::BitWriter<'a, REG, Im18>;
impl<'a, REG> Im18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im18::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im18::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im19 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im19> for bool {
    #[inline(always)]
    fn from(variant: Im19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM19` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im19R = crate::BitReader<Im19>;
impl Im19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im19 {
        match self.bits {
            false => Im19::B0x0,
            true => Im19::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im19::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im19::B0x1
    }
}
#[doc = "Field `IM19` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im19W<'a, REG> = crate::BitWriter<'a, REG, Im19>;
impl<'a, REG> Im19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im19::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im19::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im20 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im20> for bool {
    #[inline(always)]
    fn from(variant: Im20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM20` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im20R = crate::BitReader<Im20>;
impl Im20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im20 {
        match self.bits {
            false => Im20::B0x0,
            true => Im20::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im20::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im20::B0x1
    }
}
#[doc = "Field `IM20` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im20W<'a, REG> = crate::BitWriter<'a, REG, Im20>;
impl<'a, REG> Im20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im20::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im20::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im21 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im21> for bool {
    #[inline(always)]
    fn from(variant: Im21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM21` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im21R = crate::BitReader<Im21>;
impl Im21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im21 {
        match self.bits {
            false => Im21::B0x0,
            true => Im21::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im21::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im21::B0x1
    }
}
#[doc = "Field `IM21` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im21W<'a, REG> = crate::BitWriter<'a, REG, Im21>;
impl<'a, REG> Im21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im21::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im21::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im22 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im22> for bool {
    #[inline(always)]
    fn from(variant: Im22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM22` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im22R = crate::BitReader<Im22>;
impl Im22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im22 {
        match self.bits {
            false => Im22::B0x0,
            true => Im22::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im22::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im22::B0x1
    }
}
#[doc = "Field `IM22` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im22W<'a, REG> = crate::BitWriter<'a, REG, Im22>;
impl<'a, REG> Im22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im22::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im22::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im23 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im23> for bool {
    #[inline(always)]
    fn from(variant: Im23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM23` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im23R = crate::BitReader<Im23>;
impl Im23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im23 {
        match self.bits {
            false => Im23::B0x0,
            true => Im23::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im23::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im23::B0x1
    }
}
#[doc = "Field `IM23` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im23W<'a, REG> = crate::BitWriter<'a, REG, Im23>;
impl<'a, REG> Im23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im23::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im23::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im24 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im24> for bool {
    #[inline(always)]
    fn from(variant: Im24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM24` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im24R = crate::BitReader<Im24>;
impl Im24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im24 {
        match self.bits {
            false => Im24::B0x0,
            true => Im24::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im24::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im24::B0x1
    }
}
#[doc = "Field `IM24` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im24W<'a, REG> = crate::BitWriter<'a, REG, Im24>;
impl<'a, REG> Im24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im24::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im24::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im25 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im25> for bool {
    #[inline(always)]
    fn from(variant: Im25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM25` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im25R = crate::BitReader<Im25>;
impl Im25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im25 {
        match self.bits {
            false => Im25::B0x0,
            true => Im25::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im25::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im25::B0x1
    }
}
#[doc = "Field `IM25` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im25W<'a, REG> = crate::BitWriter<'a, REG, Im25>;
impl<'a, REG> Im25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im25::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im25::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im26 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im26> for bool {
    #[inline(always)]
    fn from(variant: Im26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM26` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im26R = crate::BitReader<Im26>;
impl Im26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im26 {
        match self.bits {
            false => Im26::B0x0,
            true => Im26::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im26::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im26::B0x1
    }
}
#[doc = "Field `IM26` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im26W<'a, REG> = crate::BitWriter<'a, REG, Im26>;
impl<'a, REG> Im26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im26::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im26::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im27 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im27> for bool {
    #[inline(always)]
    fn from(variant: Im27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM27` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im27R = crate::BitReader<Im27>;
impl Im27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im27 {
        match self.bits {
            false => Im27::B0x0,
            true => Im27::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im27::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im27::B0x1
    }
}
#[doc = "Field `IM27` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im27W<'a, REG> = crate::BitWriter<'a, REG, Im27>;
impl<'a, REG> Im27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im27::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im27::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im28 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im28> for bool {
    #[inline(always)]
    fn from(variant: Im28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM28` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im28R = crate::BitReader<Im28>;
impl Im28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im28 {
        match self.bits {
            false => Im28::B0x0,
            true => Im28::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im28::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im28::B0x1
    }
}
#[doc = "Field `IM28` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im28W<'a, REG> = crate::BitWriter<'a, REG, Im28>;
impl<'a, REG> Im28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im28::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im28::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im29 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im29> for bool {
    #[inline(always)]
    fn from(variant: Im29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM29` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im29R = crate::BitReader<Im29>;
impl Im29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im29 {
        match self.bits {
            false => Im29::B0x0,
            true => Im29::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im29::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im29::B0x1
    }
}
#[doc = "Field `IM29` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im29W<'a, REG> = crate::BitWriter<'a, REG, Im29>;
impl<'a, REG> Im29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im29::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im29::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im30 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im30> for bool {
    #[inline(always)]
    fn from(variant: Im30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM30` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im30R = crate::BitReader<Im30>;
impl Im30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im30 {
        match self.bits {
            false => Im30::B0x0,
            true => Im30::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im30::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im30::B0x1
    }
}
#[doc = "Field `IM30` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im30W<'a, REG> = crate::BitWriter<'a, REG, Im30>;
impl<'a, REG> Im30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im30::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im30::B0x1)
    }
}
#[doc = "CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im31 {
    #[doc = "0: wake-up with interrupt masked"]
    B0x0 = 0,
    #[doc = "1: wake-up with interrupt unasked"]
    B0x1 = 1,
}
impl From<Im31> for bool {
    #[inline(always)]
    fn from(variant: Im31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM31` reader - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im31R = crate::BitReader<Im31>;
impl Im31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im31 {
        match self.bits {
            false => Im31::B0x0,
            true => Im31::B0x1,
        }
    }
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Im31::B0x0
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Im31::B0x1
    }
}
#[doc = "Field `IM31` writer - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
pub type Im31W<'a, REG> = crate::BitWriter<'a, REG, Im31>;
impl<'a, REG> Im31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wake-up with interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Im31::B0x0)
    }
    #[doc = "wake-up with interrupt unasked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Im31::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im0(&self) -> Im0R {
        Im0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im1(&self) -> Im1R {
        Im1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im2(&self) -> Im2R {
        Im2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im3(&self) -> Im3R {
        Im3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im4(&self) -> Im4R {
        Im4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im5(&self) -> Im5R {
        Im5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im6(&self) -> Im6R {
        Im6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im7(&self) -> Im7R {
        Im7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im8(&self) -> Im8R {
        Im8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im9(&self) -> Im9R {
        Im9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im10(&self) -> Im10R {
        Im10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im11(&self) -> Im11R {
        Im11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im12(&self) -> Im12R {
        Im12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im13(&self) -> Im13R {
        Im13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im14(&self) -> Im14R {
        Im14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im15(&self) -> Im15R {
        Im15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im16(&self) -> Im16R {
        Im16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im17(&self) -> Im17R {
        Im17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im18(&self) -> Im18R {
        Im18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im19(&self) -> Im19R {
        Im19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im20(&self) -> Im20R {
        Im20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im21(&self) -> Im21R {
        Im21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im22(&self) -> Im22R {
        Im22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im23(&self) -> Im23R {
        Im23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im24(&self) -> Im24R {
        Im24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im25(&self) -> Im25R {
        Im25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im26(&self) -> Im26R {
        Im26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im27(&self) -> Im27R {
        Im27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im28(&self) -> Im28R {
        Im28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im29(&self) -> Im29R {
        Im29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im30(&self) -> Im30R {
        Im30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn im31(&self) -> Im31R {
        Im31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im0(&mut self) -> Im0W<ExtiImr1Spec> {
        Im0W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im1(&mut self) -> Im1W<ExtiImr1Spec> {
        Im1W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im2(&mut self) -> Im2W<ExtiImr1Spec> {
        Im2W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im3(&mut self) -> Im3W<ExtiImr1Spec> {
        Im3W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im4(&mut self) -> Im4W<ExtiImr1Spec> {
        Im4W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im5(&mut self) -> Im5W<ExtiImr1Spec> {
        Im5W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im6(&mut self) -> Im6W<ExtiImr1Spec> {
        Im6W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im7(&mut self) -> Im7W<ExtiImr1Spec> {
        Im7W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im8(&mut self) -> Im8W<ExtiImr1Spec> {
        Im8W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im9(&mut self) -> Im9W<ExtiImr1Spec> {
        Im9W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im10(&mut self) -> Im10W<ExtiImr1Spec> {
        Im10W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im11(&mut self) -> Im11W<ExtiImr1Spec> {
        Im11W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im12(&mut self) -> Im12W<ExtiImr1Spec> {
        Im12W::new(self, 12)
    }
    #[doc = "Bit 13 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im13(&mut self) -> Im13W<ExtiImr1Spec> {
        Im13W::new(self, 13)
    }
    #[doc = "Bit 14 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im14(&mut self) -> Im14W<ExtiImr1Spec> {
        Im14W::new(self, 14)
    }
    #[doc = "Bit 15 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im15(&mut self) -> Im15W<ExtiImr1Spec> {
        Im15W::new(self, 15)
    }
    #[doc = "Bit 16 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im16(&mut self) -> Im16W<ExtiImr1Spec> {
        Im16W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im17(&mut self) -> Im17W<ExtiImr1Spec> {
        Im17W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im18(&mut self) -> Im18W<ExtiImr1Spec> {
        Im18W::new(self, 18)
    }
    #[doc = "Bit 19 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im19(&mut self) -> Im19W<ExtiImr1Spec> {
        Im19W::new(self, 19)
    }
    #[doc = "Bit 20 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im20(&mut self) -> Im20W<ExtiImr1Spec> {
        Im20W::new(self, 20)
    }
    #[doc = "Bit 21 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im21(&mut self) -> Im21W<ExtiImr1Spec> {
        Im21W::new(self, 21)
    }
    #[doc = "Bit 22 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im22(&mut self) -> Im22W<ExtiImr1Spec> {
        Im22W::new(self, 22)
    }
    #[doc = "Bit 23 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im23(&mut self) -> Im23W<ExtiImr1Spec> {
        Im23W::new(self, 23)
    }
    #[doc = "Bit 24 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im24(&mut self) -> Im24W<ExtiImr1Spec> {
        Im24W::new(self, 24)
    }
    #[doc = "Bit 25 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im25(&mut self) -> Im25W<ExtiImr1Spec> {
        Im25W::new(self, 25)
    }
    #[doc = "Bit 26 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im26(&mut self) -> Im26W<ExtiImr1Spec> {
        Im26W::new(self, 26)
    }
    #[doc = "Bit 27 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im27(&mut self) -> Im27W<ExtiImr1Spec> {
        Im27W::new(self, 27)
    }
    #[doc = "Bit 28 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im28(&mut self) -> Im28W<ExtiImr1Spec> {
        Im28W::new(self, 28)
    }
    #[doc = "Bit 29 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im29(&mut self) -> Im29W<ExtiImr1Spec> {
        Im29W::new(self, 29)
    }
    #[doc = "Bit 30 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im30(&mut self) -> Im30W<ExtiImr1Spec> {
        Im30W::new(self, 30)
    }
    #[doc = "Bit 31 - CPU wake-up with interrupt mask on line x (x1=131 to 0) Setting/clearing each bit unmasks/masks the CPU wake-up with interrupt, by an event on the corresponding line. Bits 18, 19, 22 and 26 are available only on STM32U0x3xx devices, they are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn im31(&mut self) -> Im31W<ExtiImr1Spec> {
        Im31W::new(self, 31)
    }
}
#[doc = "EXTI CPU wake-up with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_imr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_imr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiImr1Spec;
impl crate::RegisterSpec for ExtiImr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_imr1::R`](R) reader structure"]
impl crate::Readable for ExtiImr1Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_imr1::W`](W) writer structure"]
impl crate::Writable for ExtiImr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_IMR1 to value 0xfff8_0000"]
impl crate::Resettable for ExtiImr1Spec {
    const RESET_VALUE: u32 = 0xfff8_0000;
}
