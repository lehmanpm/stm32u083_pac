#[doc = "Register `EXTI_SWIER1` reader"]
pub type R = crate::R<ExtiSwier1Spec>;
#[doc = "Register `EXTI_SWIER1` writer"]
pub type W = crate::W<ExtiSwier1Spec>;
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi0 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi0> for bool {
    #[inline(always)]
    fn from(variant: Swi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI0` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi0R = crate::BitReader<Swi0>;
impl Swi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi0 {
        match self.bits {
            false => Swi0::B0x0,
            true => Swi0::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi0::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi0::B0x1
    }
}
#[doc = "Field `SWI0` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi0W<'a, REG> = crate::BitWriter<'a, REG, Swi0>;
impl<'a, REG> Swi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi0::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi0::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi1 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi1> for bool {
    #[inline(always)]
    fn from(variant: Swi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI1` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi1R = crate::BitReader<Swi1>;
impl Swi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi1 {
        match self.bits {
            false => Swi1::B0x0,
            true => Swi1::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi1::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi1::B0x1
    }
}
#[doc = "Field `SWI1` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi1W<'a, REG> = crate::BitWriter<'a, REG, Swi1>;
impl<'a, REG> Swi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi1::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi1::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi2 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi2> for bool {
    #[inline(always)]
    fn from(variant: Swi2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI2` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi2R = crate::BitReader<Swi2>;
impl Swi2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi2 {
        match self.bits {
            false => Swi2::B0x0,
            true => Swi2::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi2::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi2::B0x1
    }
}
#[doc = "Field `SWI2` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi2W<'a, REG> = crate::BitWriter<'a, REG, Swi2>;
impl<'a, REG> Swi2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi2::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi2::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi3 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi3> for bool {
    #[inline(always)]
    fn from(variant: Swi3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI3` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi3R = crate::BitReader<Swi3>;
impl Swi3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi3 {
        match self.bits {
            false => Swi3::B0x0,
            true => Swi3::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi3::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi3::B0x1
    }
}
#[doc = "Field `SWI3` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi3W<'a, REG> = crate::BitWriter<'a, REG, Swi3>;
impl<'a, REG> Swi3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi3::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi3::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi4 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi4> for bool {
    #[inline(always)]
    fn from(variant: Swi4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI4` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi4R = crate::BitReader<Swi4>;
impl Swi4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi4 {
        match self.bits {
            false => Swi4::B0x0,
            true => Swi4::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi4::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi4::B0x1
    }
}
#[doc = "Field `SWI4` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi4W<'a, REG> = crate::BitWriter<'a, REG, Swi4>;
impl<'a, REG> Swi4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi4::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi4::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi5 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi5> for bool {
    #[inline(always)]
    fn from(variant: Swi5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI5` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi5R = crate::BitReader<Swi5>;
impl Swi5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi5 {
        match self.bits {
            false => Swi5::B0x0,
            true => Swi5::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi5::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi5::B0x1
    }
}
#[doc = "Field `SWI5` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi5W<'a, REG> = crate::BitWriter<'a, REG, Swi5>;
impl<'a, REG> Swi5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi5::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi5::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi6 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi6> for bool {
    #[inline(always)]
    fn from(variant: Swi6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI6` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi6R = crate::BitReader<Swi6>;
impl Swi6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi6 {
        match self.bits {
            false => Swi6::B0x0,
            true => Swi6::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi6::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi6::B0x1
    }
}
#[doc = "Field `SWI6` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi6W<'a, REG> = crate::BitWriter<'a, REG, Swi6>;
impl<'a, REG> Swi6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi6::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi6::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi7 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi7> for bool {
    #[inline(always)]
    fn from(variant: Swi7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI7` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi7R = crate::BitReader<Swi7>;
impl Swi7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi7 {
        match self.bits {
            false => Swi7::B0x0,
            true => Swi7::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi7::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi7::B0x1
    }
}
#[doc = "Field `SWI7` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi7W<'a, REG> = crate::BitWriter<'a, REG, Swi7>;
impl<'a, REG> Swi7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi7::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi7::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi8 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi8> for bool {
    #[inline(always)]
    fn from(variant: Swi8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI8` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi8R = crate::BitReader<Swi8>;
impl Swi8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi8 {
        match self.bits {
            false => Swi8::B0x0,
            true => Swi8::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi8::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi8::B0x1
    }
}
#[doc = "Field `SWI8` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi8W<'a, REG> = crate::BitWriter<'a, REG, Swi8>;
impl<'a, REG> Swi8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi8::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi8::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi9 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi9> for bool {
    #[inline(always)]
    fn from(variant: Swi9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI9` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi9R = crate::BitReader<Swi9>;
impl Swi9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi9 {
        match self.bits {
            false => Swi9::B0x0,
            true => Swi9::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi9::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi9::B0x1
    }
}
#[doc = "Field `SWI9` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi9W<'a, REG> = crate::BitWriter<'a, REG, Swi9>;
impl<'a, REG> Swi9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi9::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi9::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi10 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi10> for bool {
    #[inline(always)]
    fn from(variant: Swi10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI10` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi10R = crate::BitReader<Swi10>;
impl Swi10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi10 {
        match self.bits {
            false => Swi10::B0x0,
            true => Swi10::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi10::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi10::B0x1
    }
}
#[doc = "Field `SWI10` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi10W<'a, REG> = crate::BitWriter<'a, REG, Swi10>;
impl<'a, REG> Swi10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi10::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi10::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi11 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi11> for bool {
    #[inline(always)]
    fn from(variant: Swi11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI11` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi11R = crate::BitReader<Swi11>;
impl Swi11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi11 {
        match self.bits {
            false => Swi11::B0x0,
            true => Swi11::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi11::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi11::B0x1
    }
}
#[doc = "Field `SWI11` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi11W<'a, REG> = crate::BitWriter<'a, REG, Swi11>;
impl<'a, REG> Swi11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi11::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi11::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi12 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi12> for bool {
    #[inline(always)]
    fn from(variant: Swi12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI12` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi12R = crate::BitReader<Swi12>;
impl Swi12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi12 {
        match self.bits {
            false => Swi12::B0x0,
            true => Swi12::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi12::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi12::B0x1
    }
}
#[doc = "Field `SWI12` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi12W<'a, REG> = crate::BitWriter<'a, REG, Swi12>;
impl<'a, REG> Swi12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi12::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi12::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi13 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi13> for bool {
    #[inline(always)]
    fn from(variant: Swi13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI13` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi13R = crate::BitReader<Swi13>;
impl Swi13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi13 {
        match self.bits {
            false => Swi13::B0x0,
            true => Swi13::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi13::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi13::B0x1
    }
}
#[doc = "Field `SWI13` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi13W<'a, REG> = crate::BitWriter<'a, REG, Swi13>;
impl<'a, REG> Swi13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi13::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi13::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi14 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi14> for bool {
    #[inline(always)]
    fn from(variant: Swi14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI14` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi14R = crate::BitReader<Swi14>;
impl Swi14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi14 {
        match self.bits {
            false => Swi14::B0x0,
            true => Swi14::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi14::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi14::B0x1
    }
}
#[doc = "Field `SWI14` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi14W<'a, REG> = crate::BitWriter<'a, REG, Swi14>;
impl<'a, REG> Swi14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi14::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi14::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi15 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi15> for bool {
    #[inline(always)]
    fn from(variant: Swi15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI15` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi15R = crate::BitReader<Swi15>;
impl Swi15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi15 {
        match self.bits {
            false => Swi15::B0x0,
            true => Swi15::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi15::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi15::B0x1
    }
}
#[doc = "Field `SWI15` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi15W<'a, REG> = crate::BitWriter<'a, REG, Swi15>;
impl<'a, REG> Swi15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi15::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi15::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi16 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi16> for bool {
    #[inline(always)]
    fn from(variant: Swi16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI16` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi16R = crate::BitReader<Swi16>;
impl Swi16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi16 {
        match self.bits {
            false => Swi16::B0x0,
            true => Swi16::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi16::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi16::B0x1
    }
}
#[doc = "Field `SWI16` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi16W<'a, REG> = crate::BitWriter<'a, REG, Swi16>;
impl<'a, REG> Swi16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi16::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi16::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi17 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi17> for bool {
    #[inline(always)]
    fn from(variant: Swi17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI17` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi17R = crate::BitReader<Swi17>;
impl Swi17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi17 {
        match self.bits {
            false => Swi17::B0x0,
            true => Swi17::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi17::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi17::B0x1
    }
}
#[doc = "Field `SWI17` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi17W<'a, REG> = crate::BitWriter<'a, REG, Swi17>;
impl<'a, REG> Swi17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi17::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi17::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi18 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi18> for bool {
    #[inline(always)]
    fn from(variant: Swi18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI18` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi18R = crate::BitReader<Swi18>;
impl Swi18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi18 {
        match self.bits {
            false => Swi18::B0x0,
            true => Swi18::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi18::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi18::B0x1
    }
}
#[doc = "Field `SWI18` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi18W<'a, REG> = crate::BitWriter<'a, REG, Swi18>;
impl<'a, REG> Swi18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi18::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi18::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi19 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi19> for bool {
    #[inline(always)]
    fn from(variant: Swi19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI19` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi19R = crate::BitReader<Swi19>;
impl Swi19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi19 {
        match self.bits {
            false => Swi19::B0x0,
            true => Swi19::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi19::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi19::B0x1
    }
}
#[doc = "Field `SWI19` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi19W<'a, REG> = crate::BitWriter<'a, REG, Swi19>;
impl<'a, REG> Swi19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi19::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi19::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi20 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi20> for bool {
    #[inline(always)]
    fn from(variant: Swi20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI20` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi20R = crate::BitReader<Swi20>;
impl Swi20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi20 {
        match self.bits {
            false => Swi20::B0x0,
            true => Swi20::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi20::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi20::B0x1
    }
}
#[doc = "Field `SWI20` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi20W<'a, REG> = crate::BitWriter<'a, REG, Swi20>;
impl<'a, REG> Swi20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi20::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi20::B0x1)
    }
}
#[doc = "Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi21 {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Rising edge event generated on the corresponding line, followed by an interrupt"]
    B0x1 = 1,
}
impl From<Swi21> for bool {
    #[inline(always)]
    fn from(variant: Swi21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI21` reader - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi21R = crate::BitReader<Swi21>;
impl Swi21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi21 {
        match self.bits {
            false => Swi21::B0x0,
            true => Swi21::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Swi21::B0x0
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Swi21::B0x1
    }
}
#[doc = "Field `SWI21` writer - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
pub type Swi21W<'a, REG> = crate::BitWriter<'a, REG, Swi21>;
impl<'a, REG> Swi21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Swi21::B0x0)
    }
    #[doc = "Rising edge event generated on the corresponding line, followed by an interrupt"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Swi21::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi0(&self) -> Swi0R {
        Swi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi1(&self) -> Swi1R {
        Swi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi2(&self) -> Swi2R {
        Swi2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi3(&self) -> Swi3R {
        Swi3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi4(&self) -> Swi4R {
        Swi4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi5(&self) -> Swi5R {
        Swi5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi6(&self) -> Swi6R {
        Swi6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi7(&self) -> Swi7R {
        Swi7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi8(&self) -> Swi8R {
        Swi8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi9(&self) -> Swi9R {
        Swi9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi10(&self) -> Swi10R {
        Swi10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi11(&self) -> Swi11R {
        Swi11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi12(&self) -> Swi12R {
        Swi12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi13(&self) -> Swi13R {
        Swi13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi14(&self) -> Swi14R {
        Swi14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi15(&self) -> Swi15R {
        Swi15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi16(&self) -> Swi16R {
        Swi16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi17(&self) -> Swi17R {
        Swi17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi18(&self) -> Swi18R {
        Swi18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi19(&self) -> Swi19R {
        Swi19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi20(&self) -> Swi20R {
        Swi20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    pub fn swi21(&self) -> Swi21R {
        Swi21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> Swi0W<ExtiSwier1Spec> {
        Swi0W::new(self, 0)
    }
    #[doc = "Bit 1 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> Swi1W<ExtiSwier1Spec> {
        Swi1W::new(self, 1)
    }
    #[doc = "Bit 2 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> Swi2W<ExtiSwier1Spec> {
        Swi2W::new(self, 2)
    }
    #[doc = "Bit 3 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> Swi3W<ExtiSwier1Spec> {
        Swi3W::new(self, 3)
    }
    #[doc = "Bit 4 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> Swi4W<ExtiSwier1Spec> {
        Swi4W::new(self, 4)
    }
    #[doc = "Bit 5 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> Swi5W<ExtiSwier1Spec> {
        Swi5W::new(self, 5)
    }
    #[doc = "Bit 6 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> Swi6W<ExtiSwier1Spec> {
        Swi6W::new(self, 6)
    }
    #[doc = "Bit 7 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> Swi7W<ExtiSwier1Spec> {
        Swi7W::new(self, 7)
    }
    #[doc = "Bit 8 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi8(&mut self) -> Swi8W<ExtiSwier1Spec> {
        Swi8W::new(self, 8)
    }
    #[doc = "Bit 9 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi9(&mut self) -> Swi9W<ExtiSwier1Spec> {
        Swi9W::new(self, 9)
    }
    #[doc = "Bit 10 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi10(&mut self) -> Swi10W<ExtiSwier1Spec> {
        Swi10W::new(self, 10)
    }
    #[doc = "Bit 11 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi11(&mut self) -> Swi11W<ExtiSwier1Spec> {
        Swi11W::new(self, 11)
    }
    #[doc = "Bit 12 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi12(&mut self) -> Swi12W<ExtiSwier1Spec> {
        Swi12W::new(self, 12)
    }
    #[doc = "Bit 13 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi13(&mut self) -> Swi13W<ExtiSwier1Spec> {
        Swi13W::new(self, 13)
    }
    #[doc = "Bit 14 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi14(&mut self) -> Swi14W<ExtiSwier1Spec> {
        Swi14W::new(self, 14)
    }
    #[doc = "Bit 15 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi15(&mut self) -> Swi15W<ExtiSwier1Spec> {
        Swi15W::new(self, 15)
    }
    #[doc = "Bit 16 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi16(&mut self) -> Swi16W<ExtiSwier1Spec> {
        Swi16W::new(self, 16)
    }
    #[doc = "Bit 17 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi17(&mut self) -> Swi17W<ExtiSwier1Spec> {
        Swi17W::new(self, 17)
    }
    #[doc = "Bit 18 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi18(&mut self) -> Swi18W<ExtiSwier1Spec> {
        Swi18W::new(self, 18)
    }
    #[doc = "Bit 19 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi19(&mut self) -> Swi19W<ExtiSwier1Spec> {
        Swi19W::new(self, 19)
    }
    #[doc = "Bit 20 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi20(&mut self) -> Swi20W<ExtiSwier1Spec> {
        Swi20W::new(self, 20)
    }
    #[doc = "Bit 21 - Software rising edge event trigger on line x (x1=1211to10) Setting of any bit by software triggers a rising edge event on the corresponding line x, resulting in an interrupt, independently of EXTI_RTSR1 and EXTI_FTSR1 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0. Bits 18 and 19 are available only on STM32U0x3xx devices. They are reserved on STM32U031xx devices."]
    #[inline(always)]
    #[must_use]
    pub fn swi21(&mut self) -> Swi21W<ExtiSwier1Spec> {
        Swi21W::new(self, 21)
    }
}
#[doc = "EXTI software interrupt event register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_swier1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_swier1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiSwier1Spec;
impl crate::RegisterSpec for ExtiSwier1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_swier1::R`](R) reader structure"]
impl crate::Readable for ExtiSwier1Spec {}
#[doc = "`write(|w| ..)` method takes [`exti_swier1::W`](W) writer structure"]
impl crate::Writable for ExtiSwier1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_SWIER1 to value 0"]
impl crate::Resettable for ExtiSwier1Spec {
    const RESET_VALUE: u32 = 0;
}
