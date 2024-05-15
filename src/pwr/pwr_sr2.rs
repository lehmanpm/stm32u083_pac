#[doc = "Register `PWR_SR2` reader"]
pub type R = crate::R<PwrSr2Spec>;
#[doc = "Flash ready flag This bit is set by hardware to indicate when the flash memory is readey to be accessed after wake-up from power-down. To place the flash memory in power-down, set either FPD_LPRUN, FPD_LPSLP or FPD_STP bits. Note : If the system boots from SRAM, the user application must wait until the FLASH_RDY bit is set, prior to jumping to flash memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashRdy {
    #[doc = "0: Flash memory in power down"]
    B0x0 = 0,
    #[doc = "1: Flash memory ready to be accessed"]
    B0x1 = 1,
}
impl From<FlashRdy> for bool {
    #[inline(always)]
    fn from(variant: FlashRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_RDY` reader - Flash ready flag This bit is set by hardware to indicate when the flash memory is readey to be accessed after wake-up from power-down. To place the flash memory in power-down, set either FPD_LPRUN, FPD_LPSLP or FPD_STP bits. Note : If the system boots from SRAM, the user application must wait until the FLASH_RDY bit is set, prior to jumping to flash memory."]
pub type FlashRdyR = crate::BitReader<FlashRdy>;
impl FlashRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlashRdy {
        match self.bits {
            false => FlashRdy::B0x0,
            true => FlashRdy::B0x1,
        }
    }
    #[doc = "Flash memory in power down"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FlashRdy::B0x0
    }
    #[doc = "Flash memory ready to be accessed"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FlashRdy::B0x1
    }
}
#[doc = "Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wake-up from Standby mode time may be increased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reglps {
    #[doc = "0: The low-power regulator is not ready"]
    B0x0 = 0,
    #[doc = "1: The low-power regulator is ready"]
    B0x1 = 1,
}
impl From<Reglps> for bool {
    #[inline(always)]
    fn from(variant: Reglps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLPS` reader - Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wake-up from Standby mode time may be increased."]
pub type ReglpsR = crate::BitReader<Reglps>;
impl ReglpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reglps {
        match self.bits {
            false => Reglps::B0x0,
            true => Reglps::B0x1,
        }
    }
    #[doc = "The low-power regulator is not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Reglps::B0x0
    }
    #[doc = "The low-power regulator is ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Reglps::B0x1
    }
}
#[doc = "Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits from the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reglpf {
    #[doc = "0: The regulator is ready in main mode (MR)"]
    B0x0 = 0,
    #[doc = "1: The regulator is in low-power mode (LPR)"]
    B0x1 = 1,
}
impl From<Reglpf> for bool {
    #[inline(always)]
    fn from(variant: Reglpf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLPF` reader - Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits from the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready."]
pub type ReglpfR = crate::BitReader<Reglpf>;
impl ReglpfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reglpf {
        match self.bits {
            false => Reglpf::B0x0,
            true => Reglpf::B0x1,
        }
    }
    #[doc = "The regulator is ready in main mode (MR)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Reglpf::B0x0
    }
    #[doc = "The regulator is in low-power mode (LPR)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Reglpf::B0x1
    }
}
#[doc = "Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vosf {
    #[doc = "0: The regulator is ready in the selected voltage range"]
    B0x0 = 0,
    #[doc = "1: The regulator output voltage is changing to the required voltage level"]
    B0x1 = 1,
}
impl From<Vosf> for bool {
    #[inline(always)]
    fn from(variant: Vosf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOSF` reader - Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register."]
pub type VosfR = crate::BitReader<Vosf>;
impl VosfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vosf {
        match self.bits {
            false => Vosf::B0x0,
            true => Vosf::B0x1,
        }
    }
    #[doc = "The regulator is ready in the selected voltage range"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Vosf::B0x0
    }
    #[doc = "The regulator output voltage is changing to the required voltage level"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Vosf::B0x1
    }
}
#[doc = "Programmable voltage detector output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvdo {
    #[doc = "0: V&lt;sub>DD&lt;/sub> is above the selected PVD threshold"]
    B0x0 = 0,
    #[doc = "1: V&lt;sub>DD&lt;/sub> is below the selected PVD threshold"]
    B0x1 = 1,
}
impl From<Pvdo> for bool {
    #[inline(always)]
    fn from(variant: Pvdo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDO` reader - Programmable voltage detector output"]
pub type PvdoR = crate::BitReader<Pvdo>;
impl PvdoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvdo {
        match self.bits {
            false => Pvdo::B0x0,
            true => Pvdo::B0x1,
        }
    }
    #[doc = "V&lt;sub>DD&lt;/sub> is above the selected PVD threshold"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvdo::B0x0
    }
    #[doc = "V&lt;sub>DD&lt;/sub> is below the selected PVD threshold"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvdo::B0x1
    }
}
#[doc = "Peripheral voltage monitoring output: V&lt;sub>DDUSB&lt;/sub> vs. 1.2 V Note: PVMO1 is cleared when PVM1 is disabled (PVME1 = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wake-up time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvmo1 {
    #[doc = "0: V&lt;sub>DDUSB&lt;/sub> voltage is above PVM1 threshold (around 1.21V)."]
    B0x0 = 0,
    #[doc = "1: V&lt;sub>DDUSB&lt;/sub> voltage is below PVM1 threshold (around 1.21V)."]
    B0x1 = 1,
}
impl From<Pvmo1> for bool {
    #[inline(always)]
    fn from(variant: Pvmo1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMO1` reader - Peripheral voltage monitoring output: V&lt;sub>DDUSB&lt;/sub> vs. 1.2 V Note: PVMO1 is cleared when PVM1 is disabled (PVME1 = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wake-up time."]
pub type Pvmo1R = crate::BitReader<Pvmo1>;
impl Pvmo1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvmo1 {
        match self.bits {
            false => Pvmo1::B0x0,
            true => Pvmo1::B0x1,
        }
    }
    #[doc = "V&lt;sub>DDUSB&lt;/sub> voltage is above PVM1 threshold (around 1.21V)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvmo1::B0x0
    }
    #[doc = "V&lt;sub>DDUSB&lt;/sub> voltage is below PVM1 threshold (around 1.21V)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvmo1::B0x1
    }
}
#[doc = "Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 1.621V Note: PVMO3 is cleared when PVM3 is disabled (PVME3 = 0). After enabling PVM3, the PVM3 output is valid after the PVM3 wake-up time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvmo3 {
    #[doc = "0: V&lt;sub>DDA&lt;/sub> voltage is above PVM3 threshold (around 1.621V)."]
    B0x0 = 0,
    #[doc = "1: V&lt;sub>DDA&lt;/sub> voltage is below PVM3 threshold (around 1.621V)."]
    B0x1 = 1,
}
impl From<Pvmo3> for bool {
    #[inline(always)]
    fn from(variant: Pvmo3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMO3` reader - Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 1.621V Note: PVMO3 is cleared when PVM3 is disabled (PVME3 = 0). After enabling PVM3, the PVM3 output is valid after the PVM3 wake-up time."]
pub type Pvmo3R = crate::BitReader<Pvmo3>;
impl Pvmo3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvmo3 {
        match self.bits {
            false => Pvmo3::B0x0,
            true => Pvmo3::B0x1,
        }
    }
    #[doc = "V&lt;sub>DDA&lt;/sub> voltage is above PVM3 threshold (around 1.621V)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvmo3::B0x0
    }
    #[doc = "V&lt;sub>DDA&lt;/sub> voltage is below PVM3 threshold (around 1.621V)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvmo3::B0x1
    }
}
#[doc = "Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 2.21V Note: PVMO4 is cleared when PVM4 is disabled (PVME4 = 0). After enabling PVM4, the PVM4 output is valid after the PVM4 wake-up time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvmo4 {
    #[doc = "0: V&lt;sub>DDA&lt;/sub> voltage is above PVM4 threshold (around 2.21V)."]
    B0x0 = 0,
    #[doc = "1: V&lt;sub>DDA&lt;/sub> voltage is below PVM4 threshold (around 2.21V)."]
    B0x1 = 1,
}
impl From<Pvmo4> for bool {
    #[inline(always)]
    fn from(variant: Pvmo4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMO4` reader - Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 2.21V Note: PVMO4 is cleared when PVM4 is disabled (PVME4 = 0). After enabling PVM4, the PVM4 output is valid after the PVM4 wake-up time."]
pub type Pvmo4R = crate::BitReader<Pvmo4>;
impl Pvmo4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvmo4 {
        match self.bits {
            false => Pvmo4::B0x0,
            true => Pvmo4::B0x1,
        }
    }
    #[doc = "V&lt;sub>DDA&lt;/sub> voltage is above PVM4 threshold (around 2.21V)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvmo4::B0x0
    }
    #[doc = "V&lt;sub>DDA&lt;/sub> voltage is below PVM4 threshold (around 2.21V)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvmo4::B0x1
    }
}
impl R {
    #[doc = "Bit 7 - Flash ready flag This bit is set by hardware to indicate when the flash memory is readey to be accessed after wake-up from power-down. To place the flash memory in power-down, set either FPD_LPRUN, FPD_LPSLP or FPD_STP bits. Note : If the system boots from SRAM, the user application must wait until the FLASH_RDY bit is set, prior to jumping to flash memory."]
    #[inline(always)]
    pub fn flash_rdy(&self) -> FlashRdyR {
        FlashRdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wake-up from Standby mode time may be increased."]
    #[inline(always)]
    pub fn reglps(&self) -> ReglpsR {
        ReglpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits from the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready."]
    #[inline(always)]
    pub fn reglpf(&self) -> ReglpfR {
        ReglpfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register."]
    #[inline(always)]
    pub fn vosf(&self) -> VosfR {
        VosfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Programmable voltage detector output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PvdoR {
        PvdoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral voltage monitoring output: V&lt;sub>DDUSB&lt;/sub> vs. 1.2 V Note: PVMO1 is cleared when PVM1 is disabled (PVME1 = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wake-up time."]
    #[inline(always)]
    pub fn pvmo1(&self) -> Pvmo1R {
        Pvmo1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 1.621V Note: PVMO3 is cleared when PVM3 is disabled (PVME3 = 0). After enabling PVM3, the PVM3 output is valid after the PVM3 wake-up time."]
    #[inline(always)]
    pub fn pvmo3(&self) -> Pvmo3R {
        Pvmo3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral voltage monitoring output: V&lt;sub>DDA&lt;/sub> vs. 2.21V Note: PVMO4 is cleared when PVM4 is disabled (PVME4 = 0). After enabling PVM4, the PVM4 output is valid after the PVM4 wake-up time."]
    #[inline(always)]
    pub fn pvmo4(&self) -> Pvmo4R {
        Pvmo4R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_sr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrSr2Spec;
impl crate::RegisterSpec for PwrSr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_sr2::R`](R) reader structure"]
impl crate::Readable for PwrSr2Spec {}
#[doc = "`reset()` method sets PWR_SR2 to value 0"]
impl crate::Resettable for PwrSr2Spec {
    const RESET_VALUE: u32 = 0;
}
