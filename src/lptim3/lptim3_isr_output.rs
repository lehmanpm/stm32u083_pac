#[doc = "Register `LPTIM3_ISR_OUTPUT` reader"]
pub type R = crate::R<Lptim3IsrOutputSpec>;
#[doc = "Compare 1 interrupt flag If channel CC1 is configured as output: The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1if {
    #[doc = "0: No match"]
    B0x0 = 0,
    #[doc = "1: The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR1 register's value"]
    B0x1 = 1,
}
impl From<Cc1if> for bool {
    #[inline(always)]
    fn from(variant: Cc1if) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IF` reader - Compare 1 interrupt flag If channel CC1 is configured as output: The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register."]
pub type Cc1ifR = crate::BitReader<Cc1if>;
impl Cc1ifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1if {
        match self.bits {
            false => Cc1if::B0x0,
            true => Cc1if::B0x1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1if::B0x0
    }
    #[doc = "The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR1 register's value"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1if::B0x1
    }
}
#[doc = "Field `ARRM` reader - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registers value reached the LPTIM_ARR registers value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
pub type ArrmR = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `CMP1OK` reader - Compare register 1 update OK CMP1OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1 to the CMP1OKCF bit in the LPTIM_ICR register."]
pub type Cmp1okR = crate::BitReader;
#[doc = "Field `ARROK` reader - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
pub type ArrokR = crate::BitReader;
#[doc = "Field `UP` reader - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type UpR = crate::BitReader;
#[doc = "Field `DOWN` reader - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type DownR = crate::BitReader;
#[doc = "Field `UE` reader - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. The corresponding interrupt or DMA request is generated if enabled. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register. The UE flag is automatically cleared by hardware once the LPTIM_ARR register is written by any bus master like CPU or DMA."]
pub type UeR = crate::BitReader;
#[doc = "Field `REPOK` reader - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register."]
pub type RepokR = crate::BitReader;
#[doc = "Compare 2 interrupt flag If channel CC2 is configured as output: The CC2IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC2IF flag can be cleared by writing 1 to the CC2CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc2if {
    #[doc = "0: No match"]
    B0x0 = 0,
    #[doc = "1: The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR2 register's value"]
    B0x1 = 1,
}
impl From<Cc2if> for bool {
    #[inline(always)]
    fn from(variant: Cc2if) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2IF` reader - Compare 2 interrupt flag If channel CC2 is configured as output: The CC2IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC2IF flag can be cleared by writing 1 to the CC2CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cc2ifR = crate::BitReader<Cc2if>;
impl Cc2ifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2if {
        match self.bits {
            false => Cc2if::B0x0,
            true => Cc2if::B0x1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc2if::B0x0
    }
    #[doc = "The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR2 register's value"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc2if::B0x1
    }
}
#[doc = "Compare 3 interrupt flag If channel CC3 is configured as output: The CC3IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC3IF flag can be cleared by writing 1 to the CC3CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc3if {
    #[doc = "0: No match"]
    B0x0 = 0,
    #[doc = "1: The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR3 register's value."]
    B0x1 = 1,
}
impl From<Cc3if> for bool {
    #[inline(always)]
    fn from(variant: Cc3if) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC3IF` reader - Compare 3 interrupt flag If channel CC3 is configured as output: The CC3IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC3IF flag can be cleared by writing 1 to the CC3CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cc3ifR = crate::BitReader<Cc3if>;
impl Cc3ifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc3if {
        match self.bits {
            false => Cc3if::B0x0,
            true => Cc3if::B0x1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc3if::B0x0
    }
    #[doc = "The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR3 register's value."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc3if::B0x1
    }
}
#[doc = "Compare 4 interrupt flag If channel CC4 is configured as output: The CC4IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC4IF flag can be cleared by writing 1 to the CC4CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc4if {
    #[doc = "0: No match"]
    B0x0 = 0,
    #[doc = "1: The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR4 register's value"]
    B0x1 = 1,
}
impl From<Cc4if> for bool {
    #[inline(always)]
    fn from(variant: Cc4if) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC4IF` reader - Compare 4 interrupt flag If channel CC4 is configured as output: The CC4IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC4IF flag can be cleared by writing 1 to the CC4CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cc4ifR = crate::BitReader<Cc4if>;
impl Cc4ifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc4if {
        match self.bits {
            false => Cc4if::B0x0,
            true => Cc4if::B0x1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc4if::B0x0
    }
    #[doc = "The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR4 register's value"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc4if::B0x1
    }
}
#[doc = "Field `CMP2OK` reader - Compare register 2 update OK CMP2OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR2 register has been successfully completed. CMP2OK flag can be cleared by writing 1 to the CMP2OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp2okR = crate::BitReader;
#[doc = "Field `CMP3OK` reader - Compare register 3 update OK CMP3OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR3 register has been successfully completed. CMP3OK flag can be cleared by writing 1 to the CMP3OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp3okR = crate::BitReader;
#[doc = "Field `CMP4OK` reader - Compare register 4 update OK CMP4OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR4 register has been successfully completed. CMP4OK flag can be cleared by writing 1 to the CMP4OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cmp4okR = crate::BitReader;
#[doc = "Field `DIEROK` reader - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register."]
pub type DierokR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compare 1 interrupt flag If channel CC1 is configured as output: The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registers value reached the LPTIM_ARR registers value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn arrm(&self) -> ArrmR {
        ArrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn exttrig(&self) -> ExttrigR {
        ExttrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register 1 update OK CMP1OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1 to the CMP1OKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn cmp1ok(&self) -> Cmp1okR {
        Cmp1okR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn arrok(&self) -> ArrokR {
        ArrokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. The corresponding interrupt or DMA request is generated if enabled. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register. The UE flag is automatically cleared by hardware once the LPTIM_ARR register is written by any bus master like CPU or DMA."]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn repok(&self) -> RepokR {
        RepokR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare 2 interrupt flag If channel CC2 is configured as output: The CC2IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC2IF flag can be cleared by writing 1 to the CC2CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Compare 3 interrupt flag If channel CC3 is configured as output: The CC3IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC3IF flag can be cleared by writing 1 to the CC3CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare 4 interrupt flag If channel CC4 is configured as output: The CC4IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC4IF flag can be cleared by writing 1 to the CC4CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc4if(&self) -> Cc4ifR {
        Cc4ifR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare register 2 update OK CMP2OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR2 register has been successfully completed. CMP2OK flag can be cleared by writing 1 to the CMP2OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cmp2ok(&self) -> Cmp2okR {
        Cmp2okR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Compare register 3 update OK CMP3OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR3 register has been successfully completed. CMP3OK flag can be cleared by writing 1 to the CMP3OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cmp3ok(&self) -> Cmp3okR {
        Cmp3okR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Compare register 4 update OK CMP4OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR4 register has been successfully completed. CMP4OK flag can be cleared by writing 1 to the CMP4OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cmp4ok(&self) -> Cmp4okR {
        Cmp4okR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn dierok(&self) -> DierokR {
        DierokR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "LPTIM3 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_isr_output::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim3IsrOutputSpec;
impl crate::RegisterSpec for Lptim3IsrOutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim3_isr_output::R`](R) reader structure"]
impl crate::Readable for Lptim3IsrOutputSpec {}
#[doc = "`reset()` method sets LPTIM3_ISR_OUTPUT to value 0"]
impl crate::Resettable for Lptim3IsrOutputSpec {
    const RESET_VALUE: u32 = 0;
}
