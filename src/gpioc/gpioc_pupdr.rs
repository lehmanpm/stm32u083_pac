#[doc = "Register `GPIOC_PUPDR` reader"]
pub type R = crate::R<GpiocPupdrSpec>;
#[doc = "Register `GPIOC_PUPDR` writer"]
pub type W = crate::W<GpiocPupdrSpec>;
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd0 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd0> for u8 {
    #[inline(always)]
    fn from(variant: Pupd0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd0 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd0 {}
#[doc = "Field `PUPD0` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd0R = crate::FieldReader<Pupd0>;
impl Pupd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd0> {
        match self.bits {
            0 => Some(Pupd0::B0x0),
            1 => Some(Pupd0::B0x1),
            2 => Some(Pupd0::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd0::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd0::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd0::B0x2
    }
}
#[doc = "Field `PUPD0` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd0>;
impl<'a, REG> Pupd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd0::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd0::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd0::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd1 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd1> for u8 {
    #[inline(always)]
    fn from(variant: Pupd1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd1 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd1 {}
#[doc = "Field `PUPD1` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd1R = crate::FieldReader<Pupd1>;
impl Pupd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd1> {
        match self.bits {
            0 => Some(Pupd1::B0x0),
            1 => Some(Pupd1::B0x1),
            2 => Some(Pupd1::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd1::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd1::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd1::B0x2
    }
}
#[doc = "Field `PUPD1` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd1>;
impl<'a, REG> Pupd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd1::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd1::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd1::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd2 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd2> for u8 {
    #[inline(always)]
    fn from(variant: Pupd2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd2 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd2 {}
#[doc = "Field `PUPD2` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd2R = crate::FieldReader<Pupd2>;
impl Pupd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd2> {
        match self.bits {
            0 => Some(Pupd2::B0x0),
            1 => Some(Pupd2::B0x1),
            2 => Some(Pupd2::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd2::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd2::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd2::B0x2
    }
}
#[doc = "Field `PUPD2` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd2>;
impl<'a, REG> Pupd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd2::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd2::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd2::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd3 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd3> for u8 {
    #[inline(always)]
    fn from(variant: Pupd3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd3 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd3 {}
#[doc = "Field `PUPD3` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd3R = crate::FieldReader<Pupd3>;
impl Pupd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd3> {
        match self.bits {
            0 => Some(Pupd3::B0x0),
            1 => Some(Pupd3::B0x1),
            2 => Some(Pupd3::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd3::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd3::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd3::B0x2
    }
}
#[doc = "Field `PUPD3` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd3>;
impl<'a, REG> Pupd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd3::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd3::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd3::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd4 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd4> for u8 {
    #[inline(always)]
    fn from(variant: Pupd4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd4 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd4 {}
#[doc = "Field `PUPD4` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd4R = crate::FieldReader<Pupd4>;
impl Pupd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd4> {
        match self.bits {
            0 => Some(Pupd4::B0x0),
            1 => Some(Pupd4::B0x1),
            2 => Some(Pupd4::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd4::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd4::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd4::B0x2
    }
}
#[doc = "Field `PUPD4` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd4>;
impl<'a, REG> Pupd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd4::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd4::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd4::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd5 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd5> for u8 {
    #[inline(always)]
    fn from(variant: Pupd5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd5 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd5 {}
#[doc = "Field `PUPD5` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd5R = crate::FieldReader<Pupd5>;
impl Pupd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd5> {
        match self.bits {
            0 => Some(Pupd5::B0x0),
            1 => Some(Pupd5::B0x1),
            2 => Some(Pupd5::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd5::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd5::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd5::B0x2
    }
}
#[doc = "Field `PUPD5` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd5>;
impl<'a, REG> Pupd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd5::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd5::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd5::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd6 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd6> for u8 {
    #[inline(always)]
    fn from(variant: Pupd6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd6 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd6 {}
#[doc = "Field `PUPD6` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd6R = crate::FieldReader<Pupd6>;
impl Pupd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd6> {
        match self.bits {
            0 => Some(Pupd6::B0x0),
            1 => Some(Pupd6::B0x1),
            2 => Some(Pupd6::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd6::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd6::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd6::B0x2
    }
}
#[doc = "Field `PUPD6` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd6>;
impl<'a, REG> Pupd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd6::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd6::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd6::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd7 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd7> for u8 {
    #[inline(always)]
    fn from(variant: Pupd7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd7 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd7 {}
#[doc = "Field `PUPD7` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd7R = crate::FieldReader<Pupd7>;
impl Pupd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd7> {
        match self.bits {
            0 => Some(Pupd7::B0x0),
            1 => Some(Pupd7::B0x1),
            2 => Some(Pupd7::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd7::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd7::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd7::B0x2
    }
}
#[doc = "Field `PUPD7` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd7>;
impl<'a, REG> Pupd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd7::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd7::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd7::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd8 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd8> for u8 {
    #[inline(always)]
    fn from(variant: Pupd8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd8 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd8 {}
#[doc = "Field `PUPD8` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd8R = crate::FieldReader<Pupd8>;
impl Pupd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd8> {
        match self.bits {
            0 => Some(Pupd8::B0x0),
            1 => Some(Pupd8::B0x1),
            2 => Some(Pupd8::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd8::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd8::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd8::B0x2
    }
}
#[doc = "Field `PUPD8` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd8>;
impl<'a, REG> Pupd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd8::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd8::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd8::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd9 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd9> for u8 {
    #[inline(always)]
    fn from(variant: Pupd9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd9 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd9 {}
#[doc = "Field `PUPD9` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd9R = crate::FieldReader<Pupd9>;
impl Pupd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd9> {
        match self.bits {
            0 => Some(Pupd9::B0x0),
            1 => Some(Pupd9::B0x1),
            2 => Some(Pupd9::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd9::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd9::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd9::B0x2
    }
}
#[doc = "Field `PUPD9` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd9>;
impl<'a, REG> Pupd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd9::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd9::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd9::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd10 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd10> for u8 {
    #[inline(always)]
    fn from(variant: Pupd10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd10 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd10 {}
#[doc = "Field `PUPD10` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd10R = crate::FieldReader<Pupd10>;
impl Pupd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd10> {
        match self.bits {
            0 => Some(Pupd10::B0x0),
            1 => Some(Pupd10::B0x1),
            2 => Some(Pupd10::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd10::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd10::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd10::B0x2
    }
}
#[doc = "Field `PUPD10` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd10>;
impl<'a, REG> Pupd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd10::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd10::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd10::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd11 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd11> for u8 {
    #[inline(always)]
    fn from(variant: Pupd11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd11 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd11 {}
#[doc = "Field `PUPD11` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd11R = crate::FieldReader<Pupd11>;
impl Pupd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd11> {
        match self.bits {
            0 => Some(Pupd11::B0x0),
            1 => Some(Pupd11::B0x1),
            2 => Some(Pupd11::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd11::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd11::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd11::B0x2
    }
}
#[doc = "Field `PUPD11` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd11>;
impl<'a, REG> Pupd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd11::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd11::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd11::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd12 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd12> for u8 {
    #[inline(always)]
    fn from(variant: Pupd12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd12 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd12 {}
#[doc = "Field `PUPD12` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd12R = crate::FieldReader<Pupd12>;
impl Pupd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd12> {
        match self.bits {
            0 => Some(Pupd12::B0x0),
            1 => Some(Pupd12::B0x1),
            2 => Some(Pupd12::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd12::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd12::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd12::B0x2
    }
}
#[doc = "Field `PUPD12` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd12>;
impl<'a, REG> Pupd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd12::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd12::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd12::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd13 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd13> for u8 {
    #[inline(always)]
    fn from(variant: Pupd13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd13 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd13 {}
#[doc = "Field `PUPD13` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd13R = crate::FieldReader<Pupd13>;
impl Pupd13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd13> {
        match self.bits {
            0 => Some(Pupd13::B0x0),
            1 => Some(Pupd13::B0x1),
            2 => Some(Pupd13::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd13::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd13::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd13::B0x2
    }
}
#[doc = "Field `PUPD13` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd13>;
impl<'a, REG> Pupd13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd13::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd13::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd13::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd14 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd14> for u8 {
    #[inline(always)]
    fn from(variant: Pupd14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd14 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd14 {}
#[doc = "Field `PUPD14` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd14R = crate::FieldReader<Pupd14>;
impl Pupd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd14> {
        match self.bits {
            0 => Some(Pupd14::B0x0),
            1 => Some(Pupd14::B0x1),
            2 => Some(Pupd14::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd14::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd14::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd14::B0x2
    }
}
#[doc = "Field `PUPD14` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd14>;
impl<'a, REG> Pupd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd14::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd14::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd14::B0x2)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pupd15 {
    #[doc = "0: No pull-up, pull-down"]
    B0x0 = 0,
    #[doc = "1: Pull-up"]
    B0x1 = 1,
    #[doc = "2: Pull-down"]
    B0x2 = 2,
}
impl From<Pupd15> for u8 {
    #[inline(always)]
    fn from(variant: Pupd15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pupd15 {
    type Ux = u8;
}
impl crate::IsEnum for Pupd15 {}
#[doc = "Field `PUPD15` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd15R = crate::FieldReader<Pupd15>;
impl Pupd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pupd15> {
        match self.bits {
            0 => Some(Pupd15::B0x0),
            1 => Some(Pupd15::B0x1),
            2 => Some(Pupd15::B0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pupd15::B0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pupd15::B0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pupd15::B0x2
    }
}
#[doc = "Field `PUPD15` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
pub type Pupd15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pupd15>;
impl<'a, REG> Pupd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd15::B0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd15::B0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pupd15::B0x2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd0(&self) -> Pupd0R {
        Pupd0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd1(&self) -> Pupd1R {
        Pupd1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd2(&self) -> Pupd2R {
        Pupd2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd3(&self) -> Pupd3R {
        Pupd3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd4(&self) -> Pupd4R {
        Pupd4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd5(&self) -> Pupd5R {
        Pupd5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd6(&self) -> Pupd6R {
        Pupd6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd7(&self) -> Pupd7R {
        Pupd7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd8(&self) -> Pupd8R {
        Pupd8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd9(&self) -> Pupd9R {
        Pupd9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd10(&self) -> Pupd10R {
        Pupd10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd11(&self) -> Pupd11R {
        Pupd11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd12(&self) -> Pupd12R {
        Pupd12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd13(&self) -> Pupd13R {
        Pupd13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd14(&self) -> Pupd14R {
        Pupd14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    pub fn pupd15(&self) -> Pupd15R {
        Pupd15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd0(&mut self) -> Pupd0W<GpiocPupdrSpec> {
        Pupd0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd1(&mut self) -> Pupd1W<GpiocPupdrSpec> {
        Pupd1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd2(&mut self) -> Pupd2W<GpiocPupdrSpec> {
        Pupd2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd3(&mut self) -> Pupd3W<GpiocPupdrSpec> {
        Pupd3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd4(&mut self) -> Pupd4W<GpiocPupdrSpec> {
        Pupd4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd5(&mut self) -> Pupd5W<GpiocPupdrSpec> {
        Pupd5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd6(&mut self) -> Pupd6W<GpiocPupdrSpec> {
        Pupd6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd7(&mut self) -> Pupd7W<GpiocPupdrSpec> {
        Pupd7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd8(&mut self) -> Pupd8W<GpiocPupdrSpec> {
        Pupd8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd9(&mut self) -> Pupd9W<GpiocPupdrSpec> {
        Pupd9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd10(&mut self) -> Pupd10W<GpiocPupdrSpec> {
        Pupd10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd11(&mut self) -> Pupd11W<GpiocPupdrSpec> {
        Pupd11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd12(&mut self) -> Pupd12W<GpiocPupdrSpec> {
        Pupd12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd13(&mut self) -> Pupd13W<GpiocPupdrSpec> {
        Pupd13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd14(&mut self) -> Pupd14W<GpiocPupdrSpec> {
        Pupd14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O pull-up or pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pupd15(&mut self) -> Pupd15W<GpiocPupdrSpec> {
        Pupd15W::new(self, 30)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_pupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_pupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocPupdrSpec;
impl crate::RegisterSpec for GpiocPupdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc_pupdr::R`](R) reader structure"]
impl crate::Readable for GpiocPupdrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioc_pupdr::W`](W) writer structure"]
impl crate::Writable for GpiocPupdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOC_PUPDR to value 0"]
impl crate::Resettable for GpiocPupdrSpec {
    const RESET_VALUE: u32 = 0;
}
