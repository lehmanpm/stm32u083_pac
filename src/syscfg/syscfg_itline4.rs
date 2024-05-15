#[doc = "Register `SYSCFG_ITLINE4` reader"]
pub type R = crate::R<SyscfgItline4Spec>;
#[doc = "Field `RCC` reader - Reset and clock control interrupt request pending"]
pub type RccR = crate::BitReader;
#[doc = "Field `CRS` reader - CRS interrupt request pending"]
pub type CrsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reset and clock control interrupt request pending"]
    #[inline(always)]
    pub fn rcc(&self) -> RccR {
        RccR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRS interrupt request pending"]
    #[inline(always)]
    pub fn crs(&self) -> CrsR {
        CrsR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 4 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline4Spec;
impl crate::RegisterSpec for SyscfgItline4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline4::R`](R) reader structure"]
impl crate::Readable for SyscfgItline4Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE4 to value 0"]
impl crate::Resettable for SyscfgItline4Spec {
    const RESET_VALUE: u32 = 0;
}
