#[doc = "Register `GPIOA_LCKR` reader"]
pub type R = crate::R<GpioaLckrSpec>;
#[doc = "Register `GPIOA_LCKR` writer"]
pub type W = crate::W<GpioaLckrSpec>;
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck0 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck0> for bool {
    #[inline(always)]
    fn from(variant: Lck0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK0` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck0R = crate::BitReader<Lck0>;
impl Lck0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck0 {
        match self.bits {
            false => Lck0::B0x0,
            true => Lck0::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck0::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck0::B0x1
    }
}
#[doc = "Field `LCK0` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck0W<'a, REG> = crate::BitWriter<'a, REG, Lck0>;
impl<'a, REG> Lck0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck0::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck0::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck1 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck1> for bool {
    #[inline(always)]
    fn from(variant: Lck1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK1` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck1R = crate::BitReader<Lck1>;
impl Lck1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck1 {
        match self.bits {
            false => Lck1::B0x0,
            true => Lck1::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck1::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck1::B0x1
    }
}
#[doc = "Field `LCK1` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck1W<'a, REG> = crate::BitWriter<'a, REG, Lck1>;
impl<'a, REG> Lck1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck1::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck1::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck2 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck2> for bool {
    #[inline(always)]
    fn from(variant: Lck2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK2` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck2R = crate::BitReader<Lck2>;
impl Lck2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck2 {
        match self.bits {
            false => Lck2::B0x0,
            true => Lck2::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck2::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck2::B0x1
    }
}
#[doc = "Field `LCK2` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck2W<'a, REG> = crate::BitWriter<'a, REG, Lck2>;
impl<'a, REG> Lck2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck2::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck2::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck3 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck3> for bool {
    #[inline(always)]
    fn from(variant: Lck3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK3` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck3R = crate::BitReader<Lck3>;
impl Lck3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck3 {
        match self.bits {
            false => Lck3::B0x0,
            true => Lck3::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck3::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck3::B0x1
    }
}
#[doc = "Field `LCK3` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck3W<'a, REG> = crate::BitWriter<'a, REG, Lck3>;
impl<'a, REG> Lck3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck3::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck3::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck4 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck4> for bool {
    #[inline(always)]
    fn from(variant: Lck4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK4` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck4R = crate::BitReader<Lck4>;
impl Lck4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck4 {
        match self.bits {
            false => Lck4::B0x0,
            true => Lck4::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck4::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck4::B0x1
    }
}
#[doc = "Field `LCK4` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck4W<'a, REG> = crate::BitWriter<'a, REG, Lck4>;
impl<'a, REG> Lck4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck4::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck4::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck5 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck5> for bool {
    #[inline(always)]
    fn from(variant: Lck5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK5` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck5R = crate::BitReader<Lck5>;
impl Lck5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck5 {
        match self.bits {
            false => Lck5::B0x0,
            true => Lck5::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck5::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck5::B0x1
    }
}
#[doc = "Field `LCK5` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck5W<'a, REG> = crate::BitWriter<'a, REG, Lck5>;
impl<'a, REG> Lck5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck5::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck5::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck6 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck6> for bool {
    #[inline(always)]
    fn from(variant: Lck6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK6` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck6R = crate::BitReader<Lck6>;
impl Lck6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck6 {
        match self.bits {
            false => Lck6::B0x0,
            true => Lck6::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck6::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck6::B0x1
    }
}
#[doc = "Field `LCK6` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck6W<'a, REG> = crate::BitWriter<'a, REG, Lck6>;
impl<'a, REG> Lck6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck6::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck6::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck7 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck7> for bool {
    #[inline(always)]
    fn from(variant: Lck7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK7` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck7R = crate::BitReader<Lck7>;
impl Lck7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck7 {
        match self.bits {
            false => Lck7::B0x0,
            true => Lck7::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck7::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck7::B0x1
    }
}
#[doc = "Field `LCK7` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck7W<'a, REG> = crate::BitWriter<'a, REG, Lck7>;
impl<'a, REG> Lck7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck7::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck7::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck8 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck8> for bool {
    #[inline(always)]
    fn from(variant: Lck8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK8` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck8R = crate::BitReader<Lck8>;
impl Lck8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck8 {
        match self.bits {
            false => Lck8::B0x0,
            true => Lck8::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck8::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck8::B0x1
    }
}
#[doc = "Field `LCK8` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck8W<'a, REG> = crate::BitWriter<'a, REG, Lck8>;
impl<'a, REG> Lck8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck8::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck8::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck9 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck9> for bool {
    #[inline(always)]
    fn from(variant: Lck9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK9` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck9R = crate::BitReader<Lck9>;
impl Lck9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck9 {
        match self.bits {
            false => Lck9::B0x0,
            true => Lck9::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck9::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck9::B0x1
    }
}
#[doc = "Field `LCK9` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck9W<'a, REG> = crate::BitWriter<'a, REG, Lck9>;
impl<'a, REG> Lck9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck9::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck9::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck10 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck10> for bool {
    #[inline(always)]
    fn from(variant: Lck10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK10` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck10R = crate::BitReader<Lck10>;
impl Lck10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck10 {
        match self.bits {
            false => Lck10::B0x0,
            true => Lck10::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck10::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck10::B0x1
    }
}
#[doc = "Field `LCK10` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck10W<'a, REG> = crate::BitWriter<'a, REG, Lck10>;
impl<'a, REG> Lck10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck10::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck10::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck11 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck11> for bool {
    #[inline(always)]
    fn from(variant: Lck11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK11` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck11R = crate::BitReader<Lck11>;
impl Lck11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck11 {
        match self.bits {
            false => Lck11::B0x0,
            true => Lck11::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck11::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck11::B0x1
    }
}
#[doc = "Field `LCK11` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck11W<'a, REG> = crate::BitWriter<'a, REG, Lck11>;
impl<'a, REG> Lck11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck11::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck11::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck12 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck12> for bool {
    #[inline(always)]
    fn from(variant: Lck12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK12` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck12R = crate::BitReader<Lck12>;
impl Lck12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck12 {
        match self.bits {
            false => Lck12::B0x0,
            true => Lck12::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck12::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck12::B0x1
    }
}
#[doc = "Field `LCK12` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck12W<'a, REG> = crate::BitWriter<'a, REG, Lck12>;
impl<'a, REG> Lck12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck12::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck12::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck13 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck13> for bool {
    #[inline(always)]
    fn from(variant: Lck13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK13` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck13R = crate::BitReader<Lck13>;
impl Lck13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck13 {
        match self.bits {
            false => Lck13::B0x0,
            true => Lck13::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck13::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck13::B0x1
    }
}
#[doc = "Field `LCK13` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck13W<'a, REG> = crate::BitWriter<'a, REG, Lck13>;
impl<'a, REG> Lck13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck13::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck13::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck14 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck14> for bool {
    #[inline(always)]
    fn from(variant: Lck14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK14` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck14R = crate::BitReader<Lck14>;
impl Lck14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck14 {
        match self.bits {
            false => Lck14::B0x0,
            true => Lck14::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck14::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck14::B0x1
    }
}
#[doc = "Field `LCK14` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck14W<'a, REG> = crate::BitWriter<'a, REG, Lck14>;
impl<'a, REG> Lck14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck14::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck14::B0x1)
    }
}
#[doc = "Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lck15 {
    #[doc = "0: Port configuration not locked"]
    B0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B0x1 = 1,
}
impl From<Lck15> for bool {
    #[inline(always)]
    fn from(variant: Lck15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK15` reader - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck15R = crate::BitReader<Lck15>;
impl Lck15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lck15 {
        match self.bits {
            false => Lck15::B0x0,
            true => Lck15::B0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lck15::B0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lck15::B0x1
    }
}
#[doc = "Field `LCK15` writer - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
pub type Lck15W<'a, REG> = crate::BitWriter<'a, REG, Lck15>;
impl<'a, REG> Lck15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lck15::B0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lck15::B0x1)
    }
}
#[doc = "Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 0 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
RD LCKR RD LCKR\\[16\\]
= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\]
must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lckk {
    #[doc = "0: Port configuration lock key not active"]
    B0x0 = 0,
    #[doc = "1: Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU reset or peripheral reset."]
    B0x1 = 1,
}
impl From<Lckk> for bool {
    #[inline(always)]
    fn from(variant: Lckk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKK` reader - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 0 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
RD LCKR RD LCKR\\[16\\]
= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\]
must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset."]
pub type LckkR = crate::BitReader<Lckk>;
impl LckkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lckk {
        match self.bits {
            false => Lckk::B0x0,
            true => Lckk::B0x1,
        }
    }
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lckk::B0x0
    }
    #[doc = "Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU reset or peripheral reset."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lckk::B0x1
    }
}
#[doc = "Field `LCKK` writer - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 0 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
RD LCKR RD LCKR\\[16\\]
= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\]
must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset."]
pub type LckkW<'a, REG> = crate::BitWriter<'a, REG, Lckk>;
impl<'a, REG> LckkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lckk::B0x0)
    }
    #[doc = "Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU reset or peripheral reset."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lckk::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck0(&self) -> Lck0R {
        Lck0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck1(&self) -> Lck1R {
        Lck1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck2(&self) -> Lck2R {
        Lck2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck3(&self) -> Lck3R {
        Lck3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck4(&self) -> Lck4R {
        Lck4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck5(&self) -> Lck5R {
        Lck5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck6(&self) -> Lck6R {
        Lck6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck7(&self) -> Lck7R {
        Lck7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck8(&self) -> Lck8R {
        Lck8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck9(&self) -> Lck9R {
        Lck9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck10(&self) -> Lck10R {
        Lck10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck11(&self) -> Lck11R {
        Lck11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck12(&self) -> Lck12R {
        Lck12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck13(&self) -> Lck13R {
        Lck13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck14(&self) -> Lck14R {
        Lck14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    pub fn lck15(&self) -> Lck15R {
        Lck15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 0 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
RD LCKR RD LCKR\\[16\\]
= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\]
must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset."]
    #[inline(always)]
    pub fn lckk(&self) -> LckkR {
        LckkR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> Lck0W<GpioaLckrSpec> {
        Lck0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> Lck1W<GpioaLckrSpec> {
        Lck1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck2(&mut self) -> Lck2W<GpioaLckrSpec> {
        Lck2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> Lck3W<GpioaLckrSpec> {
        Lck3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck4(&mut self) -> Lck4W<GpioaLckrSpec> {
        Lck4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck5(&mut self) -> Lck5W<GpioaLckrSpec> {
        Lck5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck6(&mut self) -> Lck6W<GpioaLckrSpec> {
        Lck6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck7(&mut self) -> Lck7W<GpioaLckrSpec> {
        Lck7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck8(&mut self) -> Lck8W<GpioaLckrSpec> {
        Lck8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck9(&mut self) -> Lck9W<GpioaLckrSpec> {
        Lck9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck10(&mut self) -> Lck10W<GpioaLckrSpec> {
        Lck10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck11(&mut self) -> Lck11W<GpioaLckrSpec> {
        Lck11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck12(&mut self) -> Lck12W<GpioaLckrSpec> {
        Lck12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck13(&mut self) -> Lck13W<GpioaLckrSpec> {
        Lck13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck14(&mut self) -> Lck14W<GpioaLckrSpec> {
        Lck14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lck15(&mut self) -> Lck15W<GpioaLckrSpec> {
        Lck15W::new(self, 15)
    }
    #[doc = "Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 0 + LCKR\\[15:0\\]
WR LCKR\\[16\\]
= 1 + LCKR\\[15:0\\]
RD LCKR RD LCKR\\[16\\]
= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\]
must not change. Note: Any error in the lock sequence aborts the lock. Note: After the first lock sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset."]
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LckkW<GpioaLckrSpec> {
        LckkW::new(self, 16)
    }
}
#[doc = "GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_lckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_lckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaLckrSpec;
impl crate::RegisterSpec for GpioaLckrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa_lckr::R`](R) reader structure"]
impl crate::Readable for GpioaLckrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioa_lckr::W`](W) writer structure"]
impl crate::Writable for GpioaLckrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOA_LCKR to value 0"]
impl crate::Resettable for GpioaLckrSpec {
    const RESET_VALUE: u32 = 0;
}
