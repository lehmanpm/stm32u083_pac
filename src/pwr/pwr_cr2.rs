#[doc = "Register `PWR_CR2` reader"]
pub type R = crate::R<PwrCr2Spec>;
#[doc = "Register `PWR_CR2` writer"]
pub type W = crate::W<PwrCr2Spec>;
#[doc = "Programmable voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvde {
    #[doc = "0: Programmable voltage detector disable."]
    B0x0 = 0,
    #[doc = "1: Programmable voltage detector enable."]
    B0x1 = 1,
}
impl From<Pvde> for bool {
    #[inline(always)]
    fn from(variant: Pvde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDE` reader - Programmable voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset."]
pub type PvdeR = crate::BitReader<Pvde>;
impl PvdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvde {
        match self.bits {
            false => Pvde::B0x0,
            true => Pvde::B0x1,
        }
    }
    #[doc = "Programmable voltage detector disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvde::B0x0
    }
    #[doc = "Programmable voltage detector enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvde::B0x1
    }
}
#[doc = "Field `PVDE` writer - Programmable voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset."]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG, Pvde>;
impl<'a, REG> PvdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable voltage detector disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pvde::B0x0)
    }
    #[doc = "Programmable voltage detector enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pvde::B0x1)
    }
}
#[doc = "Programmable voltage detector level selection. These bits select the voltage threshold detected by the programmable voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pls {
    #[doc = "0: V&lt;sub>PVD0&lt;/sub> around 2.01V"]
    B0x0 = 0,
    #[doc = "1: V&lt;sub>PVD1&lt;/sub> around 2.21V"]
    B0x1 = 1,
    #[doc = "2: V&lt;sub>PVD2&lt;/sub> around 2.41V"]
    B0x2 = 2,
    #[doc = "3: V&lt;sub>PVD3&lt;/sub> around 2.51V"]
    B0x3 = 3,
    #[doc = "4: V&lt;sub>PVD4&lt;/sub> around 2.61V"]
    B0x4 = 4,
    #[doc = "5: V&lt;sub>PVD5&lt;/sub> around 2.81V"]
    B0x5 = 5,
    #[doc = "6: V&lt;sub>PVD6&lt;/sub> around 2.91V"]
    B0x6 = 6,
    #[doc = "7: External input analog voltage PVD_IN (compared internally to VREFINT)"]
    B0x7 = 7,
}
impl From<Pls> for u8 {
    #[inline(always)]
    fn from(variant: Pls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pls {
    type Ux = u8;
}
impl crate::IsEnum for Pls {}
#[doc = "Field `PLS` reader - Programmable voltage detector level selection. These bits select the voltage threshold detected by the programmable voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset."]
pub type PlsR = crate::FieldReader<Pls>;
impl PlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pls {
        match self.bits {
            0 => Pls::B0x0,
            1 => Pls::B0x1,
            2 => Pls::B0x2,
            3 => Pls::B0x3,
            4 => Pls::B0x4,
            5 => Pls::B0x5,
            6 => Pls::B0x6,
            7 => Pls::B0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "V&lt;sub>PVD0&lt;/sub> around 2.01V"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pls::B0x0
    }
    #[doc = "V&lt;sub>PVD1&lt;/sub> around 2.21V"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pls::B0x1
    }
    #[doc = "V&lt;sub>PVD2&lt;/sub> around 2.41V"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pls::B0x2
    }
    #[doc = "V&lt;sub>PVD3&lt;/sub> around 2.51V"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pls::B0x3
    }
    #[doc = "V&lt;sub>PVD4&lt;/sub> around 2.61V"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Pls::B0x4
    }
    #[doc = "V&lt;sub>PVD5&lt;/sub> around 2.81V"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Pls::B0x5
    }
    #[doc = "V&lt;sub>PVD6&lt;/sub> around 2.91V"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Pls::B0x6
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Pls::B0x7
    }
}
#[doc = "Field `PLS` writer - Programmable voltage detector level selection. These bits select the voltage threshold detected by the programmable voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset."]
pub type PlsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pls, crate::Safe>;
impl<'a, REG> PlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "V&lt;sub>PVD0&lt;/sub> around 2.01V"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pls::B0x0)
    }
    #[doc = "V&lt;sub>PVD1&lt;/sub> around 2.21V"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pls::B0x1)
    }
    #[doc = "V&lt;sub>PVD2&lt;/sub> around 2.41V"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pls::B0x2)
    }
    #[doc = "V&lt;sub>PVD3&lt;/sub> around 2.51V"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pls::B0x3)
    }
    #[doc = "V&lt;sub>PVD4&lt;/sub> around 2.61V"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Pls::B0x4)
    }
    #[doc = "V&lt;sub>PVD5&lt;/sub> around 2.81V"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Pls::B0x5)
    }
    #[doc = "V&lt;sub>PVD6&lt;/sub> around 2.91V"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Pls::B0x6)
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Pls::B0x7)
    }
}
#[doc = "Peripheral voltage monitoring 1 enable: V&lt;sub>DDUSB&lt;/sub> vs. 1.21V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvme1 {
    #[doc = "0: PVM1 (V&lt;sub>DDUSB&lt;/sub> monitoring vs. 1.21V threshold) disable."]
    B0x0 = 0,
    #[doc = "1: PVM1 (V&lt;sub>DDUSB&lt;/sub> monitoring vs. 1.21V threshold) enable."]
    B0x1 = 1,
}
impl From<Pvme1> for bool {
    #[inline(always)]
    fn from(variant: Pvme1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVME1` reader - Peripheral voltage monitoring 1 enable: V&lt;sub>DDUSB&lt;/sub> vs. 1.21V"]
pub type Pvme1R = crate::BitReader<Pvme1>;
impl Pvme1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvme1 {
        match self.bits {
            false => Pvme1::B0x0,
            true => Pvme1::B0x1,
        }
    }
    #[doc = "PVM1 (V&lt;sub>DDUSB&lt;/sub> monitoring vs. 1.21V threshold) disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvme1::B0x0
    }
    #[doc = "PVM1 (V&lt;sub>DDUSB&lt;/sub> monitoring vs. 1.21V threshold) enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvme1::B0x1
    }
}
#[doc = "Field `PVME1` writer - Peripheral voltage monitoring 1 enable: V&lt;sub>DDUSB&lt;/sub> vs. 1.21V"]
pub type Pvme1W<'a, REG> = crate::BitWriter<'a, REG, Pvme1>;
impl<'a, REG> Pvme1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVM1 (V&lt;sub>DDUSB&lt;/sub> monitoring vs. 1.21V threshold) disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pvme1::B0x0)
    }
    #[doc = "PVM1 (V&lt;sub>DDUSB&lt;/sub> monitoring vs. 1.21V threshold) enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pvme1::B0x1)
    }
}
#[doc = "Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. 1.621V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvme3 {
    #[doc = "0: PVM3 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.621V threshold) disable."]
    B0x0 = 0,
    #[doc = "1: PVM3 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.621V threshold) enable."]
    B0x1 = 1,
}
impl From<Pvme3> for bool {
    #[inline(always)]
    fn from(variant: Pvme3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVME3` reader - Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. 1.621V"]
pub type Pvme3R = crate::BitReader<Pvme3>;
impl Pvme3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvme3 {
        match self.bits {
            false => Pvme3::B0x0,
            true => Pvme3::B0x1,
        }
    }
    #[doc = "PVM3 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.621V threshold) disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvme3::B0x0
    }
    #[doc = "PVM3 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.621V threshold) enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvme3::B0x1
    }
}
#[doc = "Field `PVME3` writer - Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. 1.621V"]
pub type Pvme3W<'a, REG> = crate::BitWriter<'a, REG, Pvme3>;
impl<'a, REG> Pvme3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVM3 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.621V threshold) disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pvme3::B0x0)
    }
    #[doc = "PVM3 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.621V threshold) enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pvme3::B0x1)
    }
}
#[doc = "Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. 1.861V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvme4 {
    #[doc = "0: PVM4 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.861V threshold) disable."]
    B0x0 = 0,
    #[doc = "1: PVM4 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.86 V threshold) enable."]
    B0x1 = 1,
}
impl From<Pvme4> for bool {
    #[inline(always)]
    fn from(variant: Pvme4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVME4` reader - Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. 1.861V"]
pub type Pvme4R = crate::BitReader<Pvme4>;
impl Pvme4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvme4 {
        match self.bits {
            false => Pvme4::B0x0,
            true => Pvme4::B0x1,
        }
    }
    #[doc = "PVM4 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.861V threshold) disable."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pvme4::B0x0
    }
    #[doc = "PVM4 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.86 V threshold) enable."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pvme4::B0x1
    }
}
#[doc = "Field `PVME4` writer - Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. 1.861V"]
pub type Pvme4W<'a, REG> = crate::BitWriter<'a, REG, Pvme4>;
impl<'a, REG> Pvme4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVM4 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.861V threshold) disable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pvme4::B0x0)
    }
    #[doc = "PVM4 (V&lt;sub>DDA&lt;/sub> monitoring vs. 1.86 V threshold) enable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pvme4::B0x1)
    }
}
#[doc = "V&lt;sub>DDUSB&lt;/sub> USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB FS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usv {
    #[doc = "0: V&lt;sub>DDUSB&lt;/sub> is not present. Logical and electrical isolation is applied to ignore this supply."]
    B0x0 = 0,
    #[doc = "1: V&lt;sub>DDUSB&lt;/sub> is valid."]
    B0x1 = 1,
}
impl From<Usv> for bool {
    #[inline(always)]
    fn from(variant: Usv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USV` reader - V&lt;sub>DDUSB&lt;/sub> USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB FS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not."]
pub type UsvR = crate::BitReader<Usv>;
impl UsvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usv {
        match self.bits {
            false => Usv::B0x0,
            true => Usv::B0x1,
        }
    }
    #[doc = "V&lt;sub>DDUSB&lt;/sub> is not present. Logical and electrical isolation is applied to ignore this supply."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usv::B0x0
    }
    #[doc = "V&lt;sub>DDUSB&lt;/sub> is valid."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usv::B0x1
    }
}
#[doc = "Field `USV` writer - V&lt;sub>DDUSB&lt;/sub> USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB FS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not."]
pub type UsvW<'a, REG> = crate::BitWriter<'a, REG, Usv>;
impl<'a, REG> UsvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V&lt;sub>DDUSB&lt;/sub> is not present. Logical and electrical isolation is applied to ignore this supply."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usv::B0x0)
    }
    #[doc = "V&lt;sub>DDUSB&lt;/sub> is valid."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usv::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Programmable voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset."]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Programmable voltage detector level selection. These bits select the voltage threshold detected by the programmable voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset."]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: V&lt;sub>DDUSB&lt;/sub> vs. 1.21V"]
    #[inline(always)]
    pub fn pvme1(&self) -> Pvme1R {
        Pvme1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. 1.621V"]
    #[inline(always)]
    pub fn pvme3(&self) -> Pvme3R {
        Pvme3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. 1.861V"]
    #[inline(always)]
    pub fn pvme4(&self) -> Pvme4R {
        Pvme4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - V&lt;sub>DDUSB&lt;/sub> USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB FS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not."]
    #[inline(always)]
    pub fn usv(&self) -> UsvR {
        UsvR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programmable voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PvdeW<PwrCr2Spec> {
        PvdeW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Programmable voltage detector level selection. These bits select the voltage threshold detected by the programmable voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PlsW<PwrCr2Spec> {
        PlsW::new(self, 1)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: V&lt;sub>DDUSB&lt;/sub> vs. 1.21V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme1(&mut self) -> Pvme1W<PwrCr2Spec> {
        Pvme1W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 3 enable: V&lt;sub>DDA&lt;/sub> vs. 1.621V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme3(&mut self) -> Pvme3W<PwrCr2Spec> {
        Pvme3W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 4 enable: V&lt;sub>DDA&lt;/sub> vs. 1.861V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme4(&mut self) -> Pvme4W<PwrCr2Spec> {
        Pvme4W::new(self, 6)
    }
    #[doc = "Bit 10 - V&lt;sub>DDUSB&lt;/sub> USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB FS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not."]
    #[inline(always)]
    #[must_use]
    pub fn usv(&mut self) -> UsvW<PwrCr2Spec> {
        UsvW::new(self, 10)
    }
}
#[doc = "Power control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrCr2Spec;
impl crate::RegisterSpec for PwrCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr2::R`](R) reader structure"]
impl crate::Readable for PwrCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr2::W`](W) writer structure"]
impl crate::Writable for PwrCr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR2 to value 0"]
impl crate::Resettable for PwrCr2Spec {
    const RESET_VALUE: u32 = 0;
}
