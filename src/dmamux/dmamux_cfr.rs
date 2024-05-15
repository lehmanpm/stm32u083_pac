#[doc = "Register `DMAMUX_CFR` writer"]
pub type W = crate::W<DmamuxCfrSpec>;
#[doc = "Field `CSOF0` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF1` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF2` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF3` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF4` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF5` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF6` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF7` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF8` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF9` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF10` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF11` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type Csof11W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> Csof0W<DmamuxCfrSpec> {
        Csof0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> Csof1W<DmamuxCfrSpec> {
        Csof1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> Csof2W<DmamuxCfrSpec> {
        Csof2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> Csof3W<DmamuxCfrSpec> {
        Csof3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> Csof4W<DmamuxCfrSpec> {
        Csof4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> Csof5W<DmamuxCfrSpec> {
        Csof5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> Csof6W<DmamuxCfrSpec> {
        Csof6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof7(&mut self) -> Csof7W<DmamuxCfrSpec> {
        Csof7W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof8(&mut self) -> Csof8W<DmamuxCfrSpec> {
        Csof8W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof9(&mut self) -> Csof9W<DmamuxCfrSpec> {
        Csof9W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof10(&mut self) -> Csof10W<DmamuxCfrSpec> {
        Csof10W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof11(&mut self) -> Csof11W<DmamuxCfrSpec> {
        Csof11W::new(self, 11)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamuxCfrSpec;
impl crate::RegisterSpec for DmamuxCfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmamux_cfr::W`](W) writer structure"]
impl crate::Writable for DmamuxCfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_CFR to value 0"]
impl crate::Resettable for DmamuxCfrSpec {
    const RESET_VALUE: u32 = 0;
}
