#[doc = "Register `RCC_APBSMENR1` reader"]
pub type R = crate::R<RccApbsmenr1Spec>;
#[doc = "Register `RCC_APBSMENR1` writer"]
pub type W = crate::W<RccApbsmenr1Spec>;
#[doc = "TIM2 timer clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim2smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim2smen> for bool {
    #[inline(always)]
    fn from(variant: Tim2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim2smenR = crate::BitReader<Tim2smen>;
impl Tim2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim2smen {
        match self.bits {
            false => Tim2smen::B0x0,
            true => Tim2smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim2smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim2smen::B0x1
    }
}
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim2smenW<'a, REG> = crate::BitWriter<'a, REG, Tim2smen>;
impl<'a, REG> Tim2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2smen::B0x1)
    }
}
#[doc = "TIM3 timer clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim3smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim3smen> for bool {
    #[inline(always)]
    fn from(variant: Tim3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim3smenR = crate::BitReader<Tim3smen>;
impl Tim3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim3smen {
        match self.bits {
            false => Tim3smen::B0x0,
            true => Tim3smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim3smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim3smen::B0x1
    }
}
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim3smenW<'a, REG> = crate::BitWriter<'a, REG, Tim3smen>;
impl<'a, REG> Tim3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3smen::B0x1)
    }
}
#[doc = "TIM6 timer clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim6smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim6smen> for bool {
    #[inline(always)]
    fn from(variant: Tim6smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim6smenR = crate::BitReader<Tim6smen>;
impl Tim6smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim6smen {
        match self.bits {
            false => Tim6smen::B0x0,
            true => Tim6smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim6smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim6smen::B0x1
    }
}
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim6smenW<'a, REG> = crate::BitWriter<'a, REG, Tim6smen>;
impl<'a, REG> Tim6smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6smen::B0x1)
    }
}
#[doc = "TIM7 timer clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim7smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim7smen> for bool {
    #[inline(always)]
    fn from(variant: Tim7smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim7smenR = crate::BitReader<Tim7smen>;
impl Tim7smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim7smen {
        match self.bits {
            false => Tim7smen::B0x0,
            true => Tim7smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim7smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim7smen::B0x1
    }
}
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clock enable during Sleep mode Set and cleared by software."]
pub type Tim7smenW<'a, REG> = crate::BitWriter<'a, REG, Tim7smen>;
impl<'a, REG> Tim7smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7smen::B0x1)
    }
}
#[doc = "LPUART2 clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart2smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lpuart2smen> for bool {
    #[inline(always)]
    fn from(variant: Lpuart2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART2SMEN` reader - LPUART2 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lpuart2smenR = crate::BitReader<Lpuart2smen>;
impl Lpuart2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart2smen {
        match self.bits {
            false => Lpuart2smen::B0x0,
            true => Lpuart2smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart2smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart2smen::B0x1
    }
}
#[doc = "Field `LPUART2SMEN` writer - LPUART2 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lpuart2smenW<'a, REG> = crate::BitWriter<'a, REG, Lpuart2smen>;
impl<'a, REG> Lpuart2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2smen::B0x1)
    }
}
#[doc = "LCD clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lcdsmen> for bool {
    #[inline(always)]
    fn from(variant: Lcdsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDSMEN` reader - LCD clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type LcdsmenR = crate::BitReader<Lcdsmen>;
impl LcdsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdsmen {
        match self.bits {
            false => Lcdsmen::B0x0,
            true => Lcdsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lcdsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lcdsmen::B0x1
    }
}
#[doc = "Field `LCDSMEN` writer - LCD clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type LcdsmenW<'a, REG> = crate::BitWriter<'a, REG, Lcdsmen>;
impl<'a, REG> LcdsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdsmen::B0x1)
    }
}
#[doc = "RTC APB clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcapbsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rtcapbsmen> for bool {
    #[inline(always)]
    fn from(variant: Rtcapbsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode Set and cleared by software."]
pub type RtcapbsmenR = crate::BitReader<Rtcapbsmen>;
impl RtcapbsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcapbsmen {
        match self.bits {
            false => Rtcapbsmen::B0x0,
            true => Rtcapbsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtcapbsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtcapbsmen::B0x1
    }
}
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode Set and cleared by software."]
pub type RtcapbsmenW<'a, REG> = crate::BitWriter<'a, REG, Rtcapbsmen>;
impl<'a, REG> RtcapbsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcapbsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcapbsmen::B0x1)
    }
}
#[doc = "WWDG clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdgsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Wwdgsmen> for bool {
    #[inline(always)]
    fn from(variant: Wwdgsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGSMEN` reader - WWDG clock enable during Sleep and Stop modes Set and cleared by software."]
pub type WwdgsmenR = crate::BitReader<Wwdgsmen>;
impl WwdgsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdgsmen {
        match self.bits {
            false => Wwdgsmen::B0x0,
            true => Wwdgsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wwdgsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wwdgsmen::B0x1
    }
}
#[doc = "Field `WWDGSMEN` writer - WWDG clock enable during Sleep and Stop modes Set and cleared by software."]
pub type WwdgsmenW<'a, REG> = crate::BitWriter<'a, REG, Wwdgsmen>;
impl<'a, REG> WwdgsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdgsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdgsmen::B0x1)
    }
}
#[doc = "LPUART3 clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart3smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lpuart3smen> for bool {
    #[inline(always)]
    fn from(variant: Lpuart3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART3SMEN` reader - LPUART3 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lpuart3smenR = crate::BitReader<Lpuart3smen>;
impl Lpuart3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart3smen {
        match self.bits {
            false => Lpuart3smen::B0x0,
            true => Lpuart3smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart3smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart3smen::B0x1
    }
}
#[doc = "Field `LPUART3SMEN` writer - LPUART3 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lpuart3smenW<'a, REG> = crate::BitWriter<'a, REG, Lpuart3smen>;
impl<'a, REG> Lpuart3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3smen::B0x1)
    }
}
#[doc = "USB clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usbsmen> for bool {
    #[inline(always)]
    fn from(variant: Usbsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSMEN` reader - USB clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type UsbsmenR = crate::BitReader<Usbsmen>;
impl UsbsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbsmen {
        match self.bits {
            false => Usbsmen::B0x0,
            true => Usbsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usbsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usbsmen::B0x1
    }
}
#[doc = "Field `USBSMEN` writer - USB clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type UsbsmenW<'a, REG> = crate::BitWriter<'a, REG, Usbsmen>;
impl<'a, REG> UsbsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsmen::B0x1)
    }
}
#[doc = "SPI2 clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi2smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Spi2smen> for bool {
    #[inline(always)]
    fn from(variant: Spi2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode Set and cleared by software."]
pub type Spi2smenR = crate::BitReader<Spi2smen>;
impl Spi2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi2smen {
        match self.bits {
            false => Spi2smen::B0x0,
            true => Spi2smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi2smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi2smen::B0x1
    }
}
#[doc = "Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode Set and cleared by software."]
pub type Spi2smenW<'a, REG> = crate::BitWriter<'a, REG, Spi2smen>;
impl<'a, REG> Spi2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2smen::B0x1)
    }
}
#[doc = "SPI3 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi3smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Spi3smen> for bool {
    #[inline(always)]
    fn from(variant: Spi3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3SMEN` reader - SPI3 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type Spi3smenR = crate::BitReader<Spi3smen>;
impl Spi3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi3smen {
        match self.bits {
            false => Spi3smen::B0x0,
            true => Spi3smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi3smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi3smen::B0x1
    }
}
#[doc = "Field `SPI3SMEN` writer - SPI3 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type Spi3smenW<'a, REG> = crate::BitWriter<'a, REG, Spi3smen>;
impl<'a, REG> Spi3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3smen::B0x1)
    }
}
#[doc = "CRS clock enable during Sleep and Stop modes&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crssmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Crssmen> for bool {
    #[inline(always)]
    fn from(variant: Crssmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSSMEN` reader - CRS clock enable during Sleep and Stop modes&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type CrssmenR = crate::BitReader<Crssmen>;
impl CrssmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crssmen {
        match self.bits {
            false => Crssmen::B0x0,
            true => Crssmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crssmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crssmen::B0x1
    }
}
#[doc = "Field `CRSSMEN` writer - CRS clock enable during Sleep and Stop modes&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type CrssmenW<'a, REG> = crate::BitWriter<'a, REG, Crssmen>;
impl<'a, REG> CrssmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crssmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crssmen::B0x1)
    }
}
#[doc = "USART2 clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart2smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usart2smen> for bool {
    #[inline(always)]
    fn from(variant: Usart2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2SMEN` reader - USART2 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart2smenR = crate::BitReader<Usart2smen>;
impl Usart2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart2smen {
        match self.bits {
            false => Usart2smen::B0x0,
            true => Usart2smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart2smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart2smen::B0x1
    }
}
#[doc = "Field `USART2SMEN` writer - USART2 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart2smenW<'a, REG> = crate::BitWriter<'a, REG, Usart2smen>;
impl<'a, REG> Usart2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2smen::B0x1)
    }
}
#[doc = "USART3 clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart3smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usart3smen> for bool {
    #[inline(always)]
    fn from(variant: Usart3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3SMEN` reader - USART3 clock enable during Sleep mode Set and cleared by software."]
pub type Usart3smenR = crate::BitReader<Usart3smen>;
impl Usart3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart3smen {
        match self.bits {
            false => Usart3smen::B0x0,
            true => Usart3smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart3smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart3smen::B0x1
    }
}
#[doc = "Field `USART3SMEN` writer - USART3 clock enable during Sleep mode Set and cleared by software."]
pub type Usart3smenW<'a, REG> = crate::BitWriter<'a, REG, Usart3smen>;
impl<'a, REG> Usart3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3smen::B0x1)
    }
}
#[doc = "USART4 clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart4smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usart4smen> for bool {
    #[inline(always)]
    fn from(variant: Usart4smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART4SMEN` reader - USART4 clock enable during Sleep mode Set and cleared by software."]
pub type Usart4smenR = crate::BitReader<Usart4smen>;
impl Usart4smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart4smen {
        match self.bits {
            false => Usart4smen::B0x0,
            true => Usart4smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart4smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart4smen::B0x1
    }
}
#[doc = "Field `USART4SMEN` writer - USART4 clock enable during Sleep mode Set and cleared by software."]
pub type Usart4smenW<'a, REG> = crate::BitWriter<'a, REG, Usart4smen>;
impl<'a, REG> Usart4smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart4smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart4smen::B0x1)
    }
}
#[doc = "LPUART1 clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart1smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lpuart1smen> for bool {
    #[inline(always)]
    fn from(variant: Lpuart1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1SMEN` reader - LPUART1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lpuart1smenR = crate::BitReader<Lpuart1smen>;
impl Lpuart1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1smen {
        match self.bits {
            false => Lpuart1smen::B0x0,
            true => Lpuart1smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart1smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart1smen::B0x1
    }
}
#[doc = "Field `LPUART1SMEN` writer - LPUART1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lpuart1smenW<'a, REG> = crate::BitWriter<'a, REG, Lpuart1smen>;
impl<'a, REG> Lpuart1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1smen::B0x1)
    }
}
#[doc = "I2C1 clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2c1smen> for bool {
    #[inline(always)]
    fn from(variant: I2c1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type I2c1smenR = crate::BitReader<I2c1smen>;
impl I2c1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1smen {
        match self.bits {
            false => I2c1smen::B0x0,
            true => I2c1smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c1smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c1smen::B0x1
    }
}
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type I2c1smenW<'a, REG> = crate::BitWriter<'a, REG, I2c1smen>;
impl<'a, REG> I2c1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1smen::B0x1)
    }
}
#[doc = "I2C2 clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2c2smen> for bool {
    #[inline(always)]
    fn from(variant: I2c2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode Set and cleared by software."]
pub type I2c2smenR = crate::BitReader<I2c2smen>;
impl I2c2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c2smen {
        match self.bits {
            false => I2c2smen::B0x0,
            true => I2c2smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c2smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c2smen::B0x1
    }
}
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode Set and cleared by software."]
pub type I2c2smenW<'a, REG> = crate::BitWriter<'a, REG, I2c2smen>;
impl<'a, REG> I2c2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2smen::B0x1)
    }
}
#[doc = "I2C3 clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c3smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2c3smen> for bool {
    #[inline(always)]
    fn from(variant: I2c3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3SMEN` reader - I2C3 clock enable during Sleep mode Set and cleared by software."]
pub type I2c3smenR = crate::BitReader<I2c3smen>;
impl I2c3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c3smen {
        match self.bits {
            false => I2c3smen::B0x0,
            true => I2c3smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c3smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c3smen::B0x1
    }
}
#[doc = "Field `I2C3SMEN` writer - I2C3 clock enable during Sleep mode Set and cleared by software."]
pub type I2c3smenW<'a, REG> = crate::BitWriter<'a, REG, I2c3smen>;
impl<'a, REG> I2c3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3smen::B0x1)
    }
}
#[doc = "OPAMP clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opampsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Opampsmen> for bool {
    #[inline(always)]
    fn from(variant: Opampsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMPSMEN` reader - OPAMP clock enable during Sleep and Stop modes Set and cleared by software."]
pub type OpampsmenR = crate::BitReader<Opampsmen>;
impl OpampsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opampsmen {
        match self.bits {
            false => Opampsmen::B0x0,
            true => Opampsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Opampsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Opampsmen::B0x1
    }
}
#[doc = "Field `OPAMPSMEN` writer - OPAMP clock enable during Sleep and Stop modes Set and cleared by software."]
pub type OpampsmenW<'a, REG> = crate::BitWriter<'a, REG, Opampsmen>;
impl<'a, REG> OpampsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Opampsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Opampsmen::B0x1)
    }
}
#[doc = "I2C4 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c4smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2c4smen> for bool {
    #[inline(always)]
    fn from(variant: I2c4smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C4SMEN` reader - I2C4 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type I2c4smenR = crate::BitReader<I2c4smen>;
impl I2c4smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c4smen {
        match self.bits {
            false => I2c4smen::B0x0,
            true => I2c4smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c4smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c4smen::B0x1
    }
}
#[doc = "Field `I2C4SMEN` writer - I2C4 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type I2c4smenW<'a, REG> = crate::BitWriter<'a, REG, I2c4smen>;
impl<'a, REG> I2c4smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4smen::B0x1)
    }
}
#[doc = "Low power timer 3 clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim3smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lptim3smen> for bool {
    #[inline(always)]
    fn from(variant: Lptim3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM3SMEN` reader - Low power timer 3 clock enable during Sleep mode Set and cleared by software."]
pub type Lptim3smenR = crate::BitReader<Lptim3smen>;
impl Lptim3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim3smen {
        match self.bits {
            false => Lptim3smen::B0x0,
            true => Lptim3smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim3smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim3smen::B0x1
    }
}
#[doc = "Field `LPTIM3SMEN` writer - Low power timer 3 clock enable during Sleep mode Set and cleared by software."]
pub type Lptim3smenW<'a, REG> = crate::BitWriter<'a, REG, Lptim3smen>;
impl<'a, REG> Lptim3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3smen::B0x1)
    }
}
#[doc = "Power interface clock enable during Sleep mode Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrsmen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Pwrsmen> for bool {
    #[inline(always)]
    fn from(variant: Pwrsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSMEN` reader - Power interface clock enable during Sleep mode Set and cleared by software."]
pub type PwrsmenR = crate::BitReader<Pwrsmen>;
impl PwrsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrsmen {
        match self.bits {
            false => Pwrsmen::B0x0,
            true => Pwrsmen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pwrsmen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pwrsmen::B0x1
    }
}
#[doc = "Field `PWRSMEN` writer - Power interface clock enable during Sleep mode Set and cleared by software."]
pub type PwrsmenW<'a, REG> = crate::BitWriter<'a, REG, Pwrsmen>;
impl<'a, REG> PwrsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsmen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsmen::B0x1)
    }
}
#[doc = "DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac1smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Dac1smen> for bool {
    #[inline(always)]
    fn from(variant: Dac1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1SMEN` reader - DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Dac1smenR = crate::BitReader<Dac1smen>;
impl Dac1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac1smen {
        match self.bits {
            false => Dac1smen::B0x0,
            true => Dac1smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac1smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac1smen::B0x1
    }
}
#[doc = "Field `DAC1SMEN` writer - DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Dac1smenW<'a, REG> = crate::BitWriter<'a, REG, Dac1smen>;
impl<'a, REG> Dac1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1smen::B0x1)
    }
}
#[doc = "Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim2smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lptim2smen> for bool {
    #[inline(always)]
    fn from(variant: Lptim2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2SMEN` reader - Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lptim2smenR = crate::BitReader<Lptim2smen>;
impl Lptim2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim2smen {
        match self.bits {
            false => Lptim2smen::B0x0,
            true => Lptim2smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim2smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim2smen::B0x1
    }
}
#[doc = "Field `LPTIM2SMEN` writer - Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lptim2smenW<'a, REG> = crate::BitWriter<'a, REG, Lptim2smen>;
impl<'a, REG> Lptim2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2smen::B0x1)
    }
}
#[doc = "Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim1smen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lptim1smen> for bool {
    #[inline(always)]
    fn from(variant: Lptim1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1SMEN` reader - Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lptim1smenR = crate::BitReader<Lptim1smen>;
impl Lptim1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim1smen {
        match self.bits {
            false => Lptim1smen::B0x0,
            true => Lptim1smen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim1smen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim1smen::B0x1
    }
}
#[doc = "Field `LPTIM1SMEN` writer - Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software."]
pub type Lptim1smenW<'a, REG> = crate::BitWriter<'a, REG, Lptim1smen>;
impl<'a, REG> Lptim1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1smen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1smen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim2smen(&self) -> Tim2smenR {
        Tim2smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim3smen(&self) -> Tim3smenR {
        Tim3smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim6smen(&self) -> Tim6smenR {
        Tim6smenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn tim7smen(&self) -> Tim7smenR {
        Tim7smenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPUART2 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart2smen(&self) -> Lpuart2smenR {
        Lpuart2smenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn lcdsmen(&self) -> LcdsmenR {
        LcdsmenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RtcapbsmenR {
        RtcapbsmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WwdgsmenR {
        WwdgsmenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPUART3 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart3smen(&self) -> Lpuart3smenR {
        Lpuart3smenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USB clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn usbsmen(&self) -> UsbsmenR {
        UsbsmenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn spi2smen(&self) -> Spi2smenR {
        Spi2smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn spi3smen(&self) -> Spi3smenR {
        Spi3smenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CRS clock enable during Sleep and Stop modes&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn crssmen(&self) -> CrssmenR {
        CrssmenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart2smen(&self) -> Usart2smenR {
        Usart2smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn usart3smen(&self) -> Usart3smenR {
        Usart3smenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn usart4smen(&self) -> Usart4smenR {
        Usart4smenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> Lpuart1smenR {
        Lpuart1smenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2c1smenR {
        I2c1smenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2c2smenR {
        I2c2smenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2c3smenR {
        I2c3smenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OPAMP clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn opampsmen(&self) -> OpampsmenR {
        OpampsmenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2C4 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2c4smenR {
        I2c4smenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Low power timer 3 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn lptim3smen(&self) -> Lptim3smenR {
        Lptim3smenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PwrsmenR {
        PwrsmenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn dac1smen(&self) -> Dac1smenR {
        Dac1smenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lptim2smen(&self) -> Lptim2smenR {
        Lptim2smenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1smen(&self) -> Lptim1smenR {
        Lptim1smenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> Tim2smenW<RccApbsmenr1Spec> {
        Tim2smenW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> Tim3smenW<RccApbsmenr1Spec> {
        Tim3smenW::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> Tim6smenW<RccApbsmenr1Spec> {
        Tim6smenW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> Tim7smenW<RccApbsmenr1Spec> {
        Tim7smenW::new(self, 5)
    }
    #[doc = "Bit 7 - LPUART2 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2smen(&mut self) -> Lpuart2smenW<RccApbsmenr1Spec> {
        Lpuart2smenW::new(self, 7)
    }
    #[doc = "Bit 9 - LCD clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lcdsmen(&mut self) -> LcdsmenW<RccApbsmenr1Spec> {
        LcdsmenW::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RtcapbsmenW<RccApbsmenr1Spec> {
        RtcapbsmenW::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WwdgsmenW<RccApbsmenr1Spec> {
        WwdgsmenW::new(self, 11)
    }
    #[doc = "Bit 12 - LPUART3 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart3smen(&mut self) -> Lpuart3smenW<RccApbsmenr1Spec> {
        Lpuart3smenW::new(self, 12)
    }
    #[doc = "Bit 13 - USB clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbsmen(&mut self) -> UsbsmenW<RccApbsmenr1Spec> {
        UsbsmenW::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> Spi2smenW<RccApbsmenr1Spec> {
        Spi2smenW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3smen(&mut self) -> Spi3smenW<RccApbsmenr1Spec> {
        Spi3smenW::new(self, 15)
    }
    #[doc = "Bit 16 - CRS clock enable during Sleep and Stop modes&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crssmen(&mut self) -> CrssmenW<RccApbsmenr1Spec> {
        CrssmenW::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> Usart2smenW<RccApbsmenr1Spec> {
        Usart2smenW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> Usart3smenW<RccApbsmenr1Spec> {
        Usart3smenW::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart4smen(&mut self) -> Usart4smenW<RccApbsmenr1Spec> {
        Usart4smenW::new(self, 19)
    }
    #[doc = "Bit 20 - LPUART1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> Lpuart1smenW<RccApbsmenr1Spec> {
        Lpuart1smenW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2c1smenW<RccApbsmenr1Spec> {
        I2c1smenW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2c2smenW<RccApbsmenr1Spec> {
        I2c2smenW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2c3smenW<RccApbsmenr1Spec> {
        I2c3smenW::new(self, 23)
    }
    #[doc = "Bit 24 - OPAMP clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn opampsmen(&mut self) -> OpampsmenW<RccApbsmenr1Spec> {
        OpampsmenW::new(self, 24)
    }
    #[doc = "Bit 25 - I2C4 clock enable during Sleep mode&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4smen(&mut self) -> I2c4smenW<RccApbsmenr1Spec> {
        I2c4smenW::new(self, 25)
    }
    #[doc = "Bit 26 - Low power timer 3 clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3smen(&mut self) -> Lptim3smenW<RccApbsmenr1Spec> {
        Lptim3smenW::new(self, 26)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PwrsmenW<RccApbsmenr1Spec> {
        PwrsmenW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> Dac1smenW<RccApbsmenr1Spec> {
        Dac1smenW::new(self, 29)
    }
    #[doc = "Bit 30 - Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2smen(&mut self) -> Lptim2smenW<RccApbsmenr1Spec> {
        Lptim2smenW::new(self, 30)
    }
    #[doc = "Bit 31 - Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> Lptim1smenW<RccApbsmenr1Spec> {
        Lptim1smenW::new(self, 31)
    }
}
#[doc = "APB peripheral clock enable in Sleep/Stop mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbsmenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbsmenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApbsmenr1Spec;
impl crate::RegisterSpec for RccApbsmenr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apbsmenr1::R`](R) reader structure"]
impl crate::Readable for RccApbsmenr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apbsmenr1::W`](W) writer structure"]
impl crate::Writable for RccApbsmenr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APBSMENR1 to value 0xff7e_4c33"]
impl crate::Resettable for RccApbsmenr1Spec {
    const RESET_VALUE: u32 = 0xff7e_4c33;
}
