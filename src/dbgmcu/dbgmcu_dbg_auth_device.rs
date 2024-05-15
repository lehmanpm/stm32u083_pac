#[doc = "Register `DBGMCU_DBG_AUTH_DEVICE` reader"]
pub type R = crate::R<DbgmcuDbgAuthDeviceSpec>;
#[doc = "Field `MESSAGE` reader - Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register."]
pub type MessageR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device to debug host mailbox message. During debug authentication the device communicates with the debug host via this register."]
    #[inline(always)]
    pub fn message(&self) -> MessageR {
        MessageR::new(self.bits)
    }
}
#[doc = "DBGMCU debug authentication mailbox device register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgmcu_dbg_auth_device::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgmcuDbgAuthDeviceSpec;
impl crate::RegisterSpec for DbgmcuDbgAuthDeviceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_dbg_auth_device::R`](R) reader structure"]
impl crate::Readable for DbgmcuDbgAuthDeviceSpec {}
#[doc = "`reset()` method sets DBGMCU_DBG_AUTH_DEVICE to value 0"]
impl crate::Resettable for DbgmcuDbgAuthDeviceSpec {
    const RESET_VALUE: u32 = 0;
}
