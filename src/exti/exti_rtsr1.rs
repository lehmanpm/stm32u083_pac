#[doc = "Register `EXTI_RTSR1` reader"]
pub type R = crate::R<ExtiRtsr1Spec>;
#[doc = "Register `EXTI_RTSR1` writer"]
pub type W = crate::W<ExtiRtsr1Spec>;
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt0 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt0> for bool {
    #[inline(always)]
    fn from(variant: Rt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT0` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt0R = crate::BitReader<Rt0>;
impl Rt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0 {
        match self.bits {
            false => Rt0::B0x0,
            true => Rt0::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt0::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt0::B0x1
    }
}
#[doc = "Field `RT0` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt0W<'a, REG> = crate::BitWriter<'a, REG, Rt0>;
impl<'a, REG> Rt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt1 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt1> for bool {
    #[inline(always)]
    fn from(variant: Rt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt1R = crate::BitReader<Rt1>;
impl Rt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1 {
        match self.bits {
            false => Rt1::B0x0,
            true => Rt1::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt1::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt1::B0x1
    }
}
#[doc = "Field `RT1` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt1W<'a, REG> = crate::BitWriter<'a, REG, Rt1>;
impl<'a, REG> Rt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt2 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt2> for bool {
    #[inline(always)]
    fn from(variant: Rt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT2` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt2R = crate::BitReader<Rt2>;
impl Rt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt2 {
        match self.bits {
            false => Rt2::B0x0,
            true => Rt2::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt2::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt2::B0x1
    }
}
#[doc = "Field `RT2` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt2W<'a, REG> = crate::BitWriter<'a, REG, Rt2>;
impl<'a, REG> Rt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt2::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt2::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt3 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt3> for bool {
    #[inline(always)]
    fn from(variant: Rt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT3` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt3R = crate::BitReader<Rt3>;
impl Rt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt3 {
        match self.bits {
            false => Rt3::B0x0,
            true => Rt3::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt3::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt3::B0x1
    }
}
#[doc = "Field `RT3` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt3W<'a, REG> = crate::BitWriter<'a, REG, Rt3>;
impl<'a, REG> Rt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt3::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt3::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt4 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt4> for bool {
    #[inline(always)]
    fn from(variant: Rt4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT4` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt4R = crate::BitReader<Rt4>;
impl Rt4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt4 {
        match self.bits {
            false => Rt4::B0x0,
            true => Rt4::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt4::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt4::B0x1
    }
}
#[doc = "Field `RT4` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt4W<'a, REG> = crate::BitWriter<'a, REG, Rt4>;
impl<'a, REG> Rt4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt4::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt4::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt5 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt5> for bool {
    #[inline(always)]
    fn from(variant: Rt5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT5` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt5R = crate::BitReader<Rt5>;
impl Rt5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt5 {
        match self.bits {
            false => Rt5::B0x0,
            true => Rt5::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt5::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt5::B0x1
    }
}
#[doc = "Field `RT5` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt5W<'a, REG> = crate::BitWriter<'a, REG, Rt5>;
impl<'a, REG> Rt5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt5::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt5::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt6 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt6> for bool {
    #[inline(always)]
    fn from(variant: Rt6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT6` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt6R = crate::BitReader<Rt6>;
impl Rt6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt6 {
        match self.bits {
            false => Rt6::B0x0,
            true => Rt6::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt6::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt6::B0x1
    }
}
#[doc = "Field `RT6` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt6W<'a, REG> = crate::BitWriter<'a, REG, Rt6>;
impl<'a, REG> Rt6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt6::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt6::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt7 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt7> for bool {
    #[inline(always)]
    fn from(variant: Rt7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT7` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt7R = crate::BitReader<Rt7>;
impl Rt7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt7 {
        match self.bits {
            false => Rt7::B0x0,
            true => Rt7::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt7::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt7::B0x1
    }
}
#[doc = "Field `RT7` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt7W<'a, REG> = crate::BitWriter<'a, REG, Rt7>;
impl<'a, REG> Rt7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt7::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt7::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt8 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt8> for bool {
    #[inline(always)]
    fn from(variant: Rt8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT8` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt8R = crate::BitReader<Rt8>;
impl Rt8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt8 {
        match self.bits {
            false => Rt8::B0x0,
            true => Rt8::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt8::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt8::B0x1
    }
}
#[doc = "Field `RT8` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt8W<'a, REG> = crate::BitWriter<'a, REG, Rt8>;
impl<'a, REG> Rt8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt8::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt8::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt9 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt9> for bool {
    #[inline(always)]
    fn from(variant: Rt9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT9` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt9R = crate::BitReader<Rt9>;
impl Rt9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt9 {
        match self.bits {
            false => Rt9::B0x0,
            true => Rt9::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt9::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt9::B0x1
    }
}
#[doc = "Field `RT9` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt9W<'a, REG> = crate::BitWriter<'a, REG, Rt9>;
impl<'a, REG> Rt9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt9::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt9::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt10 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt10> for bool {
    #[inline(always)]
    fn from(variant: Rt10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT10` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt10R = crate::BitReader<Rt10>;
impl Rt10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt10 {
        match self.bits {
            false => Rt10::B0x0,
            true => Rt10::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt10::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt10::B0x1
    }
}
#[doc = "Field `RT10` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt10W<'a, REG> = crate::BitWriter<'a, REG, Rt10>;
impl<'a, REG> Rt10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt10::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt10::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt11 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt11> for bool {
    #[inline(always)]
    fn from(variant: Rt11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT11` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt11R = crate::BitReader<Rt11>;
impl Rt11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt11 {
        match self.bits {
            false => Rt11::B0x0,
            true => Rt11::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt11::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt11::B0x1
    }
}
#[doc = "Field `RT11` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt11W<'a, REG> = crate::BitWriter<'a, REG, Rt11>;
impl<'a, REG> Rt11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt11::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt11::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt12 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt12> for bool {
    #[inline(always)]
    fn from(variant: Rt12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT12` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt12R = crate::BitReader<Rt12>;
impl Rt12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt12 {
        match self.bits {
            false => Rt12::B0x0,
            true => Rt12::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt12::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt12::B0x1
    }
}
#[doc = "Field `RT12` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt12W<'a, REG> = crate::BitWriter<'a, REG, Rt12>;
impl<'a, REG> Rt12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt12::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt12::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt13 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt13> for bool {
    #[inline(always)]
    fn from(variant: Rt13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT13` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt13R = crate::BitReader<Rt13>;
impl Rt13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt13 {
        match self.bits {
            false => Rt13::B0x0,
            true => Rt13::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt13::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt13::B0x1
    }
}
#[doc = "Field `RT13` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt13W<'a, REG> = crate::BitWriter<'a, REG, Rt13>;
impl<'a, REG> Rt13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt13::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt13::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt14 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt14> for bool {
    #[inline(always)]
    fn from(variant: Rt14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT14` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt14R = crate::BitReader<Rt14>;
impl Rt14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt14 {
        match self.bits {
            false => Rt14::B0x0,
            true => Rt14::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt14::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt14::B0x1
    }
}
#[doc = "Field `RT14` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt14W<'a, REG> = crate::BitWriter<'a, REG, Rt14>;
impl<'a, REG> Rt14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt14::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt14::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt15 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt15> for bool {
    #[inline(always)]
    fn from(variant: Rt15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT15` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt15R = crate::BitReader<Rt15>;
impl Rt15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt15 {
        match self.bits {
            false => Rt15::B0x0,
            true => Rt15::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt15::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt15::B0x1
    }
}
#[doc = "Field `RT15` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt15W<'a, REG> = crate::BitWriter<'a, REG, Rt15>;
impl<'a, REG> Rt15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt15::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt15::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt16 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt16> for bool {
    #[inline(always)]
    fn from(variant: Rt16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT16` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt16R = crate::BitReader<Rt16>;
impl Rt16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt16 {
        match self.bits {
            false => Rt16::B0x0,
            true => Rt16::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt16::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt16::B0x1
    }
}
#[doc = "Field `RT16` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt16W<'a, REG> = crate::BitWriter<'a, REG, Rt16>;
impl<'a, REG> Rt16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt16::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt16::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt17 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt17> for bool {
    #[inline(always)]
    fn from(variant: Rt17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT17` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt17R = crate::BitReader<Rt17>;
impl Rt17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt17 {
        match self.bits {
            false => Rt17::B0x0,
            true => Rt17::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt17::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt17::B0x1
    }
}
#[doc = "Field `RT17` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt17W<'a, REG> = crate::BitWriter<'a, REG, Rt17>;
impl<'a, REG> Rt17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt17::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt17::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt18 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt18> for bool {
    #[inline(always)]
    fn from(variant: Rt18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT18` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt18R = crate::BitReader<Rt18>;
impl Rt18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt18 {
        match self.bits {
            false => Rt18::B0x0,
            true => Rt18::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt18::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt18::B0x1
    }
}
#[doc = "Field `RT18` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt18W<'a, REG> = crate::BitWriter<'a, REG, Rt18>;
impl<'a, REG> Rt18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt18::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt18::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt19 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt19> for bool {
    #[inline(always)]
    fn from(variant: Rt19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT19` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt19R = crate::BitReader<Rt19>;
impl Rt19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt19 {
        match self.bits {
            false => Rt19::B0x0,
            true => Rt19::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt19::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt19::B0x1
    }
}
#[doc = "Field `RT19` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt19W<'a, REG> = crate::BitWriter<'a, REG, Rt19>;
impl<'a, REG> Rt19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt19::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt19::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt20 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt20> for bool {
    #[inline(always)]
    fn from(variant: Rt20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT20` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt20R = crate::BitReader<Rt20>;
impl Rt20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt20 {
        match self.bits {
            false => Rt20::B0x0,
            true => Rt20::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt20::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt20::B0x1
    }
}
#[doc = "Field `RT20` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt20W<'a, REG> = crate::BitWriter<'a, REG, Rt20>;
impl<'a, REG> Rt20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt20::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt20::B0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt21 {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rt21> for bool {
    #[inline(always)]
    fn from(variant: Rt21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT21` reader - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt21R = crate::BitReader<Rt21>;
impl Rt21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt21 {
        match self.bits {
            false => Rt21::B0x0,
            true => Rt21::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rt21::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rt21::B0x1
    }
}
#[doc = "Field `RT21` writer - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Rt21W<'a, REG> = crate::BitWriter<'a, REG, Rt21>;
impl<'a, REG> Rt21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt21::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt21::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt0(&self) -> Rt0R {
        Rt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt1(&self) -> Rt1R {
        Rt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt2(&self) -> Rt2R {
        Rt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt3(&self) -> Rt3R {
        Rt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt4(&self) -> Rt4R {
        Rt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt5(&self) -> Rt5R {
        Rt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt6(&self) -> Rt6R {
        Rt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt7(&self) -> Rt7R {
        Rt7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt8(&self) -> Rt8R {
        Rt8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt9(&self) -> Rt9R {
        Rt9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt10(&self) -> Rt10R {
        Rt10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt11(&self) -> Rt11R {
        Rt11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt12(&self) -> Rt12R {
        Rt12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt13(&self) -> Rt13R {
        Rt13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt14(&self) -> Rt14R {
        Rt14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt15(&self) -> Rt15R {
        Rt15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt16(&self) -> Rt16R {
        Rt16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt17(&self) -> Rt17R {
        Rt17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt18(&self) -> Rt18R {
        Rt18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt19(&self) -> Rt19R {
        Rt19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt20(&self) -> Rt20R {
        Rt20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn rt21(&self) -> Rt21R {
        Rt21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt0(&mut self) -> Rt0W<ExtiRtsr1Spec> {
        Rt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt1(&mut self) -> Rt1W<ExtiRtsr1Spec> {
        Rt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt2(&mut self) -> Rt2W<ExtiRtsr1Spec> {
        Rt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt3(&mut self) -> Rt3W<ExtiRtsr1Spec> {
        Rt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt4(&mut self) -> Rt4W<ExtiRtsr1Spec> {
        Rt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt5(&mut self) -> Rt5W<ExtiRtsr1Spec> {
        Rt5W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt6(&mut self) -> Rt6W<ExtiRtsr1Spec> {
        Rt6W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt7(&mut self) -> Rt7W<ExtiRtsr1Spec> {
        Rt7W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt8(&mut self) -> Rt8W<ExtiRtsr1Spec> {
        Rt8W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt9(&mut self) -> Rt9W<ExtiRtsr1Spec> {
        Rt9W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt10(&mut self) -> Rt10W<ExtiRtsr1Spec> {
        Rt10W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt11(&mut self) -> Rt11W<ExtiRtsr1Spec> {
        Rt11W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt12(&mut self) -> Rt12W<ExtiRtsr1Spec> {
        Rt12W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt13(&mut self) -> Rt13W<ExtiRtsr1Spec> {
        Rt13W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt14(&mut self) -> Rt14W<ExtiRtsr1Spec> {
        Rt14W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt15(&mut self) -> Rt15W<ExtiRtsr1Spec> {
        Rt15W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt16(&mut self) -> Rt16W<ExtiRtsr1Spec> {
        Rt16W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt17(&mut self) -> Rt17W<ExtiRtsr1Spec> {
        Rt17W::new(self, 17)
    }
    #[doc = "Bit 18 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt18(&mut self) -> Rt18W<ExtiRtsr1Spec> {
        Rt18W::new(self, 18)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt19(&mut self) -> Rt19W<ExtiRtsr1Spec> {
        Rt19W::new(self, 19)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt20(&mut self) -> Rt20W<ExtiRtsr1Spec> {
        Rt20W::new(self, 20)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of configurable line x (x1=1211to10) Each bit enables/disables the rising edge trigger for the event and interrupt on the corresponding line. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn rt21(&mut self) -> Rt21W<ExtiRtsr1Spec> {
        Rt21W::new(self, 21)
    }
}
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rtsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rtsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiRtsr1Spec;
impl crate::RegisterSpec for ExtiRtsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_rtsr1::R`](R) reader structure"]
impl crate::Readable for ExtiRtsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_rtsr1::W`](W) writer structure"]
impl crate::Writable for ExtiRtsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_RTSR1 to value 0"]
impl crate::Resettable for ExtiRtsr1Spec {
    const RESET_VALUE: u32 = 0;
}
