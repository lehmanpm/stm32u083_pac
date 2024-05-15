#[doc = "Register `RCC_DBGCFGR` reader"]
pub type R = crate::R<RccDbgcfgrSpec>;
#[doc = "Register `RCC_DBGCFGR` writer"]
pub type W = crate::W<RccDbgcfgrSpec>;
#[doc = "Debug support clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Debug support clock enable Set and cleared by software."]
pub type DbgenR = crate::BitReader<Dbgen>;
impl DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            false => Dbgen::B0x0,
            true => Dbgen::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dbgen::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dbgen::B0x1
    }
}
#[doc = "Field `DBGEN` writer - Debug support clock enable Set and cleared by software."]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG, Dbgen>;
impl<'a, REG> DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::B0x1)
    }
}
#[doc = "Debug support reset Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DBG"]
    B0x1 = 1,
}
impl From<Dbgrst> for bool {
    #[inline(always)]
    fn from(variant: Dbgrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGRST` reader - Debug support reset Set and cleared by software."]
pub type DbgrstR = crate::BitReader<Dbgrst>;
impl DbgrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgrst {
        match self.bits {
            false => Dbgrst::B0x0,
            true => Dbgrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dbgrst::B0x0
    }
    #[doc = "Reset DBG"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dbgrst::B0x1
    }
}
#[doc = "Field `DBGRST` writer - Debug support reset Set and cleared by software."]
pub type DbgrstW<'a, REG> = crate::BitWriter<'a, REG, Dbgrst>;
impl<'a, REG> DbgrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgrst::B0x0)
    }
    #[doc = "Reset DBG"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgrst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Debug support clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug support reset Set and cleared by software."]
    #[inline(always)]
    pub fn dbgrst(&self) -> DbgrstR {
        DbgrstR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug support clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DbgenW<RccDbgcfgrSpec> {
        DbgenW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug support reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DbgrstW<RccDbgcfgrSpec> {
        DbgrstW::new(self, 1)
    }
}
#[doc = "Debug configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_dbgcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_dbgcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccDbgcfgrSpec;
impl crate::RegisterSpec for RccDbgcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_dbgcfgr::R`](R) reader structure"]
impl crate::Readable for RccDbgcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_dbgcfgr::W`](W) writer structure"]
impl crate::Writable for RccDbgcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_DBGCFGR to value 0"]
impl crate::Resettable for RccDbgcfgrSpec {
    const RESET_VALUE: u32 = 0;
}
