#[doc = "Register `FLASH_OPTR` reader"]
pub type R = crate::R<FlashOptrSpec>;
#[doc = "Register `FLASH_OPTR` writer"]
pub type W = crate::W<FlashOptrSpec>;
#[doc = "Read protection level Other: Level 1, memories read protection active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rdp {
    #[doc = "170: Level 0, read protection not active"]
    B0xAa = 170,
    #[doc = "204: Level 2, chip read protection active"]
    B0xCc = 204,
}
impl From<Rdp> for u8 {
    #[inline(always)]
    fn from(variant: Rdp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rdp {
    type Ux = u8;
}
impl crate::IsEnum for Rdp {}
#[doc = "Field `RDP` reader - Read protection level Other: Level 1, memories read protection active"]
pub type RdpR = crate::FieldReader<Rdp>;
impl RdpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rdp> {
        match self.bits {
            170 => Some(Rdp::B0xAa),
            204 => Some(Rdp::B0xCc),
            _ => None,
        }
    }
    #[doc = "Level 0, read protection not active"]
    #[inline(always)]
    pub fn is_b_0x_aa(&self) -> bool {
        *self == Rdp::B0xAa
    }
    #[doc = "Level 2, chip read protection active"]
    #[inline(always)]
    pub fn is_b_0x_cc(&self) -> bool {
        *self == Rdp::B0xCc
    }
}
#[doc = "Field `RDP` writer - Read protection level Other: Level 1, memories read protection active"]
pub type RdpW<'a, REG> = crate::FieldWriter<'a, REG, 8, Rdp>;
impl<'a, REG> RdpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Level 0, read protection not active"]
    #[inline(always)]
    pub fn b_0x_aa(self) -> &'a mut crate::W<REG> {
        self.variant(Rdp::B0xAa)
    }
    #[doc = "Level 2, chip read protection active"]
    #[inline(always)]
    pub fn b_0x_cc(self) -> &'a mut crate::W<REG> {
        self.variant(Rdp::B0xCc)
    }
}
#[doc = "BOR reset level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BorrLev {
    #[doc = "0: BOR rising level 1 with threshold around 2.1 V"]
    B0x0 = 0,
    #[doc = "1: BOR rising level 2 with threshold around 2.3 V"]
    B0x1 = 1,
    #[doc = "2: BOR rising level 3 with threshold around 2.6 V"]
    B0x2 = 2,
    #[doc = "3: BOR rising level 4 with threshold around 2.9 V"]
    B0x3 = 3,
}
impl From<BorrLev> for u8 {
    #[inline(always)]
    fn from(variant: BorrLev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BorrLev {
    type Ux = u8;
}
impl crate::IsEnum for BorrLev {}
#[doc = "Field `BORR_LEV` reader - BOR reset level"]
pub type BorrLevR = crate::FieldReader<BorrLev>;
impl BorrLevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BorrLev> {
        match self.bits {
            0 => Some(BorrLev::B0x0),
            1 => Some(BorrLev::B0x1),
            2 => Some(BorrLev::B0x2),
            3 => Some(BorrLev::B0x3),
            _ => None,
        }
    }
    #[doc = "BOR rising level 1 with threshold around 2.1 V"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BorrLev::B0x0
    }
    #[doc = "BOR rising level 2 with threshold around 2.3 V"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BorrLev::B0x1
    }
    #[doc = "BOR rising level 3 with threshold around 2.6 V"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BorrLev::B0x2
    }
    #[doc = "BOR rising level 4 with threshold around 2.9 V"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BorrLev::B0x3
    }
}
#[doc = "Field `BORR_LEV` writer - BOR reset level"]
pub type BorrLevW<'a, REG> = crate::FieldWriter<'a, REG, 3, BorrLev>;
impl<'a, REG> BorrLevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BOR rising level 1 with threshold around 2.1 V"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BorrLev::B0x0)
    }
    #[doc = "BOR rising level 2 with threshold around 2.3 V"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BorrLev::B0x1)
    }
    #[doc = "BOR rising level 3 with threshold around 2.6 V"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BorrLev::B0x2)
    }
    #[doc = "BOR rising level 4 with threshold around 2.9 V"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BorrLev::B0x3)
    }
}
#[doc = "Reset generated when entering Stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NrstStop {
    #[doc = "0: Reset generated when entering the Stop mode"]
    B0x0 = 0,
    #[doc = "1: No reset generated when entering the Stop mode"]
    B0x1 = 1,
}
impl From<NrstStop> for bool {
    #[inline(always)]
    fn from(variant: NrstStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRST_STOP` reader - Reset generated when entering Stop mode"]
pub type NrstStopR = crate::BitReader<NrstStop>;
impl NrstStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NrstStop {
        match self.bits {
            false => NrstStop::B0x0,
            true => NrstStop::B0x1,
        }
    }
    #[doc = "Reset generated when entering the Stop mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NrstStop::B0x0
    }
    #[doc = "No reset generated when entering the Stop mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NrstStop::B0x1
    }
}
#[doc = "Field `NRST_STOP` writer - Reset generated when entering Stop mode"]
pub type NrstStopW<'a, REG> = crate::BitWriter<'a, REG, NrstStop>;
impl<'a, REG> NrstStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset generated when entering the Stop mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NrstStop::B0x0)
    }
    #[doc = "No reset generated when entering the Stop mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NrstStop::B0x1)
    }
}
#[doc = "Reset generated when entering Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NrstStdby {
    #[doc = "0: Reset generated when entering the Standby mode"]
    B0x0 = 0,
    #[doc = "1: No reset generate when entering the Standby mode"]
    B0x1 = 1,
}
impl From<NrstStdby> for bool {
    #[inline(always)]
    fn from(variant: NrstStdby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRST_STDBY` reader - Reset generated when entering Standby mode"]
pub type NrstStdbyR = crate::BitReader<NrstStdby>;
impl NrstStdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NrstStdby {
        match self.bits {
            false => NrstStdby::B0x0,
            true => NrstStdby::B0x1,
        }
    }
    #[doc = "Reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NrstStdby::B0x0
    }
    #[doc = "No reset generate when entering the Standby mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NrstStdby::B0x1
    }
}
#[doc = "Field `NRST_STDBY` writer - Reset generated when entering Standby mode"]
pub type NrstStdbyW<'a, REG> = crate::BitWriter<'a, REG, NrstStdby>;
impl<'a, REG> NrstStdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset generated when entering the Standby mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NrstStdby::B0x0)
    }
    #[doc = "No reset generate when entering the Standby mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NrstStdby::B0x1)
    }
}
#[doc = "Reset generated when entering Shutdown mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NrstShdw {
    #[doc = "0: Reset generated when entering the Shutdown mode"]
    B0x0 = 0,
    #[doc = "1: No reset generated when entering the Shutdown mode"]
    B0x1 = 1,
}
impl From<NrstShdw> for bool {
    #[inline(always)]
    fn from(variant: NrstShdw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRST_SHDW` reader - Reset generated when entering Shutdown mode"]
pub type NrstShdwR = crate::BitReader<NrstShdw>;
impl NrstShdwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NrstShdw {
        match self.bits {
            false => NrstShdw::B0x0,
            true => NrstShdw::B0x1,
        }
    }
    #[doc = "Reset generated when entering the Shutdown mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NrstShdw::B0x0
    }
    #[doc = "No reset generated when entering the Shutdown mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NrstShdw::B0x1
    }
}
#[doc = "Field `NRST_SHDW` writer - Reset generated when entering Shutdown mode"]
pub type NrstShdwW<'a, REG> = crate::BitWriter<'a, REG, NrstShdw>;
impl<'a, REG> NrstShdwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset generated when entering the Shutdown mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NrstShdw::B0x0)
    }
    #[doc = "No reset generated when entering the Shutdown mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NrstShdw::B0x1)
    }
}
#[doc = "Independent watchdog selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IwdgSw {
    #[doc = "0: Hardware independent watchdog"]
    B0x0 = 0,
    #[doc = "1: Software independent watchdog"]
    B0x1 = 1,
}
impl From<IwdgSw> for bool {
    #[inline(always)]
    fn from(variant: IwdgSw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_SW` reader - Independent watchdog selection"]
pub type IwdgSwR = crate::BitReader<IwdgSw>;
impl IwdgSwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IwdgSw {
        match self.bits {
            false => IwdgSw::B0x0,
            true => IwdgSw::B0x1,
        }
    }
    #[doc = "Hardware independent watchdog"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IwdgSw::B0x0
    }
    #[doc = "Software independent watchdog"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IwdgSw::B0x1
    }
}
#[doc = "Field `IWDG_SW` writer - Independent watchdog selection"]
pub type IwdgSwW<'a, REG> = crate::BitWriter<'a, REG, IwdgSw>;
impl<'a, REG> IwdgSwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware independent watchdog"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IwdgSw::B0x0)
    }
    #[doc = "Software independent watchdog"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IwdgSw::B0x1)
    }
}
#[doc = "Independent watchdog counter freeze in Stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IwdgStop {
    #[doc = "0: Independent watchdog counter is frozen in Stop mode"]
    B0x0 = 0,
    #[doc = "1: Independent watchdog counter is running in Stop mode"]
    B0x1 = 1,
}
impl From<IwdgStop> for bool {
    #[inline(always)]
    fn from(variant: IwdgStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub type IwdgStopR = crate::BitReader<IwdgStop>;
impl IwdgStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IwdgStop {
        match self.bits {
            false => IwdgStop::B0x0,
            true => IwdgStop::B0x1,
        }
    }
    #[doc = "Independent watchdog counter is frozen in Stop mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IwdgStop::B0x0
    }
    #[doc = "Independent watchdog counter is running in Stop mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IwdgStop::B0x1
    }
}
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub type IwdgStopW<'a, REG> = crate::BitWriter<'a, REG, IwdgStop>;
impl<'a, REG> IwdgStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Independent watchdog counter is frozen in Stop mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IwdgStop::B0x0)
    }
    #[doc = "Independent watchdog counter is running in Stop mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IwdgStop::B0x1)
    }
}
#[doc = "Independent watchdog counter freeze in Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IwdgStdby {
    #[doc = "0: Independent watchdog counter is frozen in Standby mode"]
    B0x0 = 0,
    #[doc = "1: Independent watchdog counter is running in Standby mode"]
    B0x1 = 1,
}
impl From<IwdgStdby> for bool {
    #[inline(always)]
    fn from(variant: IwdgStdby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode"]
pub type IwdgStdbyR = crate::BitReader<IwdgStdby>;
impl IwdgStdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IwdgStdby {
        match self.bits {
            false => IwdgStdby::B0x0,
            true => IwdgStdby::B0x1,
        }
    }
    #[doc = "Independent watchdog counter is frozen in Standby mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IwdgStdby::B0x0
    }
    #[doc = "Independent watchdog counter is running in Standby mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IwdgStdby::B0x1
    }
}
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode"]
pub type IwdgStdbyW<'a, REG> = crate::BitWriter<'a, REG, IwdgStdby>;
impl<'a, REG> IwdgStdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Independent watchdog counter is frozen in Standby mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IwdgStdby::B0x0)
    }
    #[doc = "Independent watchdog counter is running in Standby mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IwdgStdby::B0x1)
    }
}
#[doc = "Window watchdog selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WwdgSw {
    #[doc = "0: Hardware window watchdog"]
    B0x0 = 0,
    #[doc = "1: Software window watchdog"]
    B0x1 = 1,
}
impl From<WwdgSw> for bool {
    #[inline(always)]
    fn from(variant: WwdgSw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WwdgSwR = crate::BitReader<WwdgSw>;
impl WwdgSwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WwdgSw {
        match self.bits {
            false => WwdgSw::B0x0,
            true => WwdgSw::B0x1,
        }
    }
    #[doc = "Hardware window watchdog"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WwdgSw::B0x0
    }
    #[doc = "Software window watchdog"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WwdgSw::B0x1
    }
}
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WwdgSwW<'a, REG> = crate::BitWriter<'a, REG, WwdgSw>;
impl<'a, REG> WwdgSwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware window watchdog"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WwdgSw::B0x0)
    }
    #[doc = "Software window watchdog"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WwdgSw::B0x1)
    }
}
#[doc = "Backup domain reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bdrst {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<Bdrst> for bool {
    #[inline(always)]
    fn from(variant: Bdrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDRST` reader - Backup domain reset"]
pub type BdrstR = crate::BitReader<Bdrst>;
impl BdrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bdrst {
        match self.bits {
            false => Bdrst::B0x0,
            true => Bdrst::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bdrst::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Bdrst::B0x1
    }
}
#[doc = "Field `BDRST` writer - Backup domain reset"]
pub type BdrstW<'a, REG> = crate::BitWriter<'a, REG, Bdrst>;
impl<'a, REG> BdrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bdrst::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bdrst::B0x1)
    }
}
#[doc = "SRAM parity check control enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamParityCheck {
    #[doc = "0: Enable"]
    B0x0 = 0,
    #[doc = "1: Disable"]
    B0x1 = 1,
}
impl From<RamParityCheck> for bool {
    #[inline(always)]
    fn from(variant: RamParityCheck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_PARITY_CHECK` reader - SRAM parity check control enable/disable"]
pub type RamParityCheckR = crate::BitReader<RamParityCheck>;
impl RamParityCheckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamParityCheck {
        match self.bits {
            false => RamParityCheck::B0x0,
            true => RamParityCheck::B0x1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RamParityCheck::B0x0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RamParityCheck::B0x1
    }
}
#[doc = "Field `RAM_PARITY_CHECK` writer - SRAM parity check control enable/disable"]
pub type RamParityCheckW<'a, REG> = crate::BitWriter<'a, REG, RamParityCheck>;
impl<'a, REG> RamParityCheckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RamParityCheck::B0x0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RamParityCheck::B0x1)
    }
}
#[doc = "Backup SRAM erase prevention\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BkpsramHwEraseDisable {
    #[doc = "0: Disable"]
    B0x0 = 0,
    #[doc = "1: Enable"]
    B0x1 = 1,
}
impl From<BkpsramHwEraseDisable> for bool {
    #[inline(always)]
    fn from(variant: BkpsramHwEraseDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPSRAM_HW_ERASE_DISABLE` reader - Backup SRAM erase prevention"]
pub type BkpsramHwEraseDisableR = crate::BitReader<BkpsramHwEraseDisable>;
impl BkpsramHwEraseDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BkpsramHwEraseDisable {
        match self.bits {
            false => BkpsramHwEraseDisable::B0x0,
            true => BkpsramHwEraseDisable::B0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BkpsramHwEraseDisable::B0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BkpsramHwEraseDisable::B0x1
    }
}
#[doc = "Field `BKPSRAM_HW_ERASE_DISABLE` writer - Backup SRAM erase prevention"]
pub type BkpsramHwEraseDisableW<'a, REG> = crate::BitWriter<'a, REG, BkpsramHwEraseDisable>;
impl<'a, REG> BkpsramHwEraseDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BkpsramHwEraseDisable::B0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BkpsramHwEraseDisable::B0x1)
    }
}
#[doc = "BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NbootSel {
    #[doc = "0: BOOT0 pin (legacy mode)"]
    B0x0 = 0,
    #[doc = "1: NBOOT0 option bit"]
    B0x1 = 1,
}
impl From<NbootSel> for bool {
    #[inline(always)]
    fn from(variant: NbootSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NBOOT_SEL` reader - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal."]
pub type NbootSelR = crate::BitReader<NbootSel>;
impl NbootSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NbootSel {
        match self.bits {
            false => NbootSel::B0x0,
            true => NbootSel::B0x1,
        }
    }
    #[doc = "BOOT0 pin (legacy mode)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NbootSel::B0x0
    }
    #[doc = "NBOOT0 option bit"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NbootSel::B0x1
    }
}
#[doc = "Field `NBOOT_SEL` writer - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal."]
pub type NbootSelW<'a, REG> = crate::BitWriter<'a, REG, NbootSel>;
impl<'a, REG> NbootSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BOOT0 pin (legacy mode)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NbootSel::B0x0)
    }
    #[doc = "NBOOT0 option bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NbootSel::B0x1)
    }
}
#[doc = "Field `NBOOT1` reader - Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration."]
pub type Nboot1R = crate::BitReader;
#[doc = "Field `NBOOT1` writer - Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration."]
pub type Nboot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "NBOOT0 option bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nboot0 {
    #[doc = "0: NBOOT01=10"]
    B0x0 = 0,
    #[doc = "1: NBOOT01=11"]
    B0x1 = 1,
}
impl From<Nboot0> for bool {
    #[inline(always)]
    fn from(variant: Nboot0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NBOOT0` reader - NBOOT0 option bit"]
pub type Nboot0R = crate::BitReader<Nboot0>;
impl Nboot0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nboot0 {
        match self.bits {
            false => Nboot0::B0x0,
            true => Nboot0::B0x1,
        }
    }
    #[doc = "NBOOT01=10"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Nboot0::B0x0
    }
    #[doc = "NBOOT01=11"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Nboot0::B0x1
    }
}
#[doc = "Field `NBOOT0` writer - NBOOT0 option bit"]
pub type Nboot0W<'a, REG> = crate::BitWriter<'a, REG, Nboot0>;
impl<'a, REG> Nboot0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NBOOT01=10"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Nboot0::B0x0)
    }
    #[doc = "NBOOT01=11"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Nboot0::B0x1)
    }
}
#[doc = "NRST pin configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NrstMode {
    #[doc = "1: Reset input only: a low level on the NRST pin generates system reset; internal RESET is not propagated to the NRST pin."]
    B0x1 = 1,
    #[doc = "2: Standard GPIO: only internal RESET is possible"]
    B0x2 = 2,
    #[doc = "3: Bidirectional reset: the NRST pin is configured in reset input/output (legacy) mode"]
    B0x3 = 3,
}
impl From<NrstMode> for u8 {
    #[inline(always)]
    fn from(variant: NrstMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NrstMode {
    type Ux = u8;
}
impl crate::IsEnum for NrstMode {}
#[doc = "Field `NRST_MODE` reader - NRST pin configuration"]
pub type NrstModeR = crate::FieldReader<NrstMode>;
impl NrstModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NrstMode> {
        match self.bits {
            1 => Some(NrstMode::B0x1),
            2 => Some(NrstMode::B0x2),
            3 => Some(NrstMode::B0x3),
            _ => None,
        }
    }
    #[doc = "Reset input only: a low level on the NRST pin generates system reset; internal RESET is not propagated to the NRST pin."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NrstMode::B0x1
    }
    #[doc = "Standard GPIO: only internal RESET is possible"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == NrstMode::B0x2
    }
    #[doc = "Bidirectional reset: the NRST pin is configured in reset input/output (legacy) mode"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == NrstMode::B0x3
    }
}
#[doc = "Field `NRST_MODE` writer - NRST pin configuration"]
pub type NrstModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, NrstMode>;
impl<'a, REG> NrstModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset input only: a low level on the NRST pin generates system reset; internal RESET is not propagated to the NRST pin."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NrstMode::B0x1)
    }
    #[doc = "Standard GPIO: only internal RESET is possible"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(NrstMode::B0x2)
    }
    #[doc = "Bidirectional reset: the NRST pin is configured in reset input/output (legacy) mode"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(NrstMode::B0x3)
    }
}
#[doc = "Internal reset holder enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irhen {
    #[doc = "0: Internal resets are propagated as simple pulse on NRST pin"]
    B0x0 = 0,
    #[doc = "1: Internal resets drives NRST pin low until it is seen as low level"]
    B0x1 = 1,
}
impl From<Irhen> for bool {
    #[inline(always)]
    fn from(variant: Irhen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRHEN` reader - Internal reset holder enable bit"]
pub type IrhenR = crate::BitReader<Irhen>;
impl IrhenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irhen {
        match self.bits {
            false => Irhen::B0x0,
            true => Irhen::B0x1,
        }
    }
    #[doc = "Internal resets are propagated as simple pulse on NRST pin"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Irhen::B0x0
    }
    #[doc = "Internal resets drives NRST pin low until it is seen as low level"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Irhen::B0x1
    }
}
#[doc = "Field `IRHEN` writer - Internal reset holder enable bit"]
pub type IrhenW<'a, REG> = crate::BitWriter<'a, REG, Irhen>;
impl<'a, REG> IrhenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal resets are propagated as simple pulse on NRST pin"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Irhen::B0x0)
    }
    #[doc = "Internal resets drives NRST pin low until it is seen as low level"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Irhen::B0x1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Read protection level Other: Level 1, memories read protection active"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - BOR reset level"]
    #[inline(always)]
    pub fn borr_lev(&self) -> BorrLevR {
        BorrLevR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 13 - Reset generated when entering Stop mode"]
    #[inline(always)]
    pub fn nrst_stop(&self) -> NrstStopR {
        NrstStopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset generated when entering Standby mode"]
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NrstStdbyR {
        NrstStdbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset generated when entering Shutdown mode"]
    #[inline(always)]
    pub fn nrst_shdw(&self) -> NrstShdwR {
        NrstShdwR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IwdgSwR {
        IwdgSwR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IwdgStopR {
        IwdgStopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IwdgStdbyR {
        IwdgStdbyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WwdgSwR {
        WwdgSwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Backup domain reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BdrstR {
        BdrstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SRAM parity check control enable/disable"]
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RamParityCheckR {
        RamParityCheckR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Backup SRAM erase prevention"]
    #[inline(always)]
    pub fn bkpsram_hw_erase_disable(&self) -> BkpsramHwEraseDisableR {
        BkpsramHwEraseDisableR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal."]
    #[inline(always)]
    pub fn nboot_sel(&self) -> NbootSelR {
        NbootSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration."]
    #[inline(always)]
    pub fn nboot1(&self) -> Nboot1R {
        Nboot1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - NBOOT0 option bit"]
    #[inline(always)]
    pub fn nboot0(&self) -> Nboot0R {
        Nboot0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - NRST pin configuration"]
    #[inline(always)]
    pub fn nrst_mode(&self) -> NrstModeR {
        NrstModeR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Internal reset holder enable bit"]
    #[inline(always)]
    pub fn irhen(&self) -> IrhenR {
        IrhenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection level Other: Level 1, memories read protection active"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RdpW<FlashOptrSpec> {
        RdpW::new(self, 0)
    }
    #[doc = "Bits 8:10 - BOR reset level"]
    #[inline(always)]
    #[must_use]
    pub fn borr_lev(&mut self) -> BorrLevW<FlashOptrSpec> {
        BorrLevW::new(self, 8)
    }
    #[doc = "Bit 13 - Reset generated when entering Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stop(&mut self) -> NrstStopW<FlashOptrSpec> {
        NrstStopW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset generated when entering Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stdby(&mut self) -> NrstStdbyW<FlashOptrSpec> {
        NrstStdbyW::new(self, 14)
    }
    #[doc = "Bit 15 - Reset generated when entering Shutdown mode"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_shdw(&mut self) -> NrstShdwW<FlashOptrSpec> {
        NrstShdwW::new(self, 15)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IwdgSwW<FlashOptrSpec> {
        IwdgSwW::new(self, 16)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IwdgStopW<FlashOptrSpec> {
        IwdgStopW::new(self, 17)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IwdgStdbyW<FlashOptrSpec> {
        IwdgStdbyW::new(self, 18)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WwdgSwW<FlashOptrSpec> {
        WwdgSwW::new(self, 19)
    }
    #[doc = "Bit 21 - Backup domain reset"]
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BdrstW<FlashOptrSpec> {
        BdrstW::new(self, 21)
    }
    #[doc = "Bit 22 - SRAM parity check control enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn ram_parity_check(&mut self) -> RamParityCheckW<FlashOptrSpec> {
        RamParityCheckW::new(self, 22)
    }
    #[doc = "Bit 23 - Backup SRAM erase prevention"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsram_hw_erase_disable(&mut self) -> BkpsramHwEraseDisableW<FlashOptrSpec> {
        BkpsramHwEraseDisableW::new(self, 23)
    }
    #[doc = "Bit 24 - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal."]
    #[inline(always)]
    #[must_use]
    pub fn nboot_sel(&mut self) -> NbootSelW<FlashOptrSpec> {
        NbootSelW::new(self, 24)
    }
    #[doc = "Bit 25 - Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration."]
    #[inline(always)]
    #[must_use]
    pub fn nboot1(&mut self) -> Nboot1W<FlashOptrSpec> {
        Nboot1W::new(self, 25)
    }
    #[doc = "Bit 26 - NBOOT0 option bit"]
    #[inline(always)]
    #[must_use]
    pub fn nboot0(&mut self) -> Nboot0W<FlashOptrSpec> {
        Nboot0W::new(self, 26)
    }
    #[doc = "Bits 27:28 - NRST pin configuration"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_mode(&mut self) -> NrstModeW<FlashOptrSpec> {
        NrstModeW::new(self, 27)
    }
    #[doc = "Bit 29 - Internal reset holder enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn irhen(&mut self) -> IrhenW<FlashOptrSpec> {
        IrhenW::new(self, 29)
    }
}
#[doc = "FLASH option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_optr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_optr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashOptrSpec;
impl crate::RegisterSpec for FlashOptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_optr::R`](R) reader structure"]
impl crate::Readable for FlashOptrSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_optr::W`](W) writer structure"]
impl crate::Writable for FlashOptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_OPTR to value 0"]
impl crate::Resettable for FlashOptrSpec {
    const RESET_VALUE: u32 = 0;
}
