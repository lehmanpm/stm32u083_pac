#[doc = "Register `LPTIM1_CCMR1` reader"]
pub type R = crate::R<Lptim1Ccmr1Spec>;
#[doc = "Register `LPTIM1_CCMR1` writer"]
pub type W = crate::W<Lptim1Ccmr1Spec>;
#[doc = "Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1sel {
    #[doc = "0: CC1 channel is configured in output PWM mode"]
    B0x0 = 0,
    #[doc = "1: CC1 channel is configured in input capture mode"]
    B0x1 = 1,
}
impl From<Cc1sel> for bool {
    #[inline(always)]
    fn from(variant: Cc1sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1SEL` reader - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
pub type Cc1selR = crate::BitReader<Cc1sel>;
impl Cc1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1sel {
        match self.bits {
            false => Cc1sel::B0x0,
            true => Cc1sel::B0x1,
        }
    }
    #[doc = "CC1 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1sel::B0x0
    }
    #[doc = "CC1 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1sel::B0x1
    }
}
#[doc = "Field `CC1SEL` writer - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
pub type Cc1selW<'a, REG> = crate::BitWriter<'a, REG, Cc1sel>;
impl<'a, REG> Cc1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC1 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1sel::B0x0)
    }
    #[doc = "CC1 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1sel::B0x1)
    }
}
#[doc = "Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM1_CCR1) or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1e {
    #[doc = "0: If CC1 is an Output, Off - OC1 is not active. Writing '0' to the CC1E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled. If CC1 is an Input, Capture disabled. Writing '0' to the CC1E bit resets the associated ic1_dma_req signal."]
    B0x0Cc1 = 0,
    #[doc = "1: If CC1 is an Output, On - OC1 signal is output on the corresponding output pin. If CC1 is an Input, Capture enabled."]
    B0x1Cc1 = 1,
}
impl From<Cc1e> for bool {
    #[inline(always)]
    fn from(variant: Cc1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1E` reader - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM1_CCR1) or not."]
pub type Cc1eR = crate::BitReader<Cc1e>;
impl Cc1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1e {
        match self.bits {
            false => Cc1e::B0x0Cc1,
            true => Cc1e::B0x1Cc1,
        }
    }
    #[doc = "If CC1 is an Output, Off - OC1 is not active. Writing '0' to the CC1E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled. If CC1 is an Input, Capture disabled. Writing '0' to the CC1E bit resets the associated ic1_dma_req signal."]
    #[inline(always)]
    pub fn is_b_0x0_cc1(&self) -> bool {
        *self == Cc1e::B0x0Cc1
    }
    #[doc = "If CC1 is an Output, On - OC1 signal is output on the corresponding output pin. If CC1 is an Input, Capture enabled."]
    #[inline(always)]
    pub fn is_b_0x1_cc1(&self) -> bool {
        *self == Cc1e::B0x1Cc1
    }
}
#[doc = "Field `CC1E` writer - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM1_CCR1) or not."]
pub type Cc1eW<'a, REG> = crate::BitWriter<'a, REG, Cc1e>;
impl<'a, REG> Cc1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If CC1 is an Output, Off - OC1 is not active. Writing '0' to the CC1E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled. If CC1 is an Input, Capture disabled. Writing '0' to the CC1E bit resets the associated ic1_dma_req signal."]
    #[inline(always)]
    pub fn b_0x0_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1e::B0x0Cc1)
    }
    #[doc = "If CC1 is an Output, On - OC1 signal is output on the corresponding output pin. If CC1 is an Input, Capture enabled."]
    #[inline(always)]
    pub fn b_0x1_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1e::B0x1Cc1)
    }
}
#[doc = "Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc1p {
    #[doc = "0: If CC1 is an Output, OC1 active high, the LPTIM output reflects the compare results between LPTIM1_ARR and LPTIM1_CCRx registers. If CC1 is an Input, rising edge, circuit is sensitive to IC1 rising edge."]
    B0x0Cc1 = 0,
    #[doc = "1: If CC1 is an Output, OC1 active low, the LPTIM output reflects the inverse of the compare results between LPTIM1_ARR and LPTIM1_CCRx registers. If CC1 is an Input, falling edge, circuit is sensitive to IC1 falling edge."]
    B0x1Cc1 = 1,
    #[doc = "2: reserved, do not use this configuration."]
    B0x2Cc1 = 2,
    #[doc = "3: If CC1 is an Input, both edges, circuit is sensitive to both IC1 rising and falling edges."]
    B0x3Cc1 = 3,
}
impl From<Cc1p> for u8 {
    #[inline(always)]
    fn from(variant: Cc1p) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc1p {
    type Ux = u8;
}
impl crate::IsEnum for Cc1p {}
#[doc = "Field `CC1P` reader - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
pub type Cc1pR = crate::FieldReader<Cc1p>;
impl Cc1pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1p {
        match self.bits {
            0 => Cc1p::B0x0Cc1,
            1 => Cc1p::B0x1Cc1,
            2 => Cc1p::B0x2Cc1,
            3 => Cc1p::B0x3Cc1,
            _ => unreachable!(),
        }
    }
    #[doc = "If CC1 is an Output, OC1 active high, the LPTIM output reflects the compare results between LPTIM1_ARR and LPTIM1_CCRx registers. If CC1 is an Input, rising edge, circuit is sensitive to IC1 rising edge."]
    #[inline(always)]
    pub fn is_b_0x0_cc1(&self) -> bool {
        *self == Cc1p::B0x0Cc1
    }
    #[doc = "If CC1 is an Output, OC1 active low, the LPTIM output reflects the inverse of the compare results between LPTIM1_ARR and LPTIM1_CCRx registers. If CC1 is an Input, falling edge, circuit is sensitive to IC1 falling edge."]
    #[inline(always)]
    pub fn is_b_0x1_cc1(&self) -> bool {
        *self == Cc1p::B0x1Cc1
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn is_b_0x2_cc1(&self) -> bool {
        *self == Cc1p::B0x2Cc1
    }
    #[doc = "If CC1 is an Input, both edges, circuit is sensitive to both IC1 rising and falling edges."]
    #[inline(always)]
    pub fn is_b_0x3_cc1(&self) -> bool {
        *self == Cc1p::B0x3Cc1
    }
}
#[doc = "Field `CC1P` writer - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
pub type Cc1pW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc1p, crate::Safe>;
impl<'a, REG> Cc1pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If CC1 is an Output, OC1 active high, the LPTIM output reflects the compare results between LPTIM1_ARR and LPTIM1_CCRx registers. If CC1 is an Input, rising edge, circuit is sensitive to IC1 rising edge."]
    #[inline(always)]
    pub fn b_0x0_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1p::B0x0Cc1)
    }
    #[doc = "If CC1 is an Output, OC1 active low, the LPTIM output reflects the inverse of the compare results between LPTIM1_ARR and LPTIM1_CCRx registers. If CC1 is an Input, falling edge, circuit is sensitive to IC1 falling edge."]
    #[inline(always)]
    pub fn b_0x1_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1p::B0x1Cc1)
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn b_0x2_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1p::B0x2Cc1)
    }
    #[doc = "If CC1 is an Input, both edges, circuit is sensitive to both IC1 rising and falling edges."]
    #[inline(always)]
    pub fn b_0x3_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1p::B0x3Cc1)
    }
}
#[doc = "Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1).\n\nValue on reset: 0"]
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
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
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
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
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
#[doc = "Input capture 1 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic1f {
    #[doc = "0: any external input capture signal level change is considered as a valid transition"]
    B0x0 = 0,
    #[doc = "1: external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    B0x1 = 1,
    #[doc = "2: external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    B0x2 = 2,
    #[doc = "3: external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    B0x3 = 3,
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
#[doc = "Field `IC1F` reader - Input capture 1 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
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
            _ => unreachable!(),
        }
    }
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic1f::B0x0
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic1f::B0x1
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic1f::B0x2
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic1f::B0x3
    }
}
#[doc = "Field `IC1F` writer - Input capture 1 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type Ic1fW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic1f, crate::Safe>;
impl<'a, REG> Ic1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x0)
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x1)
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x2)
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1f::B0x3)
    }
}
#[doc = "Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc2sel {
    #[doc = "0: CC2 channel is configured in output PWM mode"]
    B0x0 = 0,
    #[doc = "1: CC2 channel is configured in input capture mode"]
    B0x1 = 1,
}
impl From<Cc2sel> for bool {
    #[inline(always)]
    fn from(variant: Cc2sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2SEL` reader - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
pub type Cc2selR = crate::BitReader<Cc2sel>;
impl Cc2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2sel {
        match self.bits {
            false => Cc2sel::B0x0,
            true => Cc2sel::B0x1,
        }
    }
    #[doc = "CC2 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc2sel::B0x0
    }
    #[doc = "CC2 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc2sel::B0x1
    }
}
#[doc = "Field `CC2SEL` writer - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
pub type Cc2selW<'a, REG> = crate::BitWriter<'a, REG, Cc2sel>;
impl<'a, REG> Cc2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC2 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2sel::B0x0)
    }
    #[doc = "CC2 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2sel::B0x1)
    }
}
#[doc = "Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM1_CCR2) or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc2e {
    #[doc = "0: If CC2 is an Output, Off - OC2 is not active. Writing '0' to the CC2E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled. If CC2 is an Input, Capture disabled. Writing '0' to the CC2E bit resets the associated ic2_dma_req signal."]
    B0x0Cc2 = 0,
    #[doc = "1: If CC2 is an Output, On - OC2 signal is output on the corresponding output pin. If CC2 is an Input, Capture enabled."]
    B0x1Cc2 = 1,
}
impl From<Cc2e> for bool {
    #[inline(always)]
    fn from(variant: Cc2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2E` reader - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM1_CCR2) or not."]
pub type Cc2eR = crate::BitReader<Cc2e>;
impl Cc2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2e {
        match self.bits {
            false => Cc2e::B0x0Cc2,
            true => Cc2e::B0x1Cc2,
        }
    }
    #[doc = "If CC2 is an Output, Off - OC2 is not active. Writing '0' to the CC2E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled. If CC2 is an Input, Capture disabled. Writing '0' to the CC2E bit resets the associated ic2_dma_req signal."]
    #[inline(always)]
    pub fn is_b_0x0_cc2(&self) -> bool {
        *self == Cc2e::B0x0Cc2
    }
    #[doc = "If CC2 is an Output, On - OC2 signal is output on the corresponding output pin. If CC2 is an Input, Capture enabled."]
    #[inline(always)]
    pub fn is_b_0x1_cc2(&self) -> bool {
        *self == Cc2e::B0x1Cc2
    }
}
#[doc = "Field `CC2E` writer - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM1_CCR2) or not."]
pub type Cc2eW<'a, REG> = crate::BitWriter<'a, REG, Cc2e>;
impl<'a, REG> Cc2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If CC2 is an Output, Off - OC2 is not active. Writing '0' to the CC2E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled. If CC2 is an Input, Capture disabled. Writing '0' to the CC2E bit resets the associated ic2_dma_req signal."]
    #[inline(always)]
    pub fn b_0x0_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2e::B0x0Cc2)
    }
    #[doc = "If CC2 is an Output, On - OC2 signal is output on the corresponding output pin. If CC2 is an Input, Capture enabled."]
    #[inline(always)]
    pub fn b_0x1_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2e::B0x1Cc2)
    }
}
#[doc = "Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc2p {
    #[doc = "0: If CC2 is an Output, OC2 active high. If CC2 is an Input, rising edge, circuit is sensitive to IC2 rising edge."]
    B0x0Cc2 = 0,
    #[doc = "1: If CC2 is an Output, OC2 active low. If CC2 is an Input, falling edge, circuit is sensitive to IC2 falling edge."]
    B0x1Cc2 = 1,
    #[doc = "2: If CC2 is an Input, reserved, do not use this configuration."]
    B0x2Cc2 = 2,
    #[doc = "3: If CC2 is an Input, both edges, circuit is sensitive to both IC2 rising and falling edges."]
    B0x3Cc2 = 3,
}
impl From<Cc2p> for u8 {
    #[inline(always)]
    fn from(variant: Cc2p) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc2p {
    type Ux = u8;
}
impl crate::IsEnum for Cc2p {}
#[doc = "Field `CC2P` reader - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
pub type Cc2pR = crate::FieldReader<Cc2p>;
impl Cc2pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2p {
        match self.bits {
            0 => Cc2p::B0x0Cc2,
            1 => Cc2p::B0x1Cc2,
            2 => Cc2p::B0x2Cc2,
            3 => Cc2p::B0x3Cc2,
            _ => unreachable!(),
        }
    }
    #[doc = "If CC2 is an Output, OC2 active high. If CC2 is an Input, rising edge, circuit is sensitive to IC2 rising edge."]
    #[inline(always)]
    pub fn is_b_0x0_cc2(&self) -> bool {
        *self == Cc2p::B0x0Cc2
    }
    #[doc = "If CC2 is an Output, OC2 active low. If CC2 is an Input, falling edge, circuit is sensitive to IC2 falling edge."]
    #[inline(always)]
    pub fn is_b_0x1_cc2(&self) -> bool {
        *self == Cc2p::B0x1Cc2
    }
    #[doc = "If CC2 is an Input, reserved, do not use this configuration."]
    #[inline(always)]
    pub fn is_b_0x2_cc2(&self) -> bool {
        *self == Cc2p::B0x2Cc2
    }
    #[doc = "If CC2 is an Input, both edges, circuit is sensitive to both IC2 rising and falling edges."]
    #[inline(always)]
    pub fn is_b_0x3_cc2(&self) -> bool {
        *self == Cc2p::B0x3Cc2
    }
}
#[doc = "Field `CC2P` writer - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
pub type Cc2pW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc2p, crate::Safe>;
impl<'a, REG> Cc2pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If CC2 is an Output, OC2 active high. If CC2 is an Input, rising edge, circuit is sensitive to IC2 rising edge."]
    #[inline(always)]
    pub fn b_0x0_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2p::B0x0Cc2)
    }
    #[doc = "If CC2 is an Output, OC2 active low. If CC2 is an Input, falling edge, circuit is sensitive to IC2 falling edge."]
    #[inline(always)]
    pub fn b_0x1_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2p::B0x1Cc2)
    }
    #[doc = "If CC2 is an Input, reserved, do not use this configuration."]
    #[inline(always)]
    pub fn b_0x2_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2p::B0x2Cc2)
    }
    #[doc = "If CC2 is an Input, both edges, circuit is sensitive to both IC2 rising and falling edges."]
    #[inline(always)]
    pub fn b_0x3_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2p::B0x3Cc2)
    }
}
#[doc = "Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic2psc {
    #[doc = "0: no prescaler, capture is done each time an edge is detected on the capture input"]
    B0x0 = 0,
    #[doc = "1: capture is done once every 2 events"]
    B0x1 = 1,
    #[doc = "2: capture is done once every 4 events"]
    B0x2 = 2,
    #[doc = "3: capture is done once every 8 events"]
    B0x3 = 3,
}
impl From<Ic2psc> for u8 {
    #[inline(always)]
    fn from(variant: Ic2psc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic2psc {
    type Ux = u8;
}
impl crate::IsEnum for Ic2psc {}
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
pub type Ic2pscR = crate::FieldReader<Ic2psc>;
impl Ic2pscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic2psc {
        match self.bits {
            0 => Ic2psc::B0x0,
            1 => Ic2psc::B0x1,
            2 => Ic2psc::B0x2,
            3 => Ic2psc::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic2psc::B0x0
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic2psc::B0x1
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic2psc::B0x2
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic2psc::B0x3
    }
}
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
pub type Ic2pscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic2psc, crate::Safe>;
impl<'a, REG> Ic2pscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2psc::B0x0)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2psc::B0x1)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2psc::B0x2)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2psc::B0x3)
    }
}
#[doc = "Input capture 2 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic2f {
    #[doc = "0: any external input capture signal level change is considered as a valid transition"]
    B0x0 = 0,
    #[doc = "1: external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    B0x1 = 1,
    #[doc = "2: external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    B0x2 = 2,
    #[doc = "3: external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    B0x3 = 3,
}
impl From<Ic2f> for u8 {
    #[inline(always)]
    fn from(variant: Ic2f) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic2f {
    type Ux = u8;
}
impl crate::IsEnum for Ic2f {}
#[doc = "Field `IC2F` reader - Input capture 2 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type Ic2fR = crate::FieldReader<Ic2f>;
impl Ic2fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic2f {
        match self.bits {
            0 => Ic2f::B0x0,
            1 => Ic2f::B0x1,
            2 => Ic2f::B0x2,
            3 => Ic2f::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic2f::B0x0
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic2f::B0x1
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic2f::B0x2
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic2f::B0x3
    }
}
#[doc = "Field `IC2F` writer - Input capture 2 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type Ic2fW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic2f, crate::Safe>;
impl<'a, REG> Ic2fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2f::B0x0)
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2f::B0x1)
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2f::B0x2)
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2f::B0x3)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
    #[inline(always)]
    pub fn cc1sel(&self) -> Cc1selR {
        Cc1selR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM1_CCR1) or not."]
    #[inline(always)]
    pub fn cc1e(&self) -> Cc1eR {
        Cc1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
    #[inline(always)]
    pub fn cc1p(&self) -> Cc1pR {
        Cc1pR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
    #[inline(always)]
    pub fn ic1psc(&self) -> Ic1pscR {
        Ic1pscR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Input capture 1 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn ic1f(&self) -> Ic1fR {
        Ic1fR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
    #[inline(always)]
    pub fn cc2sel(&self) -> Cc2selR {
        Cc2selR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM1_CCR2) or not."]
    #[inline(always)]
    pub fn cc2e(&self) -> Cc2eR {
        Cc2eR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
    #[inline(always)]
    pub fn ic2psc(&self) -> Ic2pscR {
        Ic2pscR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Input capture 2 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn ic2f(&self) -> Ic2fR {
        Ic2fR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
    #[inline(always)]
    #[must_use]
    pub fn cc1sel(&mut self) -> Cc1selW<Lptim1Ccmr1Spec> {
        Cc1selW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM1_CCR1) or not."]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> Cc1eW<Lptim1Ccmr1Spec> {
        Cc1eW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> Cc1pW<Lptim1Ccmr1Spec> {
        Cc1pW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> Ic1pscW<Lptim1Ccmr1Spec> {
        Ic1pscW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Input capture 1 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> Ic1fW<Lptim1Ccmr1Spec> {
        Ic1fW::new(self, 12)
    }
    #[doc = "Bit 16 - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
    #[inline(always)]
    #[must_use]
    pub fn cc2sel(&mut self) -> Cc2selW<Lptim1Ccmr1Spec> {
        Cc2selW::new(self, 16)
    }
    #[doc = "Bit 17 - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM1_CCR2) or not."]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> Cc2eW<Lptim1Ccmr1Spec> {
        Cc2eW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> Cc2pW<Lptim1Ccmr1Spec> {
        Cc2pW::new(self, 18)
    }
    #[doc = "Bits 24:25 - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> Ic2pscW<Lptim1Ccmr1Spec> {
        Ic2pscW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Input capture 2 filter This bitfield defines the number of consecutive equal samples that are detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> Ic2fW<Lptim1Ccmr1Spec> {
        Ic2fW::new(self, 28)
    }
}
#[doc = "LPTIM capture/compare mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim1_ccmr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim1_ccmr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim1Ccmr1Spec;
impl crate::RegisterSpec for Lptim1Ccmr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim1_ccmr1::R`](R) reader structure"]
impl crate::Readable for Lptim1Ccmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`lptim1_ccmr1::W`](W) writer structure"]
impl crate::Writable for Lptim1Ccmr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM1_CCMR1 to value 0"]
impl crate::Resettable for Lptim1Ccmr1Spec {
    const RESET_VALUE: u32 = 0;
}
