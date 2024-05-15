#[doc = "Register `RCC_APBENR1` reader"]
pub type R = crate::R<RccApbenr1Spec>;
#[doc = "Register `RCC_APBENR1` writer"]
pub type W = crate::W<RccApbenr1Spec>;
#[doc = "TIM2 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim2en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim2en> for bool {
    #[inline(always)]
    fn from(variant: Tim2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2EN` reader - TIM2 timer clock enable Set and cleared by software."]
pub type Tim2enR = crate::BitReader<Tim2en>;
impl Tim2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim2en {
        match self.bits {
            false => Tim2en::B0x0,
            true => Tim2en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim2en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim2en::B0x1
    }
}
#[doc = "Field `TIM2EN` writer - TIM2 timer clock enable Set and cleared by software."]
pub type Tim2enW<'a, REG> = crate::BitWriter<'a, REG, Tim2en>;
impl<'a, REG> Tim2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2en::B0x1)
    }
}
#[doc = "TIM3 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim3en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim3en> for bool {
    #[inline(always)]
    fn from(variant: Tim3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable Set and cleared by software."]
pub type Tim3enR = crate::BitReader<Tim3en>;
impl Tim3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim3en {
        match self.bits {
            false => Tim3en::B0x0,
            true => Tim3en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim3en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim3en::B0x1
    }
}
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable Set and cleared by software."]
pub type Tim3enW<'a, REG> = crate::BitWriter<'a, REG, Tim3en>;
impl<'a, REG> Tim3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3en::B0x1)
    }
}
#[doc = "TIM6 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim6en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim6en> for bool {
    #[inline(always)]
    fn from(variant: Tim6en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6EN` reader - TIM6 timer clock enable Set and cleared by software."]
pub type Tim6enR = crate::BitReader<Tim6en>;
impl Tim6enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim6en {
        match self.bits {
            false => Tim6en::B0x0,
            true => Tim6en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim6en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim6en::B0x1
    }
}
#[doc = "Field `TIM6EN` writer - TIM6 timer clock enable Set and cleared by software."]
pub type Tim6enW<'a, REG> = crate::BitWriter<'a, REG, Tim6en>;
impl<'a, REG> Tim6enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6en::B0x1)
    }
}
#[doc = "TIM7 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim7en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Tim7en> for bool {
    #[inline(always)]
    fn from(variant: Tim7en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7EN` reader - TIM7 timer clock enable Set and cleared by software."]
pub type Tim7enR = crate::BitReader<Tim7en>;
impl Tim7enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim7en {
        match self.bits {
            false => Tim7en::B0x0,
            true => Tim7en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim7en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim7en::B0x1
    }
}
#[doc = "Field `TIM7EN` writer - TIM7 timer clock enable Set and cleared by software."]
pub type Tim7enW<'a, REG> = crate::BitWriter<'a, REG, Tim7en>;
impl<'a, REG> Tim7enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7en::B0x1)
    }
}
#[doc = "LPUART2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart2en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lpuart2en> for bool {
    #[inline(always)]
    fn from(variant: Lpuart2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART2EN` reader - LPUART2 clock enable Set and cleared by software."]
pub type Lpuart2enR = crate::BitReader<Lpuart2en>;
impl Lpuart2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart2en {
        match self.bits {
            false => Lpuart2en::B0x0,
            true => Lpuart2en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart2en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart2en::B0x1
    }
}
#[doc = "Field `LPUART2EN` writer - LPUART2 clock enable Set and cleared by software."]
pub type Lpuart2enW<'a, REG> = crate::BitWriter<'a, REG, Lpuart2en>;
impl<'a, REG> Lpuart2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2en::B0x1)
    }
}
#[doc = "LCD clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcden {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lcden> for bool {
    #[inline(always)]
    fn from(variant: Lcden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDEN` reader - LCD clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type LcdenR = crate::BitReader<Lcden>;
impl LcdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcden {
        match self.bits {
            false => Lcden::B0x0,
            true => Lcden::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lcden::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lcden::B0x1
    }
}
#[doc = "Field `LCDEN` writer - LCD clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type LcdenW<'a, REG> = crate::BitWriter<'a, REG, Lcden>;
impl<'a, REG> LcdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcden::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcden::B0x1)
    }
}
#[doc = "RTC APB clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcapben {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Rtcapben> for bool {
    #[inline(always)]
    fn from(variant: Rtcapben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable Set and cleared by software."]
pub type RtcapbenR = crate::BitReader<Rtcapben>;
impl RtcapbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcapben {
        match self.bits {
            false => Rtcapben::B0x0,
            true => Rtcapben::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtcapben::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtcapben::B0x1
    }
}
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable Set and cleared by software."]
pub type RtcapbenW<'a, REG> = crate::BitWriter<'a, REG, Rtcapben>;
impl<'a, REG> RtcapbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcapben::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcapben::B0x1)
    }
}
#[doc = "WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdgen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Wwdgen> for bool {
    #[inline(always)]
    fn from(variant: Wwdgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGEN` reader - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
pub type WwdgenR = crate::BitReader<Wwdgen>;
impl WwdgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdgen {
        match self.bits {
            false => Wwdgen::B0x0,
            true => Wwdgen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wwdgen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wwdgen::B0x1
    }
}
#[doc = "Field `WWDGEN` writer - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
pub type WwdgenW<'a, REG> = crate::BitWriter<'a, REG, Wwdgen>;
impl<'a, REG> WwdgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdgen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdgen::B0x1)
    }
}
#[doc = "LPUART3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart3en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lpuart3en> for bool {
    #[inline(always)]
    fn from(variant: Lpuart3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART3EN` reader - LPUART3 clock enable Set and cleared by software."]
pub type Lpuart3enR = crate::BitReader<Lpuart3en>;
impl Lpuart3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart3en {
        match self.bits {
            false => Lpuart3en::B0x0,
            true => Lpuart3en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart3en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart3en::B0x1
    }
}
#[doc = "Field `LPUART3EN` writer - LPUART3 clock enable Set and cleared by software."]
pub type Lpuart3enW<'a, REG> = crate::BitWriter<'a, REG, Lpuart3en>;
impl<'a, REG> Lpuart3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3en::B0x1)
    }
}
#[doc = "USB clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usben {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usben> for bool {
    #[inline(always)]
    fn from(variant: Usben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBEN` reader - USB clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type UsbenR = crate::BitReader<Usben>;
impl UsbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usben {
        match self.bits {
            false => Usben::B0x0,
            true => Usben::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usben::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usben::B0x1
    }
}
#[doc = "Field `USBEN` writer - USB clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type UsbenW<'a, REG> = crate::BitWriter<'a, REG, Usben>;
impl<'a, REG> UsbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usben::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usben::B0x1)
    }
}
#[doc = "SPI2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi2en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Spi2en> for bool {
    #[inline(always)]
    fn from(variant: Spi2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2EN` reader - SPI2 clock enable Set and cleared by software."]
pub type Spi2enR = crate::BitReader<Spi2en>;
impl Spi2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi2en {
        match self.bits {
            false => Spi2en::B0x0,
            true => Spi2en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi2en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi2en::B0x1
    }
}
#[doc = "Field `SPI2EN` writer - SPI2 clock enable Set and cleared by software."]
pub type Spi2enW<'a, REG> = crate::BitWriter<'a, REG, Spi2en>;
impl<'a, REG> Spi2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2en::B0x1)
    }
}
#[doc = "SPI3 clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi3en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Spi3en> for bool {
    #[inline(always)]
    fn from(variant: Spi3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3EN` reader - SPI3 clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type Spi3enR = crate::BitReader<Spi3en>;
impl Spi3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi3en {
        match self.bits {
            false => Spi3en::B0x0,
            true => Spi3en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi3en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi3en::B0x1
    }
}
#[doc = "Field `SPI3EN` writer - SPI3 clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type Spi3enW<'a, REG> = crate::BitWriter<'a, REG, Spi3en>;
impl<'a, REG> Spi3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3en::B0x1)
    }
}
#[doc = "CRS clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crsen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Crsen> for bool {
    #[inline(always)]
    fn from(variant: Crsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSEN` reader - CRS clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type CrsenR = crate::BitReader<Crsen>;
impl CrsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crsen {
        match self.bits {
            false => Crsen::B0x0,
            true => Crsen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crsen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crsen::B0x1
    }
}
#[doc = "Field `CRSEN` writer - CRS clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type CrsenW<'a, REG> = crate::BitWriter<'a, REG, Crsen>;
impl<'a, REG> CrsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crsen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crsen::B0x1)
    }
}
#[doc = "USART2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart2en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usart2en> for bool {
    #[inline(always)]
    fn from(variant: Usart2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2EN` reader - USART2 clock enable Set and cleared by software."]
pub type Usart2enR = crate::BitReader<Usart2en>;
impl Usart2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart2en {
        match self.bits {
            false => Usart2en::B0x0,
            true => Usart2en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart2en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart2en::B0x1
    }
}
#[doc = "Field `USART2EN` writer - USART2 clock enable Set and cleared by software."]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG, Usart2en>;
impl<'a, REG> Usart2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2en::B0x1)
    }
}
#[doc = "USART3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart3en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usart3en> for bool {
    #[inline(always)]
    fn from(variant: Usart3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3EN` reader - USART3 clock enable Set and cleared by software."]
pub type Usart3enR = crate::BitReader<Usart3en>;
impl Usart3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart3en {
        match self.bits {
            false => Usart3en::B0x0,
            true => Usart3en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart3en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart3en::B0x1
    }
}
#[doc = "Field `USART3EN` writer - USART3 clock enable Set and cleared by software."]
pub type Usart3enW<'a, REG> = crate::BitWriter<'a, REG, Usart3en>;
impl<'a, REG> Usart3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3en::B0x1)
    }
}
#[doc = "USART4 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart4en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Usart4en> for bool {
    #[inline(always)]
    fn from(variant: Usart4en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART4EN` reader - USART4 clock enable Set and cleared by software."]
pub type Usart4enR = crate::BitReader<Usart4en>;
impl Usart4enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart4en {
        match self.bits {
            false => Usart4en::B0x0,
            true => Usart4en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart4en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart4en::B0x1
    }
}
#[doc = "Field `USART4EN` writer - USART4 clock enable Set and cleared by software."]
pub type Usart4enW<'a, REG> = crate::BitWriter<'a, REG, Usart4en>;
impl<'a, REG> Usart4enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart4en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart4en::B0x1)
    }
}
#[doc = "LPUART1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart1en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lpuart1en> for bool {
    #[inline(always)]
    fn from(variant: Lpuart1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable Set and cleared by software."]
pub type Lpuart1enR = crate::BitReader<Lpuart1en>;
impl Lpuart1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1en {
        match self.bits {
            false => Lpuart1en::B0x0,
            true => Lpuart1en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart1en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart1en::B0x1
    }
}
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable Set and cleared by software."]
pub type Lpuart1enW<'a, REG> = crate::BitWriter<'a, REG, Lpuart1en>;
impl<'a, REG> Lpuart1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1en::B0x1)
    }
}
#[doc = "I2C1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2c1en> for bool {
    #[inline(always)]
    fn from(variant: I2c1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1EN` reader - I2C1 clock enable Set and cleared by software."]
pub type I2c1enR = crate::BitReader<I2c1en>;
impl I2c1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1en {
        match self.bits {
            false => I2c1en::B0x0,
            true => I2c1en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c1en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c1en::B0x1
    }
}
#[doc = "Field `I2C1EN` writer - I2C1 clock enable Set and cleared by software."]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG, I2c1en>;
impl<'a, REG> I2c1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1en::B0x1)
    }
}
#[doc = "I2C2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2c2en> for bool {
    #[inline(always)]
    fn from(variant: I2c2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2EN` reader - I2C2 clock enable Set and cleared by software."]
pub type I2c2enR = crate::BitReader<I2c2en>;
impl I2c2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c2en {
        match self.bits {
            false => I2c2en::B0x0,
            true => I2c2en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c2en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c2en::B0x1
    }
}
#[doc = "Field `I2C2EN` writer - I2C2 clock enable Set and cleared by software."]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG, I2c2en>;
impl<'a, REG> I2c2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2en::B0x1)
    }
}
#[doc = "I2C3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c3en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2c3en> for bool {
    #[inline(always)]
    fn from(variant: I2c3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3EN` reader - I2C3 clock enable Set and cleared by software."]
pub type I2c3enR = crate::BitReader<I2c3en>;
impl I2c3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c3en {
        match self.bits {
            false => I2c3en::B0x0,
            true => I2c3en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c3en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c3en::B0x1
    }
}
#[doc = "Field `I2C3EN` writer - I2C3 clock enable Set and cleared by software."]
pub type I2c3enW<'a, REG> = crate::BitWriter<'a, REG, I2c3en>;
impl<'a, REG> I2c3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3en::B0x1)
    }
}
#[doc = "OPAMP clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opampen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Opampen> for bool {
    #[inline(always)]
    fn from(variant: Opampen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMPEN` reader - OPAMP clock enable Set and cleared by software."]
pub type OpampenR = crate::BitReader<Opampen>;
impl OpampenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opampen {
        match self.bits {
            false => Opampen::B0x0,
            true => Opampen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Opampen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Opampen::B0x1
    }
}
#[doc = "Field `OPAMPEN` writer - OPAMP clock enable Set and cleared by software."]
pub type OpampenW<'a, REG> = crate::BitWriter<'a, REG, Opampen>;
impl<'a, REG> OpampenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Opampen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Opampen::B0x1)
    }
}
#[doc = "I2C4EN clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c4en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<I2c4en> for bool {
    #[inline(always)]
    fn from(variant: I2c4en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C4EN` reader - I2C4EN clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type I2c4enR = crate::BitReader<I2c4en>;
impl I2c4enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c4en {
        match self.bits {
            false => I2c4en::B0x0,
            true => I2c4en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c4en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c4en::B0x1
    }
}
#[doc = "Field `I2C4EN` writer - I2C4EN clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type I2c4enW<'a, REG> = crate::BitWriter<'a, REG, I2c4en>;
impl<'a, REG> I2c4enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4en::B0x1)
    }
}
#[doc = "LPTIM3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim3en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lptim3en> for bool {
    #[inline(always)]
    fn from(variant: Lptim3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM3EN` reader - LPTIM3 clock enable Set and cleared by software."]
pub type Lptim3enR = crate::BitReader<Lptim3en>;
impl Lptim3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim3en {
        match self.bits {
            false => Lptim3en::B0x0,
            true => Lptim3en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim3en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim3en::B0x1
    }
}
#[doc = "Field `LPTIM3EN` writer - LPTIM3 clock enable Set and cleared by software."]
pub type Lptim3enW<'a, REG> = crate::BitWriter<'a, REG, Lptim3en>;
impl<'a, REG> Lptim3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3en::B0x1)
    }
}
#[doc = "Power interface clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwren {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Pwren> for bool {
    #[inline(always)]
    fn from(variant: Pwren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWREN` reader - Power interface clock enable Set and cleared by software."]
pub type PwrenR = crate::BitReader<Pwren>;
impl PwrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwren {
        match self.bits {
            false => Pwren::B0x0,
            true => Pwren::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pwren::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pwren::B0x1
    }
}
#[doc = "Field `PWREN` writer - Power interface clock enable Set and cleared by software."]
pub type PwrenW<'a, REG> = crate::BitWriter<'a, REG, Pwren>;
impl<'a, REG> PwrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwren::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwren::B0x1)
    }
}
#[doc = "DAC1 interface clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac1en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Dac1en> for bool {
    #[inline(always)]
    fn from(variant: Dac1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1EN` reader - DAC1 interface clock enable Set and cleared by software."]
pub type Dac1enR = crate::BitReader<Dac1en>;
impl Dac1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac1en {
        match self.bits {
            false => Dac1en::B0x0,
            true => Dac1en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac1en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac1en::B0x1
    }
}
#[doc = "Field `DAC1EN` writer - DAC1 interface clock enable Set and cleared by software."]
pub type Dac1enW<'a, REG> = crate::BitWriter<'a, REG, Dac1en>;
impl<'a, REG> Dac1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1en::B0x1)
    }
}
#[doc = "LPTIM2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim2en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lptim2en> for bool {
    #[inline(always)]
    fn from(variant: Lptim2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2EN` reader - LPTIM2 clock enable Set and cleared by software."]
pub type Lptim2enR = crate::BitReader<Lptim2en>;
impl Lptim2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim2en {
        match self.bits {
            false => Lptim2en::B0x0,
            true => Lptim2en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim2en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim2en::B0x1
    }
}
#[doc = "Field `LPTIM2EN` writer - LPTIM2 clock enable Set and cleared by software."]
pub type Lptim2enW<'a, REG> = crate::BitWriter<'a, REG, Lptim2en>;
impl<'a, REG> Lptim2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2en::B0x1)
    }
}
#[doc = "LPTIM1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim1en {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Lptim1en> for bool {
    #[inline(always)]
    fn from(variant: Lptim1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable Set and cleared by software."]
pub type Lptim1enR = crate::BitReader<Lptim1en>;
impl Lptim1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim1en {
        match self.bits {
            false => Lptim1en::B0x0,
            true => Lptim1en::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim1en::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim1en::B0x1
    }
}
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable Set and cleared by software."]
pub type Lptim1enW<'a, REG> = crate::BitWriter<'a, REG, Lptim1en>;
impl<'a, REG> Lptim1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1en::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1en::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim2en(&self) -> Tim2enR {
        Tim2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim3en(&self) -> Tim3enR {
        Tim3enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim6en(&self) -> Tim6enR {
        Tim6enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim7en(&self) -> Tim7enR {
        Tim7enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPUART2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart2en(&self) -> Lpuart2enR {
        Lpuart2enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn lcden(&self) -> LcdenR {
        LcdenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn rtcapben(&self) -> RtcapbenR {
        RtcapbenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
    #[inline(always)]
    pub fn wwdgen(&self) -> WwdgenR {
        WwdgenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPUART3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart3en(&self) -> Lpuart3enR {
        Lpuart3enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USB clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn usben(&self) -> UsbenR {
        UsbenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn spi3en(&self) -> Spi3enR {
        Spi3enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CRS clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn crsen(&self) -> CrsenR {
        CrsenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart3en(&self) -> Usart3enR {
        Usart3enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart4en(&self) -> Usart4enR {
        Usart4enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1en(&self) -> Lpuart1enR {
        Lpuart1enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2c3enR {
        I2c3enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OPAMP clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn opampen(&self) -> OpampenR {
        OpampenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2C4EN clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2c4enR {
        I2c4enR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LPTIM3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lptim3en(&self) -> Lptim3enR {
        Lptim3enR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn pwren(&self) -> PwrenR {
        PwrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac1en(&self) -> Dac1enR {
        Dac1enR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LPTIM2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lptim2en(&self) -> Lptim2enR {
        Lptim2enR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1en(&self) -> Lptim1enR {
        Lptim1enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> Tim2enW<RccApbenr1Spec> {
        Tim2enW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> Tim3enW<RccApbenr1Spec> {
        Tim3enW::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> Tim6enW<RccApbenr1Spec> {
        Tim6enW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> Tim7enW<RccApbenr1Spec> {
        Tim7enW::new(self, 5)
    }
    #[doc = "Bit 7 - LPUART2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2en(&mut self) -> Lpuart2enW<RccApbenr1Spec> {
        Lpuart2enW::new(self, 7)
    }
    #[doc = "Bit 9 - LCD clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LcdenW<RccApbenr1Spec> {
        LcdenW::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RtcapbenW<RccApbenr1Spec> {
        RtcapbenW::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WwdgenW<RccApbenr1Spec> {
        WwdgenW::new(self, 11)
    }
    #[doc = "Bit 12 - LPUART3 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart3en(&mut self) -> Lpuart3enW<RccApbenr1Spec> {
        Lpuart3enW::new(self, 12)
    }
    #[doc = "Bit 13 - USB clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> UsbenW<RccApbenr1Spec> {
        UsbenW::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> Spi2enW<RccApbenr1Spec> {
        Spi2enW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> Spi3enW<RccApbenr1Spec> {
        Spi3enW::new(self, 15)
    }
    #[doc = "Bit 16 - CRS clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CrsenW<RccApbenr1Spec> {
        CrsenW::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> Usart2enW<RccApbenr1Spec> {
        Usart2enW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> Usart3enW<RccApbenr1Spec> {
        Usart3enW::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart4en(&mut self) -> Usart4enW<RccApbenr1Spec> {
        Usart4enW::new(self, 19)
    }
    #[doc = "Bit 20 - LPUART1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> Lpuart1enW<RccApbenr1Spec> {
        Lpuart1enW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2c1enW<RccApbenr1Spec> {
        I2c1enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2c2enW<RccApbenr1Spec> {
        I2c2enW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2c3enW<RccApbenr1Spec> {
        I2c3enW::new(self, 23)
    }
    #[doc = "Bit 24 - OPAMP clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn opampen(&mut self) -> OpampenW<RccApbenr1Spec> {
        OpampenW::new(self, 24)
    }
    #[doc = "Bit 25 - I2C4EN clock enable&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2c4enW<RccApbenr1Spec> {
        I2c4enW::new(self, 25)
    }
    #[doc = "Bit 26 - LPTIM3 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> Lptim3enW<RccApbenr1Spec> {
        Lptim3enW::new(self, 26)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PwrenW<RccApbenr1Spec> {
        PwrenW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> Dac1enW<RccApbenr1Spec> {
        Dac1enW::new(self, 29)
    }
    #[doc = "Bit 30 - LPTIM2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> Lptim2enW<RccApbenr1Spec> {
        Lptim2enW::new(self, 30)
    }
    #[doc = "Bit 31 - LPTIM1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> Lptim1enW<RccApbenr1Spec> {
        Lptim1enW::new(self, 31)
    }
}
#[doc = "APB peripheral clock enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApbenr1Spec;
impl crate::RegisterSpec for RccApbenr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apbenr1::R`](R) reader structure"]
impl crate::Readable for RccApbenr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apbenr1::W`](W) writer structure"]
impl crate::Writable for RccApbenr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APBENR1 to value 0"]
impl crate::Resettable for RccApbenr1Spec {
    const RESET_VALUE: u32 = 0;
}
