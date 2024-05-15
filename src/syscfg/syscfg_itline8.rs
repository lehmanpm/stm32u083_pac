#[doc = "Register `SYSCFG_ITLINE8` reader"]
pub type R = crate::R<SyscfgItline8Spec>;
#[doc = "Field `USB` reader - USB interrupt request pending"]
pub type UsbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB interrupt request pending"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 8 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline8Spec;
impl crate::RegisterSpec for SyscfgItline8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline8::R`](R) reader structure"]
impl crate::Readable for SyscfgItline8Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE8 to value 0"]
impl crate::Resettable for SyscfgItline8Spec {
    const RESET_VALUE: u32 = 0;
}
