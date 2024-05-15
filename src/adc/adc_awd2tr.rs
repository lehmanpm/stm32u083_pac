#[doc = "Register `ADC_AWD2TR` reader"]
pub type R = crate::R<AdcAwd2trSpec>;
#[doc = "Register `ADC_AWD2TR` writer"]
pub type W = crate::W<AdcAwd2trSpec>;
#[doc = "Field `LT2` reader - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section113.8: Analog window watchdogs on page1337."]
pub type Lt2R = crate::FieldReader<u16>;
#[doc = "Field `LT2` writer - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section113.8: Analog window watchdogs on page1337."]
pub type Lt2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HT2` reader - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section113.8: Analog window watchdogs on page1337."]
pub type Ht2R = crate::FieldReader<u16>;
#[doc = "Field `HT2` writer - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section113.8: Analog window watchdogs on page1337."]
pub type Ht2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section113.8: Analog window watchdogs on page1337."]
    #[inline(always)]
    pub fn lt2(&self) -> Lt2R {
        Lt2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section113.8: Analog window watchdogs on page1337."]
    #[inline(always)]
    pub fn ht2(&self) -> Ht2R {
        Ht2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to Section113.8: Analog window watchdogs on page1337."]
    #[inline(always)]
    #[must_use]
    pub fn lt2(&mut self) -> Lt2W<AdcAwd2trSpec> {
        Lt2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to Section113.8: Analog window watchdogs on page1337."]
    #[inline(always)]
    #[must_use]
    pub fn ht2(&mut self) -> Ht2W<AdcAwd2trSpec> {
        Ht2W::new(self, 16)
    }
}
#[doc = "ADC watchdog threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_awd2tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_awd2tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcAwd2trSpec;
impl crate::RegisterSpec for AdcAwd2trSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_awd2tr::R`](R) reader structure"]
impl crate::Readable for AdcAwd2trSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_awd2tr::W`](W) writer structure"]
impl crate::Writable for AdcAwd2trSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_AWD2TR to value 0x0fff_0000"]
impl crate::Resettable for AdcAwd2trSpec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
