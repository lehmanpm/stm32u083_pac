#[doc = "Register `TAMP_CR2` reader"]
pub type R = crate::R<TampCr2Spec>;
#[doc = "Register `TAMP_CR2` writer"]
pub type W = crate::W<TampCr2Spec>;
#[doc = "Tamper 1 potential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp1pom {
    #[doc = "0: Tamper 1 event detection is in confirmed mode."]
    B0x0 = 0,
    #[doc = "1: Tamper 1 event detection is in potential mode."]
    B0x1 = 1,
}
impl From<Tamp1pom> for bool {
    #[inline(always)]
    fn from(variant: Tamp1pom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1POM` reader - Tamper 1 potential mode"]
pub type Tamp1pomR = crate::BitReader<Tamp1pom>;
impl Tamp1pomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp1pom {
        match self.bits {
            false => Tamp1pom::B0x0,
            true => Tamp1pom::B0x1,
        }
    }
    #[doc = "Tamper 1 event detection is in confirmed mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp1pom::B0x0
    }
    #[doc = "Tamper 1 event detection is in potential mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp1pom::B0x1
    }
}
#[doc = "Field `TAMP1POM` writer - Tamper 1 potential mode"]
pub type Tamp1pomW<'a, REG> = crate::BitWriter<'a, REG, Tamp1pom>;
impl<'a, REG> Tamp1pomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 event detection is in confirmed mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1pom::B0x0)
    }
    #[doc = "Tamper 1 event detection is in potential mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1pom::B0x1)
    }
}
#[doc = "Tamper 2 potential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp2pom {
    #[doc = "0: Tamper 2 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    B0x0 = 0,
    #[doc = "1: Tamper 2 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    B0x1 = 1,
}
impl From<Tamp2pom> for bool {
    #[inline(always)]
    fn from(variant: Tamp2pom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2POM` reader - Tamper 2 potential mode"]
pub type Tamp2pomR = crate::BitReader<Tamp2pom>;
impl Tamp2pomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp2pom {
        match self.bits {
            false => Tamp2pom::B0x0,
            true => Tamp2pom::B0x1,
        }
    }
    #[doc = "Tamper 2 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp2pom::B0x0
    }
    #[doc = "Tamper 2 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp2pom::B0x1
    }
}
#[doc = "Field `TAMP2POM` writer - Tamper 2 potential mode"]
pub type Tamp2pomW<'a, REG> = crate::BitWriter<'a, REG, Tamp2pom>;
impl<'a, REG> Tamp2pomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2pom::B0x0)
    }
    #[doc = "Tamper 2 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2pom::B0x1)
    }
}
#[doc = "Tamper 3 potential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp3pom {
    #[doc = "0: Tamper 3 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    B0x0 = 0,
    #[doc = "1: Tamper 3 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    B0x1 = 1,
}
impl From<Tamp3pom> for bool {
    #[inline(always)]
    fn from(variant: Tamp3pom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3POM` reader - Tamper 3 potential mode"]
pub type Tamp3pomR = crate::BitReader<Tamp3pom>;
impl Tamp3pomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp3pom {
        match self.bits {
            false => Tamp3pom::B0x0,
            true => Tamp3pom::B0x1,
        }
    }
    #[doc = "Tamper 3 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp3pom::B0x0
    }
    #[doc = "Tamper 3 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp3pom::B0x1
    }
}
#[doc = "Field `TAMP3POM` writer - Tamper 3 potential mode"]
pub type Tamp3pomW<'a, REG> = crate::BitWriter<'a, REG, Tamp3pom>;
impl<'a, REG> Tamp3pomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 3 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3pom::B0x0)
    }
    #[doc = "Tamper 3 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3pom::B0x1)
    }
}
#[doc = "Tamper 4 potential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp4pom {
    #[doc = "0: Tamper 4 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    B0x0 = 0,
    #[doc = "1: Tamper 4 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    B0x1 = 1,
}
impl From<Tamp4pom> for bool {
    #[inline(always)]
    fn from(variant: Tamp4pom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP4POM` reader - Tamper 4 potential mode"]
pub type Tamp4pomR = crate::BitReader<Tamp4pom>;
impl Tamp4pomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp4pom {
        match self.bits {
            false => Tamp4pom::B0x0,
            true => Tamp4pom::B0x1,
        }
    }
    #[doc = "Tamper 4 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp4pom::B0x0
    }
    #[doc = "Tamper 4 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp4pom::B0x1
    }
}
#[doc = "Field `TAMP4POM` writer - Tamper 4 potential mode"]
pub type Tamp4pomW<'a, REG> = crate::BitWriter<'a, REG, Tamp4pom>;
impl<'a, REG> Tamp4pomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 4 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp4pom::B0x0)
    }
    #[doc = "Tamper 4 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp4pom::B0x1)
    }
}
#[doc = "Tamper 5 potential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp5pom {
    #[doc = "0: Tamper 5 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    B0x0 = 0,
    #[doc = "1: Tamper 5 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    B0x1 = 1,
}
impl From<Tamp5pom> for bool {
    #[inline(always)]
    fn from(variant: Tamp5pom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP5POM` reader - Tamper 5 potential mode"]
pub type Tamp5pomR = crate::BitReader<Tamp5pom>;
impl Tamp5pomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp5pom {
        match self.bits {
            false => Tamp5pom::B0x0,
            true => Tamp5pom::B0x1,
        }
    }
    #[doc = "Tamper 5 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp5pom::B0x0
    }
    #[doc = "Tamper 5 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp5pom::B0x1
    }
}
#[doc = "Field `TAMP5POM` writer - Tamper 5 potential mode"]
pub type Tamp5pomW<'a, REG> = crate::BitWriter<'a, REG, Tamp5pom>;
impl<'a, REG> Tamp5pomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 5 event detection is in confirmed mode&lt;sup>(1)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp5pom::B0x0)
    }
    #[doc = "Tamper 5 event detection is in potential mode&lt;sup>(2)&lt;/sup>."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp5pom::B0x1)
    }
}
#[doc = "Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp1msk {
    #[doc = "0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection."]
    B0x0 = 0,
    #[doc = "1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers and device secrets&lt;sup>(1)&lt;/sup> are not erased."]
    B0x1 = 1,
}
impl From<Tamp1msk> for bool {
    #[inline(always)]
    fn from(variant: Tamp1msk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1MSK` reader - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type Tamp1mskR = crate::BitReader<Tamp1msk>;
impl Tamp1mskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp1msk {
        match self.bits {
            false => Tamp1msk::B0x0,
            true => Tamp1msk::B0x1,
        }
    }
    #[doc = "Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp1msk::B0x0
    }
    #[doc = "Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers and device secrets&lt;sup>(1)&lt;/sup> are not erased."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp1msk::B0x1
    }
}
#[doc = "Field `TAMP1MSK` writer - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type Tamp1mskW<'a, REG> = crate::BitWriter<'a, REG, Tamp1msk>;
impl<'a, REG> Tamp1mskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1msk::B0x0)
    }
    #[doc = "Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers and device secrets&lt;sup>(1)&lt;/sup> are not erased."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1msk::B0x1)
    }
}
#[doc = "Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp2msk {
    #[doc = "0: Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection."]
    B0x0 = 0,
    #[doc = "1: Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers and device secrets&lt;sup>(1)&lt;/sup> are not erased."]
    B0x1 = 1,
}
impl From<Tamp2msk> for bool {
    #[inline(always)]
    fn from(variant: Tamp2msk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2MSK` reader - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type Tamp2mskR = crate::BitReader<Tamp2msk>;
impl Tamp2mskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp2msk {
        match self.bits {
            false => Tamp2msk::B0x0,
            true => Tamp2msk::B0x1,
        }
    }
    #[doc = "Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp2msk::B0x0
    }
    #[doc = "Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers and device secrets&lt;sup>(1)&lt;/sup> are not erased."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp2msk::B0x1
    }
}
#[doc = "Field `TAMP2MSK` writer - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type Tamp2mskW<'a, REG> = crate::BitWriter<'a, REG, Tamp2msk>;
impl<'a, REG> Tamp2mskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2msk::B0x0)
    }
    #[doc = "Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers and device secrets&lt;sup>(1)&lt;/sup> are not erased."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2msk::B0x1)
    }
}
#[doc = "Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp3msk {
    #[doc = "0: Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to allow next tamper event detection."]
    B0x0 = 0,
    #[doc = "1: Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by hardware. The backup registers and device secrets&lt;sup>(1)&lt;/sup> are not erased."]
    B0x1 = 1,
}
impl From<Tamp3msk> for bool {
    #[inline(always)]
    fn from(variant: Tamp3msk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3MSK` reader - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
pub type Tamp3mskR = crate::BitReader<Tamp3msk>;
impl Tamp3mskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp3msk {
        match self.bits {
            false => Tamp3msk::B0x0,
            true => Tamp3msk::B0x1,
        }
    }
    #[doc = "Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp3msk::B0x0
    }
    #[doc = "Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by hardware. The backup registers and device secrets&lt;sup>(1)&lt;/sup> are not erased."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp3msk::B0x1
    }
}
#[doc = "Field `TAMP3MSK` writer - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
pub type Tamp3mskW<'a, REG> = crate::BitWriter<'a, REG, Tamp3msk>;
impl<'a, REG> Tamp3mskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3msk::B0x0)
    }
    #[doc = "Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by hardware. The backup registers and device secrets&lt;sup>(1)&lt;/sup> are not erased."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3msk::B0x1)
    }
}
#[doc = "Backup registers and device secrets&lt;sup>(1)&lt;/sup> access blocked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bkblock {
    #[doc = "0: backup registers and device secrets&lt;sup>(1)&lt;/sup> can be accessed if no tamper flag is set"]
    B0x0 = 0,
    #[doc = "1: backup registers and device secrets&lt;sup>(1)&lt;/sup> cannot be accessed"]
    B0x1 = 1,
}
impl From<Bkblock> for bool {
    #[inline(always)]
    fn from(variant: Bkblock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKBLOCK` reader - Backup registers and device secrets&lt;sup>(1)&lt;/sup> access blocked"]
pub type BkblockR = crate::BitReader<Bkblock>;
impl BkblockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkblock {
        match self.bits {
            false => Bkblock::B0x0,
            true => Bkblock::B0x1,
        }
    }
    #[doc = "backup registers and device secrets&lt;sup>(1)&lt;/sup> can be accessed if no tamper flag is set"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bkblock::B0x0
    }
    #[doc = "backup registers and device secrets&lt;sup>(1)&lt;/sup> cannot be accessed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bkblock::B0x1
    }
}
#[doc = "Field `BKBLOCK` writer - Backup registers and device secrets&lt;sup>(1)&lt;/sup> access blocked"]
pub type BkblockW<'a, REG> = crate::BitWriter<'a, REG, Bkblock>;
impl<'a, REG> BkblockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "backup registers and device secrets&lt;sup>(1)&lt;/sup> can be accessed if no tamper flag is set"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bkblock::B0x0)
    }
    #[doc = "backup registers and device secrets&lt;sup>(1)&lt;/sup> cannot be accessed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bkblock::B0x1)
    }
}
#[doc = "Field `BKERASE` writer - Backup registers and device secrets&lt;sup>(1)&lt;/sup> erase Writing 1 to this bit reset the backup registers and device secrets&lt;sup>(1)&lt;/sup>. Writing 0 has no effect. This bit is always read as 0."]
pub type BkeraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Active level for tamper 1 input If TAMPFLT1=100 tamper 1 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 1 input falling edge triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp1trg {
    #[doc = "0: If TAMPFLT different from 00 tamper 1 input staying low triggers a tamper detection event"]
    B0x0 = 0,
    #[doc = "1: If TAMPFLT different from 00 tamper 1 input staying low triggers a tamper detection event"]
    B0x1 = 1,
}
impl From<Tamp1trg> for bool {
    #[inline(always)]
    fn from(variant: Tamp1trg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1 input If TAMPFLT1=100 tamper 1 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 1 input falling edge triggers a tamper detection event."]
pub type Tamp1trgR = crate::BitReader<Tamp1trg>;
impl Tamp1trgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp1trg {
        match self.bits {
            false => Tamp1trg::B0x0,
            true => Tamp1trg::B0x1,
        }
    }
    #[doc = "If TAMPFLT different from 00 tamper 1 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp1trg::B0x0
    }
    #[doc = "If TAMPFLT different from 00 tamper 1 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp1trg::B0x1
    }
}
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1 input If TAMPFLT1=100 tamper 1 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 1 input falling edge triggers a tamper detection event."]
pub type Tamp1trgW<'a, REG> = crate::BitWriter<'a, REG, Tamp1trg>;
impl<'a, REG> Tamp1trgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different from 00 tamper 1 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1trg::B0x0)
    }
    #[doc = "If TAMPFLT different from 00 tamper 1 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp1trg::B0x1)
    }
}
#[doc = "Active level for tamper 2 input If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 2 input falling edge triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp2trg {
    #[doc = "0: If TAMPFLT different from 00 tamper 2 input staying low triggers a tamper detection event"]
    B0x0 = 0,
    #[doc = "1: If TAMPFLT different from 00 tamper 2 input staying low triggers a tamper detection event"]
    B0x1 = 1,
}
impl From<Tamp2trg> for bool {
    #[inline(always)]
    fn from(variant: Tamp2trg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2 input If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 2 input falling edge triggers a tamper detection event."]
pub type Tamp2trgR = crate::BitReader<Tamp2trg>;
impl Tamp2trgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp2trg {
        match self.bits {
            false => Tamp2trg::B0x0,
            true => Tamp2trg::B0x1,
        }
    }
    #[doc = "If TAMPFLT different from 00 tamper 2 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp2trg::B0x0
    }
    #[doc = "If TAMPFLT different from 00 tamper 2 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp2trg::B0x1
    }
}
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2 input If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 2 input falling edge triggers a tamper detection event."]
pub type Tamp2trgW<'a, REG> = crate::BitWriter<'a, REG, Tamp2trg>;
impl<'a, REG> Tamp2trgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different from 00 tamper 2 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2trg::B0x0)
    }
    #[doc = "If TAMPFLT different from 00 tamper 2 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp2trg::B0x1)
    }
}
#[doc = "Active level for tamper 3 input If TAMPFLT1=100 tamper 3 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 3 input falling edge triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp3trg {
    #[doc = "0: If TAMPFLT different from 00 tamper 3 input staying low triggers a tamper detection event"]
    B0x0 = 0,
    #[doc = "1: If TAMPFLT different from 00 tamper 3 input staying low triggers a tamper detection event"]
    B0x1 = 1,
}
impl From<Tamp3trg> for bool {
    #[inline(always)]
    fn from(variant: Tamp3trg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3TRG` reader - Active level for tamper 3 input If TAMPFLT1=100 tamper 3 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 3 input falling edge triggers a tamper detection event."]
pub type Tamp3trgR = crate::BitReader<Tamp3trg>;
impl Tamp3trgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp3trg {
        match self.bits {
            false => Tamp3trg::B0x0,
            true => Tamp3trg::B0x1,
        }
    }
    #[doc = "If TAMPFLT different from 00 tamper 3 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp3trg::B0x0
    }
    #[doc = "If TAMPFLT different from 00 tamper 3 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp3trg::B0x1
    }
}
#[doc = "Field `TAMP3TRG` writer - Active level for tamper 3 input If TAMPFLT1=100 tamper 3 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 3 input falling edge triggers a tamper detection event."]
pub type Tamp3trgW<'a, REG> = crate::BitWriter<'a, REG, Tamp3trg>;
impl<'a, REG> Tamp3trgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different from 00 tamper 3 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3trg::B0x0)
    }
    #[doc = "If TAMPFLT different from 00 tamper 3 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp3trg::B0x1)
    }
}
#[doc = "Active level for tamper 4 input (active mode disabled) If TAMPFLT1=100 tamper 4 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 4 input falling edge triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp4trg {
    #[doc = "0: If TAMPFLT different from 00 tamper 4 input staying low triggers a tamper detection event"]
    B0x0 = 0,
    #[doc = "1: If TAMPFLT different from 00 tamper 4 input staying low triggers a tamper detection event"]
    B0x1 = 1,
}
impl From<Tamp4trg> for bool {
    #[inline(always)]
    fn from(variant: Tamp4trg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP4TRG` reader - Active level for tamper 4 input (active mode disabled) If TAMPFLT1=100 tamper 4 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 4 input falling edge triggers a tamper detection event."]
pub type Tamp4trgR = crate::BitReader<Tamp4trg>;
impl Tamp4trgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp4trg {
        match self.bits {
            false => Tamp4trg::B0x0,
            true => Tamp4trg::B0x1,
        }
    }
    #[doc = "If TAMPFLT different from 00 tamper 4 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp4trg::B0x0
    }
    #[doc = "If TAMPFLT different from 00 tamper 4 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp4trg::B0x1
    }
}
#[doc = "Field `TAMP4TRG` writer - Active level for tamper 4 input (active mode disabled) If TAMPFLT1=100 tamper 4 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 4 input falling edge triggers a tamper detection event."]
pub type Tamp4trgW<'a, REG> = crate::BitWriter<'a, REG, Tamp4trg>;
impl<'a, REG> Tamp4trgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different from 00 tamper 4 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp4trg::B0x0)
    }
    #[doc = "If TAMPFLT different from 00 tamper 4 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp4trg::B0x1)
    }
}
#[doc = "Active level for tamper 5 input (active mode disabled) If TAMPFLT1=100 tamper 5 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 5 input falling edge triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamp5trg {
    #[doc = "0: If TAMPFLT different from 00 tamper 5 input staying low triggers a tamper detection event"]
    B0x0 = 0,
    #[doc = "1: If TAMPFLT different from 00 tamper 5 input staying low triggers a tamper detection event"]
    B0x1 = 1,
}
impl From<Tamp5trg> for bool {
    #[inline(always)]
    fn from(variant: Tamp5trg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP5TRG` reader - Active level for tamper 5 input (active mode disabled) If TAMPFLT1=100 tamper 5 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 5 input falling edge triggers a tamper detection event."]
pub type Tamp5trgR = crate::BitReader<Tamp5trg>;
impl Tamp5trgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamp5trg {
        match self.bits {
            false => Tamp5trg::B0x0,
            true => Tamp5trg::B0x1,
        }
    }
    #[doc = "If TAMPFLT different from 00 tamper 5 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tamp5trg::B0x0
    }
    #[doc = "If TAMPFLT different from 00 tamper 5 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tamp5trg::B0x1
    }
}
#[doc = "Field `TAMP5TRG` writer - Active level for tamper 5 input (active mode disabled) If TAMPFLT1=100 tamper 5 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 5 input falling edge triggers a tamper detection event."]
pub type Tamp5trgW<'a, REG> = crate::BitWriter<'a, REG, Tamp5trg>;
impl<'a, REG> Tamp5trgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different from 00 tamper 5 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp5trg::B0x0)
    }
    #[doc = "If TAMPFLT different from 00 tamper 5 input staying low triggers a tamper detection event"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tamp5trg::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper 1 potential mode"]
    #[inline(always)]
    pub fn tamp1pom(&self) -> Tamp1pomR {
        Tamp1pomR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 potential mode"]
    #[inline(always)]
    pub fn tamp2pom(&self) -> Tamp2pomR {
        Tamp2pomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper 3 potential mode"]
    #[inline(always)]
    pub fn tamp3pom(&self) -> Tamp3pomR {
        Tamp3pomR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 4 potential mode"]
    #[inline(always)]
    pub fn tamp4pom(&self) -> Tamp4pomR {
        Tamp4pomR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper 5 potential mode"]
    #[inline(always)]
    pub fn tamp5pom(&self) -> Tamp5pomR {
        Tamp5pomR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    pub fn tamp1msk(&self) -> Tamp1mskR {
        Tamp1mskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    pub fn tamp2msk(&self) -> Tamp2mskR {
        Tamp2mskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
    #[inline(always)]
    pub fn tamp3msk(&self) -> Tamp3mskR {
        Tamp3mskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Backup registers and device secrets&lt;sup>(1)&lt;/sup> access blocked"]
    #[inline(always)]
    pub fn bkblock(&self) -> BkblockR {
        BkblockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input If TAMPFLT1=100 tamper 1 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 1 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp1trg(&self) -> Tamp1trgR {
        Tamp1trgR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 2 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp2trg(&self) -> Tamp2trgR {
        Tamp2trgR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Active level for tamper 3 input If TAMPFLT1=100 tamper 3 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 3 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp3trg(&self) -> Tamp3trgR {
        Tamp3trgR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Active level for tamper 4 input (active mode disabled) If TAMPFLT1=100 tamper 4 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 4 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp4trg(&self) -> Tamp4trgR {
        Tamp4trgR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Active level for tamper 5 input (active mode disabled) If TAMPFLT1=100 tamper 5 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 5 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp5trg(&self) -> Tamp5trgR {
        Tamp5trgR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 potential mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1pom(&mut self) -> Tamp1pomW<TampCr2Spec> {
        Tamp1pomW::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 potential mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2pom(&mut self) -> Tamp2pomW<TampCr2Spec> {
        Tamp2pomW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper 3 potential mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3pom(&mut self) -> Tamp3pomW<TampCr2Spec> {
        Tamp3pomW::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper 4 potential mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp4pom(&mut self) -> Tamp4pomW<TampCr2Spec> {
        Tamp4pomW::new(self, 3)
    }
    #[doc = "Bit 4 - Tamper 5 potential mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp5pom(&mut self) -> Tamp5pomW<TampCr2Spec> {
        Tamp5pomW::new(self, 4)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp1msk(&mut self) -> Tamp1mskW<TampCr2Spec> {
        Tamp1mskW::new(self, 16)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp2msk(&mut self) -> Tamp2mskW<TampCr2Spec> {
        Tamp2mskW::new(self, 17)
    }
    #[doc = "Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp3msk(&mut self) -> Tamp3mskW<TampCr2Spec> {
        Tamp3mskW::new(self, 18)
    }
    #[doc = "Bit 22 - Backup registers and device secrets&lt;sup>(1)&lt;/sup> access blocked"]
    #[inline(always)]
    #[must_use]
    pub fn bkblock(&mut self) -> BkblockW<TampCr2Spec> {
        BkblockW::new(self, 22)
    }
    #[doc = "Bit 23 - Backup registers and device secrets&lt;sup>(1)&lt;/sup> erase Writing 1 to this bit reset the backup registers and device secrets&lt;sup>(1)&lt;/sup>. Writing 0 has no effect. This bit is always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn bkerase(&mut self) -> BkeraseW<TampCr2Spec> {
        BkeraseW::new(self, 23)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input If TAMPFLT1=100 tamper 1 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 1 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> Tamp1trgW<TampCr2Spec> {
        Tamp1trgW::new(self, 24)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 2 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> Tamp2trgW<TampCr2Spec> {
        Tamp2trgW::new(self, 25)
    }
    #[doc = "Bit 26 - Active level for tamper 3 input If TAMPFLT1=100 tamper 3 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 3 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> Tamp3trgW<TampCr2Spec> {
        Tamp3trgW::new(self, 26)
    }
    #[doc = "Bit 27 - Active level for tamper 4 input (active mode disabled) If TAMPFLT1=100 tamper 4 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 4 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp4trg(&mut self) -> Tamp4trgW<TampCr2Spec> {
        Tamp4trgW::new(self, 27)
    }
    #[doc = "Bit 28 - Active level for tamper 5 input (active mode disabled) If TAMPFLT1=100 tamper 5 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 5 input falling edge triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp5trg(&mut self) -> Tamp5trgW<TampCr2Spec> {
        Tamp5trgW::new(self, 28)
    }
}
#[doc = "TAMP control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampCr2Spec;
impl crate::RegisterSpec for TampCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_cr2::R`](R) reader structure"]
impl crate::Readable for TampCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tamp_cr2::W`](W) writer structure"]
impl crate::Writable for TampCr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_CR2 to value 0"]
impl crate::Resettable for TampCr2Spec {
    const RESET_VALUE: u32 = 0;
}
