#[doc = "Register `SYSCFG_ITLINE17` reader"]
pub type R = crate::R<SyscfgItline17Spec>;
#[doc = "Field `TIM6` reader - Timer 6 interrupt request pending"]
pub type Tim6R = crate::BitReader;
#[doc = "Field `DAC` reader - DAC underrun interrupt request pending"]
pub type DacR = crate::BitReader;
#[doc = "Field `LPTIM1` reader - Low-power timer 1 interrupt request pending (EXTI line 29)"]
pub type Lptim1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 6 interrupt request pending"]
    #[inline(always)]
    pub fn tim6(&self) -> Tim6R {
        Tim6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC underrun interrupt request pending"]
    #[inline(always)]
    pub fn dac(&self) -> DacR {
        DacR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low-power timer 1 interrupt request pending (EXTI line 29)"]
    #[inline(always)]
    pub fn lptim1(&self) -> Lptim1R {
        Lptim1R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 17 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline17::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline17Spec;
impl crate::RegisterSpec for SyscfgItline17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline17::R`](R) reader structure"]
impl crate::Readable for SyscfgItline17Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE17 to value 0"]
impl crate::Resettable for SyscfgItline17Spec {
    const RESET_VALUE: u32 = 0;
}
