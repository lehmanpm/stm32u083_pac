#[doc = "Register `SYSCFG_ITLINE23` reader"]
pub type R = crate::R<SyscfgItline23Spec>;
#[doc = "Field `I2C1` reader - I2C1 interrupt request pending (EXTI line 33)"]
pub type I2c1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C1 interrupt request pending (EXTI line 33)"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 23 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline23::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline23Spec;
impl crate::RegisterSpec for SyscfgItline23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline23::R`](R) reader structure"]
impl crate::Readable for SyscfgItline23Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE23 to value 0"]
impl crate::Resettable for SyscfgItline23Spec {
    const RESET_VALUE: u32 = 0;
}
