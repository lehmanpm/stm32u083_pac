#[doc = "Register `TSC_ISR` reader"]
pub type R = crate::R<TscIsrSpec>;
#[doc = "End of acquisition flag This bit is set by hardware when the acquisition of all enabled group is complete (all GxS bits of all enabled analog I/O groups are set or when a max count error is detected). It is cleared by software writing 1 to the bit EOAIC of the TSC_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoaf {
    #[doc = "0: Acquisition is ongoing or not started"]
    B0x0 = 0,
    #[doc = "1: Acquisition is complete"]
    B0x1 = 1,
}
impl From<Eoaf> for bool {
    #[inline(always)]
    fn from(variant: Eoaf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOAF` reader - End of acquisition flag This bit is set by hardware when the acquisition of all enabled group is complete (all GxS bits of all enabled analog I/O groups are set or when a max count error is detected). It is cleared by software writing 1 to the bit EOAIC of the TSC_ICR register."]
pub type EoafR = crate::BitReader<Eoaf>;
impl EoafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoaf {
        match self.bits {
            false => Eoaf::B0x0,
            true => Eoaf::B0x1,
        }
    }
    #[doc = "Acquisition is ongoing or not started"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eoaf::B0x0
    }
    #[doc = "Acquisition is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eoaf::B0x1
    }
}
#[doc = "Max count error flag This bit is set by hardware as soon as an analog I/O group counter reaches the max count value specified. It is cleared by software writing 1 to the bit MCEIC of the TSC_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcef {
    #[doc = "0: No max count error (MCE) detected"]
    B0x0 = 0,
    #[doc = "1: Max count error (MCE) detected"]
    B0x1 = 1,
}
impl From<Mcef> for bool {
    #[inline(always)]
    fn from(variant: Mcef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCEF` reader - Max count error flag This bit is set by hardware as soon as an analog I/O group counter reaches the max count value specified. It is cleared by software writing 1 to the bit MCEIC of the TSC_ICR register."]
pub type McefR = crate::BitReader<Mcef>;
impl McefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcef {
        match self.bits {
            false => Mcef::B0x0,
            true => Mcef::B0x1,
        }
    }
    #[doc = "No max count error (MCE) detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mcef::B0x0
    }
    #[doc = "Max count error (MCE) detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mcef::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - End of acquisition flag This bit is set by hardware when the acquisition of all enabled group is complete (all GxS bits of all enabled analog I/O groups are set or when a max count error is detected). It is cleared by software writing 1 to the bit EOAIC of the TSC_ICR register."]
    #[inline(always)]
    pub fn eoaf(&self) -> EoafR {
        EoafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error flag This bit is set by hardware as soon as an analog I/O group counter reaches the max count value specified. It is cleared by software writing 1 to the bit MCEIC of the TSC_ICR register."]
    #[inline(always)]
    pub fn mcef(&self) -> McefR {
        McefR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "TSC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscIsrSpec;
impl crate::RegisterSpec for TscIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsc_isr::R`](R) reader structure"]
impl crate::Readable for TscIsrSpec {}
#[doc = "`reset()` method sets TSC_ISR to value 0"]
impl crate::Resettable for TscIsrSpec {
    const RESET_VALUE: u32 = 0;
}
