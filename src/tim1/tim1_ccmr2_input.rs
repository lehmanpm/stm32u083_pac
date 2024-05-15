#[doc = "Register `TIM1_CCMR2_INPUT` reader"]
pub type R = crate::R<Tim1Ccmr2InputSpec>;
#[doc = "Register `TIM1_CCMR2_INPUT` writer"]
pub type W = crate::W<Tim1Ccmr2InputSpec>;
#[doc = "Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc3s {
    #[doc = "0: CC3 channel is configured as output"]
    B0x0 = 0,
    #[doc = "1: CC3 channel is configured as input, IC3 is mapped on TI3"]
    B0x1 = 1,
    #[doc = "2: CC3 channel is configured as input, IC3 is mapped on TI4"]
    B0x2 = 2,
    #[doc = "3: CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B0x3 = 3,
}
impl From<Cc3s> for u8 {
    #[inline(always)]
    fn from(variant: Cc3s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc3s {
    type Ux = u8;
}
impl crate::IsEnum for Cc3s {}
#[doc = "Field `CC3S` reader - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
pub type Cc3sR = crate::FieldReader<Cc3s>;
impl Cc3sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc3s {
        match self.bits {
            0 => Cc3s::B0x0,
            1 => Cc3s::B0x1,
            2 => Cc3s::B0x2,
            3 => Cc3s::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc3s::B0x0
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc3s::B0x1
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc3s::B0x2
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc3s::B0x3
    }
}
#[doc = "Field `CC3S` writer - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc3s, crate::Safe>;
impl<'a, REG> Cc3sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::B0x0)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::B0x1)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::B0x2)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc3s::B0x3)
    }
}
#[doc = "Field `IC3PSC` reader - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type Ic3pscR = crate::FieldReader;
#[doc = "Field `IC3PSC` writer - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type Ic3pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3F` reader - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
pub type Ic3fR = crate::FieldReader;
#[doc = "Field `IC3F` writer - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
pub type Ic3fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cc4s {
    #[doc = "0: CC4 channel is configured as output"]
    B0x0 = 0,
    #[doc = "1: CC4 channel is configured as input, IC4 is mapped on TI4"]
    B0x1 = 1,
    #[doc = "2: CC4 channel is configured as input, IC4 is mapped on TI3"]
    B0x2 = 2,
    #[doc = "3: CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B0x3 = 3,
}
impl From<Cc4s> for u8 {
    #[inline(always)]
    fn from(variant: Cc4s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cc4s {
    type Ux = u8;
}
impl crate::IsEnum for Cc4s {}
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
pub type Cc4sR = crate::FieldReader<Cc4s>;
impl Cc4sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc4s {
        match self.bits {
            0 => Cc4s::B0x0,
            1 => Cc4s::B0x1,
            2 => Cc4s::B0x2,
            3 => Cc4s::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc4s::B0x0
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc4s::B0x1
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Cc4s::B0x2
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Cc4s::B0x3
    }
}
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cc4s, crate::Safe>;
impl<'a, REG> Cc4sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4s::B0x0)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4s::B0x1)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4s::B0x2)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cc4s::B0x3)
    }
}
#[doc = "Field `IC4PSC` reader - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type Ic4pscR = crate::FieldReader;
#[doc = "Field `IC4PSC` writer - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
pub type Ic4pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4F` reader - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
pub type Ic4fR = crate::FieldReader;
#[doc = "Field `IC4F` writer - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
pub type Ic4fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic3psc(&self) -> Ic3pscR {
        Ic3pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic3f(&self) -> Ic3fR {
        Ic3fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    pub fn ic4psc(&self) -> Ic4pscR {
        Ic4pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    pub fn ic4f(&self) -> Ic4fR {
        Ic4fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> Cc3sW<Tim1Ccmr2InputSpec> {
        Cc3sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic3psc(&mut self) -> Ic3pscW<Tim1Ccmr2InputSpec> {
        Ic3pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic3f(&mut self) -> Ic3fW<Tim1Ccmr2InputSpec> {
        Ic3fW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> Cc4sW<Tim1Ccmr2InputSpec> {
        Cc4sW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler Refer to IC1PSC\\[1:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic4psc(&mut self) -> Ic4pscW<Tim1Ccmr2InputSpec> {
        Ic4pscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter Refer to IC1F\\[3:0\\]
description."]
    #[inline(always)]
    #[must_use]
    pub fn ic4f(&mut self) -> Ic4fW<Tim1Ccmr2InputSpec> {
        Ic4fW::new(self, 12)
    }
}
#[doc = "TIM1 capture/compare mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccmr2_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccmr2_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1Ccmr2InputSpec;
impl crate::RegisterSpec for Tim1Ccmr2InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccmr2_input::R`](R) reader structure"]
impl crate::Readable for Tim1Ccmr2InputSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccmr2_input::W`](W) writer structure"]
impl crate::Writable for Tim1Ccmr2InputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCMR2_INPUT to value 0"]
impl crate::Resettable for Tim1Ccmr2InputSpec {
    const RESET_VALUE: u32 = 0;
}
