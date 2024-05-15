#[doc = "Register `RTC_WUTR` reader"]
pub type R = crate::R<RtcWutrSpec>;
#[doc = "Register `RTC_WUTR` writer"]
pub type W = crate::W<RtcWutrSpec>;
#[doc = "Field `WUT` reader - Wake-up auto-reload value bits When the wake-up timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]1+11) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register. When WUCKSEL\\[2\\]
= 1, the wake-up timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
= 011 (RTCCLK/2) is forbidden."]
pub type WutR = crate::FieldReader<u16>;
#[doc = "Field `WUT` writer - Wake-up auto-reload value bits When the wake-up timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]1+11) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register. When WUCKSEL\\[2\\]
= 1, the wake-up timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
= 011 (RTCCLK/2) is forbidden."]
pub type WutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WUTOCLR` reader - Wake-up auto-reload output clear value When WUTOCLR\\[15:0\\]
is different from 0x0000, WUTF is set by hardware when the auto-reload down-counter reaches 0 and is cleared by hardware when the auto-reload downcounter reaches WUTOCLR\\[15:0\\]. When WUTOCLR\\[15:0\\]
= 0x0000, WUTF is set by hardware when the WUT down-counter reaches 0 and is cleared by software."]
pub type WutoclrR = crate::FieldReader<u16>;
#[doc = "Field `WUTOCLR` writer - Wake-up auto-reload output clear value When WUTOCLR\\[15:0\\]
is different from 0x0000, WUTF is set by hardware when the auto-reload down-counter reaches 0 and is cleared by hardware when the auto-reload downcounter reaches WUTOCLR\\[15:0\\]. When WUTOCLR\\[15:0\\]
= 0x0000, WUTF is set by hardware when the WUT down-counter reaches 0 and is cleared by software."]
pub type WutoclrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Wake-up auto-reload value bits When the wake-up timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]1+11) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register. When WUCKSEL\\[2\\]
= 1, the wake-up timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
= 011 (RTCCLK/2) is forbidden."]
    #[inline(always)]
    pub fn wut(&self) -> WutR {
        WutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Wake-up auto-reload output clear value When WUTOCLR\\[15:0\\]
is different from 0x0000, WUTF is set by hardware when the auto-reload down-counter reaches 0 and is cleared by hardware when the auto-reload downcounter reaches WUTOCLR\\[15:0\\]. When WUTOCLR\\[15:0\\]
= 0x0000, WUTF is set by hardware when the WUT down-counter reaches 0 and is cleared by software."]
    #[inline(always)]
    pub fn wutoclr(&self) -> WutoclrR {
        WutoclrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wake-up auto-reload value bits When the wake-up timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]1+11) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register. When WUCKSEL\\[2\\]
= 1, the wake-up timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
= 011 (RTCCLK/2) is forbidden."]
    #[inline(always)]
    #[must_use]
    pub fn wut(&mut self) -> WutW<RtcWutrSpec> {
        WutW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Wake-up auto-reload output clear value When WUTOCLR\\[15:0\\]
is different from 0x0000, WUTF is set by hardware when the auto-reload down-counter reaches 0 and is cleared by hardware when the auto-reload downcounter reaches WUTOCLR\\[15:0\\]. When WUTOCLR\\[15:0\\]
= 0x0000, WUTF is set by hardware when the WUT down-counter reaches 0 and is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn wutoclr(&mut self) -> WutoclrW<RtcWutrSpec> {
        WutoclrW::new(self, 16)
    }
}
#[doc = "RTC wake-up timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_wutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_wutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcWutrSpec;
impl crate::RegisterSpec for RtcWutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_wutr::R`](R) reader structure"]
impl crate::Readable for RtcWutrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_wutr::W`](W) writer structure"]
impl crate::Writable for RtcWutrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_WUTR to value 0xffff"]
impl crate::Resettable for RtcWutrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
