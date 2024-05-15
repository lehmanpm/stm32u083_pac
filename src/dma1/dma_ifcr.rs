#[doc = "Register `DMA_IFCR` writer"]
pub type W = crate::W<DmaIfcrSpec>;
#[doc = "Field `CGIF1` writer - Global interrupt flag clear for channel 1"]
pub type Cgif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` writer - Transfer complete flag clear for channel 1"]
pub type Ctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` writer - Half transfer flag clear for channel 1"]
pub type Chtif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` writer - Transfer error flag clear for channel 1"]
pub type Cteif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF2` writer - Global interrupt flag clear for channel 2"]
pub type Cgif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` writer - Transfer complete flag clear for channel 2"]
pub type Ctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` writer - Half transfer flag clear for channel 2"]
pub type Chtif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` writer - Transfer error flag clear for channel 2"]
pub type Cteif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF3` writer - Global interrupt flag clear for channel 3"]
pub type Cgif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` writer - Transfer complete flag clear for channel 3"]
pub type Ctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` writer - Half transfer flag clear for channel 3"]
pub type Chtif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` writer - Transfer error flag clear for channel 3"]
pub type Cteif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF4` writer - Global interrupt flag clear for channel 4"]
pub type Cgif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF4` writer - Transfer complete flag clear for channel 4"]
pub type Ctcif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF4` writer - Half transfer flag clear for channel 4"]
pub type Chtif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF4` writer - Transfer error flag clear for channel 4"]
pub type Cteif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF5` writer - Global interrupt flag clear for channel 5"]
pub type Cgif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF5` writer - Transfer complete flag clear for channel 5"]
pub type Ctcif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF5` writer - Half transfer flag clear for channel 5"]
pub type Chtif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF5` writer - Transfer error flag clear for channel 5"]
pub type Cteif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF6` writer - Global interrupt flag clear for channel 6"]
pub type Cgif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF6` writer - Transfer complete flag clear for channel 6"]
pub type Ctcif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF6` writer - Half transfer flag clear for channel 6"]
pub type Chtif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF6` writer - Transfer error flag clear for channel 6"]
pub type Cteif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF7` writer - Global interrupt flag clear for channel 7"]
pub type Cgif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF7` writer - Transfer complete flag clear for channel 7"]
pub type Ctcif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF7` writer - Half transfer flag clear for channel 7"]
pub type Chtif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF7` writer - Transfer error flag clear for channel 7"]
pub type Cteif7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Global interrupt flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> Cgif1W<DmaIfcrSpec> {
        Cgif1W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> Ctcif1W<DmaIfcrSpec> {
        Ctcif1W::new(self, 1)
    }
    #[doc = "Bit 2 - Half transfer flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> Chtif1W<DmaIfcrSpec> {
        Chtif1W::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer error flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> Cteif1W<DmaIfcrSpec> {
        Cteif1W::new(self, 3)
    }
    #[doc = "Bit 4 - Global interrupt flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> Cgif2W<DmaIfcrSpec> {
        Cgif2W::new(self, 4)
    }
    #[doc = "Bit 5 - Transfer complete flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> Ctcif2W<DmaIfcrSpec> {
        Ctcif2W::new(self, 5)
    }
    #[doc = "Bit 6 - Half transfer flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> Chtif2W<DmaIfcrSpec> {
        Chtif2W::new(self, 6)
    }
    #[doc = "Bit 7 - Transfer error flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> Cteif2W<DmaIfcrSpec> {
        Cteif2W::new(self, 7)
    }
    #[doc = "Bit 8 - Global interrupt flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> Cgif3W<DmaIfcrSpec> {
        Cgif3W::new(self, 8)
    }
    #[doc = "Bit 9 - Transfer complete flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> Ctcif3W<DmaIfcrSpec> {
        Ctcif3W::new(self, 9)
    }
    #[doc = "Bit 10 - Half transfer flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> Chtif3W<DmaIfcrSpec> {
        Chtif3W::new(self, 10)
    }
    #[doc = "Bit 11 - Transfer error flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> Cteif3W<DmaIfcrSpec> {
        Cteif3W::new(self, 11)
    }
    #[doc = "Bit 12 - Global interrupt flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgif4(&mut self) -> Cgif4W<DmaIfcrSpec> {
        Cgif4W::new(self, 12)
    }
    #[doc = "Bit 13 - Transfer complete flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> Ctcif4W<DmaIfcrSpec> {
        Ctcif4W::new(self, 13)
    }
    #[doc = "Bit 14 - Half transfer flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> Chtif4W<DmaIfcrSpec> {
        Chtif4W::new(self, 14)
    }
    #[doc = "Bit 15 - Transfer error flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> Cteif4W<DmaIfcrSpec> {
        Cteif4W::new(self, 15)
    }
    #[doc = "Bit 16 - Global interrupt flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn cgif5(&mut self) -> Cgif5W<DmaIfcrSpec> {
        Cgif5W::new(self, 16)
    }
    #[doc = "Bit 17 - Transfer complete flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> Ctcif5W<DmaIfcrSpec> {
        Ctcif5W::new(self, 17)
    }
    #[doc = "Bit 18 - Half transfer flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> Chtif5W<DmaIfcrSpec> {
        Chtif5W::new(self, 18)
    }
    #[doc = "Bit 19 - Transfer error flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> Cteif5W<DmaIfcrSpec> {
        Cteif5W::new(self, 19)
    }
    #[doc = "Bit 20 - Global interrupt flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn cgif6(&mut self) -> Cgif6W<DmaIfcrSpec> {
        Cgif6W::new(self, 20)
    }
    #[doc = "Bit 21 - Transfer complete flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> Ctcif6W<DmaIfcrSpec> {
        Ctcif6W::new(self, 21)
    }
    #[doc = "Bit 22 - Half transfer flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> Chtif6W<DmaIfcrSpec> {
        Chtif6W::new(self, 22)
    }
    #[doc = "Bit 23 - Transfer error flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> Cteif6W<DmaIfcrSpec> {
        Cteif6W::new(self, 23)
    }
    #[doc = "Bit 24 - Global interrupt flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn cgif7(&mut self) -> Cgif7W<DmaIfcrSpec> {
        Cgif7W::new(self, 24)
    }
    #[doc = "Bit 25 - Transfer complete flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> Ctcif7W<DmaIfcrSpec> {
        Ctcif7W::new(self, 25)
    }
    #[doc = "Bit 26 - Half transfer flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> Chtif7W<DmaIfcrSpec> {
        Chtif7W::new(self, 26)
    }
    #[doc = "Bit 27 - Transfer error flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> Cteif7W<DmaIfcrSpec> {
        Cteif7W::new(self, 27)
    }
}
#[doc = "DMA interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIfcrSpec;
impl crate::RegisterSpec for DmaIfcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_ifcr::W`](W) writer structure"]
impl crate::Writable for DmaIfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_IFCR to value 0"]
impl crate::Resettable for DmaIfcrSpec {
    const RESET_VALUE: u32 = 0;
}
