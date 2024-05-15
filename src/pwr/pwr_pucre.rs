#[doc = "Register `PWR_PUCRE` reader"]
pub type R = crate::R<PwrPucreSpec>;
#[doc = "Register `PWR_PUCRE` writer"]
pub type W = crate::W<PwrPucreSpec>;
#[doc = "Field `PU3` reader - Port E pull-up bit 3 When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu3R = crate::BitReader;
#[doc = "Field `PU3` writer - Port E pull-up bit 3 When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU7` reader - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu7R = crate::BitReader;
#[doc = "Field `PU7` writer - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU8` reader - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu8R = crate::BitReader;
#[doc = "Field `PU8` writer - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU9` reader - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu9R = crate::BitReader;
#[doc = "Field `PU9` writer - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Port E pull-up bit 3 When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    pub fn pu3(&self) -> Pu3R {
        Pu3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    pub fn pu7(&self) -> Pu7R {
        Pu7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    pub fn pu8(&self) -> Pu8R {
        Pu8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    pub fn pu9(&self) -> Pu9R {
        Pu9R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Port E pull-up bit 3 When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> Pu3W<PwrPucreSpec> {
        Pu3W::new(self, 3)
    }
    #[doc = "Bit 7 - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> Pu7W<PwrPucreSpec> {
        Pu7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn pu8(&mut self) -> Pu8W<PwrPucreSpec> {
        Pu8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn pu9(&mut self) -> Pu9W<PwrPucreSpec> {
        Pu9W::new(self, 9)
    }
}
#[doc = "Power Port E pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrPucreSpec;
impl crate::RegisterSpec for PwrPucreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_pucre::R`](R) reader structure"]
impl crate::Readable for PwrPucreSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_pucre::W`](W) writer structure"]
impl crate::Writable for PwrPucreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_PUCRE to value 0"]
impl crate::Resettable for PwrPucreSpec {
    const RESET_VALUE: u32 = 0;
}
