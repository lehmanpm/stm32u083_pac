#[doc = "Register `OPAMP_OTR` reader"]
pub type R = crate::R<OpampOtrSpec>;
#[doc = "Register `OPAMP_OTR` writer"]
pub type W = crate::W<OpampOtrSpec>;
#[doc = "Field `TRIMOFFSETN` reader - Trim for NMOS differential pairs"]
pub type TrimoffsetnR = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - Trim for NMOS differential pairs"]
pub type TrimoffsetnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETP` reader - Trim for PMOS differential pairs"]
pub type TrimoffsetpR = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - Trim for PMOS differential pairs"]
pub type TrimoffsetpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TrimoffsetnR {
        TrimoffsetnR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TrimoffsetpR {
        TrimoffsetpR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TrimoffsetnW<OpampOtrSpec> {
        TrimoffsetnW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TrimoffsetpW<OpampOtrSpec> {
        TrimoffsetpW::new(self, 8)
    }
}
#[doc = "OPAMP offset trimming register in normal mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp_otr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp_otr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpampOtrSpec;
impl crate::RegisterSpec for OpampOtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp_otr::R`](R) reader structure"]
impl crate::Readable for OpampOtrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp_otr::W`](W) writer structure"]
impl crate::Writable for OpampOtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP_OTR to value 0"]
impl crate::Resettable for OpampOtrSpec {
    const RESET_VALUE: u32 = 0;
}
