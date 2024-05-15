#[doc = "Register `TSC_IOGCSR` reader"]
pub type R = crate::R<TscIogcsrSpec>;
#[doc = "Register `TSC_IOGCSR` writer"]
pub type W = crate::W<TscIogcsrSpec>;
#[doc = "Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1e {
    #[doc = "0: Acquisition on analog I/O group x disabled"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x enabled"]
    B0x1 = 1,
}
impl From<G1e> for bool {
    #[inline(always)]
    fn from(variant: G1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G1E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G1eR = crate::BitReader<G1e>;
impl G1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G1e {
        match self.bits {
            false => G1e::B0x0,
            true => G1e::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G1e::B0x0
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G1e::B0x1
    }
}
#[doc = "Field `G1E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G1eW<'a, REG> = crate::BitWriter<'a, REG, G1e>;
impl<'a, REG> G1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G1e::B0x0)
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G1e::B0x1)
    }
}
#[doc = "Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G2e {
    #[doc = "0: Acquisition on analog I/O group x disabled"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x enabled"]
    B0x1 = 1,
}
impl From<G2e> for bool {
    #[inline(always)]
    fn from(variant: G2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G2E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G2eR = crate::BitReader<G2e>;
impl G2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G2e {
        match self.bits {
            false => G2e::B0x0,
            true => G2e::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G2e::B0x0
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G2e::B0x1
    }
}
#[doc = "Field `G2E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G2eW<'a, REG> = crate::BitWriter<'a, REG, G2e>;
impl<'a, REG> G2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G2e::B0x0)
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G2e::B0x1)
    }
}
#[doc = "Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G3e {
    #[doc = "0: Acquisition on analog I/O group x disabled"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x enabled"]
    B0x1 = 1,
}
impl From<G3e> for bool {
    #[inline(always)]
    fn from(variant: G3e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G3E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G3eR = crate::BitReader<G3e>;
impl G3eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G3e {
        match self.bits {
            false => G3e::B0x0,
            true => G3e::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G3e::B0x0
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G3e::B0x1
    }
}
#[doc = "Field `G3E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G3eW<'a, REG> = crate::BitWriter<'a, REG, G3e>;
impl<'a, REG> G3eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G3e::B0x0)
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G3e::B0x1)
    }
}
#[doc = "Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G4e {
    #[doc = "0: Acquisition on analog I/O group x disabled"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x enabled"]
    B0x1 = 1,
}
impl From<G4e> for bool {
    #[inline(always)]
    fn from(variant: G4e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G4E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G4eR = crate::BitReader<G4e>;
impl G4eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G4e {
        match self.bits {
            false => G4e::B0x0,
            true => G4e::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G4e::B0x0
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G4e::B0x1
    }
}
#[doc = "Field `G4E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G4eW<'a, REG> = crate::BitWriter<'a, REG, G4e>;
impl<'a, REG> G4eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G4e::B0x0)
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G4e::B0x1)
    }
}
#[doc = "Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G5e {
    #[doc = "0: Acquisition on analog I/O group x disabled"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x enabled"]
    B0x1 = 1,
}
impl From<G5e> for bool {
    #[inline(always)]
    fn from(variant: G5e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G5E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G5eR = crate::BitReader<G5e>;
impl G5eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G5e {
        match self.bits {
            false => G5e::B0x0,
            true => G5e::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G5e::B0x0
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G5e::B0x1
    }
}
#[doc = "Field `G5E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G5eW<'a, REG> = crate::BitWriter<'a, REG, G5e>;
impl<'a, REG> G5eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G5e::B0x0)
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G5e::B0x1)
    }
}
#[doc = "Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G6e {
    #[doc = "0: Acquisition on analog I/O group x disabled"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x enabled"]
    B0x1 = 1,
}
impl From<G6e> for bool {
    #[inline(always)]
    fn from(variant: G6e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G6E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G6eR = crate::BitReader<G6e>;
impl G6eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G6e {
        match self.bits {
            false => G6e::B0x0,
            true => G6e::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G6e::B0x0
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G6e::B0x1
    }
}
#[doc = "Field `G6E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G6eW<'a, REG> = crate::BitWriter<'a, REG, G6e>;
impl<'a, REG> G6eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G6e::B0x0)
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G6e::B0x1)
    }
}
#[doc = "Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G7e {
    #[doc = "0: Acquisition on analog I/O group x disabled"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x enabled"]
    B0x1 = 1,
}
impl From<G7e> for bool {
    #[inline(always)]
    fn from(variant: G7e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G7E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G7eR = crate::BitReader<G7e>;
impl G7eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G7e {
        match self.bits {
            false => G7e::B0x0,
            true => G7e::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G7e::B0x0
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G7e::B0x1
    }
}
#[doc = "Field `G7E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
pub type G7eW<'a, REG> = crate::BitWriter<'a, REG, G7e>;
impl<'a, REG> G7eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Acquisition on analog I/O group x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G7e::B0x0)
    }
    #[doc = "Acquisition on analog I/O group x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G7e::B0x1)
    }
}
#[doc = "Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1s {
    #[doc = "0: Acquisition on analog I/O group x is ongoing or not started"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x is complete"]
    B0x1 = 1,
}
impl From<G1s> for bool {
    #[inline(always)]
    fn from(variant: G1s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G1S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
pub type G1sR = crate::BitReader<G1s>;
impl G1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G1s {
        match self.bits {
            false => G1s::B0x0,
            true => G1s::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x is ongoing or not started"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G1s::B0x0
    }
    #[doc = "Acquisition on analog I/O group x is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G1s::B0x1
    }
}
#[doc = "Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G2s {
    #[doc = "0: Acquisition on analog I/O group x is ongoing or not started"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x is complete"]
    B0x1 = 1,
}
impl From<G2s> for bool {
    #[inline(always)]
    fn from(variant: G2s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G2S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
pub type G2sR = crate::BitReader<G2s>;
impl G2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G2s {
        match self.bits {
            false => G2s::B0x0,
            true => G2s::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x is ongoing or not started"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G2s::B0x0
    }
    #[doc = "Acquisition on analog I/O group x is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G2s::B0x1
    }
}
#[doc = "Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G3s {
    #[doc = "0: Acquisition on analog I/O group x is ongoing or not started"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x is complete"]
    B0x1 = 1,
}
impl From<G3s> for bool {
    #[inline(always)]
    fn from(variant: G3s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G3S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
pub type G3sR = crate::BitReader<G3s>;
impl G3sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G3s {
        match self.bits {
            false => G3s::B0x0,
            true => G3s::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x is ongoing or not started"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G3s::B0x0
    }
    #[doc = "Acquisition on analog I/O group x is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G3s::B0x1
    }
}
#[doc = "Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G4s {
    #[doc = "0: Acquisition on analog I/O group x is ongoing or not started"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x is complete"]
    B0x1 = 1,
}
impl From<G4s> for bool {
    #[inline(always)]
    fn from(variant: G4s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G4S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
pub type G4sR = crate::BitReader<G4s>;
impl G4sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G4s {
        match self.bits {
            false => G4s::B0x0,
            true => G4s::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x is ongoing or not started"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G4s::B0x0
    }
    #[doc = "Acquisition on analog I/O group x is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G4s::B0x1
    }
}
#[doc = "Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G5s {
    #[doc = "0: Acquisition on analog I/O group x is ongoing or not started"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x is complete"]
    B0x1 = 1,
}
impl From<G5s> for bool {
    #[inline(always)]
    fn from(variant: G5s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G5S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
pub type G5sR = crate::BitReader<G5s>;
impl G5sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G5s {
        match self.bits {
            false => G5s::B0x0,
            true => G5s::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x is ongoing or not started"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G5s::B0x0
    }
    #[doc = "Acquisition on analog I/O group x is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G5s::B0x1
    }
}
#[doc = "Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G6s {
    #[doc = "0: Acquisition on analog I/O group x is ongoing or not started"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x is complete"]
    B0x1 = 1,
}
impl From<G6s> for bool {
    #[inline(always)]
    fn from(variant: G6s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G6S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
pub type G6sR = crate::BitReader<G6s>;
impl G6sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G6s {
        match self.bits {
            false => G6s::B0x0,
            true => G6s::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x is ongoing or not started"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G6s::B0x0
    }
    #[doc = "Acquisition on analog I/O group x is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G6s::B0x1
    }
}
#[doc = "Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G7s {
    #[doc = "0: Acquisition on analog I/O group x is ongoing or not started"]
    B0x0 = 0,
    #[doc = "1: Acquisition on analog I/O group x is complete"]
    B0x1 = 1,
}
impl From<G7s> for bool {
    #[inline(always)]
    fn from(variant: G7s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G7S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
pub type G7sR = crate::BitReader<G7s>;
impl G7sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G7s {
        match self.bits {
            false => G7s::B0x0,
            true => G7s::B0x1,
        }
    }
    #[doc = "Acquisition on analog I/O group x is ongoing or not started"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G7s::B0x0
    }
    #[doc = "Acquisition on analog I/O group x is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G7s::B0x1
    }
}
impl R {
    #[doc = "Bit 0 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    pub fn g1e(&self) -> G1eR {
        G1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    pub fn g2e(&self) -> G2eR {
        G2eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    pub fn g3e(&self) -> G3eR {
        G3eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    pub fn g4e(&self) -> G4eR {
        G4eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    pub fn g5e(&self) -> G5eR {
        G5eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    pub fn g6e(&self) -> G6eR {
        G6eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    pub fn g7e(&self) -> G7eR {
        G7eR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
    #[inline(always)]
    pub fn g1s(&self) -> G1sR {
        G1sR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
    #[inline(always)]
    pub fn g2s(&self) -> G2sR {
        G2sR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
    #[inline(always)]
    pub fn g3s(&self) -> G3sR {
        G3sR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
    #[inline(always)]
    pub fn g4s(&self) -> G4sR {
        G4sR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
    #[inline(always)]
    pub fn g5s(&self) -> G5sR {
        G5sR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
    #[inline(always)]
    pub fn g6s(&self) -> G6sR {
        G6sR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set."]
    #[inline(always)]
    pub fn g7s(&self) -> G7sR {
        G7sR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    #[must_use]
    pub fn g1e(&mut self) -> G1eW<TscIogcsrSpec> {
        G1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    #[must_use]
    pub fn g2e(&mut self) -> G2eW<TscIogcsrSpec> {
        G2eW::new(self, 1)
    }
    #[doc = "Bit 2 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    #[must_use]
    pub fn g3e(&mut self) -> G3eW<TscIogcsrSpec> {
        G3eW::new(self, 2)
    }
    #[doc = "Bit 3 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    #[must_use]
    pub fn g4e(&mut self) -> G4eW<TscIogcsrSpec> {
        G4eW::new(self, 3)
    }
    #[doc = "Bit 4 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    #[must_use]
    pub fn g5e(&mut self) -> G5eW<TscIogcsrSpec> {
        G5eW::new(self, 4)
    }
    #[doc = "Bit 5 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    #[must_use]
    pub fn g6e(&mut self) -> G6eW<TscIogcsrSpec> {
        G6eW::new(self, 5)
    }
    #[doc = "Bit 6 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x."]
    #[inline(always)]
    #[must_use]
    pub fn g7e(&mut self) -> G7eW<TscIogcsrSpec> {
        G7eW::new(self, 6)
    }
}
#[doc = "TSC I/O group control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_iogcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_iogcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscIogcsrSpec;
impl crate::RegisterSpec for TscIogcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsc_iogcsr::R`](R) reader structure"]
impl crate::Readable for TscIogcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsc_iogcsr::W`](W) writer structure"]
impl crate::Writable for TscIogcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSC_IOGCSR to value 0"]
impl crate::Resettable for TscIogcsrSpec {
    const RESET_VALUE: u32 = 0;
}
