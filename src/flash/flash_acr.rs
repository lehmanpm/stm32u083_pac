#[doc = "Register `FLASH_ACR` reader"]
pub type R = crate::R<FlashAcrSpec>;
#[doc = "Register `FLASH_ACR` writer"]
pub type W = crate::W<FlashAcrSpec>;
#[doc = "Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Latency {
    #[doc = "0: Zero wait states"]
    B0x0 = 0,
    #[doc = "1: One wait state"]
    B0x1 = 1,
}
impl From<Latency> for u8 {
    #[inline(always)]
    fn from(variant: Latency) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Latency {
    type Ux = u8;
}
impl crate::IsEnum for Latency {}
#[doc = "Field `LATENCY` reader - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read."]
pub type LatencyR = crate::FieldReader<Latency>;
impl LatencyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Latency> {
        match self.bits {
            0 => Some(Latency::B0x0),
            1 => Some(Latency::B0x1),
            _ => None,
        }
    }
    #[doc = "Zero wait states"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Latency::B0x0
    }
    #[doc = "One wait state"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Latency::B0x1
    }
}
#[doc = "Field `LATENCY` writer - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read."]
pub type LatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Latency>;
impl<'a, REG> LatencyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero wait states"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Latency::B0x0)
    }
    #[doc = "One wait state"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Latency::B0x1)
    }
}
#[doc = "CPU Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prften {
    #[doc = "0: CPU Prefetch disabled"]
    B0x0 = 0,
    #[doc = "1: CPU Prefetch enabled"]
    B0x1 = 1,
}
impl From<Prften> for bool {
    #[inline(always)]
    fn from(variant: Prften) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - CPU Prefetch enable"]
pub type PrftenR = crate::BitReader<Prften>;
impl PrftenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prften {
        match self.bits {
            false => Prften::B0x0,
            true => Prften::B0x1,
        }
    }
    #[doc = "CPU Prefetch disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Prften::B0x0
    }
    #[doc = "CPU Prefetch enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Prften::B0x1
    }
}
#[doc = "Field `PRFTEN` writer - CPU Prefetch enable"]
pub type PrftenW<'a, REG> = crate::BitWriter<'a, REG, Prften>;
impl<'a, REG> PrftenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU Prefetch disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Prften::B0x0)
    }
    #[doc = "CPU Prefetch enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Prften::B0x1)
    }
}
#[doc = "CPU Instruction cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icen {
    #[doc = "0: CPU Instruction cache is disabled"]
    B0x0 = 0,
    #[doc = "1: CPU Instruction cache is enabled"]
    B0x1 = 1,
}
impl From<Icen> for bool {
    #[inline(always)]
    fn from(variant: Icen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEN` reader - CPU Instruction cache enable"]
pub type IcenR = crate::BitReader<Icen>;
impl IcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icen {
        match self.bits {
            false => Icen::B0x0,
            true => Icen::B0x1,
        }
    }
    #[doc = "CPU Instruction cache is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Icen::B0x0
    }
    #[doc = "CPU Instruction cache is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Icen::B0x1
    }
}
#[doc = "Field `ICEN` writer - CPU Instruction cache enable"]
pub type IcenW<'a, REG> = crate::BitWriter<'a, REG, Icen>;
impl<'a, REG> IcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU Instruction cache is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Icen::B0x0)
    }
    #[doc = "CPU Instruction cache is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Icen::B0x1)
    }
}
#[doc = "CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icrst {
    #[doc = "0: CPU Instruction cache is not reset"]
    B0x0 = 0,
    #[doc = "1: CPU Instruction cache is reset"]
    B0x1 = 1,
}
impl From<Icrst> for bool {
    #[inline(always)]
    fn from(variant: Icrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICRST` reader - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled."]
pub type IcrstR = crate::BitReader<Icrst>;
impl IcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icrst {
        match self.bits {
            false => Icrst::B0x0,
            true => Icrst::B0x1,
        }
    }
    #[doc = "CPU Instruction cache is not reset"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Icrst::B0x0
    }
    #[doc = "CPU Instruction cache is reset"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Icrst::B0x1
    }
}
#[doc = "Field `ICRST` writer - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled."]
pub type IcrstW<'a, REG> = crate::BitWriter<'a, REG, Icrst>;
impl<'a, REG> IcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU Instruction cache is not reset"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Icrst::B0x0)
    }
    #[doc = "CPU Instruction cache is reset"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Icrst::B0x1)
    }
}
#[doc = "Main flash memory area empty This bit indicates whether the first location of the main flash memory area is erased or has a programmed value. The bit can be set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Empty {
    #[doc = "0: Main flash memory area programmed"]
    B0x0 = 0,
    #[doc = "1: Main flash memory area empty"]
    B0x1 = 1,
}
impl From<Empty> for bool {
    #[inline(always)]
    fn from(variant: Empty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMPTY` reader - Main flash memory area empty This bit indicates whether the first location of the main flash memory area is erased or has a programmed value. The bit can be set and reset by software."]
pub type EmptyR = crate::BitReader<Empty>;
impl EmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Empty {
        match self.bits {
            false => Empty::B0x0,
            true => Empty::B0x1,
        }
    }
    #[doc = "Main flash memory area programmed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Empty::B0x0
    }
    #[doc = "Main flash memory area empty"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Empty::B0x1
    }
}
#[doc = "Field `EMPTY` writer - Main flash memory area empty This bit indicates whether the first location of the main flash memory area is erased or has a programmed value. The bit can be set and reset by software."]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG, Empty>;
impl<'a, REG> EmptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Main flash memory area programmed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Empty::B0x0)
    }
    #[doc = "Main flash memory area empty"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Empty::B0x1)
    }
}
#[doc = "Debug access software enable Software may use this bit to enable/disable the debugger read access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgSwen {
    #[doc = "0: Debugger disabled"]
    B0x0 = 0,
    #[doc = "1: Debugger enabled"]
    B0x1 = 1,
}
impl From<DbgSwen> for bool {
    #[inline(always)]
    fn from(variant: DbgSwen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_SWEN` reader - Debug access software enable Software may use this bit to enable/disable the debugger read access."]
pub type DbgSwenR = crate::BitReader<DbgSwen>;
impl DbgSwenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgSwen {
        match self.bits {
            false => DbgSwen::B0x0,
            true => DbgSwen::B0x1,
        }
    }
    #[doc = "Debugger disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DbgSwen::B0x0
    }
    #[doc = "Debugger enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DbgSwen::B0x1
    }
}
#[doc = "Field `DBG_SWEN` writer - Debug access software enable Software may use this bit to enable/disable the debugger read access."]
pub type DbgSwenW<'a, REG> = crate::BitWriter<'a, REG, DbgSwen>;
impl<'a, REG> DbgSwenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debugger disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DbgSwen::B0x0)
    }
    #[doc = "Debugger enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DbgSwen::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read."]
    #[inline(always)]
    pub fn latency(&self) -> LatencyR {
        LatencyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - CPU Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PrftenR {
        PrftenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> IcenR {
        IcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled."]
    #[inline(always)]
    pub fn icrst(&self) -> IcrstR {
        IcrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Main flash memory area empty This bit indicates whether the first location of the main flash memory area is erased or has a programmed value. The bit can be set and reset by software."]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Debug access software enable Software may use this bit to enable/disable the debugger read access."]
    #[inline(always)]
    pub fn dbg_swen(&self) -> DbgSwenR {
        DbgSwenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read."]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LatencyW<FlashAcrSpec> {
        LatencyW::new(self, 0)
    }
    #[doc = "Bit 8 - CPU Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PrftenW<FlashAcrSpec> {
        PrftenW::new(self, 8)
    }
    #[doc = "Bit 9 - CPU Instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> IcenW<FlashAcrSpec> {
        IcenW::new(self, 9)
    }
    #[doc = "Bit 11 - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> IcrstW<FlashAcrSpec> {
        IcrstW::new(self, 11)
    }
    #[doc = "Bit 16 - Main flash memory area empty This bit indicates whether the first location of the main flash memory area is erased or has a programmed value. The bit can be set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<FlashAcrSpec> {
        EmptyW::new(self, 16)
    }
    #[doc = "Bit 18 - Debug access software enable Software may use this bit to enable/disable the debugger read access."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_swen(&mut self) -> DbgSwenW<FlashAcrSpec> {
        DbgSwenW::new(self, 18)
    }
}
#[doc = "FLASH access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashAcrSpec;
impl crate::RegisterSpec for FlashAcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_acr::R`](R) reader structure"]
impl crate::Readable for FlashAcrSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_acr::W`](W) writer structure"]
impl crate::Writable for FlashAcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_ACR to value 0x0004_0600"]
impl crate::Resettable for FlashAcrSpec {
    const RESET_VALUE: u32 = 0x0004_0600;
}
