#[doc = "Register `SYSCFG_ITLINE13` reader"]
pub type R = crate::R<SyscfgItline13Spec>;
#[doc = "Field `TIM1_CCU` reader - Timer 1 commutation interrupt request pending"]
pub type Tim1CcuR = crate::BitReader;
#[doc = "Field `TIM1_TRG` reader - Timer 1 trigger interrupt request pending"]
pub type Tim1TrgR = crate::BitReader;
#[doc = "Field `TIM1_UPD` reader - Timer 1 update interrupt request pending"]
pub type Tim1UpdR = crate::BitReader;
#[doc = "Field `TIM1_BRK` reader - Timer 1 break interrupt request pending"]
pub type Tim1BrkR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 1 commutation interrupt request pending"]
    #[inline(always)]
    pub fn tim1_ccu(&self) -> Tim1CcuR {
        Tim1CcuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 trigger interrupt request pending"]
    #[inline(always)]
    pub fn tim1_trg(&self) -> Tim1TrgR {
        Tim1TrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 1 update interrupt request pending"]
    #[inline(always)]
    pub fn tim1_upd(&self) -> Tim1UpdR {
        Tim1UpdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 1 break interrupt request pending"]
    #[inline(always)]
    pub fn tim1_brk(&self) -> Tim1BrkR {
        Tim1BrkR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 13 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline13Spec;
impl crate::RegisterSpec for SyscfgItline13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline13::R`](R) reader structure"]
impl crate::Readable for SyscfgItline13Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE13 to value 0"]
impl crate::Resettable for SyscfgItline13Spec {
    const RESET_VALUE: u32 = 0;
}
