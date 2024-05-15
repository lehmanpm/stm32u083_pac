#[doc = "Register `LPUART_ICR` writer"]
pub type W = crate::W<LpuartIcrSpec>;
#[doc = "Field `PECF` writer - Parity error clear flag Writing 1 to this bit clears the PE flag in the LPUART_ISR register."]
pub type PecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECF` writer - Framing error clear flag Writing 1 to this bit clears the FE flag in the LPUART_ISR register."]
pub type FecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NECF` writer - Noise detected clear flag Writing 1 to this bit clears the NE flag in the LPUART_ISR register."]
pub type NecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORECF` writer - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the LPUART_ISR register."]
pub type OrecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLECF` writer - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the LPUART_ISR register."]
pub type IdlecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCF` writer - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the LPUART_ISR register."]
pub type TccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` writer - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the LPUART_ISR register."]
pub type CtscfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` writer - Character match clear flag Writing 1 to this bit clears the CMF flag in the LPUART_ISR register."]
pub type CmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCF` writer - Wake-up from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section132.3: LPUART implementation on page1914."]
pub type WucfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Parity error clear flag Writing 1 to this bit clears the PE flag in the LPUART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn pecf(&mut self) -> PecfW<LpuartIcrSpec> {
        PecfW::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error clear flag Writing 1 to this bit clears the FE flag in the LPUART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn fecf(&mut self) -> FecfW<LpuartIcrSpec> {
        FecfW::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear flag Writing 1 to this bit clears the NE flag in the LPUART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn necf(&mut self) -> NecfW<LpuartIcrSpec> {
        NecfW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the LPUART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn orecf(&mut self) -> OrecfW<LpuartIcrSpec> {
        OrecfW::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the LPUART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn idlecf(&mut self) -> IdlecfW<LpuartIcrSpec> {
        IdlecfW::new(self, 4)
    }
    #[doc = "Bit 6 - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the LPUART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn tccf(&mut self) -> TccfW<LpuartIcrSpec> {
        TccfW::new(self, 6)
    }
    #[doc = "Bit 9 - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the LPUART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CtscfW<LpuartIcrSpec> {
        CtscfW::new(self, 9)
    }
    #[doc = "Bit 17 - Character match clear flag Writing 1 to this bit clears the CMF flag in the LPUART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn cmcf(&mut self) -> CmcfW<LpuartIcrSpec> {
        CmcfW::new(self, 17)
    }
    #[doc = "Bit 20 - Wake-up from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section132.3: LPUART implementation on page1914."]
    #[inline(always)]
    #[must_use]
    pub fn wucf(&mut self) -> WucfW<LpuartIcrSpec> {
        WucfW::new(self, 20)
    }
}
#[doc = "LPUART interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpuart_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpuartIcrSpec;
impl crate::RegisterSpec for LpuartIcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lpuart_icr::W`](W) writer structure"]
impl crate::Writable for LpuartIcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPUART_ICR to value 0"]
impl crate::Resettable for LpuartIcrSpec {
    const RESET_VALUE: u32 = 0;
}
