#[doc = "Register `LPTIM3_DIER_OUTPUT` reader"]
pub type R = crate::R<Lptim3DierOutputSpec>;
#[doc = "Register `LPTIM3_DIER_OUTPUT` writer"]
pub type W = crate::W<Lptim3DierOutputSpec>;
#[doc = "Capture/compare 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1ie {
    #[doc = "0: Capture/compare 1 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Capture/compare 1 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cc1ie> for bool {
    #[inline(always)]
    fn from(variant: Cc1ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IE` reader - Capture/compare 1 interrupt enable"]
pub type Cc1ieR = crate::BitReader<Cc1ie>;
impl Cc1ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1ie {
        match self.bits {
            false => Cc1ie::B0x0,
            true => Cc1ie::B0x1,
        }
    }
    #[doc = "Capture/compare 1 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1ie::B0x0
    }
    #[doc = "Capture/compare 1 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1ie::B0x1
    }
}
#[doc = "Field `CC1IE` writer - Capture/compare 1 interrupt enable"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG, Cc1ie>;
impl<'a, REG> Cc1ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare 1 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1ie::B0x0)
    }
    #[doc = "Capture/compare 1 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1ie::B0x1)
    }
}
#[doc = "Autoreload match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arrmie {
    #[doc = "0: ARRM interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: ARRM interrupt enabled"]
    B0x1 = 1,
}
impl From<Arrmie> for bool {
    #[inline(always)]
    fn from(variant: Arrmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARRMIE` reader - Autoreload match Interrupt Enable"]
pub type ArrmieR = crate::BitReader<Arrmie>;
impl ArrmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arrmie {
        match self.bits {
            false => Arrmie::B0x0,
            true => Arrmie::B0x1,
        }
    }
    #[doc = "ARRM interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Arrmie::B0x0
    }
    #[doc = "ARRM interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Arrmie::B0x1
    }
}
#[doc = "Field `ARRMIE` writer - Autoreload match Interrupt Enable"]
pub type ArrmieW<'a, REG> = crate::BitWriter<'a, REG, Arrmie>;
impl<'a, REG> ArrmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARRM interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Arrmie::B0x0)
    }
    #[doc = "ARRM interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Arrmie::B0x1)
    }
}
#[doc = "External trigger valid edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exttrigie {
    #[doc = "0: EXTTRIG interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: EXTTRIG interrupt enabled"]
    B0x1 = 1,
}
impl From<Exttrigie> for bool {
    #[inline(always)]
    fn from(variant: Exttrigie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable"]
pub type ExttrigieR = crate::BitReader<Exttrigie>;
impl ExttrigieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exttrigie {
        match self.bits {
            false => Exttrigie::B0x0,
            true => Exttrigie::B0x1,
        }
    }
    #[doc = "EXTTRIG interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Exttrigie::B0x0
    }
    #[doc = "EXTTRIG interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Exttrigie::B0x1
    }
}
#[doc = "Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable"]
pub type ExttrigieW<'a, REG> = crate::BitWriter<'a, REG, Exttrigie>;
impl<'a, REG> ExttrigieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EXTTRIG interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Exttrigie::B0x0)
    }
    #[doc = "EXTTRIG interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Exttrigie::B0x1)
    }
}
#[doc = "Compare register 1 update OK interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp1okie {
    #[doc = "0: CMPOK register 1 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: CMPOK register 1 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cmp1okie> for bool {
    #[inline(always)]
    fn from(variant: Cmp1okie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP1OKIE` reader - Compare register 1 update OK interrupt enable"]
pub type Cmp1okieR = crate::BitReader<Cmp1okie>;
impl Cmp1okieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp1okie {
        match self.bits {
            false => Cmp1okie::B0x0,
            true => Cmp1okie::B0x1,
        }
    }
    #[doc = "CMPOK register 1 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cmp1okie::B0x0
    }
    #[doc = "CMPOK register 1 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cmp1okie::B0x1
    }
}
#[doc = "Field `CMP1OKIE` writer - Compare register 1 update OK interrupt enable"]
pub type Cmp1okieW<'a, REG> = crate::BitWriter<'a, REG, Cmp1okie>;
impl<'a, REG> Cmp1okieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPOK register 1 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1okie::B0x0)
    }
    #[doc = "CMPOK register 1 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1okie::B0x1)
    }
}
#[doc = "Autoreload register update OK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arrokie {
    #[doc = "0: ARROK interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: ARROK interrupt enabled"]
    B0x1 = 1,
}
impl From<Arrokie> for bool {
    #[inline(always)]
    fn from(variant: Arrokie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable"]
pub type ArrokieR = crate::BitReader<Arrokie>;
impl ArrokieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arrokie {
        match self.bits {
            false => Arrokie::B0x0,
            true => Arrokie::B0x1,
        }
    }
    #[doc = "ARROK interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Arrokie::B0x0
    }
    #[doc = "ARROK interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Arrokie::B0x1
    }
}
#[doc = "Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable"]
pub type ArrokieW<'a, REG> = crate::BitWriter<'a, REG, Arrokie>;
impl<'a, REG> ArrokieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARROK interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Arrokie::B0x0)
    }
    #[doc = "ARROK interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Arrokie::B0x1)
    }
}
#[doc = "Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upie {
    #[doc = "0: UP interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: UP interrupt enabled"]
    B0x1 = 1,
}
impl From<Upie> for bool {
    #[inline(always)]
    fn from(variant: Upie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type UpieR = crate::BitReader<Upie>;
impl UpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upie {
        match self.bits {
            false => Upie::B0x0,
            true => Upie::B0x1,
        }
    }
    #[doc = "UP interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Upie::B0x0
    }
    #[doc = "UP interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Upie::B0x1
    }
}
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type UpieW<'a, REG> = crate::BitWriter<'a, REG, Upie>;
impl<'a, REG> UpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UP interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Upie::B0x0)
    }
    #[doc = "UP interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Upie::B0x1)
    }
}
#[doc = "Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Downie {
    #[doc = "0: DOWN interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: DOWN interrupt enabled"]
    B0x1 = 1,
}
impl From<Downie> for bool {
    #[inline(always)]
    fn from(variant: Downie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type DownieR = crate::BitReader<Downie>;
impl DownieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Downie {
        match self.bits {
            false => Downie::B0x0,
            true => Downie::B0x1,
        }
    }
    #[doc = "DOWN interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Downie::B0x0
    }
    #[doc = "DOWN interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Downie::B0x1
    }
}
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type DownieW<'a, REG> = crate::BitWriter<'a, REG, Downie>;
impl<'a, REG> DownieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DOWN interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Downie::B0x0)
    }
    #[doc = "DOWN interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Downie::B0x1)
    }
}
#[doc = "Update event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ueie {
    #[doc = "0: Update event interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Update event interrupt enabled"]
    B0x1 = 1,
}
impl From<Ueie> for bool {
    #[inline(always)]
    fn from(variant: Ueie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEIE` reader - Update event interrupt enable"]
pub type UeieR = crate::BitReader<Ueie>;
impl UeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ueie {
        match self.bits {
            false => Ueie::B0x0,
            true => Ueie::B0x1,
        }
    }
    #[doc = "Update event interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ueie::B0x0
    }
    #[doc = "Update event interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ueie::B0x1
    }
}
#[doc = "Field `UEIE` writer - Update event interrupt enable"]
pub type UeieW<'a, REG> = crate::BitWriter<'a, REG, Ueie>;
impl<'a, REG> UeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update event interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ueie::B0x0)
    }
    #[doc = "Update event interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ueie::B0x1)
    }
}
#[doc = "Repetition register update OK interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repokie {
    #[doc = "0: Repetition register update OK interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Repetition register update OK interrupt enabled"]
    B0x1 = 1,
}
impl From<Repokie> for bool {
    #[inline(always)]
    fn from(variant: Repokie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPOKIE` reader - Repetition register update OK interrupt Enable"]
pub type RepokieR = crate::BitReader<Repokie>;
impl RepokieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Repokie {
        match self.bits {
            false => Repokie::B0x0,
            true => Repokie::B0x1,
        }
    }
    #[doc = "Repetition register update OK interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Repokie::B0x0
    }
    #[doc = "Repetition register update OK interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Repokie::B0x1
    }
}
#[doc = "Field `REPOKIE` writer - Repetition register update OK interrupt Enable"]
pub type RepokieW<'a, REG> = crate::BitWriter<'a, REG, Repokie>;
impl<'a, REG> RepokieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repetition register update OK interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Repokie::B0x0)
    }
    #[doc = "Repetition register update OK interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Repokie::B0x1)
    }
}
#[doc = "Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc2ie {
    #[doc = "0: Capture/compare 2 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Capture/compare 2 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cc2ie> for bool {
    #[inline(always)]
    fn from(variant: Cc2ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2IE` reader - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cc2ieR = crate::BitReader<Cc2ie>;
impl Cc2ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2ie {
        match self.bits {
            false => Cc2ie::B0x0,
            true => Cc2ie::B0x1,
        }
    }
    #[doc = "Capture/compare 2 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc2ie::B0x0
    }
    #[doc = "Capture/compare 2 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc2ie::B0x1
    }
}
#[doc = "Field `CC2IE` writer - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG, Cc2ie>;
impl<'a, REG> Cc2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare 2 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2ie::B0x0)
    }
    #[doc = "Capture/compare 2 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2ie::B0x1)
    }
}
#[doc = "Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc3ie {
    #[doc = "0: Capture/compare 3 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Capture/compare 3 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cc3ie> for bool {
    #[inline(always)]
    fn from(variant: Cc3ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC3IE` reader - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cc3ieR = crate::BitReader<Cc3ie>;
impl Cc3ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc3ie {
        match self.bits {
            false => Cc3ie::B0x0,
            true => Cc3ie::B0x1,
        }
    }
    #[doc = "Capture/compare 3 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc3ie::B0x0
    }
    #[doc = "Capture/compare 3 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc3ie::B0x1
    }
}
#[doc = "Field `CC3IE` writer - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cc3ieW<'a, REG> = crate::BitWriter<'a, REG, Cc3ie>;
impl<'a, REG> Cc3ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare 3 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3ie::B0x0)
    }
    #[doc = "Capture/compare 3 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3ie::B0x1)
    }
}
#[doc = "Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc4ie {
    #[doc = "0: Capture/compare 4 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Capture/compare 4 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cc4ie> for bool {
    #[inline(always)]
    fn from(variant: Cc4ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC4IE` reader - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cc4ieR = crate::BitReader<Cc4ie>;
impl Cc4ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc4ie {
        match self.bits {
            false => Cc4ie::B0x0,
            true => Cc4ie::B0x1,
        }
    }
    #[doc = "Capture/compare 4 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc4ie::B0x0
    }
    #[doc = "Capture/compare 4 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc4ie::B0x1
    }
}
#[doc = "Field `CC4IE` writer - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cc4ieW<'a, REG> = crate::BitWriter<'a, REG, Cc4ie>;
impl<'a, REG> Cc4ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare 4 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4ie::B0x0)
    }
    #[doc = "Capture/compare 4 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4ie::B0x1)
    }
}
#[doc = "Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp2okie {
    #[doc = "0: CMPOK register 2 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: CMPOK register 2 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cmp2okie> for bool {
    #[inline(always)]
    fn from(variant: Cmp2okie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP2OKIE` reader - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp2okieR = crate::BitReader<Cmp2okie>;
impl Cmp2okieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp2okie {
        match self.bits {
            false => Cmp2okie::B0x0,
            true => Cmp2okie::B0x1,
        }
    }
    #[doc = "CMPOK register 2 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cmp2okie::B0x0
    }
    #[doc = "CMPOK register 2 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cmp2okie::B0x1
    }
}
#[doc = "Field `CMP2OKIE` writer - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp2okieW<'a, REG> = crate::BitWriter<'a, REG, Cmp2okie>;
impl<'a, REG> Cmp2okieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPOK register 2 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp2okie::B0x0)
    }
    #[doc = "CMPOK register 2 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp2okie::B0x1)
    }
}
#[doc = "Compare register 3 update OK interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp3okie {
    #[doc = "0: CMPOK register 3 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: CMPOK register 3 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cmp3okie> for bool {
    #[inline(always)]
    fn from(variant: Cmp3okie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP3OKIE` reader - Compare register 3 update OK interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp3okieR = crate::BitReader<Cmp3okie>;
impl Cmp3okieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp3okie {
        match self.bits {
            false => Cmp3okie::B0x0,
            true => Cmp3okie::B0x1,
        }
    }
    #[doc = "CMPOK register 3 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cmp3okie::B0x0
    }
    #[doc = "CMPOK register 3 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cmp3okie::B0x1
    }
}
#[doc = "Field `CMP3OKIE` writer - Compare register 3 update OK interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp3okieW<'a, REG> = crate::BitWriter<'a, REG, Cmp3okie>;
impl<'a, REG> Cmp3okieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPOK register 3 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp3okie::B0x0)
    }
    #[doc = "CMPOK register 3 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp3okie::B0x1)
    }
}
#[doc = "Compare register 4 update OK interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp4okie {
    #[doc = "0: CMPOK register 4 interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: CMPOK register 4 interrupt enabled"]
    B0x1 = 1,
}
impl From<Cmp4okie> for bool {
    #[inline(always)]
    fn from(variant: Cmp4okie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP4OKIE` reader - Compare register 4 update OK interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp4okieR = crate::BitReader<Cmp4okie>;
impl Cmp4okieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp4okie {
        match self.bits {
            false => Cmp4okie::B0x0,
            true => Cmp4okie::B0x1,
        }
    }
    #[doc = "CMPOK register 4 interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cmp4okie::B0x0
    }
    #[doc = "CMPOK register 4 interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cmp4okie::B0x1
    }
}
#[doc = "Field `CMP4OKIE` writer - Compare register 4 update OK interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp4okieW<'a, REG> = crate::BitWriter<'a, REG, Cmp4okie>;
impl<'a, REG> Cmp4okieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPOK register 4 interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp4okie::B0x0)
    }
    #[doc = "CMPOK register 4 interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp4okie::B0x1)
    }
}
#[doc = "Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uede {
    #[doc = "0: UE DMA request disabled. Writing '0' to the UEDE bit resets the associated ue_dma_req signal."]
    B0x0 = 0,
    #[doc = "1: UE DMA request enabled"]
    B0x1 = 1,
}
impl From<Uede> for bool {
    #[inline(always)]
    fn from(variant: Uede) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEDE` reader - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3."]
pub type UedeR = crate::BitReader<Uede>;
impl UedeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uede {
        match self.bits {
            false => Uede::B0x0,
            true => Uede::B0x1,
        }
    }
    #[doc = "UE DMA request disabled. Writing '0' to the UEDE bit resets the associated ue_dma_req signal."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uede::B0x0
    }
    #[doc = "UE DMA request enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uede::B0x1
    }
}
#[doc = "Field `UEDE` writer - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3."]
pub type UedeW<'a, REG> = crate::BitWriter<'a, REG, Uede>;
impl<'a, REG> UedeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UE DMA request disabled. Writing '0' to the UEDE bit resets the associated ue_dma_req signal."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uede::B0x0)
    }
    #[doc = "UE DMA request enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uede::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ArrmieR {
        ArrmieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> ExttrigieR {
        ExttrigieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register 1 update OK interrupt enable"]
    #[inline(always)]
    pub fn cmp1okie(&self) -> Cmp1okieR {
        Cmp1okieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ArrokieR {
        ArrokieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn downie(&self) -> DownieR {
        DownieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    pub fn ueie(&self) -> UeieR {
        UeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    pub fn repokie(&self) -> RepokieR {
        RepokieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc3ie(&self) -> Cc3ieR {
        Cc3ieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc4ie(&self) -> Cc4ieR {
        Cc4ieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cmp2okie(&self) -> Cmp2okieR {
        Cmp2okieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Compare register 3 update OK interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cmp3okie(&self) -> Cmp3okieR {
        Cmp3okieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Compare register 4 update OK interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cmp4okie(&self) -> Cmp4okieR {
        Cmp4okieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn uede(&self) -> UedeR {
        UedeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> Cc1ieW<Lptim3DierOutputSpec> {
        Cc1ieW::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ArrmieW<Lptim3DierOutputSpec> {
        ArrmieW::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> ExttrigieW<Lptim3DierOutputSpec> {
        ExttrigieW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register 1 update OK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1okie(&mut self) -> Cmp1okieW<Lptim3DierOutputSpec> {
        Cmp1okieW::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ArrokieW<Lptim3DierOutputSpec> {
        ArrokieW::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UpieW<Lptim3DierOutputSpec> {
        UpieW::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DownieW<Lptim3DierOutputSpec> {
        DownieW::new(self, 6)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ueie(&mut self) -> UeieW<Lptim3DierOutputSpec> {
        UeieW::new(self, 7)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn repokie(&mut self) -> RepokieW<Lptim3DierOutputSpec> {
        RepokieW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> Cc2ieW<Lptim3DierOutputSpec> {
        Cc2ieW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc3ie(&mut self) -> Cc3ieW<Lptim3DierOutputSpec> {
        Cc3ieW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cc4ie(&mut self) -> Cc4ieW<Lptim3DierOutputSpec> {
        Cc4ieW::new(self, 11)
    }
    #[doc = "Bit 19 - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2okie(&mut self) -> Cmp2okieW<Lptim3DierOutputSpec> {
        Cmp2okieW::new(self, 19)
    }
    #[doc = "Bit 20 - Compare register 3 update OK interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cmp3okie(&mut self) -> Cmp3okieW<Lptim3DierOutputSpec> {
        Cmp3okieW::new(self, 20)
    }
    #[doc = "Bit 21 - Compare register 4 update OK interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn cmp4okie(&mut self) -> Cmp4okieW<Lptim3DierOutputSpec> {
        Cmp4okieW::new(self, 21)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    #[must_use]
    pub fn uede(&mut self) -> UedeW<Lptim3DierOutputSpec> {
        UedeW::new(self, 23)
    }
}
#[doc = "LPTIM3 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_dier_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_dier_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim3DierOutputSpec;
impl crate::RegisterSpec for Lptim3DierOutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim3_dier_output::R`](R) reader structure"]
impl crate::Readable for Lptim3DierOutputSpec {}
#[doc = "`write(|w| ..)` method takes [`lptim3_dier_output::W`](W) writer structure"]
impl crate::Writable for Lptim3DierOutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM3_DIER_OUTPUT to value 0"]
impl crate::Resettable for Lptim3DierOutputSpec {
    const RESET_VALUE: u32 = 0;
}
