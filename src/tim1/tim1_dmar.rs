#[doc = "Register `TIM1_DMAR` reader"]
pub type R = crate::R<Tim1DmarSpec>;
#[doc = "Register `TIM1_DMAR` writer"]
pub type W = crate::W<Tim1DmarSpec>;
#[doc = "Field `DMAB` reader - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
pub type DmabR = crate::FieldReader<u32>;
#[doc = "Field `DMAB` writer - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
pub type DmabW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
    #[inline(always)]
    pub fn dmab(&self) -> DmabR {
        DmabR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DmabW<Tim1DmarSpec> {
        DmabW::new(self, 0)
    }
}
#[doc = "TIM1 DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1DmarSpec;
impl crate::RegisterSpec for Tim1DmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_dmar::R`](R) reader structure"]
impl crate::Readable for Tim1DmarSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_dmar::W`](W) writer structure"]
impl crate::Writable for Tim1DmarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_DMAR to value 0"]
impl crate::Resettable for Tim1DmarSpec {
    const RESET_VALUE: u32 = 0;
}
