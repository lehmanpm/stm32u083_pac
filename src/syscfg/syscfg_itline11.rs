#[doc = "Register `SYSCFG_ITLINE11` reader"]
pub type R = crate::R<SyscfgItline11Spec>;
#[doc = "Field `DMAMUX` reader - DMAMUX interrupt request pending"]
pub type DmamuxR = crate::BitReader;
#[doc = "Field `DMA1_CH4` reader - DMA1 channel 4 interrupt request pending"]
pub type Dma1Ch4R = crate::BitReader;
#[doc = "Field `DMA1_CH5` reader - DMA1 channel 5 interrupt request pending"]
pub type Dma1Ch5R = crate::BitReader;
#[doc = "Field `DMA1_CH6` reader - DMA1 channel 6 interrupt request pending"]
pub type Dma1Ch6R = crate::BitReader;
#[doc = "Field `DMA1_CH7` reader - DMA1 channel 7 interrupt request pending"]
pub type Dma1Ch7R = crate::BitReader;
#[doc = "Field `DMA2_CH1` reader - DMA2 channel 1 interrupt request pending"]
pub type Dma2Ch1R = crate::BitReader;
#[doc = "Field `DMA2_CH2` reader - DMA2 channel 2 interrupt request pending"]
pub type Dma2Ch2R = crate::BitReader;
#[doc = "Field `DMA2_CH3` reader - DMA2 channel 3 interrupt request pending"]
pub type Dma2Ch3R = crate::BitReader;
#[doc = "Field `DMA2_CH4` reader - DMA2 channel 4 interrupt request pending"]
pub type Dma2Ch4R = crate::BitReader;
#[doc = "Field `DMA2_CH5` reader - DMA2 channel 5 interrupt request pending"]
pub type Dma2Ch5R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMAMUX interrupt request pending"]
    #[inline(always)]
    pub fn dmamux(&self) -> DmamuxR {
        DmamuxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1 channel 4 interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch4(&self) -> Dma1Ch4R {
        Dma1Ch4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA1 channel 5 interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch5(&self) -> Dma1Ch5R {
        Dma1Ch5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA1 channel 6 interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch6(&self) -> Dma1Ch6R {
        Dma1Ch6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA1 channel 7 interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch7(&self) -> Dma1Ch7R {
        Dma1Ch7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA2 channel 1 interrupt request pending"]
    #[inline(always)]
    pub fn dma2_ch1(&self) -> Dma2Ch1R {
        Dma2Ch1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA2 channel 2 interrupt request pending"]
    #[inline(always)]
    pub fn dma2_ch2(&self) -> Dma2Ch2R {
        Dma2Ch2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA2 channel 3 interrupt request pending"]
    #[inline(always)]
    pub fn dma2_ch3(&self) -> Dma2Ch3R {
        Dma2Ch3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA2 channel 4 interrupt request pending"]
    #[inline(always)]
    pub fn dma2_ch4(&self) -> Dma2Ch4R {
        Dma2Ch4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA2 channel 5 interrupt request pending"]
    #[inline(always)]
    pub fn dma2_ch5(&self) -> Dma2Ch5R {
        Dma2Ch5R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 11 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline11Spec;
impl crate::RegisterSpec for SyscfgItline11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline11::R`](R) reader structure"]
impl crate::Readable for SyscfgItline11Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE11 to value 0"]
impl crate::Resettable for SyscfgItline11Spec {
    const RESET_VALUE: u32 = 0;
}
