#[doc = "Register `GPIOF_AFRH` reader"]
pub type R = crate::R<GpiofAfrhSpec>;
#[doc = "Register `GPIOF_AFRH` writer"]
pub type W = crate::W<GpiofAfrhSpec>;
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel8 {
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
impl From<Afsel8> for u8 {
    #[inline(always)]
    fn from(variant: Afsel8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel8 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel8 {}
#[doc = "Field `AFSEL8` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel8R = crate::FieldReader<Afsel8>;
impl Afsel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel8 {
        match self.bits {
            0 => Afsel8::B0x0,
            1 => Afsel8::B0x1,
            2 => Afsel8::B0x2,
            3 => Afsel8::B0x3,
            4 => Afsel8::B0x4,
            5 => Afsel8::B0x5,
            6 => Afsel8::B0x6,
            7 => Afsel8::B0x7,
            8 => Afsel8::B0x8,
            9 => Afsel8::B0x9,
            10 => Afsel8::B0xA,
            11 => Afsel8::B0xB,
            12 => Afsel8::B0xC,
            13 => Afsel8::B0xD,
            14 => Afsel8::B0xE,
            15 => Afsel8::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel8::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel8::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel8::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel8::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel8::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel8::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel8::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel8::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel8::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel8::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel8::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel8::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel8::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel8::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel8::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel8::B0xF
    }
}
#[doc = "Field `AFSEL8` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel8W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel8, crate::Safe>;
impl<'a, REG> Afsel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel8::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel9 {
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
impl From<Afsel9> for u8 {
    #[inline(always)]
    fn from(variant: Afsel9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel9 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel9 {}
#[doc = "Field `AFSEL9` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel9R = crate::FieldReader<Afsel9>;
impl Afsel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel9 {
        match self.bits {
            0 => Afsel9::B0x0,
            1 => Afsel9::B0x1,
            2 => Afsel9::B0x2,
            3 => Afsel9::B0x3,
            4 => Afsel9::B0x4,
            5 => Afsel9::B0x5,
            6 => Afsel9::B0x6,
            7 => Afsel9::B0x7,
            8 => Afsel9::B0x8,
            9 => Afsel9::B0x9,
            10 => Afsel9::B0xA,
            11 => Afsel9::B0xB,
            12 => Afsel9::B0xC,
            13 => Afsel9::B0xD,
            14 => Afsel9::B0xE,
            15 => Afsel9::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel9::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel9::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel9::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel9::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel9::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel9::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel9::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel9::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel9::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel9::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel9::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel9::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel9::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel9::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel9::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel9::B0xF
    }
}
#[doc = "Field `AFSEL9` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel9W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel9, crate::Safe>;
impl<'a, REG> Afsel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel9::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel10 {
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
impl From<Afsel10> for u8 {
    #[inline(always)]
    fn from(variant: Afsel10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel10 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel10 {}
#[doc = "Field `AFSEL10` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel10R = crate::FieldReader<Afsel10>;
impl Afsel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel10 {
        match self.bits {
            0 => Afsel10::B0x0,
            1 => Afsel10::B0x1,
            2 => Afsel10::B0x2,
            3 => Afsel10::B0x3,
            4 => Afsel10::B0x4,
            5 => Afsel10::B0x5,
            6 => Afsel10::B0x6,
            7 => Afsel10::B0x7,
            8 => Afsel10::B0x8,
            9 => Afsel10::B0x9,
            10 => Afsel10::B0xA,
            11 => Afsel10::B0xB,
            12 => Afsel10::B0xC,
            13 => Afsel10::B0xD,
            14 => Afsel10::B0xE,
            15 => Afsel10::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel10::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel10::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel10::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel10::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel10::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel10::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel10::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel10::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel10::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel10::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel10::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel10::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel10::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel10::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel10::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel10::B0xF
    }
}
#[doc = "Field `AFSEL10` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel10W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel10, crate::Safe>;
impl<'a, REG> Afsel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel10::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel11 {
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
impl From<Afsel11> for u8 {
    #[inline(always)]
    fn from(variant: Afsel11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel11 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel11 {}
#[doc = "Field `AFSEL11` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel11R = crate::FieldReader<Afsel11>;
impl Afsel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel11 {
        match self.bits {
            0 => Afsel11::B0x0,
            1 => Afsel11::B0x1,
            2 => Afsel11::B0x2,
            3 => Afsel11::B0x3,
            4 => Afsel11::B0x4,
            5 => Afsel11::B0x5,
            6 => Afsel11::B0x6,
            7 => Afsel11::B0x7,
            8 => Afsel11::B0x8,
            9 => Afsel11::B0x9,
            10 => Afsel11::B0xA,
            11 => Afsel11::B0xB,
            12 => Afsel11::B0xC,
            13 => Afsel11::B0xD,
            14 => Afsel11::B0xE,
            15 => Afsel11::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel11::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel11::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel11::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel11::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel11::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel11::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel11::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel11::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel11::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel11::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel11::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel11::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel11::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel11::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel11::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel11::B0xF
    }
}
#[doc = "Field `AFSEL11` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel11W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel11, crate::Safe>;
impl<'a, REG> Afsel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel11::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel12 {
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
impl From<Afsel12> for u8 {
    #[inline(always)]
    fn from(variant: Afsel12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel12 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel12 {}
#[doc = "Field `AFSEL12` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel12R = crate::FieldReader<Afsel12>;
impl Afsel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel12 {
        match self.bits {
            0 => Afsel12::B0x0,
            1 => Afsel12::B0x1,
            2 => Afsel12::B0x2,
            3 => Afsel12::B0x3,
            4 => Afsel12::B0x4,
            5 => Afsel12::B0x5,
            6 => Afsel12::B0x6,
            7 => Afsel12::B0x7,
            8 => Afsel12::B0x8,
            9 => Afsel12::B0x9,
            10 => Afsel12::B0xA,
            11 => Afsel12::B0xB,
            12 => Afsel12::B0xC,
            13 => Afsel12::B0xD,
            14 => Afsel12::B0xE,
            15 => Afsel12::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel12::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel12::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel12::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel12::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel12::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel12::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel12::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel12::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel12::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel12::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel12::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel12::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel12::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel12::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel12::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel12::B0xF
    }
}
#[doc = "Field `AFSEL12` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel12W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel12, crate::Safe>;
impl<'a, REG> Afsel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel12::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel13 {
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
impl From<Afsel13> for u8 {
    #[inline(always)]
    fn from(variant: Afsel13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel13 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel13 {}
#[doc = "Field `AFSEL13` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel13R = crate::FieldReader<Afsel13>;
impl Afsel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel13 {
        match self.bits {
            0 => Afsel13::B0x0,
            1 => Afsel13::B0x1,
            2 => Afsel13::B0x2,
            3 => Afsel13::B0x3,
            4 => Afsel13::B0x4,
            5 => Afsel13::B0x5,
            6 => Afsel13::B0x6,
            7 => Afsel13::B0x7,
            8 => Afsel13::B0x8,
            9 => Afsel13::B0x9,
            10 => Afsel13::B0xA,
            11 => Afsel13::B0xB,
            12 => Afsel13::B0xC,
            13 => Afsel13::B0xD,
            14 => Afsel13::B0xE,
            15 => Afsel13::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel13::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel13::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel13::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel13::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel13::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel13::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel13::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel13::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel13::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel13::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel13::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel13::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel13::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel13::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel13::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel13::B0xF
    }
}
#[doc = "Field `AFSEL13` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel13W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel13, crate::Safe>;
impl<'a, REG> Afsel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel13::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel14 {
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
impl From<Afsel14> for u8 {
    #[inline(always)]
    fn from(variant: Afsel14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel14 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel14 {}
#[doc = "Field `AFSEL14` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel14R = crate::FieldReader<Afsel14>;
impl Afsel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel14 {
        match self.bits {
            0 => Afsel14::B0x0,
            1 => Afsel14::B0x1,
            2 => Afsel14::B0x2,
            3 => Afsel14::B0x3,
            4 => Afsel14::B0x4,
            5 => Afsel14::B0x5,
            6 => Afsel14::B0x6,
            7 => Afsel14::B0x7,
            8 => Afsel14::B0x8,
            9 => Afsel14::B0x9,
            10 => Afsel14::B0xA,
            11 => Afsel14::B0xB,
            12 => Afsel14::B0xC,
            13 => Afsel14::B0xD,
            14 => Afsel14::B0xE,
            15 => Afsel14::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel14::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel14::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel14::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel14::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel14::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel14::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel14::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel14::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel14::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel14::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel14::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel14::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel14::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel14::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel14::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel14::B0xF
    }
}
#[doc = "Field `AFSEL14` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel14W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel14, crate::Safe>;
impl<'a, REG> Afsel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel14::B0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Afsel15 {
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
impl From<Afsel15> for u8 {
    #[inline(always)]
    fn from(variant: Afsel15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Afsel15 {
    type Ux = u8;
}
impl crate::IsEnum for Afsel15 {}
#[doc = "Field `AFSEL15` reader - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel15R = crate::FieldReader<Afsel15>;
impl Afsel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afsel15 {
        match self.bits {
            0 => Afsel15::B0x0,
            1 => Afsel15::B0x1,
            2 => Afsel15::B0x2,
            3 => Afsel15::B0x3,
            4 => Afsel15::B0x4,
            5 => Afsel15::B0x5,
            6 => Afsel15::B0x6,
            7 => Afsel15::B0x7,
            8 => Afsel15::B0x8,
            9 => Afsel15::B0x9,
            10 => Afsel15::B0xA,
            11 => Afsel15::B0xB,
            12 => Afsel15::B0xC,
            13 => Afsel15::B0xD,
            14 => Afsel15::B0xE,
            15 => Afsel15::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Afsel15::B0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Afsel15::B0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Afsel15::B0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Afsel15::B0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Afsel15::B0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Afsel15::B0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Afsel15::B0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Afsel15::B0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Afsel15::B0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Afsel15::B0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Afsel15::B0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Afsel15::B0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Afsel15::B0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Afsel15::B0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Afsel15::B0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Afsel15::B0xF
    }
}
#[doc = "Field `AFSEL15` writer - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
pub type Afsel15W<'a, REG> = crate::FieldWriter<'a, REG, 4, Afsel15, crate::Safe>;
impl<'a, REG> Afsel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Afsel15::B0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel8(&self) -> Afsel8R {
        Afsel8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel9(&self) -> Afsel9R {
        Afsel9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel10(&self) -> Afsel10R {
        Afsel10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel11(&self) -> Afsel11R {
        Afsel11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel12(&self) -> Afsel12R {
        Afsel12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel13(&self) -> Afsel13R {
        Afsel13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel14(&self) -> Afsel14R {
        Afsel14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    pub fn afsel15(&self) -> Afsel15R {
        Afsel15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel8(&mut self) -> Afsel8W<GpiofAfrhSpec> {
        Afsel8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel9(&mut self) -> Afsel9W<GpiofAfrhSpec> {
        Afsel9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel10(&mut self) -> Afsel10W<GpiofAfrhSpec> {
        Afsel10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel11(&mut self) -> Afsel11W<GpiofAfrhSpec> {
        Afsel11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel12(&mut self) -> Afsel12W<GpiofAfrhSpec> {
        Afsel12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel13(&mut self) -> Afsel13W<GpiofAfrhSpec> {
        Afsel13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel14(&mut self) -> Afsel14W<GpiofAfrhSpec> {
        Afsel14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y These bits are written by software to configure alternate function I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn afsel15(&mut self) -> Afsel15W<GpiofAfrhSpec> {
        Afsel15W::new(self, 28)
    }
}
#[doc = "GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_afrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_afrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiofAfrhSpec;
impl crate::RegisterSpec for GpiofAfrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiof_afrh::R`](R) reader structure"]
impl crate::Readable for GpiofAfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiof_afrh::W`](W) writer structure"]
impl crate::Writable for GpiofAfrhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOF_AFRH to value 0"]
impl crate::Resettable for GpiofAfrhSpec {
    const RESET_VALUE: u32 = 0;
}
