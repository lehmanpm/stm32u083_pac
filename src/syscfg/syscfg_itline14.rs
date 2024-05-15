#[doc = "Register `SYSCFG_ITLINE14` reader"]
pub type R = crate::R<SyscfgItline14Spec>;
#[doc = "Field `TIM1_CC1` reader - Timer 1 capture compare 1 interrupt request pending"]
pub type Tim1Cc1R = crate::BitReader;
#[doc = "Field `TIM1_CC2` reader - Timer 1 capture compare 2 interrupt request pending"]
pub type Tim1Cc2R = crate::BitReader;
#[doc = "Field `TIM1_CC3` reader - Timer 1 capture compare 3 interrupt request pending"]
pub type Tim1Cc3R = crate::BitReader;
#[doc = "Field `TIM1_CC4` reader - Timer 1 capture compare 4 interrupt request pending"]
pub type Tim1Cc4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 1 capture compare 1 interrupt request pending"]
    #[inline(always)]
    pub fn tim1_cc1(&self) -> Tim1Cc1R {
        Tim1Cc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 capture compare 2 interrupt request pending"]
    #[inline(always)]
    pub fn tim1_cc2(&self) -> Tim1Cc2R {
        Tim1Cc2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 1 capture compare 3 interrupt request pending"]
    #[inline(always)]
    pub fn tim1_cc3(&self) -> Tim1Cc3R {
        Tim1Cc3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 1 capture compare 4 interrupt request pending"]
    #[inline(always)]
    pub fn tim1_cc4(&self) -> Tim1Cc4R {
        Tim1Cc4R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 14 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline14Spec;
impl crate::RegisterSpec for SyscfgItline14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline14::R`](R) reader structure"]
impl crate::Readable for SyscfgItline14Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE14 to value 0"]
impl crate::Resettable for SyscfgItline14Spec {
    const RESET_VALUE: u32 = 0;
}
