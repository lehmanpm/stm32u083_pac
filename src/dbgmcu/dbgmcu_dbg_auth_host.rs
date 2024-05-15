#[doc = "Register `DBGMCU_DBG_AUTH_HOST` reader"]
pub type R = crate::R<DbgmcuDbgAuthHostSpec>;
#[doc = "Register `DBGMCU_DBG_AUTH_HOST` writer"]
pub type W = crate::W<DbgmcuDbgAuthHostSpec>;
#[doc = "Field `MESSAGE` reader - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
pub type MessageR = crate::FieldReader<u32>;
#[doc = "Field `MESSAGE` writer - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
pub type MessageW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
    #[inline(always)]
    pub fn message(&self) -> MessageR {
        MessageR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register."]
    #[inline(always)]
    #[must_use]
    pub fn message(&mut self) -> MessageW<DbgmcuDbgAuthHostSpec> {
        MessageW::new(self, 0)
    }
}
#[doc = "DBGMCU debug authentication mailbox host register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_dbg_auth_host::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgmcu_dbg_auth_host::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuDbgAuthHostSpec;
impl crate::RegisterSpec for DbgmcuDbgAuthHostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_dbg_auth_host::R`](R) reader structure"]
impl crate::Readable for DbgmcuDbgAuthHostSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgmcu_dbg_auth_host::W`](W) writer structure"]
impl crate::Writable for DbgmcuDbgAuthHostSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGMCU_DBG_AUTH_HOST to value 0"]
impl crate::Resettable for DbgmcuDbgAuthHostSpec {
    const RESET_VALUE: u32 = 0;
}
