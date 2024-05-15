#[doc = "Register `OPAMP_LPOTR` reader"]
pub type R = crate::R<OpampLpotrSpec>;
#[doc = "Register `OPAMP_LPOTR` writer"]
pub type W = crate::W<OpampLpotrSpec>;
#[doc = "Field `TRIMLPOFFSETN` reader - Low-power mode trim for NMOS differential pairs"]
pub type TrimlpoffsetnR = crate::FieldReader;
#[doc = "Field `TRIMLPOFFSETN` writer - Low-power mode trim for NMOS differential pairs"]
pub type TrimlpoffsetnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMLPOFFSETP` reader - Low-power mode trim for PMOS differential pairs"]
pub type TrimlpoffsetpR = crate::FieldReader;
#[doc = "Field `TRIMLPOFFSETP` writer - Low-power mode trim for PMOS differential pairs"]
pub type TrimlpoffsetpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Low-power mode trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetn(&self) -> TrimlpoffsetnR {
        TrimlpoffsetnR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Low-power mode trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetp(&self) -> TrimlpoffsetpR {
        TrimlpoffsetpR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Low-power mode trim for NMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimlpoffsetn(&mut self) -> TrimlpoffsetnW<OpampLpotrSpec> {
        TrimlpoffsetnW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Low-power mode trim for PMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimlpoffsetp(&mut self) -> TrimlpoffsetpW<OpampLpotrSpec> {
        TrimlpoffsetpW::new(self, 8)
    }
}
#[doc = "OPAMP offset trimming register in low-power mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp_lpotr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp_lpotr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpampLpotrSpec;
impl crate::RegisterSpec for OpampLpotrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp_lpotr::R`](R) reader structure"]
impl crate::Readable for OpampLpotrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp_lpotr::W`](W) writer structure"]
impl crate::Writable for OpampLpotrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP_LPOTR to value 0"]
impl crate::Resettable for OpampLpotrSpec {
    const RESET_VALUE: u32 = 0;
}
