#[doc = "Register `EXTI_FTSR1` reader"]
pub type R = crate::R<ExtiFtsr1Spec>;
#[doc = "Register `EXTI_FTSR1` writer"]
pub type W = crate::W<ExtiFtsr1Spec>;
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft0 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft0> for bool {
    #[inline(always)]
    fn from(variant: Ft0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT0` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft0R = crate::BitReader<Ft0>;
impl Ft0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft0 {
        match self.bits {
            false => Ft0::B0x0,
            true => Ft0::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft0::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft0::B0x1
    }
}
#[doc = "Field `FT0` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft0W<'a, REG> = crate::BitWriter<'a, REG, Ft0>;
impl<'a, REG> Ft0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft0::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft0::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft1 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft1> for bool {
    #[inline(always)]
    fn from(variant: Ft1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT1` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft1R = crate::BitReader<Ft1>;
impl Ft1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft1 {
        match self.bits {
            false => Ft1::B0x0,
            true => Ft1::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft1::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft1::B0x1
    }
}
#[doc = "Field `FT1` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft1W<'a, REG> = crate::BitWriter<'a, REG, Ft1>;
impl<'a, REG> Ft1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft1::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft1::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft2 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft2> for bool {
    #[inline(always)]
    fn from(variant: Ft2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT2` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft2R = crate::BitReader<Ft2>;
impl Ft2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft2 {
        match self.bits {
            false => Ft2::B0x0,
            true => Ft2::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft2::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft2::B0x1
    }
}
#[doc = "Field `FT2` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft2W<'a, REG> = crate::BitWriter<'a, REG, Ft2>;
impl<'a, REG> Ft2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft2::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft2::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft3 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft3> for bool {
    #[inline(always)]
    fn from(variant: Ft3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT3` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft3R = crate::BitReader<Ft3>;
impl Ft3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft3 {
        match self.bits {
            false => Ft3::B0x0,
            true => Ft3::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft3::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft3::B0x1
    }
}
#[doc = "Field `FT3` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft3W<'a, REG> = crate::BitWriter<'a, REG, Ft3>;
impl<'a, REG> Ft3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft3::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft3::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft4 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft4> for bool {
    #[inline(always)]
    fn from(variant: Ft4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT4` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft4R = crate::BitReader<Ft4>;
impl Ft4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft4 {
        match self.bits {
            false => Ft4::B0x0,
            true => Ft4::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft4::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft4::B0x1
    }
}
#[doc = "Field `FT4` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft4W<'a, REG> = crate::BitWriter<'a, REG, Ft4>;
impl<'a, REG> Ft4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft4::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft4::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft5 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft5> for bool {
    #[inline(always)]
    fn from(variant: Ft5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT5` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft5R = crate::BitReader<Ft5>;
impl Ft5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft5 {
        match self.bits {
            false => Ft5::B0x0,
            true => Ft5::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft5::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft5::B0x1
    }
}
#[doc = "Field `FT5` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft5W<'a, REG> = crate::BitWriter<'a, REG, Ft5>;
impl<'a, REG> Ft5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft5::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft5::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft6 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft6> for bool {
    #[inline(always)]
    fn from(variant: Ft6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT6` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft6R = crate::BitReader<Ft6>;
impl Ft6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft6 {
        match self.bits {
            false => Ft6::B0x0,
            true => Ft6::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft6::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft6::B0x1
    }
}
#[doc = "Field `FT6` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft6W<'a, REG> = crate::BitWriter<'a, REG, Ft6>;
impl<'a, REG> Ft6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft6::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft6::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft7 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft7> for bool {
    #[inline(always)]
    fn from(variant: Ft7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT7` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft7R = crate::BitReader<Ft7>;
impl Ft7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft7 {
        match self.bits {
            false => Ft7::B0x0,
            true => Ft7::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft7::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft7::B0x1
    }
}
#[doc = "Field `FT7` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft7W<'a, REG> = crate::BitWriter<'a, REG, Ft7>;
impl<'a, REG> Ft7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft7::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft7::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft8 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft8> for bool {
    #[inline(always)]
    fn from(variant: Ft8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT8` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft8R = crate::BitReader<Ft8>;
impl Ft8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft8 {
        match self.bits {
            false => Ft8::B0x0,
            true => Ft8::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft8::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft8::B0x1
    }
}
#[doc = "Field `FT8` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft8W<'a, REG> = crate::BitWriter<'a, REG, Ft8>;
impl<'a, REG> Ft8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft8::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft8::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft9 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft9> for bool {
    #[inline(always)]
    fn from(variant: Ft9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT9` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft9R = crate::BitReader<Ft9>;
impl Ft9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft9 {
        match self.bits {
            false => Ft9::B0x0,
            true => Ft9::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft9::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft9::B0x1
    }
}
#[doc = "Field `FT9` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft9W<'a, REG> = crate::BitWriter<'a, REG, Ft9>;
impl<'a, REG> Ft9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft9::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft9::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft10 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft10> for bool {
    #[inline(always)]
    fn from(variant: Ft10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT10` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft10R = crate::BitReader<Ft10>;
impl Ft10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft10 {
        match self.bits {
            false => Ft10::B0x0,
            true => Ft10::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft10::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft10::B0x1
    }
}
#[doc = "Field `FT10` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft10W<'a, REG> = crate::BitWriter<'a, REG, Ft10>;
impl<'a, REG> Ft10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft10::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft10::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft11 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft11> for bool {
    #[inline(always)]
    fn from(variant: Ft11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT11` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft11R = crate::BitReader<Ft11>;
impl Ft11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft11 {
        match self.bits {
            false => Ft11::B0x0,
            true => Ft11::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft11::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft11::B0x1
    }
}
#[doc = "Field `FT11` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft11W<'a, REG> = crate::BitWriter<'a, REG, Ft11>;
impl<'a, REG> Ft11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft11::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft11::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft12 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft12> for bool {
    #[inline(always)]
    fn from(variant: Ft12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT12` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft12R = crate::BitReader<Ft12>;
impl Ft12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft12 {
        match self.bits {
            false => Ft12::B0x0,
            true => Ft12::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft12::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft12::B0x1
    }
}
#[doc = "Field `FT12` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft12W<'a, REG> = crate::BitWriter<'a, REG, Ft12>;
impl<'a, REG> Ft12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft12::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft12::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft13 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft13> for bool {
    #[inline(always)]
    fn from(variant: Ft13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT13` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft13R = crate::BitReader<Ft13>;
impl Ft13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft13 {
        match self.bits {
            false => Ft13::B0x0,
            true => Ft13::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft13::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft13::B0x1
    }
}
#[doc = "Field `FT13` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft13W<'a, REG> = crate::BitWriter<'a, REG, Ft13>;
impl<'a, REG> Ft13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft13::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft13::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft14 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft14> for bool {
    #[inline(always)]
    fn from(variant: Ft14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT14` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft14R = crate::BitReader<Ft14>;
impl Ft14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft14 {
        match self.bits {
            false => Ft14::B0x0,
            true => Ft14::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft14::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft14::B0x1
    }
}
#[doc = "Field `FT14` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft14W<'a, REG> = crate::BitWriter<'a, REG, Ft14>;
impl<'a, REG> Ft14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft14::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft14::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft15 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft15> for bool {
    #[inline(always)]
    fn from(variant: Ft15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT15` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft15R = crate::BitReader<Ft15>;
impl Ft15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft15 {
        match self.bits {
            false => Ft15::B0x0,
            true => Ft15::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft15::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft15::B0x1
    }
}
#[doc = "Field `FT15` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft15W<'a, REG> = crate::BitWriter<'a, REG, Ft15>;
impl<'a, REG> Ft15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft15::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft15::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft16 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft16> for bool {
    #[inline(always)]
    fn from(variant: Ft16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT16` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft16R = crate::BitReader<Ft16>;
impl Ft16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft16 {
        match self.bits {
            false => Ft16::B0x0,
            true => Ft16::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft16::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft16::B0x1
    }
}
#[doc = "Field `FT16` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft16W<'a, REG> = crate::BitWriter<'a, REG, Ft16>;
impl<'a, REG> Ft16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft16::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft16::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft17 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft17> for bool {
    #[inline(always)]
    fn from(variant: Ft17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT17` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft17R = crate::BitReader<Ft17>;
impl Ft17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft17 {
        match self.bits {
            false => Ft17::B0x0,
            true => Ft17::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft17::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft17::B0x1
    }
}
#[doc = "Field `FT17` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft17W<'a, REG> = crate::BitWriter<'a, REG, Ft17>;
impl<'a, REG> Ft17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft17::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft17::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft18 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft18> for bool {
    #[inline(always)]
    fn from(variant: Ft18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT18` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft18R = crate::BitReader<Ft18>;
impl Ft18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft18 {
        match self.bits {
            false => Ft18::B0x0,
            true => Ft18::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft18::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft18::B0x1
    }
}
#[doc = "Field `FT18` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft18W<'a, REG> = crate::BitWriter<'a, REG, Ft18>;
impl<'a, REG> Ft18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft18::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft18::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft19 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft19> for bool {
    #[inline(always)]
    fn from(variant: Ft19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT19` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft19R = crate::BitReader<Ft19>;
impl Ft19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft19 {
        match self.bits {
            false => Ft19::B0x0,
            true => Ft19::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft19::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft19::B0x1
    }
}
#[doc = "Field `FT19` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft19W<'a, REG> = crate::BitWriter<'a, REG, Ft19>;
impl<'a, REG> Ft19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft19::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft19::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft20 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft20> for bool {
    #[inline(always)]
    fn from(variant: Ft20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT20` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft20R = crate::BitReader<Ft20>;
impl Ft20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft20 {
        match self.bits {
            false => Ft20::B0x0,
            true => Ft20::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft20::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft20::B0x1
    }
}
#[doc = "Field `FT20` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft20W<'a, REG> = crate::BitWriter<'a, REG, Ft20>;
impl<'a, REG> Ft20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft20::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft20::B0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft21 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Ft21> for bool {
    #[inline(always)]
    fn from(variant: Ft21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT21` reader - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft21R = crate::BitReader<Ft21>;
impl Ft21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft21 {
        match self.bits {
            false => Ft21::B0x0,
            true => Ft21::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ft21::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ft21::B0x1
    }
}
#[doc = "Field `FT21` writer - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Ft21W<'a, REG> = crate::BitWriter<'a, REG, Ft21>;
impl<'a, REG> Ft21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ft21::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ft21::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft0(&self) -> Ft0R {
        Ft0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft1(&self) -> Ft1R {
        Ft1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft2(&self) -> Ft2R {
        Ft2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft3(&self) -> Ft3R {
        Ft3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft4(&self) -> Ft4R {
        Ft4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft5(&self) -> Ft5R {
        Ft5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft6(&self) -> Ft6R {
        Ft6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft7(&self) -> Ft7R {
        Ft7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft8(&self) -> Ft8R {
        Ft8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft9(&self) -> Ft9R {
        Ft9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft10(&self) -> Ft10R {
        Ft10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft11(&self) -> Ft11R {
        Ft11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft12(&self) -> Ft12R {
        Ft12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft13(&self) -> Ft13R {
        Ft13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft14(&self) -> Ft14R {
        Ft14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft15(&self) -> Ft15R {
        Ft15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft16(&self) -> Ft16R {
        Ft16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft17(&self) -> Ft17R {
        Ft17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft18(&self) -> Ft18R {
        Ft18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft19(&self) -> Ft19R {
        Ft19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft20(&self) -> Ft20R {
        Ft20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn ft21(&self) -> Ft21R {
        Ft21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft0(&mut self) -> Ft0W<ExtiFtsr1Spec> {
        Ft0W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft1(&mut self) -> Ft1W<ExtiFtsr1Spec> {
        Ft1W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft2(&mut self) -> Ft2W<ExtiFtsr1Spec> {
        Ft2W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft3(&mut self) -> Ft3W<ExtiFtsr1Spec> {
        Ft3W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft4(&mut self) -> Ft4W<ExtiFtsr1Spec> {
        Ft4W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft5(&mut self) -> Ft5W<ExtiFtsr1Spec> {
        Ft5W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft6(&mut self) -> Ft6W<ExtiFtsr1Spec> {
        Ft6W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft7(&mut self) -> Ft7W<ExtiFtsr1Spec> {
        Ft7W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft8(&mut self) -> Ft8W<ExtiFtsr1Spec> {
        Ft8W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft9(&mut self) -> Ft9W<ExtiFtsr1Spec> {
        Ft9W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft10(&mut self) -> Ft10W<ExtiFtsr1Spec> {
        Ft10W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft11(&mut self) -> Ft11W<ExtiFtsr1Spec> {
        Ft11W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft12(&mut self) -> Ft12W<ExtiFtsr1Spec> {
        Ft12W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft13(&mut self) -> Ft13W<ExtiFtsr1Spec> {
        Ft13W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft14(&mut self) -> Ft14W<ExtiFtsr1Spec> {
        Ft14W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft15(&mut self) -> Ft15W<ExtiFtsr1Spec> {
        Ft15W::new(self, 15)
    }
    #[doc = "Bit 16 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft16(&mut self) -> Ft16W<ExtiFtsr1Spec> {
        Ft16W::new(self, 16)
    }
    #[doc = "Bit 17 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft17(&mut self) -> Ft17W<ExtiFtsr1Spec> {
        Ft17W::new(self, 17)
    }
    #[doc = "Bit 18 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft18(&mut self) -> Ft18W<ExtiFtsr1Spec> {
        Ft18W::new(self, 18)
    }
    #[doc = "Bit 19 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft19(&mut self) -> Ft19W<ExtiFtsr1Spec> {
        Ft19W::new(self, 19)
    }
    #[doc = "Bit 20 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft20(&mut self) -> Ft20W<ExtiFtsr1Spec> {
        Ft20W::new(self, 20)
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the falling edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn ft21(&mut self) -> Ft21W<ExtiFtsr1Spec> {
        Ft21W::new(self, 21)
    }
}
#[doc = "EXTI falling trigger selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ftsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_ftsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiFtsr1Spec;
impl crate::RegisterSpec for ExtiFtsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_ftsr1::R`](R) reader structure"]
impl crate::Readable for ExtiFtsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_ftsr1::W`](W) writer structure"]
impl crate::Writable for ExtiFtsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_FTSR1 to value 0"]
impl crate::Resettable for ExtiFtsr1Spec {
    const RESET_VALUE: u32 = 0;
}
