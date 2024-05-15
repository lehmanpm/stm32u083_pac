#[doc = "Register `SYSCFG_ITLINE12` reader"]
pub type R = crate::R<SyscfgItline12Spec>;
#[doc = "Field `ADC` reader - ADC interrupt request pending"]
pub type AdcR = crate::BitReader;
#[doc = "Field `COMP1` reader - Comparator 1 interrupt request pending (EXTI line 17)"]
pub type Comp1R = crate::BitReader;
#[doc = "Field `COMP2` reader - Comparator 2 interrupt request pending (EXTI line 18)"]
pub type Comp2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC interrupt request pending"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 interrupt request pending (EXTI line 17)"]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 interrupt request pending (EXTI line 18)"]
    #[inline(always)]
    pub fn comp2(&self) -> Comp2R {
        Comp2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 12 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline12::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline12Spec;
impl crate::RegisterSpec for SyscfgItline12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline12::R`](R) reader structure"]
impl crate::Readable for SyscfgItline12Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE12 to value 0"]
impl crate::Resettable for SyscfgItline12Spec {
    const RESET_VALUE: u32 = 0;
}
