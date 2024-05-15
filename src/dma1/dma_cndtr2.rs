#[doc = "Register `DMA_CNDTR2` reader"]
pub type R = crate::R<DmaCndtr2Spec>;
#[doc = "Register `DMA_CNDTR2` writer"]
pub type W = crate::W<DmaCndtr2Spec>;
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NdtR = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NdtW<DmaCndtr2Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "DMA channel 2 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cndtr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cndtr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCndtr2Spec;
impl crate::RegisterSpec for DmaCndtr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cndtr2::R`](R) reader structure"]
impl crate::Readable for DmaCndtr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cndtr2::W`](W) writer structure"]
impl crate::Writable for DmaCndtr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CNDTR2 to value 0"]
impl crate::Resettable for DmaCndtr2Spec {
    const RESET_VALUE: u32 = 0;
}
