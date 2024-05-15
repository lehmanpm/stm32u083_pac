#[doc = "Register `CRC_CR` reader"]
pub type R = crate::R<CrcCrSpec>;
#[doc = "Register `CRC_CR` writer"]
pub type W = crate::W<CrcCrSpec>;
#[doc = "Field `RESET` reader - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Polynomial size These bits control the size of the polynomial.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polysize {
    #[doc = "0: 32 bit polynomial"]
    B0x0 = 0,
    #[doc = "1: 16 bit polynomial"]
    B0x1 = 1,
    #[doc = "2: 8 bit polynomial"]
    B0x2 = 2,
    #[doc = "3: 7 bit polynomial"]
    B0x3 = 3,
}
impl From<Polysize> for u8 {
    #[inline(always)]
    fn from(variant: Polysize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polysize {
    type Ux = u8;
}
impl crate::IsEnum for Polysize {}
#[doc = "Field `POLYSIZE` reader - Polynomial size These bits control the size of the polynomial."]
pub type PolysizeR = crate::FieldReader<Polysize>;
impl PolysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polysize {
        match self.bits {
            0 => Polysize::B0x0,
            1 => Polysize::B0x1,
            2 => Polysize::B0x2,
            3 => Polysize::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "32 bit polynomial"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Polysize::B0x0
    }
    #[doc = "16 bit polynomial"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Polysize::B0x1
    }
    #[doc = "8 bit polynomial"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Polysize::B0x2
    }
    #[doc = "7 bit polynomial"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Polysize::B0x3
    }
}
#[doc = "Field `POLYSIZE` writer - Polynomial size These bits control the size of the polynomial."]
pub type PolysizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Polysize, crate::Safe>;
impl<'a, REG> PolysizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 bit polynomial"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Polysize::B0x0)
    }
    #[doc = "16 bit polynomial"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Polysize::B0x1)
    }
    #[doc = "8 bit polynomial"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Polysize::B0x2)
    }
    #[doc = "7 bit polynomial"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Polysize::B0x3)
    }
}
#[doc = "Reverse input data This bitfield controls the reversal of the bit order of the input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RevIn {
    #[doc = "0: Bit order not affected (RTYPE_IN1=10 or 1)"]
    B0x0 = 0,
    #[doc = "1: Bit reversal done by byte (RTYPE_IN1=10) or half-word reversal done by word (RTYPE_IN1=11)"]
    B0x1 = 1,
    #[doc = "2: Bit reversal done by half-word (RTYPE_IN1=10) or byte reversal done by word (RTYPE_IN1=11)"]
    B0x2 = 2,
    #[doc = "3: Bit reversal done by word (RTYPE_IN1=10) or bit order is not affected (RTYPE_IN1=11)"]
    B0x3 = 3,
}
impl From<RevIn> for u8 {
    #[inline(always)]
    fn from(variant: RevIn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RevIn {
    type Ux = u8;
}
impl crate::IsEnum for RevIn {}
#[doc = "Field `REV_IN` reader - Reverse input data This bitfield controls the reversal of the bit order of the input data"]
pub type RevInR = crate::FieldReader<RevIn>;
impl RevInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RevIn {
        match self.bits {
            0 => RevIn::B0x0,
            1 => RevIn::B0x1,
            2 => RevIn::B0x2,
            3 => RevIn::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Bit order not affected (RTYPE_IN1=10 or 1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RevIn::B0x0
    }
    #[doc = "Bit reversal done by byte (RTYPE_IN1=10) or half-word reversal done by word (RTYPE_IN1=11)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RevIn::B0x1
    }
    #[doc = "Bit reversal done by half-word (RTYPE_IN1=10) or byte reversal done by word (RTYPE_IN1=11)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RevIn::B0x2
    }
    #[doc = "Bit reversal done by word (RTYPE_IN1=10) or bit order is not affected (RTYPE_IN1=11)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RevIn::B0x3
    }
}
#[doc = "Field `REV_IN` writer - Reverse input data This bitfield controls the reversal of the bit order of the input data"]
pub type RevInW<'a, REG> = crate::FieldWriter<'a, REG, 2, RevIn, crate::Safe>;
impl<'a, REG> RevInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bit order not affected (RTYPE_IN1=10 or 1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RevIn::B0x0)
    }
    #[doc = "Bit reversal done by byte (RTYPE_IN1=10) or half-word reversal done by word (RTYPE_IN1=11)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RevIn::B0x1)
    }
    #[doc = "Bit reversal done by half-word (RTYPE_IN1=10) or byte reversal done by word (RTYPE_IN1=11)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RevIn::B0x2)
    }
    #[doc = "Bit reversal done by word (RTYPE_IN1=10) or bit order is not affected (RTYPE_IN1=11)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RevIn::B0x3)
    }
}
#[doc = "Reverse output data This bitfield controls the reversal of the bit order of the output data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RevOut {
    #[doc = "0: Bit order not affected (RTYPE_OUT1=10 or 1)"]
    B0x0 = 0,
    #[doc = "1: Bit-reversed output format (RTYPE_OUT1=10) or half-word reversal done by word (RTYPE_OUT1=11)"]
    B0x1 = 1,
    #[doc = "2: Bit order not affected (RTYPE_OUT1=10) or byte reversal done by word (RTYPE_OUT1=11)"]
    B0x2 = 2,
    #[doc = "3: Bit order not affected (RTYPE_OUT1=10 or 1)"]
    B0x3 = 3,
}
impl From<RevOut> for u8 {
    #[inline(always)]
    fn from(variant: RevOut) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RevOut {
    type Ux = u8;
}
impl crate::IsEnum for RevOut {}
#[doc = "Field `REV_OUT` reader - Reverse output data This bitfield controls the reversal of the bit order of the output data."]
pub type RevOutR = crate::FieldReader<RevOut>;
impl RevOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RevOut {
        match self.bits {
            0 => RevOut::B0x0,
            1 => RevOut::B0x1,
            2 => RevOut::B0x2,
            3 => RevOut::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Bit order not affected (RTYPE_OUT1=10 or 1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RevOut::B0x0
    }
    #[doc = "Bit-reversed output format (RTYPE_OUT1=10) or half-word reversal done by word (RTYPE_OUT1=11)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RevOut::B0x1
    }
    #[doc = "Bit order not affected (RTYPE_OUT1=10) or byte reversal done by word (RTYPE_OUT1=11)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RevOut::B0x2
    }
    #[doc = "Bit order not affected (RTYPE_OUT1=10 or 1)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RevOut::B0x3
    }
}
#[doc = "Field `REV_OUT` writer - Reverse output data This bitfield controls the reversal of the bit order of the output data."]
pub type RevOutW<'a, REG> = crate::FieldWriter<'a, REG, 2, RevOut, crate::Safe>;
impl<'a, REG> RevOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bit order not affected (RTYPE_OUT1=10 or 1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RevOut::B0x0)
    }
    #[doc = "Bit-reversed output format (RTYPE_OUT1=10) or half-word reversal done by word (RTYPE_OUT1=11)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RevOut::B0x1)
    }
    #[doc = "Bit order not affected (RTYPE_OUT1=10) or byte reversal done by word (RTYPE_OUT1=11)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RevOut::B0x2)
    }
    #[doc = "Bit order not affected (RTYPE_OUT1=10 or 1)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RevOut::B0x3)
    }
}
#[doc = "Reverse type input This bit controls the reversal granularity of the input data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtypeIn {
    #[doc = "0: Bit level input"]
    B0x0 = 0,
    #[doc = "1: Byte or half-word level input"]
    B0x1 = 1,
}
impl From<RtypeIn> for bool {
    #[inline(always)]
    fn from(variant: RtypeIn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTYPE_IN` reader - Reverse type input This bit controls the reversal granularity of the input data."]
pub type RtypeInR = crate::BitReader<RtypeIn>;
impl RtypeInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtypeIn {
        match self.bits {
            false => RtypeIn::B0x0,
            true => RtypeIn::B0x1,
        }
    }
    #[doc = "Bit level input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RtypeIn::B0x0
    }
    #[doc = "Byte or half-word level input"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RtypeIn::B0x1
    }
}
#[doc = "Field `RTYPE_IN` writer - Reverse type input This bit controls the reversal granularity of the input data."]
pub type RtypeInW<'a, REG> = crate::BitWriter<'a, REG, RtypeIn>;
impl<'a, REG> RtypeInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit level input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RtypeIn::B0x0)
    }
    #[doc = "Byte or half-word level input"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RtypeIn::B0x1)
    }
}
#[doc = "Reverse type output This bit controls the reversal granularity of the output data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtypeOut {
    #[doc = "0: Bit level output"]
    B0x0 = 0,
    #[doc = "1: Byte or half-word level output"]
    B0x1 = 1,
}
impl From<RtypeOut> for bool {
    #[inline(always)]
    fn from(variant: RtypeOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTYPE_OUT` reader - Reverse type output This bit controls the reversal granularity of the output data."]
pub type RtypeOutR = crate::BitReader<RtypeOut>;
impl RtypeOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtypeOut {
        match self.bits {
            false => RtypeOut::B0x0,
            true => RtypeOut::B0x1,
        }
    }
    #[doc = "Bit level output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RtypeOut::B0x0
    }
    #[doc = "Byte or half-word level output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RtypeOut::B0x1
    }
}
#[doc = "Field `RTYPE_OUT` writer - Reverse type output This bit controls the reversal granularity of the output data."]
pub type RtypeOutW<'a, REG> = crate::BitWriter<'a, REG, RtypeOut>;
impl<'a, REG> RtypeOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit level output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RtypeOut::B0x0)
    }
    #[doc = "Byte or half-word level output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RtypeOut::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial."]
    #[inline(always)]
    pub fn polysize(&self) -> PolysizeR {
        PolysizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data This bitfield controls the reversal of the bit order of the input data"]
    #[inline(always)]
    pub fn rev_in(&self) -> RevInR {
        RevInR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - Reverse output data This bitfield controls the reversal of the bit order of the output data."]
    #[inline(always)]
    pub fn rev_out(&self) -> RevOutR {
        RevOutR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Reverse type input This bit controls the reversal granularity of the input data."]
    #[inline(always)]
    pub fn rtype_in(&self) -> RtypeInR {
        RtypeInR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reverse type output This bit controls the reversal granularity of the output data."]
    #[inline(always)]
    pub fn rtype_out(&self) -> RtypeOutR {
        RtypeOutR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CrcCrSpec> {
        ResetW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial."]
    #[inline(always)]
    #[must_use]
    pub fn polysize(&mut self) -> PolysizeW<CrcCrSpec> {
        PolysizeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Reverse input data This bitfield controls the reversal of the bit order of the input data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> RevInW<CrcCrSpec> {
        RevInW::new(self, 5)
    }
    #[doc = "Bits 7:8 - Reverse output data This bitfield controls the reversal of the bit order of the output data."]
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> RevOutW<CrcCrSpec> {
        RevOutW::new(self, 7)
    }
    #[doc = "Bit 9 - Reverse type input This bit controls the reversal granularity of the input data."]
    #[inline(always)]
    #[must_use]
    pub fn rtype_in(&mut self) -> RtypeInW<CrcCrSpec> {
        RtypeInW::new(self, 9)
    }
    #[doc = "Bit 10 - Reverse type output This bit controls the reversal granularity of the output data."]
    #[inline(always)]
    #[must_use]
    pub fn rtype_out(&mut self) -> RtypeOutW<CrcCrSpec> {
        RtypeOutW::new(self, 10)
    }
}
#[doc = "CRC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCrSpec;
impl crate::RegisterSpec for CrcCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_cr::R`](R) reader structure"]
impl crate::Readable for CrcCrSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_cr::W`](W) writer structure"]
impl crate::Writable for CrcCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_CR to value 0"]
impl crate::Resettable for CrcCrSpec {
    const RESET_VALUE: u32 = 0;
}
