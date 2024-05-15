#[doc = "Register `GPIOD_OTYPER` reader"]
pub type R = crate::R<GpiodOtyperSpec>;
#[doc = "Register `GPIOD_OTYPER` writer"]
pub type W = crate::W<GpiodOtyperSpec>;
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot0 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot0> for bool {
    #[inline(always)]
    fn from(variant: Ot0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT0` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot0R = crate::BitReader<Ot0>;
impl Ot0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot0 {
        match self.bits {
            false => Ot0::B0x0,
            true => Ot0::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot0::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot0::B0x1
    }
}
#[doc = "Field `OT0` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot0W<'a, REG> = crate::BitWriter<'a, REG, Ot0>;
impl<'a, REG> Ot0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot0::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot0::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot1 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot1> for bool {
    #[inline(always)]
    fn from(variant: Ot1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT1` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot1R = crate::BitReader<Ot1>;
impl Ot1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot1 {
        match self.bits {
            false => Ot1::B0x0,
            true => Ot1::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot1::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot1::B0x1
    }
}
#[doc = "Field `OT1` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot1W<'a, REG> = crate::BitWriter<'a, REG, Ot1>;
impl<'a, REG> Ot1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot1::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot1::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot2 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot2> for bool {
    #[inline(always)]
    fn from(variant: Ot2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT2` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot2R = crate::BitReader<Ot2>;
impl Ot2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot2 {
        match self.bits {
            false => Ot2::B0x0,
            true => Ot2::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot2::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot2::B0x1
    }
}
#[doc = "Field `OT2` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot2W<'a, REG> = crate::BitWriter<'a, REG, Ot2>;
impl<'a, REG> Ot2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot2::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot2::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot3 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot3> for bool {
    #[inline(always)]
    fn from(variant: Ot3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT3` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot3R = crate::BitReader<Ot3>;
impl Ot3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot3 {
        match self.bits {
            false => Ot3::B0x0,
            true => Ot3::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot3::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot3::B0x1
    }
}
#[doc = "Field `OT3` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot3W<'a, REG> = crate::BitWriter<'a, REG, Ot3>;
impl<'a, REG> Ot3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot3::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot3::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot4 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot4> for bool {
    #[inline(always)]
    fn from(variant: Ot4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT4` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot4R = crate::BitReader<Ot4>;
impl Ot4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot4 {
        match self.bits {
            false => Ot4::B0x0,
            true => Ot4::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot4::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot4::B0x1
    }
}
#[doc = "Field `OT4` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot4W<'a, REG> = crate::BitWriter<'a, REG, Ot4>;
impl<'a, REG> Ot4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot4::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot4::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot5 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot5> for bool {
    #[inline(always)]
    fn from(variant: Ot5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT5` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot5R = crate::BitReader<Ot5>;
impl Ot5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot5 {
        match self.bits {
            false => Ot5::B0x0,
            true => Ot5::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot5::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot5::B0x1
    }
}
#[doc = "Field `OT5` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot5W<'a, REG> = crate::BitWriter<'a, REG, Ot5>;
impl<'a, REG> Ot5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot5::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot5::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot6 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot6> for bool {
    #[inline(always)]
    fn from(variant: Ot6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT6` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot6R = crate::BitReader<Ot6>;
impl Ot6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot6 {
        match self.bits {
            false => Ot6::B0x0,
            true => Ot6::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot6::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot6::B0x1
    }
}
#[doc = "Field `OT6` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot6W<'a, REG> = crate::BitWriter<'a, REG, Ot6>;
impl<'a, REG> Ot6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot6::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot6::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot7 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot7> for bool {
    #[inline(always)]
    fn from(variant: Ot7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT7` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot7R = crate::BitReader<Ot7>;
impl Ot7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot7 {
        match self.bits {
            false => Ot7::B0x0,
            true => Ot7::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot7::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot7::B0x1
    }
}
#[doc = "Field `OT7` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot7W<'a, REG> = crate::BitWriter<'a, REG, Ot7>;
impl<'a, REG> Ot7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot7::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot7::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot8 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot8> for bool {
    #[inline(always)]
    fn from(variant: Ot8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT8` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot8R = crate::BitReader<Ot8>;
impl Ot8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot8 {
        match self.bits {
            false => Ot8::B0x0,
            true => Ot8::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot8::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot8::B0x1
    }
}
#[doc = "Field `OT8` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot8W<'a, REG> = crate::BitWriter<'a, REG, Ot8>;
impl<'a, REG> Ot8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot8::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot8::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot9 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot9> for bool {
    #[inline(always)]
    fn from(variant: Ot9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT9` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot9R = crate::BitReader<Ot9>;
impl Ot9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot9 {
        match self.bits {
            false => Ot9::B0x0,
            true => Ot9::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot9::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot9::B0x1
    }
}
#[doc = "Field `OT9` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot9W<'a, REG> = crate::BitWriter<'a, REG, Ot9>;
impl<'a, REG> Ot9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot9::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot9::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot10 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot10> for bool {
    #[inline(always)]
    fn from(variant: Ot10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT10` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot10R = crate::BitReader<Ot10>;
impl Ot10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot10 {
        match self.bits {
            false => Ot10::B0x0,
            true => Ot10::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot10::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot10::B0x1
    }
}
#[doc = "Field `OT10` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot10W<'a, REG> = crate::BitWriter<'a, REG, Ot10>;
impl<'a, REG> Ot10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot10::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot10::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot11 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot11> for bool {
    #[inline(always)]
    fn from(variant: Ot11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT11` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot11R = crate::BitReader<Ot11>;
impl Ot11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot11 {
        match self.bits {
            false => Ot11::B0x0,
            true => Ot11::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot11::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot11::B0x1
    }
}
#[doc = "Field `OT11` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot11W<'a, REG> = crate::BitWriter<'a, REG, Ot11>;
impl<'a, REG> Ot11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot11::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot11::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot12 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot12> for bool {
    #[inline(always)]
    fn from(variant: Ot12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT12` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot12R = crate::BitReader<Ot12>;
impl Ot12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot12 {
        match self.bits {
            false => Ot12::B0x0,
            true => Ot12::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot12::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot12::B0x1
    }
}
#[doc = "Field `OT12` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot12W<'a, REG> = crate::BitWriter<'a, REG, Ot12>;
impl<'a, REG> Ot12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot12::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot12::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot13 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot13> for bool {
    #[inline(always)]
    fn from(variant: Ot13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT13` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot13R = crate::BitReader<Ot13>;
impl Ot13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot13 {
        match self.bits {
            false => Ot13::B0x0,
            true => Ot13::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot13::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot13::B0x1
    }
}
#[doc = "Field `OT13` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot13W<'a, REG> = crate::BitWriter<'a, REG, Ot13>;
impl<'a, REG> Ot13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot13::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot13::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot14 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot14> for bool {
    #[inline(always)]
    fn from(variant: Ot14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT14` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot14R = crate::BitReader<Ot14>;
impl Ot14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot14 {
        match self.bits {
            false => Ot14::B0x0,
            true => Ot14::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot14::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot14::B0x1
    }
}
#[doc = "Field `OT14` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot14W<'a, REG> = crate::BitWriter<'a, REG, Ot14>;
impl<'a, REG> Ot14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot14::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot14::B0x1)
    }
}
#[doc = "Port x configuration I/O pin y These bits are written by software to configure the I/O output type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ot15 {
    #[doc = "0: Output push-pull (reset state)"]
    B0x0 = 0,
    #[doc = "1: Output open-drain"]
    B0x1 = 1,
}
impl From<Ot15> for bool {
    #[inline(always)]
    fn from(variant: Ot15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT15` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot15R = crate::BitReader<Ot15>;
impl Ot15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ot15 {
        match self.bits {
            false => Ot15::B0x0,
            true => Ot15::B0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ot15::B0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ot15::B0x1
    }
}
#[doc = "Field `OT15` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
pub type Ot15W<'a, REG> = crate::BitWriter<'a, REG, Ot15>;
impl<'a, REG> Ot15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ot15::B0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ot15::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot0(&self) -> Ot0R {
        Ot0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot1(&self) -> Ot1R {
        Ot1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot2(&self) -> Ot2R {
        Ot2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot3(&self) -> Ot3R {
        Ot3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot4(&self) -> Ot4R {
        Ot4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot5(&self) -> Ot5R {
        Ot5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot6(&self) -> Ot6R {
        Ot6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot7(&self) -> Ot7R {
        Ot7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot8(&self) -> Ot8R {
        Ot8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot9(&self) -> Ot9R {
        Ot9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot10(&self) -> Ot10R {
        Ot10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot11(&self) -> Ot11R {
        Ot11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot12(&self) -> Ot12R {
        Ot12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot13(&self) -> Ot13R {
        Ot13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot14(&self) -> Ot14R {
        Ot14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    pub fn ot15(&self) -> Ot15R {
        Ot15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot0(&mut self) -> Ot0W<GpiodOtyperSpec> {
        Ot0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot1(&mut self) -> Ot1W<GpiodOtyperSpec> {
        Ot1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot2(&mut self) -> Ot2W<GpiodOtyperSpec> {
        Ot2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> Ot3W<GpiodOtyperSpec> {
        Ot3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot4(&mut self) -> Ot4W<GpiodOtyperSpec> {
        Ot4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot5(&mut self) -> Ot5W<GpiodOtyperSpec> {
        Ot5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot6(&mut self) -> Ot6W<GpiodOtyperSpec> {
        Ot6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot7(&mut self) -> Ot7W<GpiodOtyperSpec> {
        Ot7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot8(&mut self) -> Ot8W<GpiodOtyperSpec> {
        Ot8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot9(&mut self) -> Ot9W<GpiodOtyperSpec> {
        Ot9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot10(&mut self) -> Ot10W<GpiodOtyperSpec> {
        Ot10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot11(&mut self) -> Ot11W<GpiodOtyperSpec> {
        Ot11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot12(&mut self) -> Ot12W<GpiodOtyperSpec> {
        Ot12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot13(&mut self) -> Ot13W<GpiodOtyperSpec> {
        Ot13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot14(&mut self) -> Ot14W<GpiodOtyperSpec> {
        Ot14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x configuration I/O pin y These bits are written by software to configure the I/O output type."]
    #[inline(always)]
    #[must_use]
    pub fn ot15(&mut self) -> Ot15W<GpiodOtyperSpec> {
        Ot15W::new(self, 15)
    }
}
#[doc = "GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_otyper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_otyper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiodOtyperSpec;
impl crate::RegisterSpec for GpiodOtyperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiod_otyper::R`](R) reader structure"]
impl crate::Readable for GpiodOtyperSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiod_otyper::W`](W) writer structure"]
impl crate::Writable for GpiodOtyperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOD_OTYPER to value 0"]
impl crate::Resettable for GpiodOtyperSpec {
    const RESET_VALUE: u32 = 0;
}
