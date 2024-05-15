#[doc = "Register `SYSCFG_ITLINE25` reader"]
pub type R = crate::R<SyscfgItline25Spec>;
#[doc = "Field `SPI1` reader - SPI1 interrupt request pending"]
pub type Spi1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SPI1 interrupt request pending"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 25 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline25::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline25Spec;
impl crate::RegisterSpec for SyscfgItline25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline25::R`](R) reader structure"]
impl crate::Readable for SyscfgItline25Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE25 to value 0"]
impl crate::Resettable for SyscfgItline25Spec {
    const RESET_VALUE: u32 = 0;
}
