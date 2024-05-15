#[doc = "Register `AES_ISR` reader"]
pub type R = crate::R<AesIsrSpec>;
#[doc = "Computation complete flag This flag indicates whether the computation is completed. It is significant only when the DMAOUTEN bit is cleared, and it may stay high when DMAOUTEN is set. The flag setting generates an interrupt if the CCFIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccf {
    #[doc = "0: Not completed"]
    B0x0 = 0,
    #[doc = "1: Completed"]
    B0x1 = 1,
}
impl From<Ccf> for bool {
    #[inline(always)]
    fn from(variant: Ccf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCF` reader - Computation complete flag This flag indicates whether the computation is completed. It is significant only when the DMAOUTEN bit is cleared, and it may stay high when DMAOUTEN is set. The flag setting generates an interrupt if the CCFIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register."]
pub type CcfR = crate::BitReader<Ccf>;
impl CcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccf {
        match self.bits {
            false => Ccf::B0x0,
            true => Ccf::B0x1,
        }
    }
    #[doc = "Not completed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccf::B0x0
    }
    #[doc = "Completed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccf::B0x1
    }
}
#[doc = "Read or write error interrupt flag This read-only bit is set by hardware when a RDERRF or a WRERRF error flag is set in the AES_SR register. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. The flags has no meaning when key derivation mode is selected. See the AES_SR register for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rweif {
    #[doc = "0: No read or write error detected"]
    B0x0 = 0,
    #[doc = "1: Read or write error detected"]
    B0x1 = 1,
}
impl From<Rweif> for bool {
    #[inline(always)]
    fn from(variant: Rweif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWEIF` reader - Read or write error interrupt flag This read-only bit is set by hardware when a RDERRF or a WRERRF error flag is set in the AES_SR register. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. The flags has no meaning when key derivation mode is selected. See the AES_SR register for details."]
pub type RweifR = crate::BitReader<Rweif>;
impl RweifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rweif {
        match self.bits {
            false => Rweif::B0x0,
            true => Rweif::B0x1,
        }
    }
    #[doc = "No read or write error detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rweif::B0x0
    }
    #[doc = "Read or write error detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rweif::B0x1
    }
}
#[doc = "Key error interrupt flag This read-only bit is set by hardware when the key information fails to load into key registers. The flag setting generates an interrupt if the KEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. KEIF is raised upon any of the following events: AES_KEYRx register write does not respect the correct order. (For KEYSIZE1cleared, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE set, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keif {
    #[doc = "0: No key error detected"]
    B0x0 = 0,
    #[doc = "1: Key information failed to load into key registers"]
    B0x1 = 1,
}
impl From<Keif> for bool {
    #[inline(always)]
    fn from(variant: Keif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEIF` reader - Key error interrupt flag This read-only bit is set by hardware when the key information fails to load into key registers. The flag setting generates an interrupt if the KEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. KEIF is raised upon any of the following events: AES_KEYRx register write does not respect the correct order. (For KEYSIZE1cleared, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE set, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set."]
pub type KeifR = crate::BitReader<Keif>;
impl KeifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keif {
        match self.bits {
            false => Keif::B0x0,
            true => Keif::B0x1,
        }
    }
    #[doc = "No key error detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Keif::B0x0
    }
    #[doc = "Key information failed to load into key registers"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Keif::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - Computation complete flag This flag indicates whether the computation is completed. It is significant only when the DMAOUTEN bit is cleared, and it may stay high when DMAOUTEN is set. The flag setting generates an interrupt if the CCFIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register."]
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt flag This read-only bit is set by hardware when a RDERRF or a WRERRF error flag is set in the AES_SR register. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. The flags has no meaning when key derivation mode is selected. See the AES_SR register for details."]
    #[inline(always)]
    pub fn rweif(&self) -> RweifR {
        RweifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key error interrupt flag This read-only bit is set by hardware when the key information fails to load into key registers. The flag setting generates an interrupt if the KEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. KEIF is raised upon any of the following events: AES_KEYRx register write does not respect the correct order. (For KEYSIZE1cleared, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE set, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set."]
    #[inline(always)]
    pub fn keif(&self) -> KeifR {
        KeifR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "AES interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesIsrSpec;
impl crate::RegisterSpec for AesIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_isr::R`](R) reader structure"]
impl crate::Readable for AesIsrSpec {}
#[doc = "`reset()` method sets AES_ISR to value 0"]
impl crate::Resettable for AesIsrSpec {
    const RESET_VALUE: u32 = 0;
}
