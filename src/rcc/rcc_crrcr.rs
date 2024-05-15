#[doc = "Register `RCC_CRRCR` reader"]
pub type R = crate::R<RccCrrcrSpec>;
#[doc = "Register `RCC_CRRCR` writer"]
pub type W = crate::W<RccCrrcrSpec>;
#[doc = "HSI48 RC oscillator enable&lt;sup>(1)&lt;/sup>\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsi48on {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<Hsi48on> for bool {
    #[inline(always)]
    fn from(variant: Hsi48on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48ON` reader - HSI48 RC oscillator enable&lt;sup>(1)&lt;/sup>"]
pub type Hsi48onR = crate::BitReader<Hsi48on>;
impl Hsi48onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsi48on {
        match self.bits {
            false => Hsi48on::B0x0,
            true => Hsi48on::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsi48on::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsi48on::B0x1
    }
}
#[doc = "Field `HSI48ON` writer - HSI48 RC oscillator enable&lt;sup>(1)&lt;/sup>"]
pub type Hsi48onW<'a, REG> = crate::BitWriter<'a, REG, Hsi48on>;
impl<'a, REG> Hsi48onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi48on::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi48on::B0x1)
    }
}
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag&lt;sup>(1)&lt;/sup> The flag is set when the HSI48 clock is ready for use."]
pub type Hsi48rdyR = crate::BitReader;
#[doc = "Field `HSI48CAL` reader - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value."]
pub type Hsi48calR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - HSI48 RC oscillator enable&lt;sup>(1)&lt;/sup>"]
    #[inline(always)]
    pub fn hsi48on(&self) -> Hsi48onR {
        Hsi48onR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSI48 clock ready flag&lt;sup>(1)&lt;/sup> The flag is set when the HSI48 clock is ready for use."]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> Hsi48rdyR {
        Hsi48rdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:15 - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value."]
    #[inline(always)]
    pub fn hsi48cal(&self) -> Hsi48calR {
        Hsi48calR::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - HSI48 RC oscillator enable&lt;sup>(1)&lt;/sup>"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> Hsi48onW<RccCrrcrSpec> {
        Hsi48onW::new(self, 0)
    }
}
#[doc = "RCC clock recovery RC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_crrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_crrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCrrcrSpec;
impl crate::RegisterSpec for RccCrrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_crrcr::R`](R) reader structure"]
impl crate::Readable for RccCrrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_crrcr::W`](W) writer structure"]
impl crate::Writable for RccCrrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CRRCR to value 0x8800"]
impl crate::Resettable for RccCrrcrSpec {
    const RESET_VALUE: u32 = 0x8800;
}
