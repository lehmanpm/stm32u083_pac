#[doc = "Register `SYSCFG_ITLINE31` reader"]
pub type R = crate::R<SyscfgItline31Spec>;
#[doc = "Field `RNG` reader - RNG interrupt request pending"]
pub type RngR = crate::BitReader;
#[doc = "Field `AES` reader - AES interrupt request pending"]
pub type AesR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RNG interrupt request pending"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES interrupt request pending"]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 31 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline31::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline31Spec;
impl crate::RegisterSpec for SyscfgItline31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline31::R`](R) reader structure"]
impl crate::Readable for SyscfgItline31Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE31 to value 0"]
impl crate::Resettable for SyscfgItline31Spec {
    const RESET_VALUE: u32 = 0;
}
