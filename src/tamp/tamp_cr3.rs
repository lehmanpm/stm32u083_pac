#[doc = "Register `TAMP_CR3` reader"]
pub type R = crate::R<TampCr3Spec>;
#[doc = "Register `TAMP_CR3` writer"]
pub type W = crate::W<TampCr3Spec>;
#[doc = "Internal tamper 3 potential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp3pom {
    #[doc = "0: Internal tamper 3 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 3 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    B0x1 = 1,
}
impl From<Itamp3pom> for bool {
    #[inline(always)]
    fn from(variant: Itamp3pom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3POM` reader - Internal tamper 3 potential mode"]
pub type Itamp3pomR = crate::BitReader<Itamp3pom>;
impl Itamp3pomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp3pom {
        match self.bits {
            false => Itamp3pom::B0x0,
            true => Itamp3pom::B0x1,
        }
    }
    #[doc = "Internal tamper 3 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp3pom::B0x0
    }
    #[doc = "Internal tamper 3 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp3pom::B0x1
    }
}
#[doc = "Field `ITAMP3POM` writer - Internal tamper 3 potential mode"]
pub type Itamp3pomW<'a, REG> = crate::BitWriter<'a, REG, Itamp3pom>;
impl<'a, REG> Itamp3pomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 3 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp3pom::B0x0)
    }
    #[doc = "Internal tamper 3 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp3pom::B0x1)
    }
}
#[doc = "Internal tamper 4 potential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp4pom {
    #[doc = "0: Internal tamper 4 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 4 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    B0x1 = 1,
}
impl From<Itamp4pom> for bool {
    #[inline(always)]
    fn from(variant: Itamp4pom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP4POM` reader - Internal tamper 4 potential mode"]
pub type Itamp4pomR = crate::BitReader<Itamp4pom>;
impl Itamp4pomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp4pom {
        match self.bits {
            false => Itamp4pom::B0x0,
            true => Itamp4pom::B0x1,
        }
    }
    #[doc = "Internal tamper 4 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp4pom::B0x0
    }
    #[doc = "Internal tamper 4 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp4pom::B0x1
    }
}
#[doc = "Field `ITAMP4POM` writer - Internal tamper 4 potential mode"]
pub type Itamp4pomW<'a, REG> = crate::BitWriter<'a, REG, Itamp4pom>;
impl<'a, REG> Itamp4pomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 4 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp4pom::B0x0)
    }
    #[doc = "Internal tamper 4 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp4pom::B0x1)
    }
}
#[doc = "Internal tamper 5 potential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp5pom {
    #[doc = "0: Internal tamper 5 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 5 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    B0x1 = 1,
}
impl From<Itamp5pom> for bool {
    #[inline(always)]
    fn from(variant: Itamp5pom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP5POM` reader - Internal tamper 5 potential mode"]
pub type Itamp5pomR = crate::BitReader<Itamp5pom>;
impl Itamp5pomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp5pom {
        match self.bits {
            false => Itamp5pom::B0x0,
            true => Itamp5pom::B0x1,
        }
    }
    #[doc = "Internal tamper 5 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp5pom::B0x0
    }
    #[doc = "Internal tamper 5 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp5pom::B0x1
    }
}
#[doc = "Field `ITAMP5POM` writer - Internal tamper 5 potential mode"]
pub type Itamp5pomW<'a, REG> = crate::BitWriter<'a, REG, Itamp5pom>;
impl<'a, REG> Itamp5pomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 5 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp5pom::B0x0)
    }
    #[doc = "Internal tamper 5 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp5pom::B0x1)
    }
}
#[doc = "Internal tamper 6 potential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Itamp6pom {
    #[doc = "0: Internal tamper 6 event detection is in confirmed mode."]
    B0x0 = 0,
    #[doc = "1: Internal tamper 6 event detection is in potential mode."]
    B0x1 = 1,
}
impl From<Itamp6pom> for bool {
    #[inline(always)]
    fn from(variant: Itamp6pom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP6POM` reader - Internal tamper 6 potential mode"]
pub type Itamp6pomR = crate::BitReader<Itamp6pom>;
impl Itamp6pomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Itamp6pom {
        match self.bits {
            false => Itamp6pom::B0x0,
            true => Itamp6pom::B0x1,
        }
    }
    #[doc = "Internal tamper 6 event detection is in confirmed mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Itamp6pom::B0x0
    }
    #[doc = "Internal tamper 6 event detection is in potential mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Itamp6pom::B0x1
    }
}
#[doc = "Field `ITAMP6POM` writer - Internal tamper 6 potential mode"]
pub type Itamp6pomW<'a, REG> = crate::BitWriter<'a, REG, Itamp6pom>;
impl<'a, REG> Itamp6pomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 6 event detection is in confirmed mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp6pom::B0x0)
    }
    #[doc = "Internal tamper 6 event detection is in potential mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Itamp6pom::B0x1)
    }
}
impl R {
    #[doc = "Bit 2 - Internal tamper 3 potential mode"]
    #[inline(always)]
    pub fn itamp3pom(&self) -> Itamp3pomR {
        Itamp3pomR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal tamper 4 potential mode"]
    #[inline(always)]
    pub fn itamp4pom(&self) -> Itamp4pomR {
        Itamp4pomR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Internal tamper 5 potential mode"]
    #[inline(always)]
    pub fn itamp5pom(&self) -> Itamp5pomR {
        Itamp5pomR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal tamper 6 potential mode"]
    #[inline(always)]
    pub fn itamp6pom(&self) -> Itamp6pomR {
        Itamp6pomR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Internal tamper 3 potential mode"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3pom(&mut self) -> Itamp3pomW<TampCr3Spec> {
        Itamp3pomW::new(self, 2)
    }
    #[doc = "Bit 3 - Internal tamper 4 potential mode"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4pom(&mut self) -> Itamp4pomW<TampCr3Spec> {
        Itamp4pomW::new(self, 3)
    }
    #[doc = "Bit 4 - Internal tamper 5 potential mode"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5pom(&mut self) -> Itamp5pomW<TampCr3Spec> {
        Itamp5pomW::new(self, 4)
    }
    #[doc = "Bit 5 - Internal tamper 6 potential mode"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6pom(&mut self) -> Itamp6pomW<TampCr3Spec> {
        Itamp6pomW::new(self, 5)
    }
}
#[doc = "TAMP control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampCr3Spec;
impl crate::RegisterSpec for TampCr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_cr3::R`](R) reader structure"]
impl crate::Readable for TampCr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tamp_cr3::W`](W) writer structure"]
impl crate::Writable for TampCr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_CR3 to value 0"]
impl crate::Resettable for TampCr3Spec {
    const RESET_VALUE: u32 = 0;
}
