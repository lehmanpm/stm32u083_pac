#[doc = "Register `IWDG_RLR` reader"]
pub type R = crate::R<IwdgRlrSpec>;
#[doc = "Register `IWDG_RLR` writer"]
pub type W = crate::W<IwdgRlrSpec>;
#[doc = "Field `RL` reader - Watchdog counter reload value These bits are write access protected, see Section126.4.6. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG key register (IWDG_KR). The watchdog counter counts down from this value. The timeout period is a function of this value and the prescaler.clock. It is not recommended to set RL\\[11:0\\]
to a value lower than 2. The RVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the RVU bit in the IWDG status register (IWDG_SR) is reset."]
pub type RlR = crate::FieldReader<u16>;
#[doc = "Field `RL` writer - Watchdog counter reload value These bits are write access protected, see Section126.4.6. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG key register (IWDG_KR). The watchdog counter counts down from this value. The timeout period is a function of this value and the prescaler.clock. It is not recommended to set RL\\[11:0\\]
to a value lower than 2. The RVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the RVU bit in the IWDG status register (IWDG_SR) is reset."]
pub type RlW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter reload value These bits are write access protected, see Section126.4.6. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG key register (IWDG_KR). The watchdog counter counts down from this value. The timeout period is a function of this value and the prescaler.clock. It is not recommended to set RL\\[11:0\\]
to a value lower than 2. The RVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the RVU bit in the IWDG status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn rl(&self) -> RlR {
        RlR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter reload value These bits are write access protected, see Section126.4.6. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG key register (IWDG_KR). The watchdog counter counts down from this value. The timeout period is a function of this value and the prescaler.clock. It is not recommended to set RL\\[11:0\\]
to a value lower than 2. The RVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the RVU bit in the IWDG status register (IWDG_SR) is reset."]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RlW<IwdgRlrSpec> {
        RlW::new(self, 0)
    }
}
#[doc = "IWDG reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_rlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_rlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgRlrSpec;
impl crate::RegisterSpec for IwdgRlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_rlr::R`](R) reader structure"]
impl crate::Readable for IwdgRlrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_rlr::W`](W) writer structure"]
impl crate::Writable for IwdgRlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_RLR to value 0x0fff"]
impl crate::Resettable for IwdgRlrSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
