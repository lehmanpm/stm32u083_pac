#[doc = "Register `RNG_SR` reader"]
pub type R = crate::R<RngSrSpec>;
#[doc = "Register `RNG_SR` writer"]
pub type W = crate::W<RngSrSpec>;
#[doc = "Data ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN1=10 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY1=11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drdy {
    #[doc = "0: The RNG_DR register is not yet valid, no random data is available."]
    B0x0 = 0,
    #[doc = "1: The RNG_DR register contains valid random data."]
    B0x1 = 1,
}
impl From<Drdy> for bool {
    #[inline(always)]
    fn from(variant: Drdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRDY` reader - Data ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN1=10 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY1=11."]
pub type DrdyR = crate::BitReader<Drdy>;
impl DrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drdy {
        match self.bits {
            false => Drdy::B0x0,
            true => Drdy::B0x1,
        }
    }
    #[doc = "The RNG_DR register is not yet valid, no random data is available."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Drdy::B0x0
    }
    #[doc = "The RNG_DR register contains valid random data."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Drdy::B0x1
    }
}
#[doc = "Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cecs {
    #[doc = "0: The RNG clock is correct (f&lt;sub>RNGCLK&lt;/sub>> f&lt;sub>HCLK&lt;/sub>/32). If the CEIS bit is set, this means that a slow clock was detected and the situation has been recovered."]
    B0x0 = 0,
    #[doc = "1: The RNG clock is too slow (f&lt;sub>RNGCLK&lt;/sub>&lt; f&lt;sub>HCLK&lt;/sub>/32)."]
    B0x1 = 1,
}
impl From<Cecs> for bool {
    #[inline(always)]
    fn from(variant: Cecs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECS` reader - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0."]
pub type CecsR = crate::BitReader<Cecs>;
impl CecsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cecs {
        match self.bits {
            false => Cecs::B0x0,
            true => Cecs::B0x1,
        }
    }
    #[doc = "The RNG clock is correct (f&lt;sub>RNGCLK&lt;/sub>> f&lt;sub>HCLK&lt;/sub>/32). If the CEIS bit is set, this means that a slow clock was detected and the situation has been recovered."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cecs::B0x0
    }
    #[doc = "The RNG clock is too slow (f&lt;sub>RNGCLK&lt;/sub>&lt; f&lt;sub>HCLK&lt;/sub>/32)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cecs::B0x1
    }
}
#[doc = "Seed error current status Runtime repetition count test failed (noise source has provided more than 24 consecutive bits at a constant value 0 or 1, or more than 32 consecutive occurrence of two bits patterns 01 or 10) Startup or continuous adaptive proportion test on noise source failed. Startup post-processing/conditioning sanity check failed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secs {
    #[doc = "0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered."]
    B0x0 = 0,
    #[doc = "1: At least one of the following faulty sequences has been detected:"]
    B0x1 = 1,
}
impl From<Secs> for bool {
    #[inline(always)]
    fn from(variant: Secs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECS` reader - Seed error current status Runtime repetition count test failed (noise source has provided more than 24 consecutive bits at a constant value 0 or 1, or more than 32 consecutive occurrence of two bits patterns 01 or 10) Startup or continuous adaptive proportion test on noise source failed. Startup post-processing/conditioning sanity check failed."]
pub type SecsR = crate::BitReader<Secs>;
impl SecsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secs {
        match self.bits {
            false => Secs::B0x0,
            true => Secs::B0x1,
        }
    }
    #[doc = "No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Secs::B0x0
    }
    #[doc = "At least one of the following faulty sequences has been detected:"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Secs::B0x1
    }
}
#[doc = "Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceis {
    #[doc = "0: The RNG clock is correct (f&lt;sub>RNGCLK&lt;/sub>> f&lt;sub>HCLK&lt;/sub>/32)"]
    B0x0 = 0,
    #[doc = "1: The RNG clock before the internal divider is detected too slow (f&lt;sub>RNGCLK&lt;/sub>&lt; f&lt;sub>HCLK&lt;/sub>/32)"]
    B0x1 = 1,
}
impl From<Ceis> for bool {
    #[inline(always)]
    fn from(variant: Ceis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIS` reader - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type CeisR = crate::BitReader<Ceis>;
impl CeisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceis {
        match self.bits {
            false => Ceis::B0x0,
            true => Ceis::B0x1,
        }
    }
    #[doc = "The RNG clock is correct (f&lt;sub>RNGCLK&lt;/sub>> f&lt;sub>HCLK&lt;/sub>/32)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ceis::B0x0
    }
    #[doc = "The RNG clock before the internal divider is detected too slow (f&lt;sub>RNGCLK&lt;/sub>&lt; f&lt;sub>HCLK&lt;/sub>/32)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ceis::B0x1
    }
}
#[doc = "Field `CEIS` writer - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type CeisW<'a, REG> = crate::BitWriter<'a, REG, Ceis>;
impl<'a, REG> CeisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RNG clock is correct (f&lt;sub>RNGCLK&lt;/sub>> f&lt;sub>HCLK&lt;/sub>/32)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceis::B0x0)
    }
    #[doc = "The RNG clock before the internal divider is detected too slow (f&lt;sub>RNGCLK&lt;/sub>&lt; f&lt;sub>HCLK&lt;/sub>/32)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceis::B0x1)
    }
}
#[doc = "Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seis {
    #[doc = "0: No faulty sequence detected"]
    B0x0 = 0,
    #[doc = "1: At least one faulty sequence is detected. See SECS bit description for details."]
    B0x1 = 1,
}
impl From<Seis> for bool {
    #[inline(always)]
    fn from(variant: Seis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEIS` reader - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type SeisR = crate::BitReader<Seis>;
impl SeisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seis {
        match self.bits {
            false => Seis::B0x0,
            true => Seis::B0x1,
        }
    }
    #[doc = "No faulty sequence detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Seis::B0x0
    }
    #[doc = "At least one faulty sequence is detected. See SECS bit description for details."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Seis::B0x1
    }
}
#[doc = "Field `SEIS` writer - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type SeisW<'a, REG> = crate::BitWriter<'a, REG, Seis>;
impl<'a, REG> SeisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No faulty sequence detected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Seis::B0x0)
    }
    #[doc = "At least one faulty sequence is detected. See SECS bit description for details."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Seis::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Data ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN1=10 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY1=11."]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0."]
    #[inline(always)]
    pub fn cecs(&self) -> CecsR {
        CecsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status Runtime repetition count test failed (noise source has provided more than 24 consecutive bits at a constant value 0 or 1, or more than 32 consecutive occurrence of two bits patterns 01 or 10) Startup or continuous adaptive proportion test on noise source failed. Startup post-processing/conditioning sanity check failed."]
    #[inline(always)]
    pub fn secs(&self) -> SecsR {
        SecsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn ceis(&self) -> CeisR {
        CeisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn seis(&self) -> SeisR {
        SeisR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    #[must_use]
    pub fn ceis(&mut self) -> CeisW<RngSrSpec> {
        CeisW::new(self, 5)
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    #[must_use]
    pub fn seis(&mut self) -> SeisW<RngSrSpec> {
        SeisW::new(self, 6)
    }
}
#[doc = "RNG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngSrSpec;
impl crate::RegisterSpec for RngSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_sr::R`](R) reader structure"]
impl crate::Readable for RngSrSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_sr::W`](W) writer structure"]
impl crate::Writable for RngSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_SR to value 0"]
impl crate::Resettable for RngSrSpec {
    const RESET_VALUE: u32 = 0;
}
