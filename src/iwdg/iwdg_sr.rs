#[doc = "Register `IWDG_SR` reader"]
pub type R = crate::R<IwdgSrSpec>;
#[doc = "Field `PVU` reader - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the V&lt;sub>DD&lt;/sub> voltage domain (takes up to six periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset."]
pub type PvuR = crate::BitReader;
#[doc = "Field `RVU` reader - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the V&lt;sub>DD&lt;/sub> voltage domain (takes up to six periods of the IWDG kernel clock iwdg_ker_ck). The reload value can be updated only when RVU bit is reset."]
pub type RvuR = crate::BitReader;
#[doc = "Field `WVU` reader - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the V&lt;sub>DD&lt;/sub> voltage domain (takes up to one period of presc_ck and two periods of the IWDG kernel clock iwdg_ker_ck). The window value can be updated only when WVU bit is reset. This bit is generated only if generic window = 1."]
pub type WvuR = crate::BitReader;
#[doc = "Field `EWU` reader - Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\\[11:0\\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the V&lt;sub>DD&lt;/sub> voltage domain (takes up to one period of presc_ck and two periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\\[11:0\\]
and EWIE fields can be updated only when EWU bit is reset."]
pub type EwuR = crate::BitReader;
#[doc = "Watchdog enable status bit Set to 1 by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onf {
    #[doc = "0: The IWDG is not activated"]
    B0x0 = 0,
    #[doc = "1: The IWDG is activated and needs to be refreshed regularly by the application"]
    B0x1 = 1,
}
impl From<Onf> for bool {
    #[inline(always)]
    fn from(variant: Onf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONF` reader - Watchdog enable status bit Set to 1 by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'."]
pub type OnfR = crate::BitReader<Onf>;
impl OnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onf {
        match self.bits {
            false => Onf::B0x0,
            true => Onf::B0x1,
        }
    }
    #[doc = "The IWDG is not activated"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Onf::B0x0
    }
    #[doc = "The IWDG is activated and needs to be refreshed regularly by the application"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Onf::B0x1
    }
}
#[doc = "Field `EWIF` reader - Watchdog early interrupt flag This bit is set to 1 by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to 1."]
pub type EwifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the V&lt;sub>DD&lt;/sub> voltage domain (takes up to six periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset."]
    #[inline(always)]
    pub fn pvu(&self) -> PvuR {
        PvuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the V&lt;sub>DD&lt;/sub> voltage domain (takes up to six periods of the IWDG kernel clock iwdg_ker_ck). The reload value can be updated only when RVU bit is reset."]
    #[inline(always)]
    pub fn rvu(&self) -> RvuR {
        RvuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the V&lt;sub>DD&lt;/sub> voltage domain (takes up to one period of presc_ck and two periods of the IWDG kernel clock iwdg_ker_ck). The window value can be updated only when WVU bit is reset. This bit is generated only if generic window = 1."]
    #[inline(always)]
    pub fn wvu(&self) -> WvuR {
        WvuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\\[11:0\\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the V&lt;sub>DD&lt;/sub> voltage domain (takes up to one period of presc_ck and two periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\\[11:0\\]
and EWIE fields can be updated only when EWU bit is reset."]
    #[inline(always)]
    pub fn ewu(&self) -> EwuR {
        EwuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Watchdog enable status bit Set to 1 by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'."]
    #[inline(always)]
    pub fn onf(&self) -> OnfR {
        OnfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - Watchdog early interrupt flag This bit is set to 1 by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to 1."]
    #[inline(always)]
    pub fn ewif(&self) -> EwifR {
        EwifR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "IWDG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgSrSpec;
impl crate::RegisterSpec for IwdgSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_sr::R`](R) reader structure"]
impl crate::Readable for IwdgSrSpec {}
#[doc = "`reset()` method sets IWDG_SR to value 0"]
impl crate::Resettable for IwdgSrSpec {
    const RESET_VALUE: u32 = 0;
}
