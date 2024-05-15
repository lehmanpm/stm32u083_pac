#[doc = "Register `DMAMUX_RG0CR` reader"]
pub type R = crate::R<DmamuxRg0crSpec>;
#[doc = "Register `DMAMUX_RG0CR` writer"]
pub type W = crate::W<DmamuxRg0crSpec>;
#[doc = "Field `SIG_ID` reader - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
pub type SigIdR = crate::FieldReader;
#[doc = "Field `SIG_ID` writer - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
pub type SigIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Trigger overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oie {
    #[doc = "0: Interrupt on a trigger overrun event occurrence is disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt on a trigger overrun event occurrence is enabled"]
    B0x1 = 1,
}
impl From<Oie> for bool {
    #[inline(always)]
    fn from(variant: Oie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIE` reader - Trigger overrun interrupt enable"]
pub type OieR = crate::BitReader<Oie>;
impl OieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oie {
        match self.bits {
            false => Oie::B0x0,
            true => Oie::B0x1,
        }
    }
    #[doc = "Interrupt on a trigger overrun event occurrence is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Oie::B0x0
    }
    #[doc = "Interrupt on a trigger overrun event occurrence is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Oie::B0x1
    }
}
#[doc = "Field `OIE` writer - Trigger overrun interrupt enable"]
pub type OieW<'a, REG> = crate::BitWriter<'a, REG, Oie>;
impl<'a, REG> OieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt on a trigger overrun event occurrence is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Oie::B0x0)
    }
    #[doc = "Interrupt on a trigger overrun event occurrence is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Oie::B0x1)
    }
}
#[doc = "DMA request generator channel x enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ge {
    #[doc = "0: DMA request generator channel x disabled"]
    B0x0 = 0,
    #[doc = "1: DMA request generator channel x enabled"]
    B0x1 = 1,
}
impl From<Ge> for bool {
    #[inline(always)]
    fn from(variant: Ge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GE` reader - DMA request generator channel x enable"]
pub type GeR = crate::BitReader<Ge>;
impl GeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ge {
        match self.bits {
            false => Ge::B0x0,
            true => Ge::B0x1,
        }
    }
    #[doc = "DMA request generator channel x disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ge::B0x0
    }
    #[doc = "DMA request generator channel x enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ge::B0x1
    }
}
#[doc = "Field `GE` writer - DMA request generator channel x enable"]
pub type GeW<'a, REG> = crate::BitWriter<'a, REG, Ge>;
impl<'a, REG> GeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request generator channel x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ge::B0x0)
    }
    #[doc = "DMA request generator channel x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ge::B0x1)
    }
}
#[doc = "DMA request generator trigger polarity Defines the edge polarity of the selected trigger input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpol {
    #[doc = "0: No event, i.e. no trigger detection nor generation."]
    B0x0 = 0,
    #[doc = "1: Rising edge"]
    B0x1 = 1,
    #[doc = "2: Falling edge"]
    B0x2 = 2,
    #[doc = "3: Rising and falling edges"]
    B0x3 = 3,
}
impl From<Gpol> for u8 {
    #[inline(always)]
    fn from(variant: Gpol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpol {
    type Ux = u8;
}
impl crate::IsEnum for Gpol {}
#[doc = "Field `GPOL` reader - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
pub type GpolR = crate::FieldReader<Gpol>;
impl GpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpol {
        match self.bits {
            0 => Gpol::B0x0,
            1 => Gpol::B0x1,
            2 => Gpol::B0x2,
            3 => Gpol::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No event, i.e. no trigger detection nor generation."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpol::B0x0
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpol::B0x1
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Gpol::B0x2
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Gpol::B0x3
    }
}
#[doc = "Field `GPOL` writer - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
pub type GpolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpol, crate::Safe>;
impl<'a, REG> GpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, i.e. no trigger detection nor generation."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpol::B0x0)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpol::B0x1)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpol::B0x2)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpol::B0x3)
    }
}
#[doc = "Field `GNBREQ` reader - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled."]
pub type GnbreqR = crate::FieldReader;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled."]
pub type GnbreqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
    #[inline(always)]
    pub fn sig_id(&self) -> SigIdR {
        SigIdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OieR {
        OieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    pub fn ge(&self) -> GeR {
        GeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GpolR {
        GpolR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GnbreqR {
        GnbreqR::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SigIdW<DmamuxRg0crSpec> {
        SigIdW::new(self, 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OieW<DmamuxRg0crSpec> {
        OieW::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GeW<DmamuxRg0crSpec> {
        GeW::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GpolW<DmamuxRg0crSpec> {
        GpolW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GnbreqW<DmamuxRg0crSpec> {
        GnbreqW::new(self, 19)
    }
}
#[doc = "DMAMUX request generator channel 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg0cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg0cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamuxRg0crSpec;
impl crate::RegisterSpec for DmamuxRg0crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_rg0cr::R`](R) reader structure"]
impl crate::Readable for DmamuxRg0crSpec {}
#[doc = "`write(|w| ..)` method takes [`dmamux_rg0cr::W`](W) writer structure"]
impl crate::Writable for DmamuxRg0crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_RG0CR to value 0"]
impl crate::Resettable for DmamuxRg0crSpec {
    const RESET_VALUE: u32 = 0;
}
