#[doc = "Register `IWDG_EWCR` reader"]
pub type R = crate::R<IwdgEwcrSpec>;
#[doc = "Register `IWDG_EWCR` writer"]
pub type W = crate::W<IwdgEwcrSpec>;
#[doc = "Field `EWIT` reader - Watchdog counter window value These bits are write access protected (see Section126.4.6). They are written by software to define at which position of the IWDCNT down-counter the early wake-up interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]1-11. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the Early wake-up comparator value and the Interrupt enable bit from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the IWDG status register (IWDG_SR) is reset."]
pub type EwitR = crate::FieldReader<u16>;
#[doc = "Field `EWIT` writer - Watchdog counter window value These bits are write access protected (see Section126.4.6). They are written by software to define at which position of the IWDCNT down-counter the early wake-up interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]1-11. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the Early wake-up comparator value and the Interrupt enable bit from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the IWDG status register (IWDG_SR) is reset."]
pub type EwitW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EWIC` writer - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wake-up interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0."]
pub type EwicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Watchdog early interrupt enable Set and reset by software. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the value of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewie {
    #[doc = "0: The early interrupt interface is disabled."]
    B0x0 = 0,
    #[doc = "1: The early interrupt interface is enabled."]
    B0x1 = 1,
}
impl From<Ewie> for bool {
    #[inline(always)]
    fn from(variant: Ewie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIE` reader - Watchdog early interrupt enable Set and reset by software. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the value of this bit."]
pub type EwieR = crate::BitReader<Ewie>;
impl EwieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewie {
        match self.bits {
            false => Ewie::B0x0,
            true => Ewie::B0x1,
        }
    }
    #[doc = "The early interrupt interface is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ewie::B0x0
    }
    #[doc = "The early interrupt interface is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ewie::B0x1
    }
}
#[doc = "Field `EWIE` writer - Watchdog early interrupt enable Set and reset by software. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the value of this bit."]
pub type EwieW<'a, REG> = crate::BitWriter<'a, REG, Ewie>;
impl<'a, REG> EwieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The early interrupt interface is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ewie::B0x0)
    }
    #[doc = "The early interrupt interface is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewie::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected (see Section126.4.6). They are written by software to define at which position of the IWDCNT down-counter the early wake-up interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]1-11. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the Early wake-up comparator value and the Interrupt enable bit from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the IWDG status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn ewit(&self) -> EwitR {
        EwitR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the value of this bit."]
    #[inline(always)]
    pub fn ewie(&self) -> EwieR {
        EwieR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected (see Section126.4.6). They are written by software to define at which position of the IWDCNT down-counter the early wake-up interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]1-11. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the Early wake-up comparator value and the Interrupt enable bit from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the IWDG status register (IWDG_SR) is reset."]
    #[inline(always)]
    #[must_use]
    pub fn ewit(&mut self) -> EwitW<IwdgEwcrSpec> {
        EwitW::new(self, 0)
    }
    #[doc = "Bit 14 - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wake-up interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0."]
    #[inline(always)]
    #[must_use]
    pub fn ewic(&mut self) -> EwicW<IwdgEwcrSpec> {
        EwicW::new(self, 14)
    }
    #[doc = "Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the value of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EwieW<IwdgEwcrSpec> {
        EwieW::new(self, 15)
    }
}
#[doc = "IWDG early wake-up interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_ewcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_ewcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgEwcrSpec;
impl crate::RegisterSpec for IwdgEwcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_ewcr::R`](R) reader structure"]
impl crate::Readable for IwdgEwcrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_ewcr::W`](W) writer structure"]
impl crate::Writable for IwdgEwcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_EWCR to value 0"]
impl crate::Resettable for IwdgEwcrSpec {
    const RESET_VALUE: u32 = 0;
}
