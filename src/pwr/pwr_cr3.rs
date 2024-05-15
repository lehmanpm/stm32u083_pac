#[doc = "Register `PWR_CR3` reader"]
pub type R = crate::R<PwrCr3Spec>;
#[doc = "Register `PWR_CR3` writer"]
pub type W = crate::W<PwrCr3Spec>;
#[doc = "Field `EWUP1` reader - Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
pub type Ewup1R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
pub type Ewup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP2` reader - Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
pub type Ewup2R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
pub type Ewup2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP3` reader - Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
pub type Ewup3R = crate::BitReader;
#[doc = "Field `EWUP3` writer - Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
pub type Ewup3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP4` reader - Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
pub type Ewup4R = crate::BitReader;
#[doc = "Field `EWUP4` writer - Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
pub type Ewup4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP5` reader - Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
pub type Ewup5R = crate::BitReader;
#[doc = "Field `EWUP5` writer - Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
pub type Ewup5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP7` reader - Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register."]
pub type Ewup7R = crate::BitReader;
#[doc = "Field `EWUP7` writer - Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register."]
pub type Ewup7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SRAM2 retention in Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrs {
    #[doc = "0: SRAM2 is powered off in Standby mode (SRAM2 content is lost)."]
    B0x0 = 0,
    #[doc = "1: SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 content is kept)."]
    B0x1 = 1,
}
impl From<Rrs> for bool {
    #[inline(always)]
    fn from(variant: Rrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRS` reader - SRAM2 retention in Standby mode"]
pub type RrsR = crate::BitReader<Rrs>;
impl RrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrs {
        match self.bits {
            false => Rrs::B0x0,
            true => Rrs::B0x1,
        }
    }
    #[doc = "SRAM2 is powered off in Standby mode (SRAM2 content is lost)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rrs::B0x0
    }
    #[doc = "SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 content is kept)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rrs::B0x1
    }
}
#[doc = "Field `RRS` writer - SRAM2 retention in Standby mode"]
pub type RrsW<'a, REG> = crate::BitWriter<'a, REG, Rrs>;
impl<'a, REG> RrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 is powered off in Standby mode (SRAM2 content is lost)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::B0x0)
    }
    #[doc = "SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 content is kept)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::B0x1)
    }
}
#[doc = "Field `ENULP` reader - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes."]
pub type EnulpR = crate::BitReader;
#[doc = "Field `ENULP` writer - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes."]
pub type EnulpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode."]
pub type ApcR = crate::BitReader;
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode."]
pub type ApcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable internal wake-up line\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eiwul {
    #[doc = "0: Internal wake-up line disable."]
    B0x0 = 0,
    #[doc = "1: Internal wake-up line enable."]
    B0x1 = 1,
}
impl From<Eiwul> for bool {
    #[inline(always)]
    fn from(variant: Eiwul) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIWUL` reader - Enable internal wake-up line"]
pub type EiwulR = crate::BitReader<Eiwul>;
impl EiwulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eiwul {
        match self.bits {
            false => Eiwul::B0x0,
            true => Eiwul::B0x1,
        }
    }
    #[doc = "Internal wake-up line disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eiwul::B0x0
    }
    #[doc = "Internal wake-up line enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eiwul::B0x1
    }
}
#[doc = "Field `EIWUL` writer - Enable internal wake-up line"]
pub type EiwulW<'a, REG> = crate::BitWriter<'a, REG, Eiwul>;
impl<'a, REG> EiwulW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal wake-up line disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eiwul::B0x0)
    }
    #[doc = "Internal wake-up line enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eiwul::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup1(&self) -> Ewup1R {
        Ewup1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup2(&self) -> Ewup2R {
        Ewup2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup3(&self) -> Ewup3R {
        Ewup3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup4(&self) -> Ewup4R {
        Ewup4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup5(&self) -> Ewup5R {
        Ewup5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register."]
    #[inline(always)]
    pub fn ewup7(&self) -> Ewup7R {
        Ewup7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RrsR {
        RrsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes."]
    #[inline(always)]
    pub fn enulp(&self) -> EnulpR {
        EnulpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode."]
    #[inline(always)]
    pub fn apc(&self) -> ApcR {
        ApcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable internal wake-up line"]
    #[inline(always)]
    pub fn eiwul(&self) -> EiwulR {
        EiwulR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> Ewup1W<PwrCr3Spec> {
        Ewup1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> Ewup2W<PwrCr3Spec> {
        Ewup2W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> Ewup3W<PwrCr3Spec> {
        Ewup3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> Ewup4W<PwrCr3Spec> {
        Ewup4W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> Ewup5W<PwrCr3Spec> {
        Ewup5W::new(self, 4)
    }
    #[doc = "Bit 6 - Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn ewup7(&mut self) -> Ewup7W<PwrCr3Spec> {
        Ewup7W::new(self, 6)
    }
    #[doc = "Bit 8 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RrsW<PwrCr3Spec> {
        RrsW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes."]
    #[inline(always)]
    #[must_use]
    pub fn enulp(&mut self) -> EnulpW<PwrCr3Spec> {
        EnulpW::new(self, 9)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode."]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> ApcW<PwrCr3Spec> {
        ApcW::new(self, 10)
    }
    #[doc = "Bit 15 - Enable internal wake-up line"]
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EiwulW<PwrCr3Spec> {
        EiwulW::new(self, 15)
    }
}
#[doc = "Power control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrCr3Spec;
impl crate::RegisterSpec for PwrCr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr3::R`](R) reader structure"]
impl crate::Readable for PwrCr3Spec {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr3::W`](W) writer structure"]
impl crate::Writable for PwrCr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR3 to value 0x8000"]
impl crate::Resettable for PwrCr3Spec {
    const RESET_VALUE: u32 = 0x8000;
}
