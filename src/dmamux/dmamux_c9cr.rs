#[doc = "Register `DMAMUX_C9CR` reader"]
pub type R = crate::R<DmamuxC9crSpec>;
#[doc = "Register `DMAMUX_C9CR` writer"]
pub type W = crate::W<DmamuxC9crSpec>;
#[doc = "Field `DMAREQ_ID` reader - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
pub type DmareqIdR = crate::FieldReader;
#[doc = "Field `DMAREQ_ID` writer - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
pub type DmareqIdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Synchronization overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soie {
    #[doc = "0: Interrupt disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt enabled"]
    B0x1 = 1,
}
impl From<Soie> for bool {
    #[inline(always)]
    fn from(variant: Soie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOIE` reader - Synchronization overrun interrupt enable"]
pub type SoieR = crate::BitReader<Soie>;
impl SoieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soie {
        match self.bits {
            false => Soie::B0x0,
            true => Soie::B0x1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Soie::B0x0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Soie::B0x1
    }
}
#[doc = "Field `SOIE` writer - Synchronization overrun interrupt enable"]
pub type SoieW<'a, REG> = crate::BitWriter<'a, REG, Soie>;
impl<'a, REG> SoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Soie::B0x0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Soie::B0x1)
    }
}
#[doc = "Event generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ege {
    #[doc = "0: Event generation disabled"]
    B0x0 = 0,
    #[doc = "1: Event generation enabled"]
    B0x1 = 1,
}
impl From<Ege> for bool {
    #[inline(always)]
    fn from(variant: Ege) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EGE` reader - Event generation enable"]
pub type EgeR = crate::BitReader<Ege>;
impl EgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ege {
        match self.bits {
            false => Ege::B0x0,
            true => Ege::B0x1,
        }
    }
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ege::B0x0
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ege::B0x1
    }
}
#[doc = "Field `EGE` writer - Event generation enable"]
pub type EgeW<'a, REG> = crate::BitWriter<'a, REG, Ege>;
impl<'a, REG> EgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ege::B0x0)
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ege::B0x1)
    }
}
#[doc = "Synchronization enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Se {
    #[doc = "0: Synchronization disabled"]
    B0x0 = 0,
    #[doc = "1: Synchronization enabled"]
    B0x1 = 1,
}
impl From<Se> for bool {
    #[inline(always)]
    fn from(variant: Se) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE` reader - Synchronization enable"]
pub type SeR = crate::BitReader<Se>;
impl SeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Se {
        match self.bits {
            false => Se::B0x0,
            true => Se::B0x1,
        }
    }
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Se::B0x0
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Se::B0x1
    }
}
#[doc = "Field `SE` writer - Synchronization enable"]
pub type SeW<'a, REG> = crate::BitWriter<'a, REG, Se>;
impl<'a, REG> SeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Se::B0x0)
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Se::B0x1)
    }
}
#[doc = "Synchronization polarity Defines the edge polarity of the selected synchronization input:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spol {
    #[doc = "0: No event (no synchronization, no detection)."]
    B0x0 = 0,
    #[doc = "1: Rising edge"]
    B0x1 = 1,
    #[doc = "2: Falling edge"]
    B0x2 = 2,
    #[doc = "3: Rising and falling edges"]
    B0x3 = 3,
}
impl From<Spol> for u8 {
    #[inline(always)]
    fn from(variant: Spol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spol {
    type Ux = u8;
}
impl crate::IsEnum for Spol {}
#[doc = "Field `SPOL` reader - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
pub type SpolR = crate::FieldReader<Spol>;
impl SpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spol {
        match self.bits {
            0 => Spol::B0x0,
            1 => Spol::B0x1,
            2 => Spol::B0x2,
            3 => Spol::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No event (no synchronization, no detection)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spol::B0x0
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spol::B0x1
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Spol::B0x2
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Spol::B0x3
    }
}
#[doc = "Field `SPOL` writer - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
pub type SpolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spol, crate::Safe>;
impl<'a, REG> SpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event (no synchronization, no detection)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::B0x0)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::B0x1)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::B0x2)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Spol::B0x3)
    }
}
#[doc = "Field `NBREQ` reader - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low."]
pub type NbreqR = crate::FieldReader;
#[doc = "Field `NBREQ` writer - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low."]
pub type NbreqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNC_ID` reader - Synchronization identification Selects the synchronization input (see Table137: DMAMUX: assignment of synchronization inputs to resources)."]
pub type SyncIdR = crate::FieldReader;
#[doc = "Field `SYNC_ID` writer - Synchronization identification Selects the synchronization input (see Table137: DMAMUX: assignment of synchronization inputs to resources)."]
pub type SyncIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DmareqIdR {
        DmareqIdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn soie(&self) -> SoieR {
        SoieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn ege(&self) -> EgeR {
        EgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SpolR {
        SpolR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low."]
    #[inline(always)]
    pub fn nbreq(&self) -> NbreqR {
        NbreqR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization identification Selects the synchronization input (see Table137: DMAMUX: assignment of synchronization inputs to resources)."]
    #[inline(always)]
    pub fn sync_id(&self) -> SyncIdR {
        SyncIdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DmareqIdW<DmamuxC9crSpec> {
        DmareqIdW::new(self, 0)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SoieW<DmamuxC9crSpec> {
        SoieW::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EgeW<DmamuxC9crSpec> {
        EgeW::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SeW<DmamuxC9crSpec> {
        SeW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SpolW<DmamuxC9crSpec> {
        SpolW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low."]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NbreqW<DmamuxC9crSpec> {
        NbreqW::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization identification Selects the synchronization input (see Table137: DMAMUX: assignment of synchronization inputs to resources)."]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SyncIdW<DmamuxC9crSpec> {
        SyncIdW::new(self, 24)
    }
}
#[doc = "DMAMUX request line multiplexer channel 9 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c9cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c9cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamuxC9crSpec;
impl crate::RegisterSpec for DmamuxC9crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_c9cr::R`](R) reader structure"]
impl crate::Readable for DmamuxC9crSpec {}
#[doc = "`write(|w| ..)` method takes [`dmamux_c9cr::W`](W) writer structure"]
impl crate::Writable for DmamuxC9crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_C9CR to value 0"]
impl crate::Resettable for DmamuxC9crSpec {
    const RESET_VALUE: u32 = 0;
}
