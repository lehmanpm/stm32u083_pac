#[doc = "Register `TSC_ICR` reader"]
pub type R = crate::R<TscIcrSpec>;
#[doc = "Register `TSC_ICR` writer"]
pub type W = crate::W<TscIcrSpec>;
#[doc = "End of acquisition interrupt clear This bit is set by software to clear the end of acquisition flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoaic {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clears the corresponding EOAF of the TSC_ISR register"]
    B0x1 = 1,
}
impl From<Eoaic> for bool {
    #[inline(always)]
    fn from(variant: Eoaic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOAIC` reader - End of acquisition interrupt clear This bit is set by software to clear the end of acquisition flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect."]
pub type EoaicR = crate::BitReader<Eoaic>;
impl EoaicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoaic {
        match self.bits {
            false => Eoaic::B0x0,
            true => Eoaic::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eoaic::B0x0
    }
    #[doc = "Clears the corresponding EOAF of the TSC_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eoaic::B0x1
    }
}
#[doc = "Field `EOAIC` writer - End of acquisition interrupt clear This bit is set by software to clear the end of acquisition flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect."]
pub type EoaicW<'a, REG> = crate::BitWriter<'a, REG, Eoaic>;
impl<'a, REG> EoaicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eoaic::B0x0)
    }
    #[doc = "Clears the corresponding EOAF of the TSC_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eoaic::B0x1)
    }
}
#[doc = "Max count error interrupt clear This bit is set by software to clear the max count error flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mceic {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Clears the corresponding MCEF of the TSC_ISR register"]
    B0x1 = 1,
}
impl From<Mceic> for bool {
    #[inline(always)]
    fn from(variant: Mceic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCEIC` reader - Max count error interrupt clear This bit is set by software to clear the max count error flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect."]
pub type MceicR = crate::BitReader<Mceic>;
impl MceicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mceic {
        match self.bits {
            false => Mceic::B0x0,
            true => Mceic::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mceic::B0x0
    }
    #[doc = "Clears the corresponding MCEF of the TSC_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mceic::B0x1
    }
}
#[doc = "Field `MCEIC` writer - Max count error interrupt clear This bit is set by software to clear the max count error flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect."]
pub type MceicW<'a, REG> = crate::BitWriter<'a, REG, Mceic>;
impl<'a, REG> MceicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mceic::B0x0)
    }
    #[doc = "Clears the corresponding MCEF of the TSC_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mceic::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - End of acquisition interrupt clear This bit is set by software to clear the end of acquisition flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn eoaic(&self) -> EoaicR {
        EoaicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error interrupt clear This bit is set by software to clear the max count error flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn mceic(&self) -> MceicR {
        MceicR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of acquisition interrupt clear This bit is set by software to clear the end of acquisition flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn eoaic(&mut self) -> EoaicW<TscIcrSpec> {
        EoaicW::new(self, 0)
    }
    #[doc = "Bit 1 - Max count error interrupt clear This bit is set by software to clear the max count error flag and it is cleared by hardware when the flag is reset. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mceic(&mut self) -> MceicW<TscIcrSpec> {
        MceicW::new(self, 1)
    }
}
#[doc = "TSC interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscIcrSpec;
impl crate::RegisterSpec for TscIcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsc_icr::R`](R) reader structure"]
impl crate::Readable for TscIcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsc_icr::W`](W) writer structure"]
impl crate::Writable for TscIcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSC_ICR to value 0"]
impl crate::Resettable for TscIcrSpec {
    const RESET_VALUE: u32 = 0;
}
