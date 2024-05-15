#[doc = "Register `LPTIM2_ISR_INPUT` reader"]
pub type R = crate::R<Lptim2IsrInputSpec>;
#[doc = "capture 1 interrupt flag If channel CC1 is configured as input: CC1IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR1 register. The corresponding interrupt or DMA request is generated if enabled. The CC1OF flag is set if the CC1IF flag was already high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1if {
    #[doc = "0: No input capture occurred"]
    B0x0 = 0,
    #[doc = "1: The counter value has been captured in the LPTIM_CCR1 register. (An edge has been detected on IC1 which matches the selected polarity). The CC1IF flag is automatically cleared by hardware once the captured value is read (CPU or DMA).CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register."]
    B0x1 = 1,
}
impl From<Cc1if> for bool {
    #[inline(always)]
    fn from(variant: Cc1if) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IF` reader - capture 1 interrupt flag If channel CC1 is configured as input: CC1IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR1 register. The corresponding interrupt or DMA request is generated if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
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
    #[doc = "No input capture occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1if::B0x0
    }
    #[doc = "The counter value has been captured in the LPTIM_CCR1 register. (An edge has been detected on IC1 which matches the selected polarity). The CC1IF flag is automatically cleared by hardware once the captured value is read (CPU or DMA).CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1if::B0x1
    }
}
#[doc = "Field `ARRM` reader - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registers value reached the LPTIM_ARR registers value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
pub type ArrmR = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `ARROK` reader - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
pub type ArrokR = crate::BitReader;
#[doc = "Field `UP` reader - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type UpR = crate::BitReader;
#[doc = "Field `DOWN` reader - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3."]
pub type DownR = crate::BitReader;
#[doc = "Field `UE` reader - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register."]
pub type UeR = crate::BitReader;
#[doc = "Field `REPOK` reader - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register."]
pub type RepokR = crate::BitReader;
#[doc = "Capture 2 interrupt flag If channel CC2 is configured as input: CC2IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR2 register. The corresponding interrupt or DMA request is generated if enabled. The CC2OF flag is set if the CC2IF flag was already high. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc2if {
    #[doc = "0: No input capture occurred"]
    B0x0 = 0,
    #[doc = "1: The counter value has been captured in the LPTIM_CCR2 register. (An edge has been detected on IC2 which matches the selected polarity). The CC2IF flag is automatically cleared by hardware once the captured value is read (CPU or DMA). The CC2IF flag can be cleared by writing 1 to the CC2CF bit in the LPTIM_ICR register."]
    B0x1 = 1,
}
impl From<Cc2if> for bool {
    #[inline(always)]
    fn from(variant: Cc2if) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2IF` reader - Capture 2 interrupt flag If channel CC2 is configured as input: CC2IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR2 register. The corresponding interrupt or DMA request is generated if enabled. The CC2OF flag is set if the CC2IF flag was already high. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
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
    #[doc = "No input capture occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc2if::B0x0
    }
    #[doc = "The counter value has been captured in the LPTIM_CCR2 register. (An edge has been detected on IC2 which matches the selected polarity). The CC2IF flag is automatically cleared by hardware once the captured value is read (CPU or DMA). The CC2IF flag can be cleared by writing 1 to the CC2CF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc2if::B0x1
    }
}
#[doc = "Capture 3 interrupt flag If channel CC3 is configured as input: CC3IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR3 register. The corresponding interrupt or DMA request is generated if enabled. The CC3OF flag is set if the CC3IF flag was already high. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc3if {
    #[doc = "0: No input capture occurred"]
    B0x0 = 0,
    #[doc = "1: The counter value has been captured in the LPTIM_CCR3 register. (An edge has been detected on IC3 which matches the selected polarity). The CC3IF flag is automatically cleared by hardware once the captured value is read (CPU or DMA). The CC3IF flag can be cleared by writing 1 to the CC3CF bit in the LPTIM_ICR register."]
    B0x1 = 1,
}
impl From<Cc3if> for bool {
    #[inline(always)]
    fn from(variant: Cc3if) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC3IF` reader - Capture 3 interrupt flag If channel CC3 is configured as input: CC3IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR3 register. The corresponding interrupt or DMA request is generated if enabled. The CC3OF flag is set if the CC3IF flag was already high. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
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
    #[doc = "No input capture occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc3if::B0x0
    }
    #[doc = "The counter value has been captured in the LPTIM_CCR3 register. (An edge has been detected on IC3 which matches the selected polarity). The CC3IF flag is automatically cleared by hardware once the captured value is read (CPU or DMA). The CC3IF flag can be cleared by writing 1 to the CC3CF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc3if::B0x1
    }
}
#[doc = "Capture 4 interrupt flag If channel CC4 is configured as input: CC4IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR4 register. The corresponding interrupt or DMA request is generated if enabled. The CC4OF flag is set if the CC4IF flag was already high. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc4if {
    #[doc = "0: No input capture occurred"]
    B0x0 = 0,
    #[doc = "1: The counter value has been captured in the LPTIM_CCR4 register. (An edge has been detected on IC4 which matches the selected polarity). The CC4IF flag is automatically cleared by hardware once the captured value is read (CPU or DMA). The CC4IF flag can be cleared by writing 1 to the CC4CF bit in the LPTIM_ICR register."]
    B0x1 = 1,
}
impl From<Cc4if> for bool {
    #[inline(always)]
    fn from(variant: Cc4if) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC4IF` reader - Capture 4 interrupt flag If channel CC4 is configured as input: CC4IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR4 register. The corresponding interrupt or DMA request is generated if enabled. The CC4OF flag is set if the CC4IF flag was already high. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
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
    #[doc = "No input capture occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc4if::B0x0
    }
    #[doc = "The counter value has been captured in the LPTIM_CCR4 register. (An edge has been detected on IC4 which matches the selected polarity). The CC4IF flag is automatically cleared by hardware once the captured value is read (CPU or DMA). The CC4IF flag can be cleared by writing 1 to the CC4CF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc4if::B0x1
    }
}
#[doc = "Capture 1 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1of {
    #[doc = "0: No over-capture has been detected."]
    B0x0 = 0,
    #[doc = "1: The counter value has been captured in LPTIM_CCR1 register while CC1IF flag was already set."]
    B0x1 = 1,
}
impl From<Cc1of> for bool {
    #[inline(always)]
    fn from(variant: Cc1of) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1OF` reader - Capture 1 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3."]
pub type Cc1ofR = crate::BitReader<Cc1of>;
impl Cc1ofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1of {
        match self.bits {
            false => Cc1of::B0x0,
            true => Cc1of::B0x1,
        }
    }
    #[doc = "No over-capture has been detected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc1of::B0x0
    }
    #[doc = "The counter value has been captured in LPTIM_CCR1 register while CC1IF flag was already set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc1of::B0x1
    }
}
#[doc = "Capture 2 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC2OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc2of {
    #[doc = "0: No over-capture has been detected."]
    B0x0 = 0,
    #[doc = "1: The counter value has been captured in LPTIM_CCR2 register while CC2IF flag was already set."]
    B0x1 = 1,
}
impl From<Cc2of> for bool {
    #[inline(always)]
    fn from(variant: Cc2of) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2OF` reader - Capture 2 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC2OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
pub type Cc2ofR = crate::BitReader<Cc2of>;
impl Cc2ofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2of {
        match self.bits {
            false => Cc2of::B0x0,
            true => Cc2of::B0x1,
        }
    }
    #[doc = "No over-capture has been detected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc2of::B0x0
    }
    #[doc = "The counter value has been captured in LPTIM_CCR2 register while CC2IF flag was already set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc2of::B0x1
    }
}
#[doc = "Capture 3 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC3OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc3of {
    #[doc = "0: No over-capture has been detected."]
    B0x0 = 0,
    #[doc = "1: The counter value has been captured in LPTIM_CCR3 register while CC3IF flag was already set"]
    B0x1 = 1,
}
impl From<Cc3of> for bool {
    #[inline(always)]
    fn from(variant: Cc3of) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC3OF` reader - Capture 3 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC3OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
pub type Cc3ofR = crate::BitReader<Cc3of>;
impl Cc3ofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc3of {
        match self.bits {
            false => Cc3of::B0x0,
            true => Cc3of::B0x1,
        }
    }
    #[doc = "No over-capture has been detected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc3of::B0x0
    }
    #[doc = "The counter value has been captured in LPTIM_CCR3 register while CC3IF flag was already set"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc3of::B0x1
    }
}
#[doc = "Capture 4 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC4OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc4of {
    #[doc = "0: No over-capture has been detected."]
    B0x0 = 0,
    #[doc = "1: The counter value has been captured in LPTIM_CCR4 register while CC4IF flag was already set"]
    B0x1 = 1,
}
impl From<Cc4of> for bool {
    #[inline(always)]
    fn from(variant: Cc4of) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC4OF` reader - Capture 4 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC4OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
pub type Cc4ofR = crate::BitReader<Cc4of>;
impl Cc4ofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc4of {
        match self.bits {
            false => Cc4of::B0x0,
            true => Cc4of::B0x1,
        }
    }
    #[doc = "No over-capture has been detected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cc4of::B0x0
    }
    #[doc = "The counter value has been captured in LPTIM_CCR4 register while CC4IF flag was already set"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cc4of::B0x1
    }
}
#[doc = "Field `DIEROK` reader - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register."]
pub type DierokR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - capture 1 interrupt flag If channel CC1 is configured as input: CC1IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR1 register. The corresponding interrupt or DMA request is generated if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
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
    #[doc = "Bit 7 - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn repok(&self) -> RepokR {
        RepokR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture 2 interrupt flag If channel CC2 is configured as input: CC2IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR2 register. The corresponding interrupt or DMA request is generated if enabled. The CC2OF flag is set if the CC2IF flag was already high. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture 3 interrupt flag If channel CC3 is configured as input: CC3IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR3 register. The corresponding interrupt or DMA request is generated if enabled. The CC3OF flag is set if the CC3IF flag was already high. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture 4 interrupt flag If channel CC4 is configured as input: CC4IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR4 register. The corresponding interrupt or DMA request is generated if enabled. The CC4OF flag is set if the CC4IF flag was already high. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc4if(&self) -> Cc4ifR {
        Cc4ifR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture 1 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc1of(&self) -> Cc1ofR {
        Cc1ofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture 2 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC2OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc2of(&self) -> Cc2ofR {
        Cc2ofR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Capture 3 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC3OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc3of(&self) -> Cc3ofR {
        Cc3ofR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture 4 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC4OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3."]
    #[inline(always)]
    pub fn cc4of(&self) -> Cc4ofR {
        Cc4ofR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn dierok(&self) -> DierokR {
        DierokR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "LPTIM2 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim2_isr_input::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim2IsrInputSpec;
impl crate::RegisterSpec for Lptim2IsrInputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim2_isr_input::R`](R) reader structure"]
impl crate::Readable for Lptim2IsrInputSpec {}
#[doc = "`reset()` method sets LPTIM2_ISR_INPUT to value 0"]
impl crate::Resettable for Lptim2IsrInputSpec {
    const RESET_VALUE: u32 = 0;
}
