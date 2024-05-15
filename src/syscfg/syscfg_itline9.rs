#[doc = "Register `SYSCFG_ITLINE9` reader"]
pub type R = crate::R<SyscfgItline9Spec>;
#[doc = "Field `DMA1_CH1` reader - DMA1 channel 1 interrupt request pending"]
pub type Dma1Ch1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA1 channel 1 interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch1(&self) -> Dma1Ch1R {
        Dma1Ch1R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 9 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline9Spec;
impl crate::RegisterSpec for SyscfgItline9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline9::R`](R) reader structure"]
impl crate::Readable for SyscfgItline9Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE9 to value 0"]
impl crate::Resettable for SyscfgItline9Spec {
    const RESET_VALUE: u32 = 0;
}
