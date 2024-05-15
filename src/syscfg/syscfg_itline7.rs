#[doc = "Register `SYSCFG_ITLINE7` reader"]
pub type R = crate::R<SyscfgItline7Spec>;
#[doc = "Field `EXTI4` reader - EXTI line 4 interrupt request pending"]
pub type Exti4R = crate::BitReader;
#[doc = "Field `EXTI5` reader - EXTI line 5 interrupt request pending"]
pub type Exti5R = crate::BitReader;
#[doc = "Field `EXTI6` reader - EXTI line 6 interrupt request pending"]
pub type Exti6R = crate::BitReader;
#[doc = "Field `EXTI7` reader - EXTI line 7 interrupt request pending"]
pub type Exti7R = crate::BitReader;
#[doc = "Field `EXTI8` reader - EXTI line 8 interrupt request pending"]
pub type Exti8R = crate::BitReader;
#[doc = "Field `EXTI9` reader - EXTI line 9 interrupt request pending"]
pub type Exti9R = crate::BitReader;
#[doc = "Field `EXTI10` reader - EXTI line 10 interrupt request pending"]
pub type Exti10R = crate::BitReader;
#[doc = "Field `EXTI11` reader - EXTI line 11 interrupt request pending"]
pub type Exti11R = crate::BitReader;
#[doc = "Field `EXTI12` reader - EXTI line 12 interrupt request pending"]
pub type Exti12R = crate::BitReader;
#[doc = "Field `EXTI13` reader - EXTI line 13 interrupt request pending"]
pub type Exti13R = crate::BitReader;
#[doc = "Field `EXTI14` reader - EXTI line 14 interrupt request pending"]
pub type Exti14R = crate::BitReader;
#[doc = "Field `EXTI15` reader - EXTI line 15 interrupt request pending"]
pub type Exti15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EXTI line 4 interrupt request pending"]
    #[inline(always)]
    pub fn exti4(&self) -> Exti4R {
        Exti4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI line 5 interrupt request pending"]
    #[inline(always)]
    pub fn exti5(&self) -> Exti5R {
        Exti5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI line 6 interrupt request pending"]
    #[inline(always)]
    pub fn exti6(&self) -> Exti6R {
        Exti6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI line 7 interrupt request pending"]
    #[inline(always)]
    pub fn exti7(&self) -> Exti7R {
        Exti7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI line 8 interrupt request pending"]
    #[inline(always)]
    pub fn exti8(&self) -> Exti8R {
        Exti8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI line 9 interrupt request pending"]
    #[inline(always)]
    pub fn exti9(&self) -> Exti9R {
        Exti9R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI line 10 interrupt request pending"]
    #[inline(always)]
    pub fn exti10(&self) -> Exti10R {
        Exti10R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI line 11 interrupt request pending"]
    #[inline(always)]
    pub fn exti11(&self) -> Exti11R {
        Exti11R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI line 12 interrupt request pending"]
    #[inline(always)]
    pub fn exti12(&self) -> Exti12R {
        Exti12R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI line 13 interrupt request pending"]
    #[inline(always)]
    pub fn exti13(&self) -> Exti13R {
        Exti13R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI line 14 interrupt request pending"]
    #[inline(always)]
    pub fn exti14(&self) -> Exti14R {
        Exti14R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI line 15 interrupt request pending"]
    #[inline(always)]
    pub fn exti15(&self) -> Exti15R {
        Exti15R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 7 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgItline7Spec;
impl crate::RegisterSpec for SyscfgItline7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline7::R`](R) reader structure"]
impl crate::Readable for SyscfgItline7Spec {}
#[doc = "`reset()` method sets SYSCFG_ITLINE7 to value 0"]
impl crate::Resettable for SyscfgItline7Spec {
    const RESET_VALUE: u32 = 0;
}
