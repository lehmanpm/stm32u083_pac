#[doc = "Register `TAMP_BKP2R` reader"]
pub type R = crate::R<TampBkp2rSpec>;
#[doc = "Register `TAMP_BKP2R` writer"]
pub type W = crate::W<TampBkp2rSpec>;
#[doc = "Field `BKP` reader - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
pub type BkpR = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
pub type BkpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BkpW<TampBkp2rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "TAMP backup 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampBkp2rSpec;
impl crate::RegisterSpec for TampBkp2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_bkp2r::R`](R) reader structure"]
impl crate::Readable for TampBkp2rSpec {}
#[doc = "`write(|w| ..)` method takes [`tamp_bkp2r::W`](W) writer structure"]
impl crate::Writable for TampBkp2rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_BKP2R to value 0"]
impl crate::Resettable for TampBkp2rSpec {
    const RESET_VALUE: u32 = 0;
}
