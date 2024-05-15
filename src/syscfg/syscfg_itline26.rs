#[doc = "Register `SYSCFG_ITLINE26` reader"]
pub type R = crate::R<SyscfgItline26Spec>;
#[doc = "Field `SPI2` reader - SPI2 interrupt request pending"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI3` reader - SPI3 interrupt request pending"]
pub type Spi3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SPI2 interrupt request pending"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI3 interrupt request pending"]
    #[inline(always)]
    pub fn spi3(&self) -> Spi3R {
        Spi3R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 26 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline26::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline26Spec;
impl crate::RegisterSpec for SyscfgItline26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline26::R`](R) reader structure"]
impl crate::Readable for SyscfgItline26Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE26 to value 0"]
impl crate::Resettable for SyscfgItline26Spec {
    const RESET_VALUE: u32 = 0;
}
