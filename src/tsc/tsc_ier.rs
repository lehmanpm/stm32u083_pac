#[doc = "Register `TSC_IER` reader"]
pub type R = crate::R<TscIerSpec>;
#[doc = "Register `TSC_IER` writer"]
pub type W = crate::W<TscIerSpec>;
#[doc = "End of acquisition interrupt enable This bit is set and cleared by software to enable/disable the end of acquisition interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoaie {
    #[doc = "0: End of acquisition interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: End of acquisition interrupt enabled"]
    B0x1 = 1,
}
impl From<Eoaie> for bool {
    #[inline(always)]
    fn from(variant: Eoaie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOAIE` reader - End of acquisition interrupt enable This bit is set and cleared by software to enable/disable the end of acquisition interrupt."]
pub type EoaieR = crate::BitReader<Eoaie>;
impl EoaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoaie {
        match self.bits {
            false => Eoaie::B0x0,
            true => Eoaie::B0x1,
        }
    }
    #[doc = "End of acquisition interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eoaie::B0x0
    }
    #[doc = "End of acquisition interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eoaie::B0x1
    }
}
#[doc = "Field `EOAIE` writer - End of acquisition interrupt enable This bit is set and cleared by software to enable/disable the end of acquisition interrupt."]
pub type EoaieW<'a, REG> = crate::BitWriter<'a, REG, Eoaie>;
impl<'a, REG> EoaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of acquisition interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eoaie::B0x0)
    }
    #[doc = "End of acquisition interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eoaie::B0x1)
    }
}
#[doc = "Max count error interrupt enable This bit is set and cleared by software to enable/disable the max count error interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mceie {
    #[doc = "0: Max count error interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Max count error interrupt enabled"]
    B0x1 = 1,
}
impl From<Mceie> for bool {
    #[inline(always)]
    fn from(variant: Mceie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCEIE` reader - Max count error interrupt enable This bit is set and cleared by software to enable/disable the max count error interrupt."]
pub type MceieR = crate::BitReader<Mceie>;
impl MceieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mceie {
        match self.bits {
            false => Mceie::B0x0,
            true => Mceie::B0x1,
        }
    }
    #[doc = "Max count error interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mceie::B0x0
    }
    #[doc = "Max count error interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mceie::B0x1
    }
}
#[doc = "Field `MCEIE` writer - Max count error interrupt enable This bit is set and cleared by software to enable/disable the max count error interrupt."]
pub type MceieW<'a, REG> = crate::BitWriter<'a, REG, Mceie>;
impl<'a, REG> MceieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Max count error interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mceie::B0x0)
    }
    #[doc = "Max count error interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mceie::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - End of acquisition interrupt enable This bit is set and cleared by software to enable/disable the end of acquisition interrupt."]
    #[inline(always)]
    pub fn eoaie(&self) -> EoaieR {
        EoaieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error interrupt enable This bit is set and cleared by software to enable/disable the max count error interrupt."]
    #[inline(always)]
    pub fn mceie(&self) -> MceieR {
        MceieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of acquisition interrupt enable This bit is set and cleared by software to enable/disable the end of acquisition interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn eoaie(&mut self) -> EoaieW<TscIerSpec> {
        EoaieW::new(self, 0)
    }
    #[doc = "Bit 1 - Max count error interrupt enable This bit is set and cleared by software to enable/disable the max count error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mceie(&mut self) -> MceieW<TscIerSpec> {
        MceieW::new(self, 1)
    }
}
#[doc = "TSC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscIerSpec;
impl crate::RegisterSpec for TscIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsc_ier::R`](R) reader structure"]
impl crate::Readable for TscIerSpec {}
#[doc = "`write(|w| ..)` method takes [`tsc_ier::W`](W) writer structure"]
impl crate::Writable for TscIerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSC_IER to value 0"]
impl crate::Resettable for TscIerSpec {
    const RESET_VALUE: u32 = 0;
}
