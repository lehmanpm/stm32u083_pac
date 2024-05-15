#[doc = "Register `SYSCFG_TSCCR` reader"]
pub type R = crate::R<SyscfgTsccrSpec>;
#[doc = "Register `SYSCFG_TSCCR` writer"]
pub type W = crate::W<SyscfgTsccrSpec>;
#[doc = "Comparator mode for group 2 on I/O 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G2Io1 {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enable connection of PB4 to COMP2"]
    B0x1 = 1,
}
impl From<G2Io1> for bool {
    #[inline(always)]
    fn from(variant: G2Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G2_IO1` reader - Comparator mode for group 2 on I/O 1"]
pub type G2Io1R = crate::BitReader<G2Io1>;
impl G2Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G2Io1 {
        match self.bits {
            false => G2Io1::B0x0,
            true => G2Io1::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G2Io1::B0x0
    }
    #[doc = "Enable connection of PB4 to COMP2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G2Io1::B0x1
    }
}
#[doc = "Field `G2_IO1` writer - Comparator mode for group 2 on I/O 1"]
pub type G2Io1W<'a, REG> = crate::BitWriter<'a, REG, G2Io1>;
impl<'a, REG> G2Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io1::B0x0)
    }
    #[doc = "Enable connection of PB4 to COMP2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io1::B0x1)
    }
}
#[doc = "Comparator mode for group 2 on I/O 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G2Io3 {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enable connection of PB6 to COMP2"]
    B0x1 = 1,
}
impl From<G2Io3> for bool {
    #[inline(always)]
    fn from(variant: G2Io3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G2_IO3` reader - Comparator mode for group 2 on I/O 3"]
pub type G2Io3R = crate::BitReader<G2Io3>;
impl G2Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G2Io3 {
        match self.bits {
            false => G2Io3::B0x0,
            true => G2Io3::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G2Io3::B0x0
    }
    #[doc = "Enable connection of PB6 to COMP2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G2Io3::B0x1
    }
}
#[doc = "Field `G2_IO3` writer - Comparator mode for group 2 on I/O 3"]
pub type G2Io3W<'a, REG> = crate::BitWriter<'a, REG, G2Io3>;
impl<'a, REG> G2Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io3::B0x0)
    }
    #[doc = "Enable connection of PB6 to COMP2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io3::B0x1)
    }
}
#[doc = "Comparator mode for group 4 on I/O 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G4Io3 {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enable connection of PC6 to COMP2"]
    B0x1 = 1,
}
impl From<G4Io3> for bool {
    #[inline(always)]
    fn from(variant: G4Io3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G4_IO3` reader - Comparator mode for group 4 on I/O 3"]
pub type G4Io3R = crate::BitReader<G4Io3>;
impl G4Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G4Io3 {
        match self.bits {
            false => G4Io3::B0x0,
            true => G4Io3::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G4Io3::B0x0
    }
    #[doc = "Enable connection of PC6 to COMP2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G4Io3::B0x1
    }
}
#[doc = "Field `G4_IO3` writer - Comparator mode for group 4 on I/O 3"]
pub type G4Io3W<'a, REG> = crate::BitWriter<'a, REG, G4Io3>;
impl<'a, REG> G4Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io3::B0x0)
    }
    #[doc = "Enable connection of PC6 to COMP2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io3::B0x1)
    }
}
#[doc = "Comparator mode for group 6 on I/O 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G6Io1 {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enable connection of PD10 to COMP1"]
    B0x1 = 1,
}
impl From<G6Io1> for bool {
    #[inline(always)]
    fn from(variant: G6Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G6_IO1` reader - Comparator mode for group 6 on I/O 1"]
pub type G6Io1R = crate::BitReader<G6Io1>;
impl G6Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G6Io1 {
        match self.bits {
            false => G6Io1::B0x0,
            true => G6Io1::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G6Io1::B0x0
    }
    #[doc = "Enable connection of PD10 to COMP1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G6Io1::B0x1
    }
}
#[doc = "Field `G6_IO1` writer - Comparator mode for group 6 on I/O 1"]
pub type G6Io1W<'a, REG> = crate::BitWriter<'a, REG, G6Io1>;
impl<'a, REG> G6Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io1::B0x0)
    }
    #[doc = "Enable connection of PD10 to COMP1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io1::B0x1)
    }
}
#[doc = "Comparator mode for group 7 on I/O 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G7Io1 {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enable connection of PA9 to COMP1"]
    B0x1 = 1,
}
impl From<G7Io1> for bool {
    #[inline(always)]
    fn from(variant: G7Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G7_IO1` reader - Comparator mode for group 7 on I/O 1"]
pub type G7Io1R = crate::BitReader<G7Io1>;
impl G7Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G7Io1 {
        match self.bits {
            false => G7Io1::B0x0,
            true => G7Io1::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G7Io1::B0x0
    }
    #[doc = "Enable connection of PA9 to COMP1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G7Io1::B0x1
    }
}
#[doc = "Field `G7_IO1` writer - Comparator mode for group 7 on I/O 1"]
pub type G7Io1W<'a, REG> = crate::BitWriter<'a, REG, G7Io1>;
impl<'a, REG> G7Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io1::B0x0)
    }
    #[doc = "Enable connection of PA9 to COMP1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io1::B0x1)
    }
}
#[doc = "I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TscIoctrl {
    #[doc = "0: I/O configured through the corresponding control register"]
    B0x0 = 0,
    #[doc = "1: I/O configured as analog when TSC AF is activated"]
    B0x1 = 1,
}
impl From<TscIoctrl> for bool {
    #[inline(always)]
    fn from(variant: TscIoctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSC_IOCTRL` reader - I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware."]
pub type TscIoctrlR = crate::BitReader<TscIoctrl>;
impl TscIoctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TscIoctrl {
        match self.bits {
            false => TscIoctrl::B0x0,
            true => TscIoctrl::B0x1,
        }
    }
    #[doc = "I/O configured through the corresponding control register"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TscIoctrl::B0x0
    }
    #[doc = "I/O configured as analog when TSC AF is activated"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TscIoctrl::B0x1
    }
}
#[doc = "Field `TSC_IOCTRL` writer - I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware."]
pub type TscIoctrlW<'a, REG> = crate::BitWriter<'a, REG, TscIoctrl>;
impl<'a, REG> TscIoctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O configured through the corresponding control register"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TscIoctrl::B0x0)
    }
    #[doc = "I/O configured as analog when TSC AF is activated"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TscIoctrl::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator mode for group 2 on I/O 1"]
    #[inline(always)]
    pub fn g2_io1(&self) -> G2Io1R {
        G2Io1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator mode for group 2 on I/O 3"]
    #[inline(always)]
    pub fn g2_io3(&self) -> G2Io3R {
        G2Io3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator mode for group 4 on I/O 3"]
    #[inline(always)]
    pub fn g4_io3(&self) -> G4Io3R {
        G4Io3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator mode for group 6 on I/O 1"]
    #[inline(always)]
    pub fn g6_io1(&self) -> G6Io1R {
        G6Io1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator mode for group 7 on I/O 1"]
    #[inline(always)]
    pub fn g7_io1(&self) -> G7Io1R {
        G7Io1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware."]
    #[inline(always)]
    pub fn tsc_ioctrl(&self) -> TscIoctrlR {
        TscIoctrlR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator mode for group 2 on I/O 1"]
    #[inline(always)]
    #[must_use]
    pub fn g2_io1(&mut self) -> G2Io1W<SyscfgTsccrSpec> {
        G2Io1W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator mode for group 2 on I/O 3"]
    #[inline(always)]
    #[must_use]
    pub fn g2_io3(&mut self) -> G2Io3W<SyscfgTsccrSpec> {
        G2Io3W::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator mode for group 4 on I/O 3"]
    #[inline(always)]
    #[must_use]
    pub fn g4_io3(&mut self) -> G4Io3W<SyscfgTsccrSpec> {
        G4Io3W::new(self, 2)
    }
    #[doc = "Bit 3 - Comparator mode for group 6 on I/O 1"]
    #[inline(always)]
    #[must_use]
    pub fn g6_io1(&mut self) -> G6Io1W<SyscfgTsccrSpec> {
        G6Io1W::new(self, 3)
    }
    #[doc = "Bit 4 - Comparator mode for group 7 on I/O 1"]
    #[inline(always)]
    #[must_use]
    pub fn g7_io1(&mut self) -> G7Io1W<SyscfgTsccrSpec> {
        G7Io1W::new(self, 4)
    }
    #[doc = "Bit 5 - I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn tsc_ioctrl(&mut self) -> TscIoctrlW<SyscfgTsccrSpec> {
        TscIoctrlW::new(self, 5)
    }
}
#[doc = "SYSCFG TSC comparator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_tsccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_tsccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgTsccrSpec;
impl crate::RegisterSpec for SyscfgTsccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_tsccr::R`](R) reader structure"]
impl crate::Readable for SyscfgTsccrSpec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_tsccr::W`](W) writer structure"]
impl crate::Writable for SyscfgTsccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_TSCCR to value 0"]
impl crate::Resettable for SyscfgTsccrSpec {
    const RESET_VALUE: u32 = 0;
}
