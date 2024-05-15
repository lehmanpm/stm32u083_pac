#[doc = "Register `AES_SR` reader"]
pub type R = crate::R<AesSrSpec>;
#[doc = "Read error flag This bit is set when an unexpected read to the AES_DOUTR register occurred. When set RDERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rderrf {
    #[doc = "0: No error"]
    B0x0 = 0,
    #[doc = "1: Unexpected read to AES_DOUTR register occurred during computation or data input phase."]
    B0x1 = 1,
}
impl From<Rderrf> for bool {
    #[inline(always)]
    fn from(variant: Rderrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERRF` reader - Read error flag This bit is set when an unexpected read to the AES_DOUTR register occurred. When set RDERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register."]
pub type RderrfR = crate::BitReader<Rderrf>;
impl RderrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rderrf {
        match self.bits {
            false => Rderrf::B0x0,
            true => Rderrf::B0x1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rderrf::B0x0
    }
    #[doc = "Unexpected read to AES_DOUTR register occurred during computation or data input phase."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rderrf::B0x1
    }
}
#[doc = "Write error flag This bit is set when an unexpected write to the AES_DINR register occurred. When set WRERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrerrf {
    #[doc = "0: No error"]
    B0x0 = 0,
    #[doc = "1: Unexpected write to AES_DINR register occurred during computation or data output phase."]
    B0x1 = 1,
}
impl From<Wrerrf> for bool {
    #[inline(always)]
    fn from(variant: Wrerrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRERRF` reader - Write error flag This bit is set when an unexpected write to the AES_DINR register occurred. When set WRERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register."]
pub type WrerrfR = crate::BitReader<Wrerrf>;
impl WrerrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrerrf {
        match self.bits {
            false => Wrerrf::B0x0,
            true => Wrerrf::B0x1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wrerrf::B0x0
    }
    #[doc = "Unexpected write to AES_DINR register occurred during computation or data output phase."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wrerrf::B0x1
    }
}
#[doc = "Busy This flag indicates whether AES is idle or busy. AES is flagged as idle when disabled (when EN is low) or when the last processing is completed. AES is flagged as busy when processing a block data, preparing a key (ECB or CBC decryption only). When GCM encryption is selected, this flag must be at zero before suspending current process to manage a higher-priority message.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: Idle"]
    B0x0 = 0,
    #[doc = "1: Busy"]
    B0x1 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy This flag indicates whether AES is idle or busy. AES is flagged as idle when disabled (when EN is low) or when the last processing is completed. AES is flagged as busy when processing a block data, preparing a key (ECB or CBC decryption only). When GCM encryption is selected, this flag must be at zero before suspending current process to manage a higher-priority message."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::B0x0,
            true => Busy::B0x1,
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Busy::B0x0
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Busy::B0x1
    }
}
#[doc = "Key valid flag This bit is set by hardware when the key of size defined by KEYSIZE is loaded in AES_KEYRx key registers. The EN bit can only be set when KEYVALID is set. The key must be written in the key registers in the correct sequence, otherwise the KEIF flag is set and KEYVALID remains cleared. If set, KEIF must be cleared through the AES_ICR register, otherwise KEYVALID cannot be set. See the KEIF flag description for more details. For further information on key loading, refer to Section121.4.15: AES key registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyvalid {
    #[doc = "0: Key not valid"]
    B0x0 = 0,
    #[doc = "1: Key valid"]
    B0x1 = 1,
}
impl From<Keyvalid> for bool {
    #[inline(always)]
    fn from(variant: Keyvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYVALID` reader - Key valid flag This bit is set by hardware when the key of size defined by KEYSIZE is loaded in AES_KEYRx key registers. The EN bit can only be set when KEYVALID is set. The key must be written in the key registers in the correct sequence, otherwise the KEIF flag is set and KEYVALID remains cleared. If set, KEIF must be cleared through the AES_ICR register, otherwise KEYVALID cannot be set. See the KEIF flag description for more details. For further information on key loading, refer to Section121.4.15: AES key registers."]
pub type KeyvalidR = crate::BitReader<Keyvalid>;
impl KeyvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keyvalid {
        match self.bits {
            false => Keyvalid::B0x0,
            true => Keyvalid::B0x1,
        }
    }
    #[doc = "Key not valid"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Keyvalid::B0x0
    }
    #[doc = "Key valid"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Keyvalid::B0x1
    }
}
impl R {
    #[doc = "Bit 1 - Read error flag This bit is set when an unexpected read to the AES_DOUTR register occurred. When set RDERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register."]
    #[inline(always)]
    pub fn rderrf(&self) -> RderrfR {
        RderrfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write error flag This bit is set when an unexpected write to the AES_DINR register occurred. When set WRERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register."]
    #[inline(always)]
    pub fn wrerrf(&self) -> WrerrfR {
        WrerrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy This flag indicates whether AES is idle or busy. AES is flagged as idle when disabled (when EN is low) or when the last processing is completed. AES is flagged as busy when processing a block data, preparing a key (ECB or CBC decryption only). When GCM encryption is selected, this flag must be at zero before suspending current process to manage a higher-priority message."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Key valid flag This bit is set by hardware when the key of size defined by KEYSIZE is loaded in AES_KEYRx key registers. The EN bit can only be set when KEYVALID is set. The key must be written in the key registers in the correct sequence, otherwise the KEIF flag is set and KEYVALID remains cleared. If set, KEIF must be cleared through the AES_ICR register, otherwise KEYVALID cannot be set. See the KEIF flag description for more details. For further information on key loading, refer to Section121.4.15: AES key registers."]
    #[inline(always)]
    pub fn keyvalid(&self) -> KeyvalidR {
        KeyvalidR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "AES status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesSrSpec;
impl crate::RegisterSpec for AesSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_sr::R`](R) reader structure"]
impl crate::Readable for AesSrSpec {}
#[doc = "`reset()` method sets AES_SR to value 0"]
impl crate::Resettable for AesSrSpec {
    const RESET_VALUE: u32 = 0;
}
