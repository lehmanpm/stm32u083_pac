#[doc = "Register `AES_IER` reader"]
pub type R = crate::R<AesIerSpec>;
#[doc = "Register `AES_IER` writer"]
pub type W = crate::W<AesIerSpec>;
#[doc = "Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccfie {
    #[doc = "0: Disabled (masked)"]
    B0x0 = 0,
    #[doc = "1: Enabled (not masked)"]
    B0x1 = 1,
}
impl From<Ccfie> for bool {
    #[inline(always)]
    fn from(variant: Ccfie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCFIE` reader - Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set."]
pub type CcfieR = crate::BitReader<Ccfie>;
impl CcfieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccfie {
        match self.bits {
            false => Ccfie::B0x0,
            true => Ccfie::B0x1,
        }
    }
    #[doc = "Disabled (masked)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccfie::B0x0
    }
    #[doc = "Enabled (not masked)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccfie::B0x1
    }
}
#[doc = "Field `CCFIE` writer - Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set."]
pub type CcfieW<'a, REG> = crate::BitWriter<'a, REG, Ccfie>;
impl<'a, REG> CcfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled (masked)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccfie::B0x0)
    }
    #[doc = "Enabled (not masked)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccfie::B0x1)
    }
}
#[doc = "Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rweie {
    #[doc = "0: Disabled (masked)"]
    B0x0 = 0,
    #[doc = "1: Enabled (not masked)"]
    B0x1 = 1,
}
impl From<Rweie> for bool {
    #[inline(always)]
    fn from(variant: Rweie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWEIE` reader - Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set."]
pub type RweieR = crate::BitReader<Rweie>;
impl RweieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rweie {
        match self.bits {
            false => Rweie::B0x0,
            true => Rweie::B0x1,
        }
    }
    #[doc = "Disabled (masked)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rweie::B0x0
    }
    #[doc = "Enabled (not masked)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rweie::B0x1
    }
}
#[doc = "Field `RWEIE` writer - Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set."]
pub type RweieW<'a, REG> = crate::BitWriter<'a, REG, Rweie>;
impl<'a, REG> RweieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled (masked)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rweie::B0x0)
    }
    #[doc = "Enabled (not masked)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rweie::B0x1)
    }
}
#[doc = "Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keie {
    #[doc = "0: Disabled (masked)"]
    B0x0 = 0,
    #[doc = "1: Enabled (not masked)"]
    B0x1 = 1,
}
impl From<Keie> for bool {
    #[inline(always)]
    fn from(variant: Keie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEIE` reader - Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set."]
pub type KeieR = crate::BitReader<Keie>;
impl KeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keie {
        match self.bits {
            false => Keie::B0x0,
            true => Keie::B0x1,
        }
    }
    #[doc = "Disabled (masked)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Keie::B0x0
    }
    #[doc = "Enabled (not masked)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Keie::B0x1
    }
}
#[doc = "Field `KEIE` writer - Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set."]
pub type KeieW<'a, REG> = crate::BitWriter<'a, REG, Keie>;
impl<'a, REG> KeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled (masked)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Keie::B0x0)
    }
    #[doc = "Enabled (not masked)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Keie::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set."]
    #[inline(always)]
    pub fn ccfie(&self) -> CcfieR {
        CcfieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set."]
    #[inline(always)]
    pub fn rweie(&self) -> RweieR {
        RweieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set."]
    #[inline(always)]
    pub fn keie(&self) -> KeieR {
        KeieR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set."]
    #[inline(always)]
    #[must_use]
    pub fn ccfie(&mut self) -> CcfieW<AesIerSpec> {
        CcfieW::new(self, 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set."]
    #[inline(always)]
    #[must_use]
    pub fn rweie(&mut self) -> RweieW<AesIerSpec> {
        RweieW::new(self, 1)
    }
    #[doc = "Bit 2 - Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set."]
    #[inline(always)]
    #[must_use]
    pub fn keie(&mut self) -> KeieW<AesIerSpec> {
        KeieW::new(self, 2)
    }
}
#[doc = "AES interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesIerSpec;
impl crate::RegisterSpec for AesIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_ier::R`](R) reader structure"]
impl crate::Readable for AesIerSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_ier::W`](W) writer structure"]
impl crate::Writable for AesIerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_IER to value 0"]
impl crate::Resettable for AesIerSpec {
    const RESET_VALUE: u32 = 0;
}
