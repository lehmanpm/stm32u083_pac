#[doc = "Register `DMAMUX_CSR` reader"]
pub type R = crate::R<DmamuxCsrSpec>;
#[doc = "Field `SOF0` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof0R = crate::BitReader;
#[doc = "Field `SOF1` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof1R = crate::BitReader;
#[doc = "Field `SOF2` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof2R = crate::BitReader;
#[doc = "Field `SOF3` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof3R = crate::BitReader;
#[doc = "Field `SOF4` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof4R = crate::BitReader;
#[doc = "Field `SOF5` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof5R = crate::BitReader;
#[doc = "Field `SOF6` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof6R = crate::BitReader;
#[doc = "Field `SOF7` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof7R = crate::BitReader;
#[doc = "Field `SOF8` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof8R = crate::BitReader;
#[doc = "Field `SOF9` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof9R = crate::BitReader;
#[doc = "Field `SOF10` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof10R = crate::BitReader;
#[doc = "Field `SOF11` reader - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
pub type Sof11R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof0(&self) -> Sof0R {
        Sof0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof1(&self) -> Sof1R {
        Sof1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof2(&self) -> Sof2R {
        Sof2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof3(&self) -> Sof3R {
        Sof3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof4(&self) -> Sof4R {
        Sof4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof5(&self) -> Sof5R {
        Sof5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof6(&self) -> Sof6R {
        Sof6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof7(&self) -> Sof7R {
        Sof7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof8(&self) -> Sof8R {
        Sof8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof9(&self) -> Sof9R {
        Sof9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof10(&self) -> Sof10R {
        Sof10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization overrun event flag The flag is set when a synchronization event occurs on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ. The flag is cleared by writing 1 to the corresponding CSOFx bit in DMAMUX_CFR register."]
    #[inline(always)]
    pub fn sof11(&self) -> Sof11R {
        Sof11R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamuxCsrSpec;
impl crate::RegisterSpec for DmamuxCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_csr::R`](R) reader structure"]
impl crate::Readable for DmamuxCsrSpec {}
#[doc = "`reset()` method sets DMAMUX_CSR to value 0"]
impl crate::Resettable for DmamuxCsrSpec {
    const RESET_VALUE: u32 = 0;
}
