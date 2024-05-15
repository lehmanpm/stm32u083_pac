#[doc = "Register `GPIOA_AFRL` reader"]
pub type R = crate::R<GpioaAfrlSpec>;
#[doc = "Register `GPIOA_AFRL` writer"]
pub type W = crate::W<GpioaAfrlSpec>;
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel0 {
    #[doc = "0: AF0"]
    B0x0 = 0,
    #[doc = "1: AF1"]
    B0x1 = 1,
    #[doc = "2: AF2"]
    B0x2 = 2,
    #[doc = "3: AF3"]
    B0x3 = 3,
    #[doc = "4: AF4"]
    B0x4 = 4,
    #[doc = "5: AF5"]
    B0x5 = 5,
    #[doc = "6: AF6"]
    B0x6 = 6,
    #[doc = "7: AF7"]
    B0x7 = 7,
    #[doc = "8: AF8"]
    B0x8 = 8,
    #[doc = "9: AF9"]
    B0x9 = 9,
    #[doc = "10: AF10"]
    B0xA = 10,
    #[doc = "11: AF11"]
    B0xB = 11,
    #[doc = "12: AF12"]
    B0xC = 12,
    #[doc = "13: AF13"]
    B0xD = 13,
    #[doc = "14: AF14"]
    B0xE = 14,
    #[doc = "15: AF15"]
    B0xF = 15,
}
impl From<Afsel0> for u8 {
    #[inline(always)]
    fn from(variant: Afsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel0 {}
#[doc = "Field `AFSEL0` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel0R = crate::FieldReader<Afsel0>;
impl Afsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel0 {
        match self.bits {
            0 => Afsel0::B0x0,
            1 => Afsel0::B0x1,
            2 => Afsel0::B0x2,
            3 => Afsel0::B0x3,
            4 => Afsel0::B0x4,
            5 => Afsel0::B0x5,
            6 => Afsel0::B0x6,
            7 => Afsel0::B0x7,
            8 => Afsel0::B0x8,
            9 => Afsel0::B0x9,
            10 => Afsel0::B0xA,
            11 => Afsel0::B0xB,
            12 => Afsel0::B0xC,
            13 => Afsel0::B0xD,
            14 => Afsel0::B0xE,
            15 => Afsel0::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel0::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel0::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel0::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel0::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel0::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel0::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel0::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel0::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel0::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel0::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel0::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel0::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel0::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel0::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel0::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel0::B0xF
    }
}
#[doc = "Field `AFSEL0` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel0, crate::Safe>;
impl<'a, REG> Afsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel0::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel1 {
    #[doc = "0: AF0"]
    B0x0 = 0,
    #[doc = "1: AF1"]
    B0x1 = 1,
    #[doc = "2: AF2"]
    B0x2 = 2,
    #[doc = "3: AF3"]
    B0x3 = 3,
    #[doc = "4: AF4"]
    B0x4 = 4,
    #[doc = "5: AF5"]
    B0x5 = 5,
    #[doc = "6: AF6"]
    B0x6 = 6,
    #[doc = "7: AF7"]
    B0x7 = 7,
    #[doc = "8: AF8"]
    B0x8 = 8,
    #[doc = "9: AF9"]
    B0x9 = 9,
    #[doc = "10: AF10"]
    B0xA = 10,
    #[doc = "11: AF11"]
    B0xB = 11,
    #[doc = "12: AF12"]
    B0xC = 12,
    #[doc = "13: AF13"]
    B0xD = 13,
    #[doc = "14: AF14"]
    B0xE = 14,
    #[doc = "15: AF15"]
    B0xF = 15,
}
impl From<Afsel1> for u8 {
    #[inline(always)]
    fn from(variant: Afsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel1 {}
#[doc = "Field `AFSEL1` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel1R = crate::FieldReader<Afsel1>;
impl Afsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel1 {
        match self.bits {
            0 => Afsel1::B0x0,
            1 => Afsel1::B0x1,
            2 => Afsel1::B0x2,
            3 => Afsel1::B0x3,
            4 => Afsel1::B0x4,
            5 => Afsel1::B0x5,
            6 => Afsel1::B0x6,
            7 => Afsel1::B0x7,
            8 => Afsel1::B0x8,
            9 => Afsel1::B0x9,
            10 => Afsel1::B0xA,
            11 => Afsel1::B0xB,
            12 => Afsel1::B0xC,
            13 => Afsel1::B0xD,
            14 => Afsel1::B0xE,
            15 => Afsel1::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel1::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel1::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel1::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel1::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel1::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel1::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel1::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel1::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel1::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel1::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel1::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel1::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel1::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel1::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel1::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel1::B0xF
    }
}
#[doc = "Field `AFSEL1` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel1, crate::Safe>;
impl<'a, REG> Afsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel1::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel2 {
    #[doc = "0: AF0"]
    B0x0 = 0,
    #[doc = "1: AF1"]
    B0x1 = 1,
    #[doc = "2: AF2"]
    B0x2 = 2,
    #[doc = "3: AF3"]
    B0x3 = 3,
    #[doc = "4: AF4"]
    B0x4 = 4,
    #[doc = "5: AF5"]
    B0x5 = 5,
    #[doc = "6: AF6"]
    B0x6 = 6,
    #[doc = "7: AF7"]
    B0x7 = 7,
    #[doc = "8: AF8"]
    B0x8 = 8,
    #[doc = "9: AF9"]
    B0x9 = 9,
    #[doc = "10: AF10"]
    B0xA = 10,
    #[doc = "11: AF11"]
    B0xB = 11,
    #[doc = "12: AF12"]
    B0xC = 12,
    #[doc = "13: AF13"]
    B0xD = 13,
    #[doc = "14: AF14"]
    B0xE = 14,
    #[doc = "15: AF15"]
    B0xF = 15,
}
impl From<Afsel2> for u8 {
    #[inline(always)]
    fn from(variant: Afsel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel2 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel2 {}
#[doc = "Field `AFSEL2` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel2R = crate::FieldReader<Afsel2>;
impl Afsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel2 {
        match self.bits {
            0 => Afsel2::B0x0,
            1 => Afsel2::B0x1,
            2 => Afsel2::B0x2,
            3 => Afsel2::B0x3,
            4 => Afsel2::B0x4,
            5 => Afsel2::B0x5,
            6 => Afsel2::B0x6,
            7 => Afsel2::B0x7,
            8 => Afsel2::B0x8,
            9 => Afsel2::B0x9,
            10 => Afsel2::B0xA,
            11 => Afsel2::B0xB,
            12 => Afsel2::B0xC,
            13 => Afsel2::B0xD,
            14 => Afsel2::B0xE,
            15 => Afsel2::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel2::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel2::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel2::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel2::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel2::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel2::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel2::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel2::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel2::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel2::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel2::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel2::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel2::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel2::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel2::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel2::B0xF
    }
}
#[doc = "Field `AFSEL2` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel2, crate::Safe>;
impl<'a, REG> Afsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel2::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel3 {
    #[doc = "0: AF0"]
    B0x0 = 0,
    #[doc = "1: AF1"]
    B0x1 = 1,
    #[doc = "2: AF2"]
    B0x2 = 2,
    #[doc = "3: AF3"]
    B0x3 = 3,
    #[doc = "4: AF4"]
    B0x4 = 4,
    #[doc = "5: AF5"]
    B0x5 = 5,
    #[doc = "6: AF6"]
    B0x6 = 6,
    #[doc = "7: AF7"]
    B0x7 = 7,
    #[doc = "8: AF8"]
    B0x8 = 8,
    #[doc = "9: AF9"]
    B0x9 = 9,
    #[doc = "10: AF10"]
    B0xA = 10,
    #[doc = "11: AF11"]
    B0xB = 11,
    #[doc = "12: AF12"]
    B0xC = 12,
    #[doc = "13: AF13"]
    B0xD = 13,
    #[doc = "14: AF14"]
    B0xE = 14,
    #[doc = "15: AF15"]
    B0xF = 15,
}
impl From<Afsel3> for u8 {
    #[inline(always)]
    fn from(variant: Afsel3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel3 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel3 {}
#[doc = "Field `AFSEL3` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel3R = crate::FieldReader<Afsel3>;
impl Afsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel3 {
        match self.bits {
            0 => Afsel3::B0x0,
            1 => Afsel3::B0x1,
            2 => Afsel3::B0x2,
            3 => Afsel3::B0x3,
            4 => Afsel3::B0x4,
            5 => Afsel3::B0x5,
            6 => Afsel3::B0x6,
            7 => Afsel3::B0x7,
            8 => Afsel3::B0x8,
            9 => Afsel3::B0x9,
            10 => Afsel3::B0xA,
            11 => Afsel3::B0xB,
            12 => Afsel3::B0xC,
            13 => Afsel3::B0xD,
            14 => Afsel3::B0xE,
            15 => Afsel3::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel3::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel3::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel3::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel3::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel3::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel3::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel3::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel3::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel3::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel3::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel3::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel3::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel3::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel3::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel3::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel3::B0xF
    }
}
#[doc = "Field `AFSEL3` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel3, crate::Safe>;
impl<'a, REG> Afsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel3::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel4 {
    #[doc = "0: AF0"]
    B0x0 = 0,
    #[doc = "1: AF1"]
    B0x1 = 1,
    #[doc = "2: AF2"]
    B0x2 = 2,
    #[doc = "3: AF3"]
    B0x3 = 3,
    #[doc = "4: AF4"]
    B0x4 = 4,
    #[doc = "5: AF5"]
    B0x5 = 5,
    #[doc = "6: AF6"]
    B0x6 = 6,
    #[doc = "7: AF7"]
    B0x7 = 7,
    #[doc = "8: AF8"]
    B0x8 = 8,
    #[doc = "9: AF9"]
    B0x9 = 9,
    #[doc = "10: AF10"]
    B0xA = 10,
    #[doc = "11: AF11"]
    B0xB = 11,
    #[doc = "12: AF12"]
    B0xC = 12,
    #[doc = "13: AF13"]
    B0xD = 13,
    #[doc = "14: AF14"]
    B0xE = 14,
    #[doc = "15: AF15"]
    B0xF = 15,
}
impl From<Afsel4> for u8 {
    #[inline(always)]
    fn from(variant: Afsel4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel4 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel4 {}
#[doc = "Field `AFSEL4` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel4R = crate::FieldReader<Afsel4>;
impl Afsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel4 {
        match self.bits {
            0 => Afsel4::B0x0,
            1 => Afsel4::B0x1,
            2 => Afsel4::B0x2,
            3 => Afsel4::B0x3,
            4 => Afsel4::B0x4,
            5 => Afsel4::B0x5,
            6 => Afsel4::B0x6,
            7 => Afsel4::B0x7,
            8 => Afsel4::B0x8,
            9 => Afsel4::B0x9,
            10 => Afsel4::B0xA,
            11 => Afsel4::B0xB,
            12 => Afsel4::B0xC,
            13 => Afsel4::B0xD,
            14 => Afsel4::B0xE,
            15 => Afsel4::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel4::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel4::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel4::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel4::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel4::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel4::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel4::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel4::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel4::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel4::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel4::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel4::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel4::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel4::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel4::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel4::B0xF
    }
}
#[doc = "Field `AFSEL4` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel4, crate::Safe>;
impl<'a, REG> Afsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel4::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel5 {
    #[doc = "0: AF0"]
    B0x0 = 0,
    #[doc = "1: AF1"]
    B0x1 = 1,
    #[doc = "2: AF2"]
    B0x2 = 2,
    #[doc = "3: AF3"]
    B0x3 = 3,
    #[doc = "4: AF4"]
    B0x4 = 4,
    #[doc = "5: AF5"]
    B0x5 = 5,
    #[doc = "6: AF6"]
    B0x6 = 6,
    #[doc = "7: AF7"]
    B0x7 = 7,
    #[doc = "8: AF8"]
    B0x8 = 8,
    #[doc = "9: AF9"]
    B0x9 = 9,
    #[doc = "10: AF10"]
    B0xA = 10,
    #[doc = "11: AF11"]
    B0xB = 11,
    #[doc = "12: AF12"]
    B0xC = 12,
    #[doc = "13: AF13"]
    B0xD = 13,
    #[doc = "14: AF14"]
    B0xE = 14,
    #[doc = "15: AF15"]
    B0xF = 15,
}
impl From<Afsel5> for u8 {
    #[inline(always)]
    fn from(variant: Afsel5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel5 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel5 {}
#[doc = "Field `AFSEL5` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel5R = crate::FieldReader<Afsel5>;
impl Afsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel5 {
        match self.bits {
            0 => Afsel5::B0x0,
            1 => Afsel5::B0x1,
            2 => Afsel5::B0x2,
            3 => Afsel5::B0x3,
            4 => Afsel5::B0x4,
            5 => Afsel5::B0x5,
            6 => Afsel5::B0x6,
            7 => Afsel5::B0x7,
            8 => Afsel5::B0x8,
            9 => Afsel5::B0x9,
            10 => Afsel5::B0xA,
            11 => Afsel5::B0xB,
            12 => Afsel5::B0xC,
            13 => Afsel5::B0xD,
            14 => Afsel5::B0xE,
            15 => Afsel5::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel5::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel5::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel5::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel5::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel5::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel5::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel5::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel5::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel5::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel5::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel5::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel5::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel5::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel5::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel5::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel5::B0xF
    }
}
#[doc = "Field `AFSEL5` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel5W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel5, crate::Safe>;
impl<'a, REG> Afsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel5::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel6 {
    #[doc = "0: AF0"]
    B0x0 = 0,
    #[doc = "1: AF1"]
    B0x1 = 1,
    #[doc = "2: AF2"]
    B0x2 = 2,
    #[doc = "3: AF3"]
    B0x3 = 3,
    #[doc = "4: AF4"]
    B0x4 = 4,
    #[doc = "5: AF5"]
    B0x5 = 5,
    #[doc = "6: AF6"]
    B0x6 = 6,
    #[doc = "7: AF7"]
    B0x7 = 7,
    #[doc = "8: AF8"]
    B0x8 = 8,
    #[doc = "9: AF9"]
    B0x9 = 9,
    #[doc = "10: AF10"]
    B0xA = 10,
    #[doc = "11: AF11"]
    B0xB = 11,
    #[doc = "12: AF12"]
    B0xC = 12,
    #[doc = "13: AF13"]
    B0xD = 13,
    #[doc = "14: AF14"]
    B0xE = 14,
    #[doc = "15: AF15"]
    B0xF = 15,
}
impl From<Afsel6> for u8 {
    #[inline(always)]
    fn from(variant: Afsel6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel6 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel6 {}
#[doc = "Field `AFSEL6` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel6R = crate::FieldReader<Afsel6>;
impl Afsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel6 {
        match self.bits {
            0 => Afsel6::B0x0,
            1 => Afsel6::B0x1,
            2 => Afsel6::B0x2,
            3 => Afsel6::B0x3,
            4 => Afsel6::B0x4,
            5 => Afsel6::B0x5,
            6 => Afsel6::B0x6,
            7 => Afsel6::B0x7,
            8 => Afsel6::B0x8,
            9 => Afsel6::B0x9,
            10 => Afsel6::B0xA,
            11 => Afsel6::B0xB,
            12 => Afsel6::B0xC,
            13 => Afsel6::B0xD,
            14 => Afsel6::B0xE,
            15 => Afsel6::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel6::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel6::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel6::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel6::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel6::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel6::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel6::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel6::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel6::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel6::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel6::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel6::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel6::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel6::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel6::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel6::B0xF
    }
}
#[doc = "Field `AFSEL6` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel6W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel6, crate::Safe>;
impl<'a, REG> Afsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel6::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel7 {
    #[doc = "0: AF0"]
    B0x0 = 0,
    #[doc = "1: AF1"]
    B0x1 = 1,
    #[doc = "2: AF2"]
    B0x2 = 2,
    #[doc = "3: AF3"]
    B0x3 = 3,
    #[doc = "4: AF4"]
    B0x4 = 4,
    #[doc = "5: AF5"]
    B0x5 = 5,
    #[doc = "6: AF6"]
    B0x6 = 6,
    #[doc = "7: AF7"]
    B0x7 = 7,
    #[doc = "8: AF8"]
    B0x8 = 8,
    #[doc = "9: AF9"]
    B0x9 = 9,
    #[doc = "10: AF10"]
    B0xA = 10,
    #[doc = "11: AF11"]
    B0xB = 11,
    #[doc = "12: AF12"]
    B0xC = 12,
    #[doc = "13: AF13"]
    B0xD = 13,
    #[doc = "14: AF14"]
    B0xE = 14,
    #[doc = "15: AF15"]
    B0xF = 15,
}
impl From<Afsel7> for u8 {
    #[inline(always)]
    fn from(variant: Afsel7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel7 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel7 {}
#[doc = "Field `AFSEL7` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel7R = crate::FieldReader<Afsel7>;
impl Afsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel7 {
        match self.bits {
            0 => Afsel7::B0x0,
            1 => Afsel7::B0x1,
            2 => Afsel7::B0x2,
            3 => Afsel7::B0x3,
            4 => Afsel7::B0x4,
            5 => Afsel7::B0x5,
            6 => Afsel7::B0x6,
            7 => Afsel7::B0x7,
            8 => Afsel7::B0x8,
            9 => Afsel7::B0x9,
            10 => Afsel7::B0xA,
            11 => Afsel7::B0xB,
            12 => Afsel7::B0xC,
            13 => Afsel7::B0xD,
            14 => Afsel7::B0xE,
            15 => Afsel7::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel7::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel7::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel7::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel7::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel7::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel7::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel7::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel7::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel7::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel7::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel7::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel7::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel7::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel7::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel7::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel7::B0xF
    }
}
#[doc = "Field `AFSEL7` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel7W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel7, crate::Safe>;
impl<'a, REG> Afsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel7::B0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel0(&self) -> Afsel0R {
        Afsel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel1(&self) -> Afsel1R {
        Afsel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel2(&self) -> Afsel2R {
        Afsel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel3(&self) -> Afsel3R {
        Afsel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel4(&self) -> Afsel4R {
        Afsel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel5(&self) -> Afsel5R {
        Afsel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel6(&self) -> Afsel6R {
        Afsel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel7(&self) -> Afsel7R {
        Afsel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel0(&mut self) -> Afsel0W<GpioaAfrlSpec> {
        Afsel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel1(&mut self) -> Afsel1W<GpioaAfrlSpec> {
        Afsel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel2(&mut self) -> Afsel2W<GpioaAfrlSpec> {
        Afsel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel3(&mut self) -> Afsel3W<GpioaAfrlSpec> {
        Afsel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel4(&mut self) -> Afsel4W<GpioaAfrlSpec> {
        Afsel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel5(&mut self) -> Afsel5W<GpioaAfrlSpec> {
        Afsel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel6(&mut self) -> Afsel6W<GpioaAfrlSpec> {
        Afsel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel7(&mut self) -> Afsel7W<GpioaAfrlSpec> {
        Afsel7W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_afrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_afrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaAfrlSpec;
impl crate::RegisterSpec for GpioaAfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa_afrl::R`](R) reader structure"]
impl crate::Readable for GpioaAfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa_afrl::W`](W) writer structure"]
impl crate::Writable for GpioaAfrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOA_AFRL to value 0"]
impl crate::Resettable for GpioaAfrlSpec {
    const RESET_VALUE: u32 = 0;
}
