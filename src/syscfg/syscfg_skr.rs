#[doc = "Register `SYSCFG_SKR` writer"]
pub type W = crate::W<SyscfgSkrSpec>;
#[doc = "Field `KEY` writer - SRAM2 write protection key for software erase The following steps are required to unlock the write protection of the SRAM2ER bit in the SYSCFG_CFGR2 register: Write 0xCA into KEY\\[7:0\\]
Write 0x53 into KEY\\[7:0\\]
Writing a wrong key reactivates the write protection."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - SRAM2 write protection key for software erase The following steps are required to unlock the write protection of the SRAM2ER bit in the SYSCFG_CFGR2 register: Write 0xCA into KEY\\[7:0\\]
Write 0x53 into KEY\\[7:0\\]
Writing a wrong key reactivates the write protection."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<SyscfgSkrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "SYSCFG SRAM2 key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_skr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgSkrSpec;
impl crate::RegisterSpec for SyscfgSkrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`syscfg_skr::W`](W) writer structure"]
impl crate::Writable for SyscfgSkrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_SKR to value 0"]
impl crate::Resettable for SyscfgSkrSpec {
    const RESET_VALUE: u32 = 0;
}
