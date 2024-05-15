#[doc = "Register `SYSCFG_ITLINE21` reader"]
pub type R = crate::R<SyscfgItline21Spec>;
#[doc = "Field `TSC_MCE` reader - TSC max count error interrupt request pending"]
pub type TscMceR = crate::BitReader;
#[doc = "Field `TSC_EOA` reader - TSC end of acquisition interrupt request pending"]
pub type TscEoaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TSC max count error interrupt request pending"]
    #[inline(always)]
    pub fn tsc_mce(&self) -> TscMceR {
        TscMceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSC end of acquisition interrupt request pending"]
    #[inline(always)]
    pub fn tsc_eoa(&self) -> TscEoaR {
        TscEoaR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 21 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline21::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline21Spec;
impl crate::RegisterSpec for SyscfgItline21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline21::R`](R) reader structure"]
impl crate::Readable for SyscfgItline21Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE21 to value 0"]
impl crate::Resettable for SyscfgItline21Spec {
    const RESET_VALUE: u32 = 0;
}
