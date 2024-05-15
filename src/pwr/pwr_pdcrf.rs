#[doc = "Register `PWR_PDCRF` reader"]
pub type R = crate::R<PwrPdcrfSpec>;
#[doc = "Register `PWR_PDCRF` writer"]
pub type W = crate::W<PwrPdcrfSpec>;
#[doc = "Field `PD0` reader - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd0R = crate::BitReader;
#[doc = "Field `PD0` writer - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd1R = crate::BitReader;
#[doc = "Field `PD1` writer - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd2R = crate::BitReader;
#[doc = "Field `PD2` writer - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd3R = crate::BitReader;
#[doc = "Field `PD3` writer - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd0(&self) -> Pd0R {
        Pd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd1(&self) -> Pd1R {
        Pd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd2(&self) -> Pd2R {
        Pd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd3(&self) -> Pd3R {
        Pd3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> Pd0W<PwrPdcrfSpec> {
        Pd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> Pd1W<PwrPdcrfSpec> {
        Pd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> Pd2W<PwrPdcrfSpec> {
        Pd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> Pd3W<PwrPdcrfSpec> {
        Pd3W::new(self, 3)
    }
}
#[doc = "Power Port F pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrPdcrfSpec;
impl crate::RegisterSpec for PwrPdcrfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_pdcrf::R`](R) reader structure"]
impl crate::Readable for PwrPdcrfSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_pdcrf::W`](W) writer structure"]
impl crate::Writable for PwrPdcrfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_PDCRF to value 0"]
impl crate::Resettable for PwrPdcrfSpec {
    const RESET_VALUE: u32 = 0;
}
