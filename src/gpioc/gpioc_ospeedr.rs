#[doc = "Register `GPIOC_OSPEEDR` reader"]
pub type R = crate::R<GpiocOspeedrSpec>;
#[doc = "Register `GPIOC_OSPEEDR` writer"]
pub type W = crate::W<GpiocOspeedrSpec>;
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed0 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed0> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed0 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed0 {}
#[doc = "Field `OSPEED0` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed0R = crate::FieldReader<Ospeed0>;
impl Ospeed0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed0 {
        match self.bits {
            0 => Ospeed0::B0x0,
            1 => Ospeed0::B0x1,
            2 => Ospeed0::B0x2,
            3 => Ospeed0::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed0::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed0::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed0::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed0::B0x3
    }
}
#[doc = "Field `OSPEED0` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed0, crate::Safe>;
impl<'a, REG> Ospeed0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed0::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed0::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed0::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed0::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed1 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed1> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed1 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed1 {}
#[doc = "Field `OSPEED1` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed1R = crate::FieldReader<Ospeed1>;
impl Ospeed1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed1 {
        match self.bits {
            0 => Ospeed1::B0x0,
            1 => Ospeed1::B0x1,
            2 => Ospeed1::B0x2,
            3 => Ospeed1::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed1::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed1::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed1::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed1::B0x3
    }
}
#[doc = "Field `OSPEED1` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed1, crate::Safe>;
impl<'a, REG> Ospeed1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed1::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed1::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed1::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed1::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed2 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed2> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed2 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed2 {}
#[doc = "Field `OSPEED2` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed2R = crate::FieldReader<Ospeed2>;
impl Ospeed2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed2 {
        match self.bits {
            0 => Ospeed2::B0x0,
            1 => Ospeed2::B0x1,
            2 => Ospeed2::B0x2,
            3 => Ospeed2::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed2::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed2::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed2::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed2::B0x3
    }
}
#[doc = "Field `OSPEED2` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed2, crate::Safe>;
impl<'a, REG> Ospeed2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed2::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed2::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed2::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed2::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed3 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed3> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed3 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed3 {}
#[doc = "Field `OSPEED3` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed3R = crate::FieldReader<Ospeed3>;
impl Ospeed3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed3 {
        match self.bits {
            0 => Ospeed3::B0x0,
            1 => Ospeed3::B0x1,
            2 => Ospeed3::B0x2,
            3 => Ospeed3::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed3::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed3::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed3::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed3::B0x3
    }
}
#[doc = "Field `OSPEED3` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed3, crate::Safe>;
impl<'a, REG> Ospeed3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed3::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed3::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed3::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed3::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed4 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed4> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed4 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed4 {}
#[doc = "Field `OSPEED4` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed4R = crate::FieldReader<Ospeed4>;
impl Ospeed4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed4 {
        match self.bits {
            0 => Ospeed4::B0x0,
            1 => Ospeed4::B0x1,
            2 => Ospeed4::B0x2,
            3 => Ospeed4::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed4::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed4::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed4::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed4::B0x3
    }
}
#[doc = "Field `OSPEED4` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed4, crate::Safe>;
impl<'a, REG> Ospeed4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed4::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed4::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed4::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed4::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed5 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed5> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed5 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed5 {}
#[doc = "Field `OSPEED5` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed5R = crate::FieldReader<Ospeed5>;
impl Ospeed5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed5 {
        match self.bits {
            0 => Ospeed5::B0x0,
            1 => Ospeed5::B0x1,
            2 => Ospeed5::B0x2,
            3 => Ospeed5::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed5::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed5::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed5::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed5::B0x3
    }
}
#[doc = "Field `OSPEED5` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed5, crate::Safe>;
impl<'a, REG> Ospeed5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed5::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed5::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed5::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed5::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed6 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed6> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed6 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed6 {}
#[doc = "Field `OSPEED6` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed6R = crate::FieldReader<Ospeed6>;
impl Ospeed6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed6 {
        match self.bits {
            0 => Ospeed6::B0x0,
            1 => Ospeed6::B0x1,
            2 => Ospeed6::B0x2,
            3 => Ospeed6::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed6::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed6::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed6::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed6::B0x3
    }
}
#[doc = "Field `OSPEED6` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed6, crate::Safe>;
impl<'a, REG> Ospeed6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed6::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed6::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed6::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed6::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed7 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed7> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed7 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed7 {}
#[doc = "Field `OSPEED7` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed7R = crate::FieldReader<Ospeed7>;
impl Ospeed7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed7 {
        match self.bits {
            0 => Ospeed7::B0x0,
            1 => Ospeed7::B0x1,
            2 => Ospeed7::B0x2,
            3 => Ospeed7::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed7::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed7::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed7::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed7::B0x3
    }
}
#[doc = "Field `OSPEED7` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed7, crate::Safe>;
impl<'a, REG> Ospeed7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed7::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed7::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed7::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed7::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed8 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed8> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed8 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed8 {}
#[doc = "Field `OSPEED8` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed8R = crate::FieldReader<Ospeed8>;
impl Ospeed8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed8 {
        match self.bits {
            0 => Ospeed8::B0x0,
            1 => Ospeed8::B0x1,
            2 => Ospeed8::B0x2,
            3 => Ospeed8::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed8::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed8::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed8::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed8::B0x3
    }
}
#[doc = "Field `OSPEED8` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed8, crate::Safe>;
impl<'a, REG> Ospeed8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed8::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed8::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed8::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed8::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed9 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed9> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed9 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed9 {}
#[doc = "Field `OSPEED9` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed9R = crate::FieldReader<Ospeed9>;
impl Ospeed9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed9 {
        match self.bits {
            0 => Ospeed9::B0x0,
            1 => Ospeed9::B0x1,
            2 => Ospeed9::B0x2,
            3 => Ospeed9::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed9::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed9::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed9::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed9::B0x3
    }
}
#[doc = "Field `OSPEED9` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed9, crate::Safe>;
impl<'a, REG> Ospeed9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed9::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed9::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed9::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed9::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed10 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed10> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed10 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed10 {}
#[doc = "Field `OSPEED10` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed10R = crate::FieldReader<Ospeed10>;
impl Ospeed10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed10 {
        match self.bits {
            0 => Ospeed10::B0x0,
            1 => Ospeed10::B0x1,
            2 => Ospeed10::B0x2,
            3 => Ospeed10::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed10::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed10::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed10::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed10::B0x3
    }
}
#[doc = "Field `OSPEED10` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed10, crate::Safe>;
impl<'a, REG> Ospeed10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed10::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed10::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed10::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed10::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed11 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed11> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed11 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed11 {}
#[doc = "Field `OSPEED11` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed11R = crate::FieldReader<Ospeed11>;
impl Ospeed11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed11 {
        match self.bits {
            0 => Ospeed11::B0x0,
            1 => Ospeed11::B0x1,
            2 => Ospeed11::B0x2,
            3 => Ospeed11::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed11::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed11::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed11::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed11::B0x3
    }
}
#[doc = "Field `OSPEED11` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed11, crate::Safe>;
impl<'a, REG> Ospeed11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed11::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed11::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed11::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed11::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed12 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed12> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed12 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed12 {}
#[doc = "Field `OSPEED12` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed12R = crate::FieldReader<Ospeed12>;
impl Ospeed12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed12 {
        match self.bits {
            0 => Ospeed12::B0x0,
            1 => Ospeed12::B0x1,
            2 => Ospeed12::B0x2,
            3 => Ospeed12::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed12::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed12::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed12::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed12::B0x3
    }
}
#[doc = "Field `OSPEED12` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed12, crate::Safe>;
impl<'a, REG> Ospeed12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed12::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed12::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed12::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed12::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed13 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed13> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed13 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed13 {}
#[doc = "Field `OSPEED13` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed13R = crate::FieldReader<Ospeed13>;
impl Ospeed13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed13 {
        match self.bits {
            0 => Ospeed13::B0x0,
            1 => Ospeed13::B0x1,
            2 => Ospeed13::B0x2,
            3 => Ospeed13::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed13::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed13::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed13::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed13::B0x3
    }
}
#[doc = "Field `OSPEED13` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed13, crate::Safe>;
impl<'a, REG> Ospeed13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed13::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed13::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed13::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed13::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed14 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed14> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed14 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed14 {}
#[doc = "Field `OSPEED14` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed14R = crate::FieldReader<Ospeed14>;
impl Ospeed14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed14 {
        match self.bits {
            0 => Ospeed14::B0x0,
            1 => Ospeed14::B0x1,
            2 => Ospeed14::B0x2,
            3 => Ospeed14::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed14::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed14::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed14::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed14::B0x3
    }
}
#[doc = "Field `OSPEED14` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed14, crate::Safe>;
impl<'a, REG> Ospeed14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed14::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed14::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed14::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed14::B0x3)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospeed15 {
    #[doc = "0: Low speed"]
    B0x0 = 0,
    #[doc = "1: Medium speed"]
    B0x1 = 1,
    #[doc = "2: High speed"]
    B0x2 = 2,
    #[doc = "3: Very high speed"]
    B0x3 = 3,
}
impl From<Ospeed15> for u8 {
    #[inline(always)]
    fn from(variant: Ospeed15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospeed15 {
    type Ux = u8;
}
impl crate::IsEnum for Ospeed15 {}
#[doc = "Field `OSPEED15` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed15R = crate::FieldReader<Ospeed15>;
impl Ospeed15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ospeed15 {
        match self.bits {
            0 => Ospeed15::B0x0,
            1 => Ospeed15::B0x1,
            2 => Ospeed15::B0x2,
            3 => Ospeed15::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ospeed15::B0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ospeed15::B0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ospeed15::B0x2
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ospeed15::B0x3
    }
}
#[doc = "Field `OSPEED15` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
pub type Ospeed15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospeed15, crate::Safe>;
impl<'a, REG> Ospeed15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed15::B0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed15::B0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed15::B0x2)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ospeed15::B0x3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed0(&self) -> Ospeed0R {
        Ospeed0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed1(&self) -> Ospeed1R {
        Ospeed1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed2(&self) -> Ospeed2R {
        Ospeed2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed3(&self) -> Ospeed3R {
        Ospeed3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed4(&self) -> Ospeed4R {
        Ospeed4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed5(&self) -> Ospeed5R {
        Ospeed5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed6(&self) -> Ospeed6R {
        Ospeed6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed7(&self) -> Ospeed7R {
        Ospeed7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed8(&self) -> Ospeed8R {
        Ospeed8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed9(&self) -> Ospeed9R {
        Ospeed9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed10(&self) -> Ospeed10R {
        Ospeed10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed11(&self) -> Ospeed11R {
        Ospeed11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed12(&self) -> Ospeed12R {
        Ospeed12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed13(&self) -> Ospeed13R {
        Ospeed13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed14(&self) -> Ospeed14R {
        Ospeed14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    pub fn ospeed15(&self) -> Ospeed15R {
        Ospeed15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed0(&mut self) -> Ospeed0W<GpiocOspeedrSpec> {
        Ospeed0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed1(&mut self) -> Ospeed1W<GpiocOspeedrSpec> {
        Ospeed1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed2(&mut self) -> Ospeed2W<GpiocOspeedrSpec> {
        Ospeed2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed3(&mut self) -> Ospeed3W<GpiocOspeedrSpec> {
        Ospeed3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed4(&mut self) -> Ospeed4W<GpiocOspeedrSpec> {
        Ospeed4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed5(&mut self) -> Ospeed5W<GpiocOspeedrSpec> {
        Ospeed5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed6(&mut self) -> Ospeed6W<GpiocOspeedrSpec> {
        Ospeed6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed7(&mut self) -> Ospeed7W<GpiocOspeedrSpec> {
        Ospeed7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed8(&mut self) -> Ospeed8W<GpiocOspeedrSpec> {
        Ospeed8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed9(&mut self) -> Ospeed9W<GpiocOspeedrSpec> {
        Ospeed9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed10(&mut self) -> Ospeed10W<GpiocOspeedrSpec> {
        Ospeed10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed11(&mut self) -> Ospeed11W<GpiocOspeedrSpec> {
        Ospeed11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed12(&mut self) -> Ospeed12W<GpiocOspeedrSpec> {
        Ospeed12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed13(&mut self) -> Ospeed13W<GpiocOspeedrSpec> {
        Ospeed13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed14(&mut self) -> Ospeed14W<GpiocOspeedrSpec> {
        Ospeed14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed15(&mut self) -> Ospeed15W<GpiocOspeedrSpec> {
        Ospeed15W::new(self, 30)
    }
}
#[doc = "GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioc_ospeedr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioc_ospeedr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiocOspeedrSpec;
impl crate::RegisterSpec for GpiocOspeedrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc_ospeedr::R`](R) reader structure"]
impl crate::Readable for GpiocOspeedrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioc_ospeedr::W`](W) writer structure"]
impl crate::Writable for GpiocOspeedrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOC_OSPEEDR to value 0"]
impl crate::Resettable for GpiocOspeedrSpec {
    const RESET_VALUE: u32 = 0;
}
