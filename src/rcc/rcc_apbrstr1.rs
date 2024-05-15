#[doc = "Register `RCC_APBRSTR1` reader"]
pub type R = crate::R<RccApbrstr1Spec>;
#[doc = "Register `RCC_APBRSTR1` writer"]
pub type W = crate::W<RccApbrstr1Spec>;
#[doc = "TIM2 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM2"]
    B0x1 = 1,
}
impl From<Tim2rst> for bool {
    #[inline(always)]
    fn from(variant: Tim2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2RST` reader - TIM2 timer reset Set and cleared by software."]
pub type Tim2rstR = crate::BitReader<Tim2rst>;
impl Tim2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim2rst {
        match self.bits {
            false => Tim2rst::B0x0,
            true => Tim2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim2rst::B0x0
    }
    #[doc = "Reset TIM2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim2rst::B0x1
    }
}
#[doc = "Field `TIM2RST` writer - TIM2 timer reset Set and cleared by software."]
pub type Tim2rstW<'a, REG> = crate::BitWriter<'a, REG, Tim2rst>;
impl<'a, REG> Tim2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2rst::B0x0)
    }
    #[doc = "Reset TIM2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2rst::B0x1)
    }
}
#[doc = "TIM3 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM3"]
    B0x1 = 1,
}
impl From<Tim3rst> for bool {
    #[inline(always)]
    fn from(variant: Tim3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3RST` reader - TIM3 timer reset Set and cleared by software."]
pub type Tim3rstR = crate::BitReader<Tim3rst>;
impl Tim3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim3rst {
        match self.bits {
            false => Tim3rst::B0x0,
            true => Tim3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim3rst::B0x0
    }
    #[doc = "Reset TIM3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim3rst::B0x1
    }
}
#[doc = "Field `TIM3RST` writer - TIM3 timer reset Set and cleared by software."]
pub type Tim3rstW<'a, REG> = crate::BitWriter<'a, REG, Tim3rst>;
impl<'a, REG> Tim3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3rst::B0x0)
    }
    #[doc = "Reset TIM3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3rst::B0x1)
    }
}
#[doc = "TIM6 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim6rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM6"]
    B0x1 = 1,
}
impl From<Tim6rst> for bool {
    #[inline(always)]
    fn from(variant: Tim6rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6RST` reader - TIM6 timer reset Set and cleared by software."]
pub type Tim6rstR = crate::BitReader<Tim6rst>;
impl Tim6rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim6rst {
        match self.bits {
            false => Tim6rst::B0x0,
            true => Tim6rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim6rst::B0x0
    }
    #[doc = "Reset TIM6"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim6rst::B0x1
    }
}
#[doc = "Field `TIM6RST` writer - TIM6 timer reset Set and cleared by software."]
pub type Tim6rstW<'a, REG> = crate::BitWriter<'a, REG, Tim6rst>;
impl<'a, REG> Tim6rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6rst::B0x0)
    }
    #[doc = "Reset TIM6"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6rst::B0x1)
    }
}
#[doc = "TIM7 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim7rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM7"]
    B0x1 = 1,
}
impl From<Tim7rst> for bool {
    #[inline(always)]
    fn from(variant: Tim7rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7RST` reader - TIM7 timer reset Set and cleared by software."]
pub type Tim7rstR = crate::BitReader<Tim7rst>;
impl Tim7rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim7rst {
        match self.bits {
            false => Tim7rst::B0x0,
            true => Tim7rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim7rst::B0x0
    }
    #[doc = "Reset TIM7"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim7rst::B0x1
    }
}
#[doc = "Field `TIM7RST` writer - TIM7 timer reset Set and cleared by software."]
pub type Tim7rstW<'a, REG> = crate::BitWriter<'a, REG, Tim7rst>;
impl<'a, REG> Tim7rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7rst::B0x0)
    }
    #[doc = "Reset TIM7"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7rst::B0x1)
    }
}
#[doc = "LPUART2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset LPUART2"]
    B0x1 = 1,
}
impl From<Lpuart2rst> for bool {
    #[inline(always)]
    fn from(variant: Lpuart2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART2RST` reader - LPUART2 reset Set and cleared by software."]
pub type Lpuart2rstR = crate::BitReader<Lpuart2rst>;
impl Lpuart2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart2rst {
        match self.bits {
            false => Lpuart2rst::B0x0,
            true => Lpuart2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart2rst::B0x0
    }
    #[doc = "Reset LPUART2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart2rst::B0x1
    }
}
#[doc = "Field `LPUART2RST` writer - LPUART2 reset Set and cleared by software."]
pub type Lpuart2rstW<'a, REG> = crate::BitWriter<'a, REG, Lpuart2rst>;
impl<'a, REG> Lpuart2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2rst::B0x0)
    }
    #[doc = "Reset LPUART2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2rst::B0x1)
    }
}
#[doc = "LCD reset&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset LCD"]
    B0x1 = 1,
}
impl From<Lcdrst> for bool {
    #[inline(always)]
    fn from(variant: Lcdrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDRST` reader - LCD reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type LcdrstR = crate::BitReader<Lcdrst>;
impl LcdrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdrst {
        match self.bits {
            false => Lcdrst::B0x0,
            true => Lcdrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lcdrst::B0x0
    }
    #[doc = "Reset LCD"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lcdrst::B0x1
    }
}
#[doc = "Field `LCDRST` writer - LCD reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type LcdrstW<'a, REG> = crate::BitWriter<'a, REG, Lcdrst>;
impl<'a, REG> LcdrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdrst::B0x0)
    }
    #[doc = "Reset LCD"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdrst::B0x1)
    }
}
#[doc = "LPUART3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset LPUART3"]
    B0x1 = 1,
}
impl From<Lpuart3rst> for bool {
    #[inline(always)]
    fn from(variant: Lpuart3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART3RST` reader - LPUART3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type Lpuart3rstR = crate::BitReader<Lpuart3rst>;
impl Lpuart3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart3rst {
        match self.bits {
            false => Lpuart3rst::B0x0,
            true => Lpuart3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart3rst::B0x0
    }
    #[doc = "Reset LPUART3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart3rst::B0x1
    }
}
#[doc = "Field `LPUART3RST` writer - LPUART3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type Lpuart3rstW<'a, REG> = crate::BitWriter<'a, REG, Lpuart3rst>;
impl<'a, REG> Lpuart3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3rst::B0x0)
    }
    #[doc = "Reset LPUART3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3rst::B0x1)
    }
}
#[doc = "USB reset&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset USB"]
    B0x1 = 1,
}
impl From<Usbrst> for bool {
    #[inline(always)]
    fn from(variant: Usbrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRST` reader - USB reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type UsbrstR = crate::BitReader<Usbrst>;
impl UsbrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrst {
        match self.bits {
            false => Usbrst::B0x0,
            true => Usbrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usbrst::B0x0
    }
    #[doc = "Reset USB"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usbrst::B0x1
    }
}
#[doc = "Field `USBRST` writer - USB reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG, Usbrst>;
impl<'a, REG> UsbrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::B0x0)
    }
    #[doc = "Reset USB"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::B0x1)
    }
}
#[doc = "SPI2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SPI2"]
    B0x1 = 1,
}
impl From<Spi2rst> for bool {
    #[inline(always)]
    fn from(variant: Spi2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2RST` reader - SPI2 reset Set and cleared by software."]
pub type Spi2rstR = crate::BitReader<Spi2rst>;
impl Spi2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi2rst {
        match self.bits {
            false => Spi2rst::B0x0,
            true => Spi2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi2rst::B0x0
    }
    #[doc = "Reset SPI2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi2rst::B0x1
    }
}
#[doc = "Field `SPI2RST` writer - SPI2 reset Set and cleared by software."]
pub type Spi2rstW<'a, REG> = crate::BitWriter<'a, REG, Spi2rst>;
impl<'a, REG> Spi2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2rst::B0x0)
    }
    #[doc = "Reset SPI2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2rst::B0x1)
    }
}
#[doc = "SPI3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SPI3"]
    B0x1 = 1,
}
impl From<Spi3rst> for bool {
    #[inline(always)]
    fn from(variant: Spi3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3RST` reader - SPI3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type Spi3rstR = crate::BitReader<Spi3rst>;
impl Spi3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi3rst {
        match self.bits {
            false => Spi3rst::B0x0,
            true => Spi3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi3rst::B0x0
    }
    #[doc = "Reset SPI3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi3rst::B0x1
    }
}
#[doc = "Field `SPI3RST` writer - SPI3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type Spi3rstW<'a, REG> = crate::BitWriter<'a, REG, Spi3rst>;
impl<'a, REG> Spi3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3rst::B0x0)
    }
    #[doc = "Reset SPI3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3rst::B0x1)
    }
}
#[doc = "CRS reset&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crsrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset CRS"]
    B0x1 = 1,
}
impl From<Crsrst> for bool {
    #[inline(always)]
    fn from(variant: Crsrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSRST` reader - CRS reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type CrsrstR = crate::BitReader<Crsrst>;
impl CrsrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crsrst {
        match self.bits {
            false => Crsrst::B0x0,
            true => Crsrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crsrst::B0x0
    }
    #[doc = "Reset CRS"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crsrst::B0x1
    }
}
#[doc = "Field `CRSRST` writer - CRS reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type CrsrstW<'a, REG> = crate::BitWriter<'a, REG, Crsrst>;
impl<'a, REG> CrsrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crsrst::B0x0)
    }
    #[doc = "Reset CRS"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crsrst::B0x1)
    }
}
#[doc = "USART2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset USART2"]
    B0x1 = 1,
}
impl From<Usart2rst> for bool {
    #[inline(always)]
    fn from(variant: Usart2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2RST` reader - USART2 reset Set and cleared by software."]
pub type Usart2rstR = crate::BitReader<Usart2rst>;
impl Usart2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart2rst {
        match self.bits {
            false => Usart2rst::B0x0,
            true => Usart2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart2rst::B0x0
    }
    #[doc = "Reset USART2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart2rst::B0x1
    }
}
#[doc = "Field `USART2RST` writer - USART2 reset Set and cleared by software."]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG, Usart2rst>;
impl<'a, REG> Usart2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2rst::B0x0)
    }
    #[doc = "Reset USART2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2rst::B0x1)
    }
}
#[doc = "USART3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset USART3"]
    B0x1 = 1,
}
impl From<Usart3rst> for bool {
    #[inline(always)]
    fn from(variant: Usart3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3RST` reader - USART3 reset Set and cleared by software."]
pub type Usart3rstR = crate::BitReader<Usart3rst>;
impl Usart3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart3rst {
        match self.bits {
            false => Usart3rst::B0x0,
            true => Usart3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart3rst::B0x0
    }
    #[doc = "Reset USART3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart3rst::B0x1
    }
}
#[doc = "Field `USART3RST` writer - USART3 reset Set and cleared by software."]
pub type Usart3rstW<'a, REG> = crate::BitWriter<'a, REG, Usart3rst>;
impl<'a, REG> Usart3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3rst::B0x0)
    }
    #[doc = "Reset USART3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3rst::B0x1)
    }
}
#[doc = "USART4 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart4rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset USART4"]
    B0x1 = 1,
}
impl From<Usart4rst> for bool {
    #[inline(always)]
    fn from(variant: Usart4rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART4RST` reader - USART4 reset Set and cleared by software."]
pub type Usart4rstR = crate::BitReader<Usart4rst>;
impl Usart4rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart4rst {
        match self.bits {
            false => Usart4rst::B0x0,
            true => Usart4rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart4rst::B0x0
    }
    #[doc = "Reset USART4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart4rst::B0x1
    }
}
#[doc = "Field `USART4RST` writer - USART4 reset Set and cleared by software."]
pub type Usart4rstW<'a, REG> = crate::BitWriter<'a, REG, Usart4rst>;
impl<'a, REG> Usart4rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart4rst::B0x0)
    }
    #[doc = "Reset USART4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart4rst::B0x1)
    }
}
#[doc = "LPUART1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset LPUART1"]
    B0x1 = 1,
}
impl From<Lpuart1rst> for bool {
    #[inline(always)]
    fn from(variant: Lpuart1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1RST` reader - LPUART1 reset Set and cleared by software."]
pub type Lpuart1rstR = crate::BitReader<Lpuart1rst>;
impl Lpuart1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1rst {
        match self.bits {
            false => Lpuart1rst::B0x0,
            true => Lpuart1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart1rst::B0x0
    }
    #[doc = "Reset LPUART1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart1rst::B0x1
    }
}
#[doc = "Field `LPUART1RST` writer - LPUART1 reset Set and cleared by software."]
pub type Lpuart1rstW<'a, REG> = crate::BitWriter<'a, REG, Lpuart1rst>;
impl<'a, REG> Lpuart1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1rst::B0x0)
    }
    #[doc = "Reset LPUART1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1rst::B0x1)
    }
}
#[doc = "I2C1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset I2C1"]
    B0x1 = 1,
}
impl From<I2c1rst> for bool {
    #[inline(always)]
    fn from(variant: I2c1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1RST` reader - I2C1 reset Set and cleared by software."]
pub type I2c1rstR = crate::BitReader<I2c1rst>;
impl I2c1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1rst {
        match self.bits {
            false => I2c1rst::B0x0,
            true => I2c1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c1rst::B0x0
    }
    #[doc = "Reset I2C1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c1rst::B0x1
    }
}
#[doc = "Field `I2C1RST` writer - I2C1 reset Set and cleared by software."]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG, I2c1rst>;
impl<'a, REG> I2c1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1rst::B0x0)
    }
    #[doc = "Reset I2C1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1rst::B0x1)
    }
}
#[doc = "I2C2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset I2C2"]
    B0x1 = 1,
}
impl From<I2c2rst> for bool {
    #[inline(always)]
    fn from(variant: I2c2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2RST` reader - I2C2 reset Set and cleared by software."]
pub type I2c2rstR = crate::BitReader<I2c2rst>;
impl I2c2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c2rst {
        match self.bits {
            false => I2c2rst::B0x0,
            true => I2c2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c2rst::B0x0
    }
    #[doc = "Reset I2C2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c2rst::B0x1
    }
}
#[doc = "Field `I2C2RST` writer - I2C2 reset Set and cleared by software."]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG, I2c2rst>;
impl<'a, REG> I2c2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2rst::B0x0)
    }
    #[doc = "Reset I2C2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2rst::B0x1)
    }
}
#[doc = "I2C3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset I2C3"]
    B0x1 = 1,
}
impl From<I2c3rst> for bool {
    #[inline(always)]
    fn from(variant: I2c3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3RST` reader - I2C3 reset Set and cleared by software."]
pub type I2c3rstR = crate::BitReader<I2c3rst>;
impl I2c3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c3rst {
        match self.bits {
            false => I2c3rst::B0x0,
            true => I2c3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c3rst::B0x0
    }
    #[doc = "Reset I2C3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c3rst::B0x1
    }
}
#[doc = "Field `I2C3RST` writer - I2C3 reset Set and cleared by software."]
pub type I2c3rstW<'a, REG> = crate::BitWriter<'a, REG, I2c3rst>;
impl<'a, REG> I2c3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3rst::B0x0)
    }
    #[doc = "Reset I2C3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3rst::B0x1)
    }
}
#[doc = "OPAMP reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opamprst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset the OPAMP"]
    B0x1 = 1,
}
impl From<Opamprst> for bool {
    #[inline(always)]
    fn from(variant: Opamprst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMPRST` reader - OPAMP reset Set and cleared by software."]
pub type OpamprstR = crate::BitReader<Opamprst>;
impl OpamprstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opamprst {
        match self.bits {
            false => Opamprst::B0x0,
            true => Opamprst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Opamprst::B0x0
    }
    #[doc = "Reset the OPAMP"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Opamprst::B0x1
    }
}
#[doc = "Field `OPAMPRST` writer - OPAMP reset Set and cleared by software."]
pub type OpamprstW<'a, REG> = crate::BitWriter<'a, REG, Opamprst>;
impl<'a, REG> OpamprstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Opamprst::B0x0)
    }
    #[doc = "Reset the OPAMP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Opamprst::B0x1)
    }
}
#[doc = "I2C4 reset&lt;sup>(1)&lt;/sup> Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c4rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset I2C4"]
    B0x1 = 1,
}
impl From<I2c4rst> for bool {
    #[inline(always)]
    fn from(variant: I2c4rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C4RST` reader - I2C4 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type I2c4rstR = crate::BitReader<I2c4rst>;
impl I2c4rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c4rst {
        match self.bits {
            false => I2c4rst::B0x0,
            true => I2c4rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c4rst::B0x0
    }
    #[doc = "Reset I2C4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c4rst::B0x1
    }
}
#[doc = "Field `I2C4RST` writer - I2C4 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
pub type I2c4rstW<'a, REG> = crate::BitWriter<'a, REG, I2c4rst>;
impl<'a, REG> I2c4rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4rst::B0x0)
    }
    #[doc = "Reset I2C4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4rst::B0x1)
    }
}
#[doc = "LPTIM3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset LPTIM3"]
    B0x1 = 1,
}
impl From<Lptim3rst> for bool {
    #[inline(always)]
    fn from(variant: Lptim3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM3RST` reader - LPTIM3 reset Set and cleared by software."]
pub type Lptim3rstR = crate::BitReader<Lptim3rst>;
impl Lptim3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim3rst {
        match self.bits {
            false => Lptim3rst::B0x0,
            true => Lptim3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim3rst::B0x0
    }
    #[doc = "Reset LPTIM3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim3rst::B0x1
    }
}
#[doc = "Field `LPTIM3RST` writer - LPTIM3 reset Set and cleared by software."]
pub type Lptim3rstW<'a, REG> = crate::BitWriter<'a, REG, Lptim3rst>;
impl<'a, REG> Lptim3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3rst::B0x0)
    }
    #[doc = "Reset LPTIM3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3rst::B0x1)
    }
}
#[doc = "Power interface reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset PWR"]
    B0x1 = 1,
}
impl From<Pwrrst> for bool {
    #[inline(always)]
    fn from(variant: Pwrrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRRST` reader - Power interface reset Set and cleared by software."]
pub type PwrrstR = crate::BitReader<Pwrrst>;
impl PwrrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrrst {
        match self.bits {
            false => Pwrrst::B0x0,
            true => Pwrrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pwrrst::B0x0
    }
    #[doc = "Reset PWR"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pwrrst::B0x1
    }
}
#[doc = "Field `PWRRST` writer - Power interface reset Set and cleared by software."]
pub type PwrrstW<'a, REG> = crate::BitWriter<'a, REG, Pwrrst>;
impl<'a, REG> PwrrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrrst::B0x0)
    }
    #[doc = "Reset PWR"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrrst::B0x1)
    }
}
#[doc = "DAC1 interface reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DAC1 interface"]
    B0x1 = 1,
}
impl From<Dac1rst> for bool {
    #[inline(always)]
    fn from(variant: Dac1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1RST` reader - DAC1 interface reset Set and cleared by software."]
pub type Dac1rstR = crate::BitReader<Dac1rst>;
impl Dac1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac1rst {
        match self.bits {
            false => Dac1rst::B0x0,
            true => Dac1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac1rst::B0x0
    }
    #[doc = "Reset DAC1 interface"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac1rst::B0x1
    }
}
#[doc = "Field `DAC1RST` writer - DAC1 interface reset Set and cleared by software."]
pub type Dac1rstW<'a, REG> = crate::BitWriter<'a, REG, Dac1rst>;
impl<'a, REG> Dac1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1rst::B0x0)
    }
    #[doc = "Reset DAC1 interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1rst::B0x1)
    }
}
#[doc = "Low Power Timer 2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset LPTIM2"]
    B0x1 = 1,
}
impl From<Lptim2rst> for bool {
    #[inline(always)]
    fn from(variant: Lptim2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2RST` reader - Low Power Timer 2 reset Set and cleared by software."]
pub type Lptim2rstR = crate::BitReader<Lptim2rst>;
impl Lptim2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim2rst {
        match self.bits {
            false => Lptim2rst::B0x0,
            true => Lptim2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim2rst::B0x0
    }
    #[doc = "Reset LPTIM2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim2rst::B0x1
    }
}
#[doc = "Field `LPTIM2RST` writer - Low Power Timer 2 reset Set and cleared by software."]
pub type Lptim2rstW<'a, REG> = crate::BitWriter<'a, REG, Lptim2rst>;
impl<'a, REG> Lptim2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2rst::B0x0)
    }
    #[doc = "Reset LPTIM2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2rst::B0x1)
    }
}
#[doc = "Low Power Timer 1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset LPTIM1"]
    B0x1 = 1,
}
impl From<Lptim1rst> for bool {
    #[inline(always)]
    fn from(variant: Lptim1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1RST` reader - Low Power Timer 1 reset Set and cleared by software."]
pub type Lptim1rstR = crate::BitReader<Lptim1rst>;
impl Lptim1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim1rst {
        match self.bits {
            false => Lptim1rst::B0x0,
            true => Lptim1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim1rst::B0x0
    }
    #[doc = "Reset LPTIM1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim1rst::B0x1
    }
}
#[doc = "Field `LPTIM1RST` writer - Low Power Timer 1 reset Set and cleared by software."]
pub type Lptim1rstW<'a, REG> = crate::BitWriter<'a, REG, Lptim1rst>;
impl<'a, REG> Lptim1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1rst::B0x0)
    }
    #[doc = "Reset LPTIM1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1rst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim2rst(&self) -> Tim2rstR {
        Tim2rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim3rst(&self) -> Tim3rstR {
        Tim3rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim6rst(&self) -> Tim6rstR {
        Tim6rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim7rst(&self) -> Tim7rstR {
        Tim7rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPUART2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart2rst(&self) -> Lpuart2rstR {
        Lpuart2rstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn lcdrst(&self) -> LcdrstR {
        LcdrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - LPUART3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart3rst(&self) -> Lpuart3rstR {
        Lpuart3rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USB reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi2rst(&self) -> Spi2rstR {
        Spi2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn spi3rst(&self) -> Spi3rstR {
        Spi3rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CRS reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn crsrst(&self) -> CrsrstR {
        CrsrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart3rst(&self) -> Usart3rstR {
        Usart3rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart4rst(&self) -> Usart4rstR {
        Usart4rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> Lpuart1rstR {
        Lpuart1rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2c3rstR {
        I2c3rstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OPAMP reset Set and cleared by software."]
    #[inline(always)]
    pub fn opamprst(&self) -> OpamprstR {
        OpamprstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2C4 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2c4rstR {
        I2c4rstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LPTIM3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn lptim3rst(&self) -> Lptim3rstR {
        Lptim3rstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset Set and cleared by software."]
    #[inline(always)]
    pub fn pwrrst(&self) -> PwrrstR {
        PwrrstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac1rst(&self) -> Dac1rstR {
        Dac1rstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low Power Timer 2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn lptim2rst(&self) -> Lptim2rstR {
        Lptim2rstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1rst(&self) -> Lptim1rstR {
        Lptim1rstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> Tim2rstW<RccApbrstr1Spec> {
        Tim2rstW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> Tim3rstW<RccApbrstr1Spec> {
        Tim3rstW::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> Tim6rstW<RccApbrstr1Spec> {
        Tim6rstW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> Tim7rstW<RccApbrstr1Spec> {
        Tim7rstW::new(self, 5)
    }
    #[doc = "Bit 7 - LPUART2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2rst(&mut self) -> Lpuart2rstW<RccApbrstr1Spec> {
        Lpuart2rstW::new(self, 7)
    }
    #[doc = "Bit 9 - LCD reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lcdrst(&mut self) -> LcdrstW<RccApbrstr1Spec> {
        LcdrstW::new(self, 9)
    }
    #[doc = "Bit 12 - LPUART3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart3rst(&mut self) -> Lpuart3rstW<RccApbrstr1Spec> {
        Lpuart3rstW::new(self, 12)
    }
    #[doc = "Bit 13 - USB reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<RccApbrstr1Spec> {
        UsbrstW::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> Spi2rstW<RccApbrstr1Spec> {
        Spi2rstW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> Spi3rstW<RccApbrstr1Spec> {
        Spi3rstW::new(self, 15)
    }
    #[doc = "Bit 16 - CRS reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CrsrstW<RccApbrstr1Spec> {
        CrsrstW::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> Usart2rstW<RccApbrstr1Spec> {
        Usart2rstW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> Usart3rstW<RccApbrstr1Spec> {
        Usart3rstW::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart4rst(&mut self) -> Usart4rstW<RccApbrstr1Spec> {
        Usart4rstW::new(self, 19)
    }
    #[doc = "Bit 20 - LPUART1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> Lpuart1rstW<RccApbrstr1Spec> {
        Lpuart1rstW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2c1rstW<RccApbrstr1Spec> {
        I2c1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2c2rstW<RccApbrstr1Spec> {
        I2c2rstW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2c3rstW<RccApbrstr1Spec> {
        I2c3rstW::new(self, 23)
    }
    #[doc = "Bit 24 - OPAMP reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OpamprstW<RccApbrstr1Spec> {
        OpamprstW::new(self, 24)
    }
    #[doc = "Bit 25 - I2C4 reset&lt;sup>(1)&lt;/sup> Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2c4rstW<RccApbrstr1Spec> {
        I2c4rstW::new(self, 25)
    }
    #[doc = "Bit 26 - LPTIM3 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> Lptim3rstW<RccApbrstr1Spec> {
        Lptim3rstW::new(self, 26)
    }
    #[doc = "Bit 28 - Power interface reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PwrrstW<RccApbrstr1Spec> {
        PwrrstW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> Dac1rstW<RccApbrstr1Spec> {
        Dac1rstW::new(self, 29)
    }
    #[doc = "Bit 30 - Low Power Timer 2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> Lptim2rstW<RccApbrstr1Spec> {
        Lptim2rstW::new(self, 30)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> Lptim1rstW<RccApbrstr1Spec> {
        Lptim1rstW::new(self, 31)
    }
}
#[doc = "APB peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apbrstr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apbrstr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApbrstr1Spec;
impl crate::RegisterSpec for RccApbrstr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apbrstr1::R`](R) reader structure"]
impl crate::Readable for RccApbrstr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apbrstr1::W`](W) writer structure"]
impl crate::Writable for RccApbrstr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APBRSTR1 to value 0"]
impl crate::Resettable for RccApbrstr1Spec {
    const RESET_VALUE: u32 = 0;
}
