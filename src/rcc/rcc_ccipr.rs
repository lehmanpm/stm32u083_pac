#[doc = "Register `RCC_CCIPR` reader"]
pub type R = crate::R<RccCciprSpec>;
#[doc = "Register `RCC_CCIPR` writer"]
pub type W = crate::W<RccCciprSpec>;
#[doc = "USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart1sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: SYSCLK"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
    #[doc = "3: LSE"]
    B0x3 = 3,
}
impl From<Usart1sel> for u8 {
    #[inline(always)]
    fn from(variant: Usart1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usart1sel {
    type Ux = u8;
}
impl crate::IsEnum for Usart1sel {}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:"]
pub type Usart1selR = crate::FieldReader<Usart1sel>;
impl Usart1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1sel {
        match self.bits {
            0 => Usart1sel::B0x0,
            1 => Usart1sel::B0x1,
            2 => Usart1sel::B0x2,
            3 => Usart1sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart1sel::B0x0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart1sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Usart1sel::B0x2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Usart1sel::B0x3
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:"]
pub type Usart1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usart1sel, crate::Safe>;
impl<'a, REG> Usart1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::B0x0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::B0x2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::B0x3)
    }
}
#[doc = "USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart2sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: SYSCLK"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
    #[doc = "3: LSE"]
    B0x3 = 3,
}
impl From<Usart2sel> for u8 {
    #[inline(always)]
    fn from(variant: Usart2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usart2sel {
    type Ux = u8;
}
impl crate::IsEnum for Usart2sel {}
#[doc = "Field `USART2SEL` reader - USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:"]
pub type Usart2selR = crate::FieldReader<Usart2sel>;
impl Usart2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart2sel {
        match self.bits {
            0 => Usart2sel::B0x0,
            1 => Usart2sel::B0x1,
            2 => Usart2sel::B0x2,
            3 => Usart2sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart2sel::B0x0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart2sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Usart2sel::B0x2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Usart2sel::B0x3
    }
}
#[doc = "Field `USART2SEL` writer - USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:"]
pub type Usart2selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usart2sel, crate::Safe>;
impl<'a, REG> Usart2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2sel::B0x0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2sel::B0x2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2sel::B0x3)
    }
}
#[doc = "LPUART3 clock source selection&lt;sup>(1)&lt;/sup> This bitfield is controlled by software to select LPUART3 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpuart3sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: SYSCLK"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
    #[doc = "3: LSE"]
    B0x3 = 3,
}
impl From<Lpuart3sel> for u8 {
    #[inline(always)]
    fn from(variant: Lpuart3sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpuart3sel {
    type Ux = u8;
}
impl crate::IsEnum for Lpuart3sel {}
#[doc = "Field `LPUART3SEL` reader - LPUART3 clock source selection&lt;sup>(1)&lt;/sup> This bitfield is controlled by software to select LPUART3 clock source as follows:"]
pub type Lpuart3selR = crate::FieldReader<Lpuart3sel>;
impl Lpuart3selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart3sel {
        match self.bits {
            0 => Lpuart3sel::B0x0,
            1 => Lpuart3sel::B0x1,
            2 => Lpuart3sel::B0x2,
            3 => Lpuart3sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart3sel::B0x0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart3sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lpuart3sel::B0x2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lpuart3sel::B0x3
    }
}
#[doc = "Field `LPUART3SEL` writer - LPUART3 clock source selection&lt;sup>(1)&lt;/sup> This bitfield is controlled by software to select LPUART3 clock source as follows:"]
pub type Lpuart3selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpuart3sel, crate::Safe>;
impl<'a, REG> Lpuart3selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3sel::B0x0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3sel::B0x2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart3sel::B0x3)
    }
}
#[doc = "LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpuart2sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: SYSCLK"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
    #[doc = "3: LSE"]
    B0x3 = 3,
}
impl From<Lpuart2sel> for u8 {
    #[inline(always)]
    fn from(variant: Lpuart2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpuart2sel {
    type Ux = u8;
}
impl crate::IsEnum for Lpuart2sel {}
#[doc = "Field `LPUART2SEL` reader - LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:"]
pub type Lpuart2selR = crate::FieldReader<Lpuart2sel>;
impl Lpuart2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart2sel {
        match self.bits {
            0 => Lpuart2sel::B0x0,
            1 => Lpuart2sel::B0x1,
            2 => Lpuart2sel::B0x2,
            3 => Lpuart2sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart2sel::B0x0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart2sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lpuart2sel::B0x2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lpuart2sel::B0x3
    }
}
#[doc = "Field `LPUART2SEL` writer - LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:"]
pub type Lpuart2selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpuart2sel, crate::Safe>;
impl<'a, REG> Lpuart2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2sel::B0x0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2sel::B0x2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart2sel::B0x3)
    }
}
#[doc = "LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpuart1sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: SYSCLK"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
    #[doc = "3: LSE"]
    B0x3 = 3,
}
impl From<Lpuart1sel> for u8 {
    #[inline(always)]
    fn from(variant: Lpuart1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpuart1sel {
    type Ux = u8;
}
impl crate::IsEnum for Lpuart1sel {}
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:"]
pub type Lpuart1selR = crate::FieldReader<Lpuart1sel>;
impl Lpuart1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1sel {
        match self.bits {
            0 => Lpuart1sel::B0x0,
            1 => Lpuart1sel::B0x1,
            2 => Lpuart1sel::B0x2,
            3 => Lpuart1sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart1sel::B0x0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart1sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lpuart1sel::B0x2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lpuart1sel::B0x3
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:"]
pub type Lpuart1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpuart1sel, crate::Safe>;
impl<'a, REG> Lpuart1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1sel::B0x0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1sel::B0x2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1sel::B0x3)
    }
}
#[doc = "I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2c1sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: SYSCLK"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
}
impl From<I2c1sel> for u8 {
    #[inline(always)]
    fn from(variant: I2c1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2c1sel {
    type Ux = u8;
}
impl crate::IsEnum for I2c1sel {}
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:"]
pub type I2c1selR = crate::FieldReader<I2c1sel>;
impl I2c1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2c1sel> {
        match self.bits {
            0 => Some(I2c1sel::B0x0),
            1 => Some(I2c1sel::B0x1),
            2 => Some(I2c1sel::B0x2),
            _ => None,
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c1sel::B0x0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c1sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2c1sel::B0x2
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:"]
pub type I2c1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2c1sel>;
impl<'a, REG> I2c1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1sel::B0x0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1sel::B0x2)
    }
}
#[doc = "I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2c3sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: SYSCLK"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
}
impl From<I2c3sel> for u8 {
    #[inline(always)]
    fn from(variant: I2c3sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2c3sel {
    type Ux = u8;
}
impl crate::IsEnum for I2c3sel {}
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:"]
pub type I2c3selR = crate::FieldReader<I2c3sel>;
impl I2c3selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2c3sel> {
        match self.bits {
            0 => Some(I2c3sel::B0x0),
            1 => Some(I2c3sel::B0x1),
            2 => Some(I2c3sel::B0x2),
            _ => None,
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c3sel::B0x0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c3sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2c3sel::B0x2
    }
}
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:"]
pub type I2c3selW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2c3sel>;
impl<'a, REG> I2c3selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3sel::B0x0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3sel::B0x2)
    }
}
#[doc = "LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lptim1sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: LSI"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
    #[doc = "3: LSE"]
    B0x3 = 3,
}
impl From<Lptim1sel> for u8 {
    #[inline(always)]
    fn from(variant: Lptim1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lptim1sel {
    type Ux = u8;
}
impl crate::IsEnum for Lptim1sel {}
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:"]
pub type Lptim1selR = crate::FieldReader<Lptim1sel>;
impl Lptim1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim1sel {
        match self.bits {
            0 => Lptim1sel::B0x0,
            1 => Lptim1sel::B0x1,
            2 => Lptim1sel::B0x2,
            3 => Lptim1sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim1sel::B0x0
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim1sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lptim1sel::B0x2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lptim1sel::B0x3
    }
}
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:"]
pub type Lptim1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lptim1sel, crate::Safe>;
impl<'a, REG> Lptim1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1sel::B0x0)
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1sel::B0x2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1sel::B0x3)
    }
}
#[doc = "LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lptim2sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: LSI"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
    #[doc = "3: LSE"]
    B0x3 = 3,
}
impl From<Lptim2sel> for u8 {
    #[inline(always)]
    fn from(variant: Lptim2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lptim2sel {
    type Ux = u8;
}
impl crate::IsEnum for Lptim2sel {}
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:"]
pub type Lptim2selR = crate::FieldReader<Lptim2sel>;
impl Lptim2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim2sel {
        match self.bits {
            0 => Lptim2sel::B0x0,
            1 => Lptim2sel::B0x1,
            2 => Lptim2sel::B0x2,
            3 => Lptim2sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim2sel::B0x0
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim2sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lptim2sel::B0x2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lptim2sel::B0x3
    }
}
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:"]
pub type Lptim2selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lptim2sel, crate::Safe>;
impl<'a, REG> Lptim2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2sel::B0x0)
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2sel::B0x2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim2sel::B0x3)
    }
}
#[doc = "LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lptim3sel {
    #[doc = "0: PCLK"]
    B0x0 = 0,
    #[doc = "1: LSI"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
    #[doc = "3: LSE"]
    B0x3 = 3,
}
impl From<Lptim3sel> for u8 {
    #[inline(always)]
    fn from(variant: Lptim3sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lptim3sel {
    type Ux = u8;
}
impl crate::IsEnum for Lptim3sel {}
#[doc = "Field `LPTIM3SEL` reader - LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:"]
pub type Lptim3selR = crate::FieldReader<Lptim3sel>;
impl Lptim3selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim3sel {
        match self.bits {
            0 => Lptim3sel::B0x0,
            1 => Lptim3sel::B0x1,
            2 => Lptim3sel::B0x2,
            3 => Lptim3sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim3sel::B0x0
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim3sel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lptim3sel::B0x2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lptim3sel::B0x3
    }
}
#[doc = "Field `LPTIM3SEL` writer - LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:"]
pub type Lptim3selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lptim3sel, crate::Safe>;
impl<'a, REG> Lptim3selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3sel::B0x0)
    }
    #[doc = "LSI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3sel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3sel::B0x2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim3sel::B0x3)
    }
}
#[doc = "TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim1sel {
    #[doc = "0: TIMPCLK"]
    B0x0 = 0,
    #[doc = "1: PLLQCLK"]
    B0x1 = 1,
}
impl From<Tim1sel> for bool {
    #[inline(always)]
    fn from(variant: Tim1sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1SEL` reader - TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:"]
pub type Tim1selR = crate::BitReader<Tim1sel>;
impl Tim1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim1sel {
        match self.bits {
            false => Tim1sel::B0x0,
            true => Tim1sel::B0x1,
        }
    }
    #[doc = "TIMPCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim1sel::B0x0
    }
    #[doc = "PLLQCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim1sel::B0x1
    }
}
#[doc = "Field `TIM1SEL` writer - TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:"]
pub type Tim1selW<'a, REG> = crate::BitWriter<'a, REG, Tim1sel>;
impl<'a, REG> Tim1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMPCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1sel::B0x0)
    }
    #[doc = "PLLQCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1sel::B0x1)
    }
}
#[doc = "TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim15sel {
    #[doc = "0: TIMPCLK"]
    B0x0 = 0,
    #[doc = "1: PLLQCLK"]
    B0x1 = 1,
}
impl From<Tim15sel> for bool {
    #[inline(always)]
    fn from(variant: Tim15sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM15SEL` reader - TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:"]
pub type Tim15selR = crate::BitReader<Tim15sel>;
impl Tim15selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim15sel {
        match self.bits {
            false => Tim15sel::B0x0,
            true => Tim15sel::B0x1,
        }
    }
    #[doc = "TIMPCLK"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim15sel::B0x0
    }
    #[doc = "PLLQCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim15sel::B0x1
    }
}
#[doc = "Field `TIM15SEL` writer - TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:"]
pub type Tim15selW<'a, REG> = crate::BitWriter<'a, REG, Tim15sel>;
impl<'a, REG> Tim15selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMPCLK"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15sel::B0x0)
    }
    #[doc = "PLLQCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15sel::B0x1)
    }
}
#[doc = "481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clk48sel {
    #[doc = "0: No clock"]
    B0x0 = 0,
    #[doc = "1: MSI"]
    B0x1 = 1,
    #[doc = "2: PLLQCLK"]
    B0x2 = 2,
    #[doc = "3: HSI48&lt;sup>(1)&lt;/sup>"]
    B0x3 = 3,
}
impl From<Clk48sel> for u8 {
    #[inline(always)]
    fn from(variant: Clk48sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clk48sel {
    type Ux = u8;
}
impl crate::IsEnum for Clk48sel {}
#[doc = "Field `CLK48SEL` reader - 481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:"]
pub type Clk48selR = crate::FieldReader<Clk48sel>;
impl Clk48selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clk48sel {
        match self.bits {
            0 => Clk48sel::B0x0,
            1 => Clk48sel::B0x1,
            2 => Clk48sel::B0x2,
            3 => Clk48sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Clk48sel::B0x0
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Clk48sel::B0x1
    }
    #[doc = "PLLQCLK"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Clk48sel::B0x2
    }
    #[doc = "HSI48&lt;sup>(1)&lt;/sup>"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Clk48sel::B0x3
    }
}
#[doc = "Field `CLK48SEL` writer - 481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:"]
pub type Clk48selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clk48sel, crate::Safe>;
impl<'a, REG> Clk48selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Clk48sel::B0x0)
    }
    #[doc = "MSI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Clk48sel::B0x1)
    }
    #[doc = "PLLQCLK"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Clk48sel::B0x2)
    }
    #[doc = "HSI48&lt;sup>(1)&lt;/sup>"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Clk48sel::B0x3)
    }
}
#[doc = "ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcsel {
    #[doc = "0: System clock"]
    B0x0 = 0,
    #[doc = "1: PLLPCLK"]
    B0x1 = 1,
    #[doc = "2: HSI16"]
    B0x2 = 2,
}
impl From<Adcsel> for u8 {
    #[inline(always)]
    fn from(variant: Adcsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcsel {
    type Ux = u8;
}
impl crate::IsEnum for Adcsel {}
#[doc = "Field `ADCSEL` reader - ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:"]
pub type AdcselR = crate::FieldReader<Adcsel>;
impl AdcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adcsel> {
        match self.bits {
            0 => Some(Adcsel::B0x0),
            1 => Some(Adcsel::B0x1),
            2 => Some(Adcsel::B0x2),
            _ => None,
        }
    }
    #[doc = "System clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adcsel::B0x0
    }
    #[doc = "PLLPCLK"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adcsel::B0x1
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Adcsel::B0x2
    }
}
#[doc = "Field `ADCSEL` writer - ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:"]
pub type AdcselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcsel>;
impl<'a, REG> AdcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsel::B0x0)
    }
    #[doc = "PLLPCLK"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsel::B0x1)
    }
    #[doc = "HSI16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsel::B0x2)
    }
}
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:"]
    #[inline(always)]
    pub fn usart1sel(&self) -> Usart1selR {
        Usart1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:"]
    #[inline(always)]
    pub fn usart2sel(&self) -> Usart2selR {
        Usart2selR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - LPUART3 clock source selection&lt;sup>(1)&lt;/sup> This bitfield is controlled by software to select LPUART3 clock source as follows:"]
    #[inline(always)]
    pub fn lpuart3sel(&self) -> Lpuart3selR {
        Lpuart3selR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:"]
    #[inline(always)]
    pub fn lpuart2sel(&self) -> Lpuart2selR {
        Lpuart2selR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> Lpuart1selR {
        Lpuart1selR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2c1selR {
        I2c1selR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2c3selR {
        I2c3selR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> Lptim1selR {
        Lptim1selR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> Lptim2selR {
        Lptim2selR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:"]
    #[inline(always)]
    pub fn lptim3sel(&self) -> Lptim3selR {
        Lptim3selR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:"]
    #[inline(always)]
    pub fn tim1sel(&self) -> Tim1selR {
        Tim1selR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:"]
    #[inline(always)]
    pub fn tim15sel(&self) -> Tim15selR {
        Tim15selR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - 481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:"]
    #[inline(always)]
    pub fn clk48sel(&self) -> Clk48selR {
        Clk48selR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:"]
    #[inline(always)]
    pub fn adcsel(&self) -> AdcselR {
        AdcselR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> Usart1selW<RccCciprSpec> {
        Usart1selW::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> Usart2selW<RccCciprSpec> {
        Usart2selW::new(self, 2)
    }
    #[doc = "Bits 6:7 - LPUART3 clock source selection&lt;sup>(1)&lt;/sup> This bitfield is controlled by software to select LPUART3 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart3sel(&mut self) -> Lpuart3selW<RccCciprSpec> {
        Lpuart3selW::new(self, 6)
    }
    #[doc = "Bits 8:9 - LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2sel(&mut self) -> Lpuart2selW<RccCciprSpec> {
        Lpuart2selW::new(self, 8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> Lpuart1selW<RccCciprSpec> {
        Lpuart1selW::new(self, 10)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2c1selW<RccCciprSpec> {
        I2c1selW::new(self, 12)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2c3selW<RccCciprSpec> {
        I2c3selW::new(self, 16)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> Lptim1selW<RccCciprSpec> {
        Lptim1selW::new(self, 18)
    }
    #[doc = "Bits 20:21 - LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> Lptim2selW<RccCciprSpec> {
        Lptim2selW::new(self, 20)
    }
    #[doc = "Bits 22:23 - LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3sel(&mut self) -> Lptim3selW<RccCciprSpec> {
        Lptim3selW::new(self, 22)
    }
    #[doc = "Bit 24 - TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sel(&mut self) -> Tim1selW<RccCciprSpec> {
        Tim1selW::new(self, 24)
    }
    #[doc = "Bit 25 - TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn tim15sel(&mut self) -> Tim15selW<RccCciprSpec> {
        Tim15selW::new(self, 25)
    }
    #[doc = "Bits 26:27 - 481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:"]
    #[inline(always)]
    #[must_use]
    pub fn clk48sel(&mut self) -> Clk48selW<RccCciprSpec> {
        Clk48selW::new(self, 26)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> AdcselW<RccCciprSpec> {
        AdcselW::new(self, 28)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCciprSpec;
impl crate::RegisterSpec for RccCciprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ccipr::R`](R) reader structure"]
impl crate::Readable for RccCciprSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ccipr::W`](W) writer structure"]
impl crate::Writable for RccCciprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CCIPR to value 0"]
impl crate::Resettable for RccCciprSpec {
    const RESET_VALUE: u32 = 0;
}
