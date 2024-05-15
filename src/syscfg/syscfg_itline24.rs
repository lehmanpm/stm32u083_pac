#[doc = "Register `SYSCFG_ITLINE24` reader"]
pub type R = crate::R<SyscfgItline24Spec>;
#[doc = "Field `I2C2` reader - I2C2 interrupt request pending"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C4` reader - I2C4 interrupt request pending"]
pub type I2c4R = crate::BitReader;
#[doc = "Field `I2C3` reader - I2C3 interrupt request pending (EXTI line 23)"]
pub type I2c3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C2 interrupt request pending"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 interrupt request pending"]
    #[inline(always)]
    pub fn i2c4(&self) -> I2c4R {
        I2c4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C3 interrupt request pending (EXTI line 23)"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2c3R {
        I2c3R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 24 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline24::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline24Spec;
impl crate::RegisterSpec for SyscfgItline24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline24::R`](R) reader structure"]
impl crate::Readable for SyscfgItline24Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE24 to value 0"]
impl crate::Resettable for SyscfgItline24Spec {
    const RESET_VALUE: u32 = 0;
}
