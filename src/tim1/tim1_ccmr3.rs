#[doc = "Register `TIM1_CCMR3` reader"]
pub type R = crate::R<Tim1Ccmr3Spec>;
#[doc = "Register `TIM1_CCMR3` writer"]
pub type W = crate::W<Tim1Ccmr3Spec>;
#[doc = "Field `OC5FE` reader - Output compare 5 fast enable Refer to OC1FE description."]
pub type Oc5feR = crate::BitReader;
#[doc = "Field `OC5FE` writer - Output compare 5 fast enable Refer to OC1FE description."]
pub type Oc5feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5PE` reader - Output compare 5 preload enable Refer to OC1PE description."]
pub type Oc5peR = crate::BitReader;
#[doc = "Field `OC5PE` writer - Output compare 5 preload enable Refer to OC1PE description."]
pub type Oc5peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M` reader - OC5M\\[0\\]: Output compare 5 mode Refer to OC1M description."]
pub type Oc5mR = crate::FieldReader;
#[doc = "Field `OC5M` writer - OC5M\\[0\\]: Output compare 5 mode Refer to OC1M description."]
pub type Oc5mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC5CE` reader - Output compare 5 clear enable Refer to OC1CE description."]
pub type Oc5ceR = crate::BitReader;
#[doc = "Field `OC5CE` writer - Output compare 5 clear enable Refer to OC1CE description."]
pub type Oc5ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6FE` reader - Output compare 6 fast enable Refer to OC1FE description."]
pub type Oc6feR = crate::BitReader;
#[doc = "Field `OC6FE` writer - Output compare 6 fast enable Refer to OC1FE description."]
pub type Oc6feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6PE` reader - Output compare 6 preload enable Refer to OC1PE description."]
pub type Oc6peR = crate::BitReader;
#[doc = "Field `OC6PE` writer - Output compare 6 preload enable Refer to OC1PE description."]
pub type Oc6peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M` reader - OC6M\\[0\\]: Output compare 6 mode Refer to OC1M description."]
pub type Oc6mR = crate::FieldReader;
#[doc = "Field `OC6M` writer - OC6M\\[0\\]: Output compare 6 mode Refer to OC1M description."]
pub type Oc6mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC6CE` reader - Output compare 6 clear enable Refer to OC1CE description."]
pub type Oc6ceR = crate::BitReader;
#[doc = "Field `OC6CE` writer - Output compare 6 clear enable Refer to OC1CE description."]
pub type Oc6ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M_1` reader - OC5M\\[3\\]"]
pub type Oc5m1R = crate::BitReader;
#[doc = "Field `OC5M_1` writer - OC5M\\[3\\]"]
pub type Oc5m1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M_1` reader - OC6M\\[3\\]"]
pub type Oc6m1R = crate::BitReader;
#[doc = "Field `OC6M_1` writer - OC6M\\[3\\]"]
pub type Oc6m1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Output compare 5 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc5fe(&self) -> Oc5feR {
        Oc5feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc5pe(&self) -> Oc5peR {
        Oc5peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC5M\\[0\\]: Output compare 5 mode Refer to OC1M description."]
    #[inline(always)]
    pub fn oc5m(&self) -> Oc5mR {
        Oc5mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc5ce(&self) -> Oc5ceR {
        Oc5ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable Refer to OC1FE description."]
    #[inline(always)]
    pub fn oc6fe(&self) -> Oc6feR {
        Oc6feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable Refer to OC1PE description."]
    #[inline(always)]
    pub fn oc6pe(&self) -> Oc6peR {
        Oc6peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC6M\\[0\\]: Output compare 6 mode Refer to OC1M description."]
    #[inline(always)]
    pub fn oc6m(&self) -> Oc6mR {
        Oc6mR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable Refer to OC1CE description."]
    #[inline(always)]
    pub fn oc6ce(&self) -> Oc6ceR {
        Oc6ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OC5M\\[3\\]"]
    #[inline(always)]
    pub fn oc5m_1(&self) -> Oc5m1R {
        Oc5m1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - OC6M\\[3\\]"]
    #[inline(always)]
    pub fn oc6m_1(&self) -> Oc6m1R {
        Oc6m1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Output compare 5 fast enable Refer to OC1FE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc5fe(&mut self) -> Oc5feW<Tim1Ccmr3Spec> {
        Oc5feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable Refer to OC1PE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc5pe(&mut self) -> Oc5peW<Tim1Ccmr3Spec> {
        Oc5peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC5M\\[0\\]: Output compare 5 mode Refer to OC1M description."]
    #[inline(always)]
    #[must_use]
    pub fn oc5m(&mut self) -> Oc5mW<Tim1Ccmr3Spec> {
        Oc5mW::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable Refer to OC1CE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc5ce(&mut self) -> Oc5ceW<Tim1Ccmr3Spec> {
        Oc5ceW::new(self, 7)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable Refer to OC1FE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc6fe(&mut self) -> Oc6feW<Tim1Ccmr3Spec> {
        Oc6feW::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable Refer to OC1PE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc6pe(&mut self) -> Oc6peW<Tim1Ccmr3Spec> {
        Oc6peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - OC6M\\[0\\]: Output compare 6 mode Refer to OC1M description."]
    #[inline(always)]
    #[must_use]
    pub fn oc6m(&mut self) -> Oc6mW<Tim1Ccmr3Spec> {
        Oc6mW::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable Refer to OC1CE description."]
    #[inline(always)]
    #[must_use]
    pub fn oc6ce(&mut self) -> Oc6ceW<Tim1Ccmr3Spec> {
        Oc6ceW::new(self, 15)
    }
    #[doc = "Bit 16 - OC5M\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn oc5m_1(&mut self) -> Oc5m1W<Tim1Ccmr3Spec> {
        Oc5m1W::new(self, 16)
    }
    #[doc = "Bit 24 - OC6M\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn oc6m_1(&mut self) -> Oc6m1W<Tim1Ccmr3Spec> {
        Oc6m1W::new(self, 24)
    }
}
#[doc = "TIM1 capture/compare mode register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccmr3Spec;
impl crate::RegisterSpec for Tim1Ccmr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccmr3::R`](R) reader structure"]
impl crate::Readable for Tim1Ccmr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccmr3::W`](W) writer structure"]
impl crate::Writable for Tim1Ccmr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCMR3 to value 0"]
impl crate::Resettable for Tim1Ccmr3Spec {
    const RESET_VALUE: u32 = 0;
}
