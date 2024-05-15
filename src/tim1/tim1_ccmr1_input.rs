#[doc = "Register `TIM1_CCMR1_INPUT` reader"]
pub type R = crate::R<Tim1Ccmr1InputSpec>;
#[doc = "Register `TIM1_CCMR1_INPUT` writer"]
pub type W = crate::W<Tim1Ccmr1InputSpec>;
#[doc = "Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc1s {
    #[doc = "0: CC1 channel is configured as output"]
    B0x0 = 0,
    #[doc = "1: CC1 channel is configured as input, IC1 is mapped on TI1"]
    B0x1 = 1,
    #[doc = "2: CC1 channel is configured as input, IC1 is mapped on TI2"]
    B0x2 = 2,
    #[doc = "3: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B0x3 = 3,
}
impl From<Cc1s> for u8 {
    #[inline(always)]
    fn from(variant: Cc1s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc1s {
    type Ux = u8;
}
impl crate::IsEnum for Cc1s {}
#[doc = "Field `CC1S` reader - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type Cc1sR = crate::FieldReader<Cc1s>;
impl Cc1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1s {
        match self.bits {
            0 => Cc1s::B0x0,
            1 => Cc1s::B0x1,
            2 => Cc1s::B0x2,
            3 => Cc1s::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC1 channel is configured as output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1s::B0x0
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1s::B0x1
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc1s::B0x2
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc1s::B0x3
    }
}
#[doc = "Field `CC1S` writer - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc1s, crate::Safe>;
impl<'a, REG> Cc1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC1 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x0)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x1)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x2)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1s::B0x3)
    }
}
#[doc = "Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic1psc {
    #[doc = "0: no prescaler, capture is done each time an edge is detected on the capture input"]
    B0x0 = 0,
    #[doc = "1: capture is done once every 2 events"]
    B0x1 = 1,
    #[doc = "2: capture is done once every 4 events"]
    B0x2 = 2,
    #[doc = "3: capture is done once every 8 events"]
    B0x3 = 3,
}
impl From<Ic1psc> for u8 {
    #[inline(always)]
    fn from(variant: Ic1psc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic1psc {
    type Ux = u8;
}
impl crate::IsEnum for Ic1psc {}
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
pub type Ic1pscR = crate::FieldReader<Ic1psc>;
impl Ic1pscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic1psc {
        match self.bits {
            0 => Ic1psc::B0x0,
            1 => Ic1psc::B0x1,
            2 => Ic1psc::B0x2,
            3 => Ic1psc::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic1psc::B0x0
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic1psc::B0x1
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic1psc::B0x2
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic1psc::B0x3
    }
}
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
pub type Ic1pscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic1psc, crate::Safe>;
impl<'a, REG> Ic1pscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1psc::B0x0)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1psc::B0x1)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1psc::B0x2)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1psc::B0x3)
    }
}
#[doc = "Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic1f {
    #[doc = "0: No filter, sampling is done at f&lt;sub>DTS&lt;/sub>"]
    B0x0 = 0,
    #[doc = "1: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=2"]
    B0x1 = 1,
    #[doc = "2: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=4"]
    B0x2 = 2,
    #[doc = "3: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=8"]
    B0x3 = 3,
    #[doc = "4: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=6"]
    B0x4 = 4,
    #[doc = "5: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=8"]
    B0x5 = 5,
    #[doc = "6: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=6"]
    B0x6 = 6,
    #[doc = "7: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=8"]
    B0x7 = 7,
    #[doc = "8: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=6"]
    B0x8 = 8,
    #[doc = "9: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=8"]
    B0x9 = 9,
    #[doc = "10: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=5"]
    B0xA = 10,
    #[doc = "11: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=6"]
    B0xB = 11,
    #[doc = "12: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=8"]
    B0xC = 12,
    #[doc = "13: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=5"]
    B0xD = 13,
    #[doc = "14: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=6"]
    B0xE = 14,
    #[doc = "15: f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=8"]
    B0xF = 15,
}
impl From<Ic1f> for u8 {
    #[inline(always)]
    fn from(variant: Ic1f) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic1f {
    type Ux = u8;
}
impl crate::IsEnum for Ic1f {}
#[doc = "Field `IC1F` reader - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type Ic1fR = crate::FieldReader<Ic1f>;
impl Ic1fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic1f {
        match self.bits {
            0 => Ic1f::B0x0,
            1 => Ic1f::B0x1,
            2 => Ic1f::B0x2,
            3 => Ic1f::B0x3,
            4 => Ic1f::B0x4,
            5 => Ic1f::B0x5,
            6 => Ic1f::B0x6,
            7 => Ic1f::B0x7,
            8 => Ic1f::B0x8,
            9 => Ic1f::B0x9,
            10 => Ic1f::B0xA,
            11 => Ic1f::B0xB,
            12 => Ic1f::B0xC,
            13 => Ic1f::B0xD,
            14 => Ic1f::B0xE,
            15 => Ic1f::B0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at f&lt;sub>DTS&lt;/sub>"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic1f::B0x0
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic1f::B0x1
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic1f::B0x2
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic1f::B0x3
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=6"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Ic1f::B0x4
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=8"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Ic1f::B0x5
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Ic1f::B0x6
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=8"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Ic1f::B0x7
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=6"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Ic1f::B0x8
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=8"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Ic1f::B0x9
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=5"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Ic1f::B0xA
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=6"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Ic1f::B0xB
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=8"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Ic1f::B0xC
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=5"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Ic1f::B0xD
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=6"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Ic1f::B0xE
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=8"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Ic1f::B0xF
    }
}
#[doc = "Field `IC1F` writer - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type Ic1fW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ic1f, crate::Safe>;
impl<'a, REG> Ic1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at f&lt;sub>DTS&lt;/sub>"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x0)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x1)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x2)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>CK_INT&lt;/sub>, N=8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x3)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=6"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x4)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/2, N=8"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x5)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x6)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/4, N=8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x7)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=6"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x8)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/8, N=8"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x9)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=5"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0xA)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=6"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0xB)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/16, N=8"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0xC)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=5"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0xD)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=6"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0xE)
    }
    #[doc = "f&lt;sub>SAMPLING&lt;/sub>=f&lt;sub>DTS&lt;/sub>/32, N=8"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0xF)
    }
}
#[doc = "Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc2s {
    #[doc = "0: CC2 channel is configured as output"]
    B0x0 = 0,
    #[doc = "1: CC2 channel is configured as input, IC2 is mapped on TI2"]
    B0x1 = 1,
    #[doc = "2: CC2 channel is configured as input, IC2 is mapped on TI1"]
    B0x2 = 2,
    #[doc = "3: CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B0x3 = 3,
}
impl From<Cc2s> for u8 {
    #[inline(always)]
    fn from(variant: Cc2s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc2s {
    type Ux = u8;
}
impl crate::IsEnum for Cc2s {}
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type Cc2sR = crate::FieldReader<Cc2s>;
impl Cc2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2s {
        match self.bits {
            0 => Cc2s::B0x0,
            1 => Cc2s::B0x1,
            2 => Cc2s::B0x2,
            3 => Cc2s::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC2 channel is configured as output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc2s::B0x0
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc2s::B0x1
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc2s::B0x2
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc2s::B0x3
    }
}
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type Cc2sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc2s, crate::Safe>;
impl<'a, REG> Cc2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC2 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::B0x0)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::B0x1)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::B0x2)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2s::B0x3)
    }
}
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type Ic2pscR = crate::FieldReader;
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type Ic2pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2F` reader - Input capture 2 filter Refer to IC1F\\[3:0\\]
description."]
pub type Ic2fR = crate::FieldReader;
#[doc = "Field `IC2F` writer - Input capture 2 filter Refer to IC1F\\[3:0\\]
description."]
pub type Ic2fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
    #[inline(always)]
    pub fn ic1psc(&self) -> Ic1pscR {
        Ic1pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn ic1f(&self) -> Ic1fR {
        Ic1fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc2s(&self) -> Cc2sR {
        Cc2sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic2psc(&self) -> Ic2pscR {
        Ic2pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic2f(&self) -> Ic2fR {
        Ic2fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> Cc1sW<Tim1Ccmr1InputSpec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> Ic1pscW<Tim1Ccmr1InputSpec> {
        Ic1pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> Ic1fW<Tim1Ccmr1InputSpec> {
        Ic1fW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> Cc2sW<Tim1Ccmr1InputSpec> {
        Cc2sW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> Ic2pscW<Tim1Ccmr1InputSpec> {
        Ic2pscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> Ic2fW<Tim1Ccmr1InputSpec> {
        Ic2fW::new(self, 12)
    }
}
#[doc = "TIM1 capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr1_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr1_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccmr1InputSpec;
impl crate::RegisterSpec for Tim1Ccmr1InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccmr1_input::R`](R) reader structure"]
impl crate::Readable for Tim1Ccmr1InputSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccmr1_input::W`](W) writer structure"]
impl crate::Writable for Tim1Ccmr1InputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCMR1_INPUT to value 0"]
impl crate::Resettable for Tim1Ccmr1InputSpec {
    const RESET_VALUE: u32 = 0;
}
