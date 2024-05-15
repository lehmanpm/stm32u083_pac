#[doc = "Register `DMA_CPAR3` reader"]
pub type R = crate::R<DmaCpar3Spec>;
#[doc = "Register `DMA_CPAR3` writer"]
pub type W = crate::W<DmaCpar3Spec>;
#[doc = "Field `PA` reader - Peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of PA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When PSIZE\\[1:0\\]
= 10 (32 bits), bits 1 and 0 of PA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination address if DIR1= 1 and the peripheral source address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type PaR = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of PA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When PSIZE\\[1:0\\]
= 10 (32 bits), bits 1 and 0 of PA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination address if DIR1= 1 and the peripheral source address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of PA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When PSIZE\\[1:0\\]
= 10 (32 bits), bits 1 and 0 of PA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination address if DIR1= 1 and the peripheral source address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address It contains the base address of the peripheral data register from/to which the data is read/written. When PSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of PA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When PSIZE\\[1:0\\]
= 10 (32 bits), bits 1 and 0 of PA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this bitfield identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination address if DIR1= 1 and the peripheral source address if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PaW<DmaCpar3Spec> {
        PaW::new(self, 0)
    }
}
#[doc = "DMA channel 3 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_cpar3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cpar3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCpar3Spec;
impl crate::RegisterSpec for DmaCpar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cpar3::R`](R) reader structure"]
impl crate::Readable for DmaCpar3Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cpar3::W`](W) writer structure"]
impl crate::Writable for DmaCpar3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CPAR3 to value 0"]
impl crate::Resettable for DmaCpar3Spec {
    const RESET_VALUE: u32 = 0;
}
