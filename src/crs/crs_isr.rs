#[doc = "Register `CRS_ISR` reader"]
pub type R = crate::R<CrsIsrSpec>;
#[doc = "SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncokf {
    #[doc = "0: No SYNC event OK signaled"]
    B0x0 = 0,
    #[doc = "1: SYNC event OK signaled"]
    B0x1 = 1,
}
impl From<Syncokf> for bool {
    #[inline(always)]
    fn from(variant: Syncokf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCOKF` reader - SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register."]
pub type SyncokfR = crate::BitReader<Syncokf>;
impl SyncokfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncokf {
        match self.bits {
            false => Syncokf::B0x0,
            true => Syncokf::B0x1,
        }
    }
    #[doc = "No SYNC event OK signaled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncokf::B0x0
    }
    #[doc = "SYNC event OK signaled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncokf::B0x1
    }
}
#[doc = "SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncwarnf {
    #[doc = "0: No SYNC warning signaled"]
    B0x0 = 0,
    #[doc = "1: SYNC warning signaled"]
    B0x1 = 1,
}
impl From<Syncwarnf> for bool {
    #[inline(always)]
    fn from(variant: Syncwarnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCWARNF` reader - SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register."]
pub type SyncwarnfR = crate::BitReader<Syncwarnf>;
impl SyncwarnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncwarnf {
        match self.bits {
            false => Syncwarnf::B0x0,
            true => Syncwarnf::B0x1,
        }
    }
    #[doc = "No SYNC warning signaled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncwarnf::B0x0
    }
    #[doc = "SYNC warning signaled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncwarnf::B0x1
    }
}
#[doc = "Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errf {
    #[doc = "0: No synchronization or trimming error signaled"]
    B0x0 = 0,
    #[doc = "1: Synchronization or trimming error signaled"]
    B0x1 = 1,
}
impl From<Errf> for bool {
    #[inline(always)]
    fn from(variant: Errf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRF` reader - Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits."]
pub type ErrfR = crate::BitReader<Errf>;
impl ErrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errf {
        match self.bits {
            false => Errf::B0x0,
            true => Errf::B0x1,
        }
    }
    #[doc = "No synchronization or trimming error signaled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Errf::B0x0
    }
    #[doc = "Synchronization or trimming error signaled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Errf::B0x1
    }
}
#[doc = "Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esyncf {
    #[doc = "0: No expected SYNC signaled"]
    B0x0 = 0,
    #[doc = "1: Expected SYNC signaled"]
    B0x1 = 1,
}
impl From<Esyncf> for bool {
    #[inline(always)]
    fn from(variant: Esyncf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESYNCF` reader - Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register."]
pub type EsyncfR = crate::BitReader<Esyncf>;
impl EsyncfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esyncf {
        match self.bits {
            false => Esyncf::B0x0,
            true => Esyncf::B0x1,
        }
    }
    #[doc = "No expected SYNC signaled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Esyncf::B0x0
    }
    #[doc = "Expected SYNC signaled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Esyncf::B0x1
    }
}
#[doc = "SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action has to be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncerr {
    #[doc = "0: No SYNC error signaled"]
    B0x0 = 0,
    #[doc = "1: SYNC error signaled"]
    B0x1 = 1,
}
impl From<Syncerr> for bool {
    #[inline(always)]
    fn from(variant: Syncerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCERR` reader - SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action has to be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
pub type SyncerrR = crate::BitReader<Syncerr>;
impl SyncerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncerr {
        match self.bits {
            false => Syncerr::B0x0,
            true => Syncerr::B0x1,
        }
    }
    #[doc = "No SYNC error signaled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncerr::B0x0
    }
    #[doc = "SYNC error signaled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncerr::B0x1
    }
}
#[doc = "SYNC missed This flag is set by hardware when the frequency error counter reaches value FELIM * 128 and no SYNC is detected, meaning either that a SYNC pulse was missed, or the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, hence some other action must be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC), and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncmiss {
    #[doc = "0: No SYNC missed error signaled"]
    B0x0 = 0,
    #[doc = "1: SYNC missed error signaled"]
    B0x1 = 1,
}
impl From<Syncmiss> for bool {
    #[inline(always)]
    fn from(variant: Syncmiss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCMISS` reader - SYNC missed This flag is set by hardware when the frequency error counter reaches value FELIM * 128 and no SYNC is detected, meaning either that a SYNC pulse was missed, or the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, hence some other action must be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC), and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
pub type SyncmissR = crate::BitReader<Syncmiss>;
impl SyncmissR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncmiss {
        match self.bits {
            false => Syncmiss::B0x0,
            true => Syncmiss::B0x1,
        }
    }
    #[doc = "No SYNC missed error signaled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syncmiss::B0x0
    }
    #[doc = "SYNC missed error signaled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syncmiss::B0x1
    }
}
#[doc = "Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trimovf {
    #[doc = "0: No trimming error signaled"]
    B0x0 = 0,
    #[doc = "1: Trimming error signaled"]
    B0x1 = 1,
}
impl From<Trimovf> for bool {
    #[inline(always)]
    fn from(variant: Trimovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIMOVF` reader - Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
pub type TrimovfR = crate::BitReader<Trimovf>;
impl TrimovfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trimovf {
        match self.bits {
            false => Trimovf::B0x0,
            true => Trimovf::B0x1,
        }
    }
    #[doc = "No trimming error signaled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Trimovf::B0x0
    }
    #[doc = "Trimming error signaled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Trimovf::B0x1
    }
}
#[doc = "Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fedir {
    #[doc = "0: Up-counting direction, the actual frequency is above the target"]
    B0x0 = 0,
    #[doc = "1: Down-counting direction, the actual frequency is below the target"]
    B0x1 = 1,
}
impl From<Fedir> for bool {
    #[inline(always)]
    fn from(variant: Fedir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEDIR` reader - Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target."]
pub type FedirR = crate::BitReader<Fedir>;
impl FedirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fedir {
        match self.bits {
            false => Fedir::B0x0,
            true => Fedir::B0x1,
        }
    }
    #[doc = "Up-counting direction, the actual frequency is above the target"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fedir::B0x0
    }
    #[doc = "Down-counting direction, the actual frequency is below the target"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fedir::B0x1
    }
}
#[doc = "Field `FECAP` reader - Frequency error capture FECAP is the frequency error counter value latched in the time of the last SYNC event. Refer to Section15.4.4 for more details about FECAP usage."]
pub type FecapR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn syncokf(&self) -> SyncokfR {
        SyncokfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn syncwarnf(&self) -> SyncwarnfR {
        SyncwarnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits."]
    #[inline(always)]
    pub fn errf(&self) -> ErrfR {
        ErrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn esyncf(&self) -> EsyncfR {
        EsyncfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action has to be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn syncerr(&self) -> SyncerrR {
        SyncerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SYNC missed This flag is set by hardware when the frequency error counter reaches value FELIM * 128 and no SYNC is detected, meaning either that a SYNC pulse was missed, or the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, hence some other action must be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC), and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn syncmiss(&self) -> SyncmissR {
        SyncmissR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn trimovf(&self) -> TrimovfR {
        TrimovfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target."]
    #[inline(always)]
    pub fn fedir(&self) -> FedirR {
        FedirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Frequency error capture FECAP is the frequency error counter value latched in the time of the last SYNC event. Refer to Section15.4.4 for more details about FECAP usage."]
    #[inline(always)]
    pub fn fecap(&self) -> FecapR {
        FecapR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "CRS interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crs_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsIsrSpec;
impl crate::RegisterSpec for CrsIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crs_isr::R`](R) reader structure"]
impl crate::Readable for CrsIsrSpec {}
#[doc = "`reset()` method sets CRS_ISR to value 0"]
impl crate::Resettable for CrsIsrSpec {
    const RESET_VALUE: u32 = 0;
}
