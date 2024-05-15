#[doc = "Register `TAMP_IER` reader"]
pub type R = crate::R<TampIerSpec>;
#[doc = "Register `TAMP_IER` writer"]
pub type W = crate::W<TampIerSpec>;
#[doc = "Tamper 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp1ie {
    #[doc = "0: Tamper 1 interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Tamper 1 interrupt enabled."]
    B0x1 = 1,
}
impl From<Tamp1ie> for bool {
    #[inline(always)]
    fn from(variant: Tamp1ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable"]
pub type Tamp1ieR = crate::BitReader<Tamp1ie>;
impl Tamp1ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp1ie {
        match self.bits {
            false => Tamp1ie::B0x0,
            true => Tamp1ie::B0x1,
        }
    }
    #[doc = "Tamper 1 interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp1ie::B0x0
    }
    #[doc = "Tamper 1 interrupt enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp1ie::B0x1
    }
}
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable"]
pub type Tamp1ieW<'a, REG> = crate::BitWriter<'a, REG, Tamp1ie>;
impl<'a, REG> Tamp1ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1ie::B0x0)
    }
    #[doc = "Tamper 1 interrupt enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1ie::B0x1)
    }
}
#[doc = "Tamper 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp2ie {
    #[doc = "0: Tamper 2 interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Tamper 2 interrupt enabled."]
    B0x1 = 1,
}
impl From<Tamp2ie> for bool {
    #[inline(always)]
    fn from(variant: Tamp2ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2IE` reader - Tamper 2 interrupt enable"]
pub type Tamp2ieR = crate::BitReader<Tamp2ie>;
impl Tamp2ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp2ie {
        match self.bits {
            false => Tamp2ie::B0x0,
            true => Tamp2ie::B0x1,
        }
    }
    #[doc = "Tamper 2 interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp2ie::B0x0
    }
    #[doc = "Tamper 2 interrupt enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp2ie::B0x1
    }
}
#[doc = "Field `TAMP2IE` writer - Tamper 2 interrupt enable"]
pub type Tamp2ieW<'a, REG> = crate::BitWriter<'a, REG, Tamp2ie>;
impl<'a, REG> Tamp2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2ie::B0x0)
    }
    #[doc = "Tamper 2 interrupt enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2ie::B0x1)
    }
}
#[doc = "Tamper 3 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp3ie {
    #[doc = "0: Tamper 3 interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Tamper 3 interrupt enabled.."]
    B0x1 = 1,
}
impl From<Tamp3ie> for bool {
    #[inline(always)]
    fn from(variant: Tamp3ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3IE` reader - Tamper 3 interrupt enable"]
pub type Tamp3ieR = crate::BitReader<Tamp3ie>;
impl Tamp3ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp3ie {
        match self.bits {
            false => Tamp3ie::B0x0,
            true => Tamp3ie::B0x1,
        }
    }
    #[doc = "Tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp3ie::B0x0
    }
    #[doc = "Tamper 3 interrupt enabled.."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp3ie::B0x1
    }
}
#[doc = "Field `TAMP3IE` writer - Tamper 3 interrupt enable"]
pub type Tamp3ieW<'a, REG> = crate::BitWriter<'a, REG, Tamp3ie>;
impl<'a, REG> Tamp3ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3ie::B0x0)
    }
    #[doc = "Tamper 3 interrupt enabled.."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3ie::B0x1)
    }
}
#[doc = "Tamper 4 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp4ie {
    #[doc = "0: Tamper 4 interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Tamper 4 interrupt enabled."]
    B0x1 = 1,
}
impl From<Tamp4ie> for bool {
    #[inline(always)]
    fn from(variant: Tamp4ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP4IE` reader - Tamper 4 interrupt enable"]
pub type Tamp4ieR = crate::BitReader<Tamp4ie>;
impl Tamp4ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp4ie {
        match self.bits {
            false => Tamp4ie::B0x0,
            true => Tamp4ie::B0x1,
        }
    }
    #[doc = "Tamper 4 interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp4ie::B0x0
    }
    #[doc = "Tamper 4 interrupt enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp4ie::B0x1
    }
}
#[doc = "Field `TAMP4IE` writer - Tamper 4 interrupt enable"]
pub type Tamp4ieW<'a, REG> = crate::BitWriter<'a, REG, Tamp4ie>;
impl<'a, REG> Tamp4ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 4 interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp4ie::B0x0)
    }
    #[doc = "Tamper 4 interrupt enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp4ie::B0x1)
    }
}
#[doc = "Tamper 5 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp5ie {
    #[doc = "0: Tamper 5 interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Tamper 5 interrupt enabled."]
    B0x1 = 1,
}
impl From<Tamp5ie> for bool {
    #[inline(always)]
    fn from(variant: Tamp5ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP5IE` reader - Tamper 5 interrupt enable"]
pub type Tamp5ieR = crate::BitReader<Tamp5ie>;
impl Tamp5ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp5ie {
        match self.bits {
            false => Tamp5ie::B0x0,
            true => Tamp5ie::B0x1,
        }
    }
    #[doc = "Tamper 5 interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp5ie::B0x0
    }
    #[doc = "Tamper 5 interrupt enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp5ie::B0x1
    }
}
#[doc = "Field `TAMP5IE` writer - Tamper 5 interrupt enable"]
pub type Tamp5ieW<'a, REG> = crate::BitWriter<'a, REG, Tamp5ie>;
impl<'a, REG> Tamp5ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 5 interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp5ie::B0x0)
    }
    #[doc = "Tamper 5 interrupt enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp5ie::B0x1)
    }
}
#[doc = "Internal tamper 3 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp3ie {
    #[doc = "0: Internal tamper 3 interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 3 interrupt enabled."]
    B0x1 = 1,
}
impl From<Itamp3ie> for bool {
    #[inline(always)]
    fn from(variant: Itamp3ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3IE` reader - Internal tamper 3 interrupt enable"]
pub type Itamp3ieR = crate::BitReader<Itamp3ie>;
impl Itamp3ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp3ie {
        match self.bits {
            false => Itamp3ie::B0x0,
            true => Itamp3ie::B0x1,
        }
    }
    #[doc = "Internal tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp3ie::B0x0
    }
    #[doc = "Internal tamper 3 interrupt enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp3ie::B0x1
    }
}
#[doc = "Field `ITAMP3IE` writer - Internal tamper 3 interrupt enable"]
pub type Itamp3ieW<'a, REG> = crate::BitWriter<'a, REG, Itamp3ie>;
impl<'a, REG> Itamp3ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp3ie::B0x0)
    }
    #[doc = "Internal tamper 3 interrupt enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp3ie::B0x1)
    }
}
#[doc = "Internal tamper 4 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp4ie {
    #[doc = "0: Internal tamper 4 interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 4 interrupt enabled."]
    B0x1 = 1,
}
impl From<Itamp4ie> for bool {
    #[inline(always)]
    fn from(variant: Itamp4ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP4IE` reader - Internal tamper 4 interrupt enable"]
pub type Itamp4ieR = crate::BitReader<Itamp4ie>;
impl Itamp4ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp4ie {
        match self.bits {
            false => Itamp4ie::B0x0,
            true => Itamp4ie::B0x1,
        }
    }
    #[doc = "Internal tamper 4 interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp4ie::B0x0
    }
    #[doc = "Internal tamper 4 interrupt enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp4ie::B0x1
    }
}
#[doc = "Field `ITAMP4IE` writer - Internal tamper 4 interrupt enable"]
pub type Itamp4ieW<'a, REG> = crate::BitWriter<'a, REG, Itamp4ie>;
impl<'a, REG> Itamp4ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 4 interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp4ie::B0x0)
    }
    #[doc = "Internal tamper 4 interrupt enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp4ie::B0x1)
    }
}
#[doc = "Internal tamper 5 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp5ie {
    #[doc = "0: Internal tamper 5 interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 5 interrupt enabled."]
    B0x1 = 1,
}
impl From<Itamp5ie> for bool {
    #[inline(always)]
    fn from(variant: Itamp5ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP5IE` reader - Internal tamper 5 interrupt enable"]
pub type Itamp5ieR = crate::BitReader<Itamp5ie>;
impl Itamp5ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp5ie {
        match self.bits {
            false => Itamp5ie::B0x0,
            true => Itamp5ie::B0x1,
        }
    }
    #[doc = "Internal tamper 5 interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp5ie::B0x0
    }
    #[doc = "Internal tamper 5 interrupt enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp5ie::B0x1
    }
}
#[doc = "Field `ITAMP5IE` writer - Internal tamper 5 interrupt enable"]
pub type Itamp5ieW<'a, REG> = crate::BitWriter<'a, REG, Itamp5ie>;
impl<'a, REG> Itamp5ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 5 interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp5ie::B0x0)
    }
    #[doc = "Internal tamper 5 interrupt enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp5ie::B0x1)
    }
}
#[doc = "Internal tamper 6 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp6ie {
    #[doc = "0: Internal tamper 6 interrupt disabled."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 6 interrupt enabled."]
    B0x1 = 1,
}
impl From<Itamp6ie> for bool {
    #[inline(always)]
    fn from(variant: Itamp6ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP6IE` reader - Internal tamper 6 interrupt enable"]
pub type Itamp6ieR = crate::BitReader<Itamp6ie>;
impl Itamp6ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp6ie {
        match self.bits {
            false => Itamp6ie::B0x0,
            true => Itamp6ie::B0x1,
        }
    }
    #[doc = "Internal tamper 6 interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp6ie::B0x0
    }
    #[doc = "Internal tamper 6 interrupt enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp6ie::B0x1
    }
}
#[doc = "Field `ITAMP6IE` writer - Internal tamper 6 interrupt enable"]
pub type Itamp6ieW<'a, REG> = crate::BitWriter<'a, REG, Itamp6ie>;
impl<'a, REG> Itamp6ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 6 interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp6ie::B0x0)
    }
    #[doc = "Internal tamper 6 interrupt enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp6ie::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> Tamp1ieR {
        Tamp1ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> Tamp2ieR {
        Tamp2ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> Tamp3ieR {
        Tamp3ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 4 interrupt enable"]
    #[inline(always)]
    pub fn tamp4ie(&self) -> Tamp4ieR {
        Tamp4ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper 5 interrupt enable"]
    #[inline(always)]
    pub fn tamp5ie(&self) -> Tamp5ieR {
        Tamp5ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn itamp3ie(&self) -> Itamp3ieR {
        Itamp3ieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 interrupt enable"]
    #[inline(always)]
    pub fn itamp4ie(&self) -> Itamp4ieR {
        Itamp4ieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 interrupt enable"]
    #[inline(always)]
    pub fn itamp5ie(&self) -> Itamp5ieR {
        Itamp5ieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 interrupt enable"]
    #[inline(always)]
    pub fn itamp6ie(&self) -> Itamp6ieR {
        Itamp6ieR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1ie(&mut self) -> Tamp1ieW<TampIerSpec> {
        Tamp1ieW::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2ie(&mut self) -> Tamp2ieW<TampIerSpec> {
        Tamp2ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3ie(&mut self) -> Tamp3ieW<TampIerSpec> {
        Tamp3ieW::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper 4 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp4ie(&mut self) -> Tamp4ieW<TampIerSpec> {
        Tamp4ieW::new(self, 3)
    }
    #[doc = "Bit 4 - Tamper 5 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp5ie(&mut self) -> Tamp5ieW<TampIerSpec> {
        Tamp5ieW::new(self, 4)
    }
    #[doc = "Bit 18 - Internal tamper 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3ie(&mut self) -> Itamp3ieW<TampIerSpec> {
        Itamp3ieW::new(self, 18)
    }
    #[doc = "Bit 19 - Internal tamper 4 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4ie(&mut self) -> Itamp4ieW<TampIerSpec> {
        Itamp4ieW::new(self, 19)
    }
    #[doc = "Bit 20 - Internal tamper 5 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5ie(&mut self) -> Itamp5ieW<TampIerSpec> {
        Itamp5ieW::new(self, 20)
    }
    #[doc = "Bit 21 - Internal tamper 6 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6ie(&mut self) -> Itamp6ieW<TampIerSpec> {
        Itamp6ieW::new(self, 21)
    }
}
#[doc = "TAMP interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampIerSpec;
impl crate::RegisterSpec for TampIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_ier::R`](R) reader structure"]
impl crate::Readable for TampIerSpec {}
#[doc = "`write(|w| ..)` method takes [`tamp_ier::W`](W) writer structure"]
impl crate::Writable for TampIerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_IER to value 0"]
impl crate::Resettable for TampIerSpec {
    const RESET_VALUE: u32 = 0;
}
