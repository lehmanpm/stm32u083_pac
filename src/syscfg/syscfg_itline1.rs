#[doc = "Register `SYSCFG_ITLINE1` reader"]
pub type R = crate::R<SyscfgItline1Spec>;
#[doc = "Field `PVDOUT` reader - PVD supply monitoring interrupt request pending (EXTI line 16)."]
pub type PvdoutR = crate::BitReader;
#[doc = "Field `PVMOUT1` reader - V&lt;sub>DDUSB&lt;/sub> supply monitoring interrupt request pending (EXTI line 19)"]
pub type Pvmout1R = crate::BitReader;
#[doc = "Field `PVMOUT3` reader - ADC supply monitoring interrupt request pending (EXTI line 20)"]
pub type Pvmout3R = crate::BitReader;
#[doc = "Field `PVMOUT4` reader - DAC supply monitoring interrupt request pending (EXTI line 21)"]
pub type Pvmout4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PVD supply monitoring interrupt request pending (EXTI line 16)."]
    #[inline(always)]
    pub fn pvdout(&self) -> PvdoutR {
        PvdoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - V&lt;sub>DDUSB&lt;/sub> supply monitoring interrupt request pending (EXTI line 19)"]
    #[inline(always)]
    pub fn pvmout1(&self) -> Pvmout1R {
        Pvmout1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC supply monitoring interrupt request pending (EXTI line 20)"]
    #[inline(always)]
    pub fn pvmout3(&self) -> Pvmout3R {
        Pvmout3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC supply monitoring interrupt request pending (EXTI line 21)"]
    #[inline(always)]
    pub fn pvmout4(&self) -> Pvmout4R {
        Pvmout4R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline1Spec;
impl crate::RegisterSpec for SyscfgItline1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline1::R`](R) reader structure"]
impl crate::Readable for SyscfgItline1Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE1 to value 0"]
impl crate::Resettable for SyscfgItline1Spec {
    const RESET_VALUE: u32 = 0;
}
