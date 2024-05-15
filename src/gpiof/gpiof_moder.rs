#[doc = "Register `GPIOF_MODER` reader"]
pub type R = crate::R<GpiofModerSpec>;
#[doc = "Register `GPIOF_MODER` writer"]
pub type W = crate::W<GpiofModerSpec>;
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode0 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode0> for u8 {
    #[inline(always)]
    fn from(variant: Mode0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode0 {
    type Ux = u8;
}
impl crate::IsEnum for Mode0 {}
#[doc = "Field `MODE0` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode0R = crate::FieldReader<Mode0>;
impl Mode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode0 {
        match self.bits {
            0 => Mode0::B0x0,
            1 => Mode0::B0x1,
            2 => Mode0::B0x2,
            3 => Mode0::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode0::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode0::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode0::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode0::B0x3
    }
}
#[doc = "Field `MODE0` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode0, crate::Safe>;
impl<'a, REG> Mode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode1 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode1> for u8 {
    #[inline(always)]
    fn from(variant: Mode1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode1 {
    type Ux = u8;
}
impl crate::IsEnum for Mode1 {}
#[doc = "Field `MODE1` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode1R = crate::FieldReader<Mode1>;
impl Mode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode1 {
        match self.bits {
            0 => Mode1::B0x0,
            1 => Mode1::B0x1,
            2 => Mode1::B0x2,
            3 => Mode1::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode1::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode1::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode1::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode1::B0x3
    }
}
#[doc = "Field `MODE1` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode1, crate::Safe>;
impl<'a, REG> Mode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode2 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode2> for u8 {
    #[inline(always)]
    fn from(variant: Mode2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode2 {
    type Ux = u8;
}
impl crate::IsEnum for Mode2 {}
#[doc = "Field `MODE2` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode2R = crate::FieldReader<Mode2>;
impl Mode2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode2 {
        match self.bits {
            0 => Mode2::B0x0,
            1 => Mode2::B0x1,
            2 => Mode2::B0x2,
            3 => Mode2::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode2::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode2::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode2::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode2::B0x3
    }
}
#[doc = "Field `MODE2` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode2, crate::Safe>;
impl<'a, REG> Mode2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode3 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode3> for u8 {
    #[inline(always)]
    fn from(variant: Mode3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode3 {
    type Ux = u8;
}
impl crate::IsEnum for Mode3 {}
#[doc = "Field `MODE3` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode3R = crate::FieldReader<Mode3>;
impl Mode3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode3 {
        match self.bits {
            0 => Mode3::B0x0,
            1 => Mode3::B0x1,
            2 => Mode3::B0x2,
            3 => Mode3::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode3::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode3::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode3::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode3::B0x3
    }
}
#[doc = "Field `MODE3` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode3, crate::Safe>;
impl<'a, REG> Mode3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode4 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode4> for u8 {
    #[inline(always)]
    fn from(variant: Mode4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode4 {
    type Ux = u8;
}
impl crate::IsEnum for Mode4 {}
#[doc = "Field `MODE4` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode4R = crate::FieldReader<Mode4>;
impl Mode4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode4 {
        match self.bits {
            0 => Mode4::B0x0,
            1 => Mode4::B0x1,
            2 => Mode4::B0x2,
            3 => Mode4::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode4::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode4::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode4::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode4::B0x3
    }
}
#[doc = "Field `MODE4` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode4, crate::Safe>;
impl<'a, REG> Mode4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode5 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode5> for u8 {
    #[inline(always)]
    fn from(variant: Mode5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode5 {
    type Ux = u8;
}
impl crate::IsEnum for Mode5 {}
#[doc = "Field `MODE5` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode5R = crate::FieldReader<Mode5>;
impl Mode5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode5 {
        match self.bits {
            0 => Mode5::B0x0,
            1 => Mode5::B0x1,
            2 => Mode5::B0x2,
            3 => Mode5::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode5::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode5::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode5::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode5::B0x3
    }
}
#[doc = "Field `MODE5` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode5, crate::Safe>;
impl<'a, REG> Mode5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode6 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode6> for u8 {
    #[inline(always)]
    fn from(variant: Mode6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode6 {
    type Ux = u8;
}
impl crate::IsEnum for Mode6 {}
#[doc = "Field `MODE6` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode6R = crate::FieldReader<Mode6>;
impl Mode6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode6 {
        match self.bits {
            0 => Mode6::B0x0,
            1 => Mode6::B0x1,
            2 => Mode6::B0x2,
            3 => Mode6::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode6::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode6::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode6::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode6::B0x3
    }
}
#[doc = "Field `MODE6` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode6, crate::Safe>;
impl<'a, REG> Mode6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode6::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode6::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode6::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode6::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode7 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode7> for u8 {
    #[inline(always)]
    fn from(variant: Mode7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode7 {
    type Ux = u8;
}
impl crate::IsEnum for Mode7 {}
#[doc = "Field `MODE7` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode7R = crate::FieldReader<Mode7>;
impl Mode7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode7 {
        match self.bits {
            0 => Mode7::B0x0,
            1 => Mode7::B0x1,
            2 => Mode7::B0x2,
            3 => Mode7::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode7::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode7::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode7::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode7::B0x3
    }
}
#[doc = "Field `MODE7` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode7, crate::Safe>;
impl<'a, REG> Mode7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode7::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode7::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode7::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode7::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode8 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode8> for u8 {
    #[inline(always)]
    fn from(variant: Mode8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode8 {
    type Ux = u8;
}
impl crate::IsEnum for Mode8 {}
#[doc = "Field `MODE8` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode8R = crate::FieldReader<Mode8>;
impl Mode8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode8 {
        match self.bits {
            0 => Mode8::B0x0,
            1 => Mode8::B0x1,
            2 => Mode8::B0x2,
            3 => Mode8::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode8::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode8::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode8::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode8::B0x3
    }
}
#[doc = "Field `MODE8` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode8, crate::Safe>;
impl<'a, REG> Mode8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode8::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode9 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode9> for u8 {
    #[inline(always)]
    fn from(variant: Mode9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode9 {
    type Ux = u8;
}
impl crate::IsEnum for Mode9 {}
#[doc = "Field `MODE9` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode9R = crate::FieldReader<Mode9>;
impl Mode9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode9 {
        match self.bits {
            0 => Mode9::B0x0,
            1 => Mode9::B0x1,
            2 => Mode9::B0x2,
            3 => Mode9::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode9::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode9::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode9::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode9::B0x3
    }
}
#[doc = "Field `MODE9` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode9, crate::Safe>;
impl<'a, REG> Mode9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode9::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode10 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode10> for u8 {
    #[inline(always)]
    fn from(variant: Mode10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode10 {
    type Ux = u8;
}
impl crate::IsEnum for Mode10 {}
#[doc = "Field `MODE10` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode10R = crate::FieldReader<Mode10>;
impl Mode10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode10 {
        match self.bits {
            0 => Mode10::B0x0,
            1 => Mode10::B0x1,
            2 => Mode10::B0x2,
            3 => Mode10::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode10::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode10::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode10::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode10::B0x3
    }
}
#[doc = "Field `MODE10` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode10, crate::Safe>;
impl<'a, REG> Mode10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode10::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode11 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode11> for u8 {
    #[inline(always)]
    fn from(variant: Mode11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode11 {
    type Ux = u8;
}
impl crate::IsEnum for Mode11 {}
#[doc = "Field `MODE11` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode11R = crate::FieldReader<Mode11>;
impl Mode11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode11 {
        match self.bits {
            0 => Mode11::B0x0,
            1 => Mode11::B0x1,
            2 => Mode11::B0x2,
            3 => Mode11::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode11::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode11::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode11::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode11::B0x3
    }
}
#[doc = "Field `MODE11` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode11, crate::Safe>;
impl<'a, REG> Mode11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode11::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode12 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode12> for u8 {
    #[inline(always)]
    fn from(variant: Mode12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode12 {
    type Ux = u8;
}
impl crate::IsEnum for Mode12 {}
#[doc = "Field `MODE12` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode12R = crate::FieldReader<Mode12>;
impl Mode12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode12 {
        match self.bits {
            0 => Mode12::B0x0,
            1 => Mode12::B0x1,
            2 => Mode12::B0x2,
            3 => Mode12::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode12::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode12::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode12::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode12::B0x3
    }
}
#[doc = "Field `MODE12` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode12, crate::Safe>;
impl<'a, REG> Mode12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode12::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode13 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode13> for u8 {
    #[inline(always)]
    fn from(variant: Mode13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode13 {
    type Ux = u8;
}
impl crate::IsEnum for Mode13 {}
#[doc = "Field `MODE13` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode13R = crate::FieldReader<Mode13>;
impl Mode13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode13 {
        match self.bits {
            0 => Mode13::B0x0,
            1 => Mode13::B0x1,
            2 => Mode13::B0x2,
            3 => Mode13::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode13::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode13::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode13::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode13::B0x3
    }
}
#[doc = "Field `MODE13` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode13, crate::Safe>;
impl<'a, REG> Mode13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode13::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode14 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode14> for u8 {
    #[inline(always)]
    fn from(variant: Mode14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode14 {
    type Ux = u8;
}
impl crate::IsEnum for Mode14 {}
#[doc = "Field `MODE14` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode14R = crate::FieldReader<Mode14>;
impl Mode14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode14 {
        match self.bits {
            0 => Mode14::B0x0,
            1 => Mode14::B0x1,
            2 => Mode14::B0x2,
            3 => Mode14::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode14::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode14::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode14::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode14::B0x3
    }
}
#[doc = "Field `MODE14` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode14, crate::Safe>;
impl<'a, REG> Mode14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode14::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode15 {
    #[doc = "0: Input mode"]
    B0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B0x3 = 3,
}
impl From<Mode15> for u8 {
    #[inline(always)]
    fn from(variant: Mode15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode15 {
    type Ux = u8;
}
impl crate::IsEnum for Mode15 {}
#[doc = "Field `MODE15` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode15R = crate::FieldReader<Mode15>;
impl Mode15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode15 {
        match self.bits {
            0 => Mode15::B0x0,
            1 => Mode15::B0x1,
            2 => Mode15::B0x2,
            3 => Mode15::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mode15::B0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mode15::B0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mode15::B0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mode15::B0x3
    }
}
#[doc = "Field `MODE15` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
pub type Mode15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode15, crate::Safe>;
impl<'a, REG> Mode15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::B0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::B0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::B0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode15::B0x3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode4(&self) -> Mode4R {
        Mode4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode5(&self) -> Mode5R {
        Mode5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode6(&self) -> Mode6R {
        Mode6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode7(&self) -> Mode7R {
        Mode7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode8(&self) -> Mode8R {
        Mode8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode9(&self) -> Mode9R {
        Mode9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode10(&self) -> Mode10R {
        Mode10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode11(&self) -> Mode11R {
        Mode11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode12(&self) -> Mode12R {
        Mode12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode13(&self) -> Mode13R {
        Mode13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode14(&self) -> Mode14R {
        Mode14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    pub fn mode15(&self) -> Mode15R {
        Mode15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> Mode0W<GpiofModerSpec> {
        Mode0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> Mode1W<GpiofModerSpec> {
        Mode1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> Mode2W<GpiofModerSpec> {
        Mode2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> Mode3W<GpiofModerSpec> {
        Mode3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> Mode4W<GpiofModerSpec> {
        Mode4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> Mode5W<GpiofModerSpec> {
        Mode5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> Mode6W<GpiofModerSpec> {
        Mode6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> Mode7W<GpiofModerSpec> {
        Mode7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> Mode8W<GpiofModerSpec> {
        Mode8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> Mode9W<GpiofModerSpec> {
        Mode9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> Mode10W<GpiofModerSpec> {
        Mode10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> Mode11W<GpiofModerSpec> {
        Mode11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> Mode12W<GpiofModerSpec> {
        Mode12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> Mode13W<GpiofModerSpec> {
        Mode13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> Mode14W<GpiofModerSpec> {
        Mode14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> Mode15W<GpiofModerSpec> {
        Mode15W::new(self, 30)
    }
}
#[doc = "GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_moder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_moder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiofModerSpec;
impl crate::RegisterSpec for GpiofModerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiof_moder::R`](R) reader structure"]
impl crate::Readable for GpiofModerSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiof_moder::W`](W) writer structure"]
impl crate::Writable for GpiofModerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOF_MODER to value 0xffff_ffff"]
impl crate::Resettable for GpiofModerSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
