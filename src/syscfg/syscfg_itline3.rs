#[doc = "Register `SYSCFG_ITLINE3` reader"]
pub type R = crate::R<SyscfgItline3Spec>;
#[doc = "Field `FLASH_ITF` reader - Flash interface interrupt request pending"]
pub type FlashItfR = crate::BitReader;
#[doc = "Field `FLASH_ECC` reader - Flash interface ECC interrupt request pending"]
pub type FlashEccR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Flash interface interrupt request pending"]
    #[inline(always)]
    pub fn flash_itf(&self) -> FlashItfR {
        FlashItfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash interface ECC interrupt request pending"]
    #[inline(always)]
    pub fn flash_ecc(&self) -> FlashEccR {
        FlashEccR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline3Spec;
impl crate::RegisterSpec for SyscfgItline3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline3::R`](R) reader structure"]
impl crate::Readable for SyscfgItline3Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE3 to value 0"]
impl crate::Resettable for SyscfgItline3Spec {
    const RESET_VALUE: u32 = 0;
}
