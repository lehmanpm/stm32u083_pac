#[doc = "Register `AES_ICR` writer"]
pub type W = crate::W<AesIcrSpec>;
#[doc = "Field `CCF` writer - Computation complete flag clear Setting this bit clears the CCF status bit of the AES_ISR register."]
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWEIF` writer - Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the AES_ISR register, and clears both RDERRF and WRERRF flags in the AES_SR register."]
pub type RweifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEIF` writer - Key error interrupt flag clear Setting this bit clears the KEIF status bit of the AES_ISR register."]
pub type KeifW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Computation complete flag clear Setting this bit clears the CCF status bit of the AES_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CcfW<AesIcrSpec> {
        CcfW::new(self, 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the AES_ISR register, and clears both RDERRF and WRERRF flags in the AES_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn rweif(&mut self) -> RweifW<AesIcrSpec> {
        RweifW::new(self, 1)
    }
    #[doc = "Bit 2 - Key error interrupt flag clear Setting this bit clears the KEIF status bit of the AES_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn keif(&mut self) -> KeifW<AesIcrSpec> {
        KeifW::new(self, 2)
    }
}
#[doc = "AES interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesIcrSpec;
impl crate::RegisterSpec for AesIcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_icr::W`](W) writer structure"]
impl crate::Writable for AesIcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_ICR to value 0"]
impl crate::Resettable for AesIcrSpec {
    const RESET_VALUE: u32 = 0;
}
