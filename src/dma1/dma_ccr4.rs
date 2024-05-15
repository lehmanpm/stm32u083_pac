#[doc = "Register `DMA_CCR4` reader"]
pub type R = crate::R<DmaCcr4Spec>;
#[doc = "Register `DMA_CCR4` writer"]
pub type W = crate::W<DmaCcr4Spec>;
#[doc = "Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by1setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enabled"]
    B0x1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by1setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::B0x0,
            true => En::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == En::B0x0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == En::B0x1
    }
}
#[doc = "Field `EN` writer - Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by1setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(En::B0x0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(En::B0x1)
    }
}
#[doc = "Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enabled"]
    B0x1 = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type TcieR = crate::BitReader<Tcie>;
impl TcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcie {
        match self.bits {
            false => Tcie::B0x0,
            true => Tcie::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tcie::B0x0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tcie::B0x1
    }
}
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0x1)
    }
}
#[doc = "Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htie {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enabled"]
    B0x1 = 1,
}
impl From<Htie> for bool {
    #[inline(always)]
    fn from(variant: Htie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIE` reader - Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type HtieR = crate::BitReader<Htie>;
impl HtieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htie {
        match self.bits {
            false => Htie::B0x0,
            true => Htie::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Htie::B0x0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Htie::B0x1
    }
}
#[doc = "Field `HTIE` writer - Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type HtieW<'a, REG> = crate::BitWriter<'a, REG, Htie>;
impl<'a, REG> HtieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Htie::B0x0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Htie::B0x1)
    }
}
#[doc = "Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teie {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enabled"]
    B0x1 = 1,
}
impl From<Teie> for bool {
    #[inline(always)]
    fn from(variant: Teie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type TeieR = crate::BitReader<Teie>;
impl TeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teie {
        match self.bits {
            false => Teie::B0x0,
            true => Teie::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Teie::B0x0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Teie::B0x1
    }
}
#[doc = "Field `TEIE` writer - Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG, Teie>;
impl<'a, REG> TeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::B0x0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::B0x1)
    }
}
#[doc = "Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Read from peripheral"]
    B0x0 = 0,
    #[doc = "1: Read from memory"]
    B0x1 = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::B0x0,
            true => Dir::B0x1,
        }
    }
    #[doc = "Read from peripheral"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dir::B0x0
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dir::B0x1
    }
}
#[doc = "Field `DIR` writer - Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from peripheral"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::B0x0)
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::B0x1)
    }
}
#[doc = "Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Circ {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enabled"]
    B0x1 = 1,
}
impl From<Circ> for bool {
    #[inline(always)]
    fn from(variant: Circ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIRC` reader - Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type CircR = crate::BitReader<Circ>;
impl CircR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Circ {
        match self.bits {
            false => Circ::B0x0,
            true => Circ::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Circ::B0x0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Circ::B0x1
    }
}
#[doc = "Field `CIRC` writer - Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type CircW<'a, REG> = crate::BitWriter<'a, REG, Circ>;
impl<'a, REG> CircW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Circ::B0x0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Circ::B0x1)
    }
}
#[doc = "Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pinc {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enabled"]
    B0x1 = 1,
}
impl From<Pinc> for bool {
    #[inline(always)]
    fn from(variant: Pinc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINC` reader - Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type PincR = crate::BitReader<Pinc>;
impl PincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pinc {
        match self.bits {
            false => Pinc::B0x0,
            true => Pinc::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pinc::B0x0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pinc::B0x1
    }
}
#[doc = "Field `PINC` writer - Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type PincW<'a, REG> = crate::BitWriter<'a, REG, Pinc>;
impl<'a, REG> PincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pinc::B0x0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pinc::B0x1)
    }
}
#[doc = "Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Minc {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enabled"]
    B0x1 = 1,
}
impl From<Minc> for bool {
    #[inline(always)]
    fn from(variant: Minc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MINC` reader - Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type MincR = crate::BitReader<Minc>;
impl MincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Minc {
        match self.bits {
            false => Minc::B0x0,
            true => Minc::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Minc::B0x0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Minc::B0x1
    }
}
#[doc = "Field `MINC` writer - Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type MincW<'a, REG> = crate::BitWriter<'a, REG, Minc>;
impl<'a, REG> MincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Minc::B0x0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Minc::B0x1)
    }
}
#[doc = "Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psize {
    #[doc = "0: 8 bits"]
    B0x0 = 0,
    #[doc = "1: 16 bits"]
    B0x1 = 1,
    #[doc = "2: 32 bits"]
    B0x2 = 2,
}
impl From<Psize> for u8 {
    #[inline(always)]
    fn from(variant: Psize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psize {
    type Ux = u8;
}
impl crate::IsEnum for Psize {}
#[doc = "Field `PSIZE` reader - Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type PsizeR = crate::FieldReader<Psize>;
impl PsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Psize> {
        match self.bits {
            0 => Some(Psize::B0x0),
            1 => Some(Psize::B0x1),
            2 => Some(Psize::B0x2),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Psize::B0x0
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Psize::B0x1
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Psize::B0x2
    }
}
#[doc = "Field `PSIZE` writer - Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type PsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Psize>;
impl<'a, REG> PsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::B0x0)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::B0x1)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::B0x2)
    }
}
#[doc = "Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msize {
    #[doc = "0: 8 bits"]
    B0x0 = 0,
    #[doc = "1: 16 bits"]
    B0x1 = 1,
    #[doc = "2: 32 bits"]
    B0x2 = 2,
}
impl From<Msize> for u8 {
    #[inline(always)]
    fn from(variant: Msize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msize {
    type Ux = u8;
}
impl crate::IsEnum for Msize {}
#[doc = "Field `MSIZE` reader - Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type MsizeR = crate::FieldReader<Msize>;
impl MsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Msize> {
        match self.bits {
            0 => Some(Msize::B0x0),
            1 => Some(Msize::B0x1),
            2 => Some(Msize::B0x2),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Msize::B0x0
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Msize::B0x1
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Msize::B0x2
    }
}
#[doc = "Field `MSIZE` writer - Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type MsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Msize>;
impl<'a, REG> MsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Msize::B0x0)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Msize::B0x1)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Msize::B0x2)
    }
}
#[doc = "Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pl {
    #[doc = "0: Low"]
    B0x0 = 0,
    #[doc = "1: Medium"]
    B0x1 = 1,
    #[doc = "2: High"]
    B0x2 = 2,
    #[doc = "3: Very high"]
    B0x3 = 3,
}
impl From<Pl> for u8 {
    #[inline(always)]
    fn from(variant: Pl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pl {
    type Ux = u8;
}
impl crate::IsEnum for Pl {}
#[doc = "Field `PL` reader - Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type PlR = crate::FieldReader<Pl>;
impl PlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pl {
        match self.bits {
            0 => Pl::B0x0,
            1 => Pl::B0x1,
            2 => Pl::B0x2,
            3 => Pl::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pl::B0x0
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pl::B0x1
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Pl::B0x2
    }
    #[doc = "Very high"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Pl::B0x3
    }
}
#[doc = "Field `PL` writer - Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pl, crate::Safe>;
impl<'a, REG> PlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pl::B0x0)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pl::B0x1)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Pl::B0x2)
    }
    #[doc = "Very high"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Pl::B0x3)
    }
}
#[doc = "Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mem2mem {
    #[doc = "0: Disabled"]
    B0x0 = 0,
    #[doc = "1: Enabled"]
    B0x1 = 1,
}
impl From<Mem2mem> for bool {
    #[inline(always)]
    fn from(variant: Mem2mem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM2MEM` reader - Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type Mem2memR = crate::BitReader<Mem2mem>;
impl Mem2memR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mem2mem {
        match self.bits {
            false => Mem2mem::B0x0,
            true => Mem2mem::B0x1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mem2mem::B0x0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mem2mem::B0x1
    }
}
#[doc = "Field `MEM2MEM` writer - Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
pub type Mem2memW<'a, REG> = crate::BitWriter<'a, REG, Mem2mem>;
impl<'a, REG> Mem2memW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mem2mem::B0x0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mem2mem::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by1setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn htie(&self) -> HtieR {
        HtieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn circ(&self) -> CircR {
        CircR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn pinc(&self) -> PincR {
        PincR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn minc(&self) -> MincR {
        MincR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn psize(&self) -> PsizeR {
        PsizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn msize(&self) -> MsizeR {
        MsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn mem2mem(&self) -> Mem2memR {
        Mem2memR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by1setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<DmaCcr4Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<DmaCcr4Spec> {
        TcieW::new(self, 1)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HtieW<DmaCcr4Spec> {
        HtieW::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TeieW<DmaCcr4Spec> {
        TeieW::new(self, 3)
    }
    #[doc = "Bit 4 - Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<DmaCcr4Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bit 5 - Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CircW<DmaCcr4Spec> {
        CircW::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PincW<DmaCcr4Spec> {
        PincW::new(self, 6)
    }
    #[doc = "Bit 7 - Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MincW<DmaCcr4Spec> {
        MincW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PsizeW<DmaCcr4Spec> {
        PsizeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MsizeW<DmaCcr4Spec> {
        MsizeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PlW<DmaCcr4Spec> {
        PlW::new(self, 12)
    }
    #[doc = "Bit 14 - Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> Mem2memW<DmaCcr4Spec> {
        Mem2memW::new(self, 14)
    }
}
#[doc = "DMA channel 4 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCcr4Spec;
impl crate::RegisterSpec for DmaCcr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ccr4::R`](R) reader structure"]
impl crate::Readable for DmaCcr4Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_ccr4::W`](W) writer structure"]
impl crate::Writable for DmaCcr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CCR4 to value 0"]
impl crate::Resettable for DmaCcr4Spec {
    const RESET_VALUE: u32 = 0;
}
