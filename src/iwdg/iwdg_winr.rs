#[doc = "Register `IWDG_WINR` reader"]
pub type R = crate::R<IwdgWinrSpec>;
#[doc = "Register `IWDG_WINR` writer"]
pub type W = crate::W<IwdgWinrSpec>;
#[doc = "Field `WIN` reader - Watchdog counter window value These bits are write access protected, see Section126.4.6.They contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the IWDCNT downcounter must be reloaded when its value is lower than WIN\\[11:0\\]1+11 and greater than 1. The WVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG status register (IWDG_SR) is reset."]
pub type WinR = crate::FieldReader<u16>;
#[doc = "Field `WIN` writer - Watchdog counter window value These bits are write access protected, see Section126.4.6.They contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the IWDCNT downcounter must be reloaded when its value is lower than WIN\\[11:0\\]1+11 and greater than 1. The WVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG status register (IWDG_SR) is reset."]
pub type WinW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected, see Section126.4.6.They contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the IWDCNT downcounter must be reloaded when its value is lower than WIN\\[11:0\\]1+11 and greater than 1. The WVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected, see Section126.4.6.They contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the IWDCNT downcounter must be reloaded when its value is lower than WIN\\[11:0\\]1+11 and greater than 1. The WVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the V&lt;sub>DD&lt;/sub> voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG status register (IWDG_SR) is reset."]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WinW<IwdgWinrSpec> {
        WinW::new(self, 0)
    }
}
#[doc = "IWDG window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_winr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_winr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgWinrSpec;
impl crate::RegisterSpec for IwdgWinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_winr::R`](R) reader structure"]
impl crate::Readable for IwdgWinrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_winr::W`](W) writer structure"]
impl crate::Writable for IwdgWinrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_WINR to value 0x0fff"]
impl crate::Resettable for IwdgWinrSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
