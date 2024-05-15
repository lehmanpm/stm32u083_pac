#[doc = "Register `PWR_PUCRF` reader"]
pub type R = crate::R<PwrPucrfSpec>;
#[doc = "Register `PWR_PUCRF` writer"]
pub type W = crate::W<PwrPucrfSpec>;
#[doc = "Field `PU0` reader - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu0R = crate::BitReader;
#[doc = "Field `PU0` writer - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu1R = crate::BitReader;
#[doc = "Field `PU1` writer - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU2` reader - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu2R = crate::BitReader;
#[doc = "Field `PU2` writer - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU3` reader - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu3R = crate::BitReader;
#[doc = "Field `PU3` writer - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
pub type Pu3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    pub fn pu0(&self) -> Pu0R {
        Pu0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    pub fn pu1(&self) -> Pu1R {
        Pu1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    pub fn pu2(&self) -> Pu2R {
        Pu2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    pub fn pu3(&self) -> Pu3R {
        Pu3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> Pu0W<PwrPucrfSpec> {
        Pu0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> Pu1W<PwrPucrfSpec> {
        Pu1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> Pu2W<PwrPucrfSpec> {
        Pu2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> Pu3W<PwrPucrfSpec> {
        Pu3W::new(self, 3)
    }
}
#[doc = "Power Port F pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrPucrfSpec;
impl crate::RegisterSpec for PwrPucrfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_pucrf::R`](R) reader structure"]
impl crate::Readable for PwrPucrfSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_pucrf::W`](W) writer structure"]
impl crate::Writable for PwrPucrfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_PUCRF to value 0"]
impl crate::Resettable for PwrPucrfSpec {
    const RESET_VALUE: u32 = 0;
}
