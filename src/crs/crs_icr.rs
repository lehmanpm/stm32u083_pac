#[doc = "Register `CRS_ICR` reader"]
pub type R = crate::R<CrsIcrSpec>;
#[doc = "Register `CRS_ICR` writer"]
pub type W = crate::W<CrsIcrSpec>;
#[doc = "Field `SYNCOKC` reader - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register."]
pub type SyncokcR = crate::BitReader;
#[doc = "Field `SYNCOKC` writer - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register."]
pub type SyncokcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCWARNC` reader - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register."]
pub type SyncwarncR = crate::BitReader;
#[doc = "Field `SYNCWARNC` writer - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register."]
pub type SyncwarncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRC` reader - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register."]
pub type ErrcR = crate::BitReader;
#[doc = "Field `ERRC` writer - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register."]
pub type ErrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESYNCC` reader - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register."]
pub type EsynccR = crate::BitReader;
#[doc = "Field `ESYNCC` writer - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register."]
pub type EsynccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn syncokc(&self) -> SyncokcR {
        SyncokcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn syncwarnc(&self) -> SyncwarncR {
        SyncwarncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn errc(&self) -> ErrcR {
        ErrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn esyncc(&self) -> EsynccR {
        EsynccR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn syncokc(&mut self) -> SyncokcW<CrsIcrSpec> {
        SyncokcW::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn syncwarnc(&mut self) -> SyncwarncW<CrsIcrSpec> {
        SyncwarncW::new(self, 1)
    }
    #[doc = "Bit 2 - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ErrcW<CrsIcrSpec> {
        ErrcW::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn esyncc(&mut self) -> EsynccW<CrsIcrSpec> {
        EsynccW::new(self, 3)
    }
}
#[doc = "CRS interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs_icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crs_icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsIcrSpec;
impl crate::RegisterSpec for CrsIcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crs_icr::R`](R) reader structure"]
impl crate::Readable for CrsIcrSpec {}
#[doc = "`write(|w| ..)` method takes [`crs_icr::W`](W) writer structure"]
impl crate::Writable for CrsIcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRS_ICR to value 0"]
impl crate::Resettable for CrsIcrSpec {
    const RESET_VALUE: u32 = 0;
}
