#[doc = "Register `TIM1_CCER` reader"]
pub type R = crate::R<Tim1CcerSpec>;
#[doc = "Register `TIM1_CCER` writer"]
pub type W = crate::W<Tim1CcerSpec>;
#[doc = "Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table1119 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1e {
    #[doc = "0: Capture mode disabled / OC1 is not active (see below)"]
    B0x0 = 0,
    #[doc = "1: Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    B0x1 = 1,
}
impl From<Cc1e> for bool {
    #[inline(always)]
    fn from(variant: Cc1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1E` reader - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table1119 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1eR = crate::BitReader<Cc1e>;
impl Cc1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1e {
        match self.bits {
            false => Cc1e::B0x0,
            true => Cc1e::B0x1,
        }
    }
    #[doc = "Capture mode disabled / OC1 is not active (see below)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1e::B0x0
    }
    #[doc = "Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1e::B0x1
    }
}
#[doc = "Field `CC1E` writer - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table1119 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1eW<'a, REG> = crate::BitWriter<'a, REG, Cc1e>;
impl<'a, REG> Cc1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture mode disabled / OC1 is not active (see below)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1e::B0x0)
    }
    #[doc = "Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1e::B0x1)
    }
}
#[doc = "Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1p {
    #[doc = "0: OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    B0x0 = 0,
    #[doc = "1: OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    B0x1 = 1,
}
impl From<Cc1p> for bool {
    #[inline(always)]
    fn from(variant: Cc1p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1P` reader - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1pR = crate::BitReader<Cc1p>;
impl Cc1pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1p {
        match self.bits {
            false => Cc1p::B0x0,
            true => Cc1p::B0x1,
        }
    }
    #[doc = "OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1p::B0x0
    }
    #[doc = "OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1p::B0x1
    }
}
#[doc = "Field `CC1P` writer - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1pW<'a, REG> = crate::BitWriter<'a, REG, Cc1p>;
impl<'a, REG> Cc1pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1p::B0x0)
    }
    #[doc = "OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1p::B0x1)
    }
}
#[doc = "Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1ne {
    #[doc = "0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    B0x0 = 0,
    #[doc = "1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    B0x1 = 1,
}
impl From<Cc1ne> for bool {
    #[inline(always)]
    fn from(variant: Cc1ne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1NE` reader - Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1neR = crate::BitReader<Cc1ne>;
impl Cc1neR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1ne {
        match self.bits {
            false => Cc1ne::B0x0,
            true => Cc1ne::B0x1,
        }
    }
    #[doc = "Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1ne::B0x0
    }
    #[doc = "On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1ne::B0x1
    }
}
#[doc = "Field `CC1NE` writer - Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1neW<'a, REG> = crate::BitWriter<'a, REG, Cc1ne>;
impl<'a, REG> Cc1neW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1ne::B0x0)
    }
    #[doc = "On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1ne::B0x1)
    }
}
#[doc = "Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1np {
    #[doc = "0: OC1N active high."]
    B0x0 = 0,
    #[doc = "1: OC1N active low."]
    B0x1 = 1,
}
impl From<Cc1np> for bool {
    #[inline(always)]
    fn from(variant: Cc1np) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1NP` reader - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1npR = crate::BitReader<Cc1np>;
impl Cc1npR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1np {
        match self.bits {
            false => Cc1np::B0x0,
            true => Cc1np::B0x1,
        }
    }
    #[doc = "OC1N active high."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1np::B0x0
    }
    #[doc = "OC1N active low."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1np::B0x1
    }
}
#[doc = "Field `CC1NP` writer - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type Cc1npW<'a, REG> = crate::BitWriter<'a, REG, Cc1np>;
impl<'a, REG> Cc1npW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1N active high."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1np::B0x0)
    }
    #[doc = "OC1N active low."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1np::B0x1)
    }
}
#[doc = "Field `CC2E` reader - Capture/Compare 2 output enable Refer to CC1E description"]
pub type Cc2eR = crate::BitReader;
#[doc = "Field `CC2E` writer - Capture/Compare 2 output enable Refer to CC1E description"]
pub type Cc2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - Capture/Compare 2 output polarity Refer to CC1P description"]
pub type Cc2pR = crate::BitReader;
#[doc = "Field `CC2P` writer - Capture/Compare 2 output polarity Refer to CC1P description"]
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NE` reader - Capture/Compare 2 complementary output enable Refer to CC1NE description"]
pub type Cc2neR = crate::BitReader;
#[doc = "Field `CC2NE` writer - Capture/Compare 2 complementary output enable Refer to CC1NE description"]
pub type Cc2neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Capture/Compare 2 complementary output polarity Refer to CC1NP description"]
pub type Cc2npR = crate::BitReader;
#[doc = "Field `CC2NP` writer - Capture/Compare 2 complementary output polarity Refer to CC1NP description"]
pub type Cc2npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - Capture/Compare 3 output enable Refer to CC1E description"]
pub type Cc3eR = crate::BitReader;
#[doc = "Field `CC3E` writer - Capture/Compare 3 output enable Refer to CC1E description"]
pub type Cc3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - Capture/Compare 3 output polarity Refer to CC1P description"]
pub type Cc3pR = crate::BitReader;
#[doc = "Field `CC3P` writer - Capture/Compare 3 output polarity Refer to CC1P description"]
pub type Cc3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NE` reader - Capture/Compare 3 complementary output enable Refer to CC1NE description"]
pub type Cc3neR = crate::BitReader;
#[doc = "Field `CC3NE` writer - Capture/Compare 3 complementary output enable Refer to CC1NE description"]
pub type Cc3neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - Capture/Compare 3 complementary output polarity Refer to CC1NP description"]
pub type Cc3npR = crate::BitReader;
#[doc = "Field `CC3NP` writer - Capture/Compare 3 complementary output polarity Refer to CC1NP description"]
pub type Cc3npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - Capture/Compare 4 output enable Refer to CC1E description"]
pub type Cc4eR = crate::BitReader;
#[doc = "Field `CC4E` writer - Capture/Compare 4 output enable Refer to CC1E description"]
pub type Cc4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - Capture/Compare 4 output polarity Refer to CC1P description"]
pub type Cc4pR = crate::BitReader;
#[doc = "Field `CC4P` writer - Capture/Compare 4 output polarity Refer to CC1P description"]
pub type Cc4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NP` reader - Capture/Compare 4 complementary output polarity Refer to CC1NP description"]
pub type Cc4npR = crate::BitReader;
#[doc = "Field `CC4NP` writer - Capture/Compare 4 complementary output polarity Refer to CC1NP description"]
pub type Cc4npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5E` reader - Capture/Compare 5 output enable Refer to CC1E description"]
pub type Cc5eR = crate::BitReader;
#[doc = "Field `CC5E` writer - Capture/Compare 5 output enable Refer to CC1E description"]
pub type Cc5eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5P` reader - Capture/Compare 5 output polarity Refer to CC1P description"]
pub type Cc5pR = crate::BitReader;
#[doc = "Field `CC5P` writer - Capture/Compare 5 output polarity Refer to CC1P description"]
pub type Cc5pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6E` reader - Capture/Compare 6 output enable Refer to CC1E description"]
pub type Cc6eR = crate::BitReader;
#[doc = "Field `CC6E` writer - Capture/Compare 6 output enable Refer to CC1E description"]
pub type Cc6eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6P` reader - Capture/Compare 6 output polarity Refer to CC1P description"]
pub type Cc6pR = crate::BitReader;
#[doc = "Field `CC6P` writer - Capture/Compare 6 output polarity Refer to CC1P description"]
pub type Cc6pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table1119 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1e(&self) -> Cc1eR {
        Cc1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1p(&self) -> Cc1pR {
        Cc1pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1ne(&self) -> Cc1neR {
        Cc1neR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1np(&self) -> Cc1npR {
        Cc1npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc2e(&self) -> Cc2eR {
        Cc2eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn cc2ne(&self) -> Cc2neR {
        Cc2neR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn cc2np(&self) -> Cc2npR {
        Cc2npR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc3e(&self) -> Cc3eR {
        Cc3eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc3p(&self) -> Cc3pR {
        Cc3pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn cc3ne(&self) -> Cc3neR {
        Cc3neR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn cc3np(&self) -> Cc3npR {
        Cc3npR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc4e(&self) -> Cc4eR {
        Cc4eR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc4p(&self) -> Cc4pR {
        Cc4pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture/Compare 4 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn cc4np(&self) -> Cc4npR {
        Cc4npR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture/Compare 5 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc5e(&self) -> Cc5eR {
        Cc5eR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture/Compare 5 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc5p(&self) -> Cc5pR {
        Cc5pR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Capture/Compare 6 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc6e(&self) -> Cc6eR {
        Cc6eR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Capture/Compare 6 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc6p(&self) -> Cc6pR {
        Cc6pR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table1119 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> Cc1eW<Tim1CcerSpec> {
        Cc1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> Cc1pW<Tim1CcerSpec> {
        Cc1pW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> Cc1neW<Tim1CcerSpec> {
        Cc1neW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=00 (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> Cc1npW<Tim1CcerSpec> {
        Cc1npW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> Cc2eW<Tim1CcerSpec> {
        Cc2eW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> Cc2pW<Tim1CcerSpec> {
        Cc2pW::new(self, 5)
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ne(&mut self) -> Cc2neW<Tim1CcerSpec> {
        Cc2neW::new(self, 6)
    }
    #[doc = "Bit 7 - Capture/Compare 2 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> Cc2npW<Tim1CcerSpec> {
        Cc2npW::new(self, 7)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> Cc3eW<Tim1CcerSpec> {
        Cc3eW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> Cc3pW<Tim1CcerSpec> {
        Cc3pW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3ne(&mut self) -> Cc3neW<Tim1CcerSpec> {
        Cc3neW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3np(&mut self) -> Cc3npW<Tim1CcerSpec> {
        Cc3npW::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> Cc4eW<Tim1CcerSpec> {
        Cc4eW::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> Cc4pW<Tim1CcerSpec> {
        Cc4pW::new(self, 13)
    }
    #[doc = "Bit 15 - Capture/Compare 4 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4np(&mut self) -> Cc4npW<Tim1CcerSpec> {
        Cc4npW::new(self, 15)
    }
    #[doc = "Bit 16 - Capture/Compare 5 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc5e(&mut self) -> Cc5eW<Tim1CcerSpec> {
        Cc5eW::new(self, 16)
    }
    #[doc = "Bit 17 - Capture/Compare 5 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc5p(&mut self) -> Cc5pW<Tim1CcerSpec> {
        Cc5pW::new(self, 17)
    }
    #[doc = "Bit 20 - Capture/Compare 6 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc6e(&mut self) -> Cc6eW<Tim1CcerSpec> {
        Cc6eW::new(self, 20)
    }
    #[doc = "Bit 21 - Capture/Compare 6 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc6p(&mut self) -> Cc6pW<Tim1CcerSpec> {
        Cc6pW::new(self, 21)
    }
}
#[doc = "TIM1 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim1CcerSpec;
impl crate::RegisterSpec for Tim1CcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccer::R`](R) reader structure"]
impl crate::Readable for Tim1CcerSpec {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccer::W`](W) writer structure"]
impl crate::Writable for Tim1CcerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCER to value 0"]
impl crate::Resettable for Tim1CcerSpec {
    const RESET_VALUE: u32 = 0;
}
