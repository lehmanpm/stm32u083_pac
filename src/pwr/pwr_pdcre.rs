#[doc = "Register `PWR_PDCRE` reader"]
pub type R = crate::R<PwrPdcreSpec>;
#[doc = "Register `PWR_PDCRE` writer"]
pub type W = crate::W<PwrPdcreSpec>;
#[doc = "Field `PD3` reader - Port E pull-down bit 3 When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd3R = crate::BitReader;
#[doc = "Field `PD3` writer - Port E pull-down bit 3 When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd7R = crate::BitReader;
#[doc = "Field `PD7` writer - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd8R = crate::BitReader;
#[doc = "Field `PD8` writer - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd9R = crate::BitReader;
#[doc = "Field `PD9` writer - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
pub type Pd9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Port E pull-down bit 3 When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd3(&self) -> Pd3R {
        Pd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd7(&self) -> Pd7R {
        Pd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd8(&self) -> Pd8R {
        Pd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd9(&self) -> Pd9R {
        Pd9R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Port E pull-down bit 3 When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> Pd3W<PwrPdcreSpec> {
        Pd3W::new(self, 3)
    }
    #[doc = "Bit 7 - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> Pd7W<PwrPdcreSpec> {
        Pd7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> Pd8W<PwrPdcreSpec> {
        Pd8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> Pd9W<PwrPdcreSpec> {
        Pd9W::new(self, 9)
    }
}
#[doc = "Power Port E pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrPdcreSpec;
impl crate::RegisterSpec for PwrPdcreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_pdcre::R`](R) reader structure"]
impl crate::Readable for PwrPdcreSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_pdcre::W`](W) writer structure"]
impl crate::Writable for PwrPdcreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_PDCRE to value 0"]
impl crate::Resettable for PwrPdcreSpec {
    const RESET_VALUE: u32 = 0;
}
