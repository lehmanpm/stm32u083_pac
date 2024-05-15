#[doc = "Register `TSC_IOSCR` reader"]
pub type R = crate::R<TscIoscrSpec>;
#[doc = "Register `TSC_IOSCR` writer"]
pub type W = crate::W<TscIoscrSpec>;
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1Io1 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G1Io1> for bool {
    #[inline(always)]
    fn from(variant: G1Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G1_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G1Io1R = crate::BitReader<G1Io1>;
impl G1Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G1Io1 {
        match self.bits {
            false => G1Io1::B0x0,
            true => G1Io1::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G1Io1::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G1Io1::B0x1
    }
}
#[doc = "Field `G1_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G1Io1W<'a, REG> = crate::BitWriter<'a, REG, G1Io1>;
impl<'a, REG> G1Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G1Io1::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G1Io1::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1Io2 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G1Io2> for bool {
    #[inline(always)]
    fn from(variant: G1Io2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G1_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G1Io2R = crate::BitReader<G1Io2>;
impl G1Io2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G1Io2 {
        match self.bits {
            false => G1Io2::B0x0,
            true => G1Io2::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G1Io2::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G1Io2::B0x1
    }
}
#[doc = "Field `G1_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G1Io2W<'a, REG> = crate::BitWriter<'a, REG, G1Io2>;
impl<'a, REG> G1Io2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G1Io2::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G1Io2::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1Io3 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G1Io3> for bool {
    #[inline(always)]
    fn from(variant: G1Io3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G1_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G1Io3R = crate::BitReader<G1Io3>;
impl G1Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G1Io3 {
        match self.bits {
            false => G1Io3::B0x0,
            true => G1Io3::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G1Io3::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G1Io3::B0x1
    }
}
#[doc = "Field `G1_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G1Io3W<'a, REG> = crate::BitWriter<'a, REG, G1Io3>;
impl<'a, REG> G1Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G1Io3::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G1Io3::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1Io4 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G1Io4> for bool {
    #[inline(always)]
    fn from(variant: G1Io4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G1_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G1Io4R = crate::BitReader<G1Io4>;
impl G1Io4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G1Io4 {
        match self.bits {
            false => G1Io4::B0x0,
            true => G1Io4::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G1Io4::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G1Io4::B0x1
    }
}
#[doc = "Field `G1_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G1Io4W<'a, REG> = crate::BitWriter<'a, REG, G1Io4>;
impl<'a, REG> G1Io4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G1Io4::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G1Io4::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G2Io1 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G2Io1> for bool {
    #[inline(always)]
    fn from(variant: G2Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G2_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G2Io1R = crate::BitReader<G2Io1>;
impl G2Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G2Io1 {
        match self.bits {
            false => G2Io1::B0x0,
            true => G2Io1::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G2Io1::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G2Io1::B0x1
    }
}
#[doc = "Field `G2_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G2Io1W<'a, REG> = crate::BitWriter<'a, REG, G2Io1>;
impl<'a, REG> G2Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io1::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io1::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G2Io2 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G2Io2> for bool {
    #[inline(always)]
    fn from(variant: G2Io2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G2_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G2Io2R = crate::BitReader<G2Io2>;
impl G2Io2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G2Io2 {
        match self.bits {
            false => G2Io2::B0x0,
            true => G2Io2::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G2Io2::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G2Io2::B0x1
    }
}
#[doc = "Field `G2_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G2Io2W<'a, REG> = crate::BitWriter<'a, REG, G2Io2>;
impl<'a, REG> G2Io2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io2::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io2::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G2Io3 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G2Io3> for bool {
    #[inline(always)]
    fn from(variant: G2Io3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G2_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G2Io3R = crate::BitReader<G2Io3>;
impl G2Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G2Io3 {
        match self.bits {
            false => G2Io3::B0x0,
            true => G2Io3::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G2Io3::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G2Io3::B0x1
    }
}
#[doc = "Field `G2_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G2Io3W<'a, REG> = crate::BitWriter<'a, REG, G2Io3>;
impl<'a, REG> G2Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io3::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io3::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G2Io4 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G2Io4> for bool {
    #[inline(always)]
    fn from(variant: G2Io4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G2_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G2Io4R = crate::BitReader<G2Io4>;
impl G2Io4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G2Io4 {
        match self.bits {
            false => G2Io4::B0x0,
            true => G2Io4::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G2Io4::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G2Io4::B0x1
    }
}
#[doc = "Field `G2_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G2Io4W<'a, REG> = crate::BitWriter<'a, REG, G2Io4>;
impl<'a, REG> G2Io4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io4::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G2Io4::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G3Io1 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G3Io1> for bool {
    #[inline(always)]
    fn from(variant: G3Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G3_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G3Io1R = crate::BitReader<G3Io1>;
impl G3Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G3Io1 {
        match self.bits {
            false => G3Io1::B0x0,
            true => G3Io1::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G3Io1::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G3Io1::B0x1
    }
}
#[doc = "Field `G3_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G3Io1W<'a, REG> = crate::BitWriter<'a, REG, G3Io1>;
impl<'a, REG> G3Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G3Io1::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G3Io1::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G3Io2 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G3Io2> for bool {
    #[inline(always)]
    fn from(variant: G3Io2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G3_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G3Io2R = crate::BitReader<G3Io2>;
impl G3Io2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G3Io2 {
        match self.bits {
            false => G3Io2::B0x0,
            true => G3Io2::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G3Io2::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G3Io2::B0x1
    }
}
#[doc = "Field `G3_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G3Io2W<'a, REG> = crate::BitWriter<'a, REG, G3Io2>;
impl<'a, REG> G3Io2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G3Io2::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G3Io2::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G3Io3 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G3Io3> for bool {
    #[inline(always)]
    fn from(variant: G3Io3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G3_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G3Io3R = crate::BitReader<G3Io3>;
impl G3Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G3Io3 {
        match self.bits {
            false => G3Io3::B0x0,
            true => G3Io3::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G3Io3::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G3Io3::B0x1
    }
}
#[doc = "Field `G3_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G3Io3W<'a, REG> = crate::BitWriter<'a, REG, G3Io3>;
impl<'a, REG> G3Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G3Io3::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G3Io3::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G3Io4 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G3Io4> for bool {
    #[inline(always)]
    fn from(variant: G3Io4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G3_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G3Io4R = crate::BitReader<G3Io4>;
impl G3Io4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G3Io4 {
        match self.bits {
            false => G3Io4::B0x0,
            true => G3Io4::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G3Io4::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G3Io4::B0x1
    }
}
#[doc = "Field `G3_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G3Io4W<'a, REG> = crate::BitWriter<'a, REG, G3Io4>;
impl<'a, REG> G3Io4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G3Io4::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G3Io4::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G4Io1 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G4Io1> for bool {
    #[inline(always)]
    fn from(variant: G4Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G4_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G4Io1R = crate::BitReader<G4Io1>;
impl G4Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G4Io1 {
        match self.bits {
            false => G4Io1::B0x0,
            true => G4Io1::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G4Io1::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G4Io1::B0x1
    }
}
#[doc = "Field `G4_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G4Io1W<'a, REG> = crate::BitWriter<'a, REG, G4Io1>;
impl<'a, REG> G4Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io1::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io1::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G4Io2 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G4Io2> for bool {
    #[inline(always)]
    fn from(variant: G4Io2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G4_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G4Io2R = crate::BitReader<G4Io2>;
impl G4Io2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G4Io2 {
        match self.bits {
            false => G4Io2::B0x0,
            true => G4Io2::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G4Io2::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G4Io2::B0x1
    }
}
#[doc = "Field `G4_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G4Io2W<'a, REG> = crate::BitWriter<'a, REG, G4Io2>;
impl<'a, REG> G4Io2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io2::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io2::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G4Io3 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G4Io3> for bool {
    #[inline(always)]
    fn from(variant: G4Io3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G4_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G4Io3R = crate::BitReader<G4Io3>;
impl G4Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G4Io3 {
        match self.bits {
            false => G4Io3::B0x0,
            true => G4Io3::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G4Io3::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G4Io3::B0x1
    }
}
#[doc = "Field `G4_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G4Io3W<'a, REG> = crate::BitWriter<'a, REG, G4Io3>;
impl<'a, REG> G4Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io3::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io3::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G4Io4 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G4Io4> for bool {
    #[inline(always)]
    fn from(variant: G4Io4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G4_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G4Io4R = crate::BitReader<G4Io4>;
impl G4Io4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G4Io4 {
        match self.bits {
            false => G4Io4::B0x0,
            true => G4Io4::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G4Io4::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G4Io4::B0x1
    }
}
#[doc = "Field `G4_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G4Io4W<'a, REG> = crate::BitWriter<'a, REG, G4Io4>;
impl<'a, REG> G4Io4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io4::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G4Io4::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G5Io1 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G5Io1> for bool {
    #[inline(always)]
    fn from(variant: G5Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G5_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G5Io1R = crate::BitReader<G5Io1>;
impl G5Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G5Io1 {
        match self.bits {
            false => G5Io1::B0x0,
            true => G5Io1::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G5Io1::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G5Io1::B0x1
    }
}
#[doc = "Field `G5_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G5Io1W<'a, REG> = crate::BitWriter<'a, REG, G5Io1>;
impl<'a, REG> G5Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G5Io1::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G5Io1::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G5Io2 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G5Io2> for bool {
    #[inline(always)]
    fn from(variant: G5Io2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G5_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G5Io2R = crate::BitReader<G5Io2>;
impl G5Io2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G5Io2 {
        match self.bits {
            false => G5Io2::B0x0,
            true => G5Io2::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G5Io2::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G5Io2::B0x1
    }
}
#[doc = "Field `G5_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G5Io2W<'a, REG> = crate::BitWriter<'a, REG, G5Io2>;
impl<'a, REG> G5Io2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G5Io2::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G5Io2::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G5Io3 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G5Io3> for bool {
    #[inline(always)]
    fn from(variant: G5Io3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G5_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G5Io3R = crate::BitReader<G5Io3>;
impl G5Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G5Io3 {
        match self.bits {
            false => G5Io3::B0x0,
            true => G5Io3::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G5Io3::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G5Io3::B0x1
    }
}
#[doc = "Field `G5_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G5Io3W<'a, REG> = crate::BitWriter<'a, REG, G5Io3>;
impl<'a, REG> G5Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G5Io3::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G5Io3::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G5Io4 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G5Io4> for bool {
    #[inline(always)]
    fn from(variant: G5Io4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G5_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G5Io4R = crate::BitReader<G5Io4>;
impl G5Io4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G5Io4 {
        match self.bits {
            false => G5Io4::B0x0,
            true => G5Io4::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G5Io4::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G5Io4::B0x1
    }
}
#[doc = "Field `G5_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G5Io4W<'a, REG> = crate::BitWriter<'a, REG, G5Io4>;
impl<'a, REG> G5Io4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G5Io4::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G5Io4::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G6Io1 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G6Io1> for bool {
    #[inline(always)]
    fn from(variant: G6Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G6_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G6Io1R = crate::BitReader<G6Io1>;
impl G6Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G6Io1 {
        match self.bits {
            false => G6Io1::B0x0,
            true => G6Io1::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G6Io1::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G6Io1::B0x1
    }
}
#[doc = "Field `G6_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G6Io1W<'a, REG> = crate::BitWriter<'a, REG, G6Io1>;
impl<'a, REG> G6Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io1::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io1::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G6Io2 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G6Io2> for bool {
    #[inline(always)]
    fn from(variant: G6Io2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G6_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G6Io2R = crate::BitReader<G6Io2>;
impl G6Io2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G6Io2 {
        match self.bits {
            false => G6Io2::B0x0,
            true => G6Io2::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G6Io2::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G6Io2::B0x1
    }
}
#[doc = "Field `G6_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G6Io2W<'a, REG> = crate::BitWriter<'a, REG, G6Io2>;
impl<'a, REG> G6Io2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io2::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io2::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G6Io3 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G6Io3> for bool {
    #[inline(always)]
    fn from(variant: G6Io3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G6_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G6Io3R = crate::BitReader<G6Io3>;
impl G6Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G6Io3 {
        match self.bits {
            false => G6Io3::B0x0,
            true => G6Io3::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G6Io3::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G6Io3::B0x1
    }
}
#[doc = "Field `G6_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G6Io3W<'a, REG> = crate::BitWriter<'a, REG, G6Io3>;
impl<'a, REG> G6Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io3::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io3::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G6Io4 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G6Io4> for bool {
    #[inline(always)]
    fn from(variant: G6Io4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G6_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G6Io4R = crate::BitReader<G6Io4>;
impl G6Io4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G6Io4 {
        match self.bits {
            false => G6Io4::B0x0,
            true => G6Io4::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G6Io4::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G6Io4::B0x1
    }
}
#[doc = "Field `G6_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G6Io4W<'a, REG> = crate::BitWriter<'a, REG, G6Io4>;
impl<'a, REG> G6Io4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io4::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G6Io4::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G7Io1 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G7Io1> for bool {
    #[inline(always)]
    fn from(variant: G7Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G7_IO1` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G7Io1R = crate::BitReader<G7Io1>;
impl G7Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G7Io1 {
        match self.bits {
            false => G7Io1::B0x0,
            true => G7Io1::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G7Io1::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G7Io1::B0x1
    }
}
#[doc = "Field `G7_IO1` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G7Io1W<'a, REG> = crate::BitWriter<'a, REG, G7Io1>;
impl<'a, REG> G7Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io1::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io1::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G7Io2 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G7Io2> for bool {
    #[inline(always)]
    fn from(variant: G7Io2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G7_IO2` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G7Io2R = crate::BitReader<G7Io2>;
impl G7Io2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G7Io2 {
        match self.bits {
            false => G7Io2::B0x0,
            true => G7Io2::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G7Io2::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G7Io2::B0x1
    }
}
#[doc = "Field `G7_IO2` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G7Io2W<'a, REG> = crate::BitWriter<'a, REG, G7Io2>;
impl<'a, REG> G7Io2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io2::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io2::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G7Io3 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G7Io3> for bool {
    #[inline(always)]
    fn from(variant: G7Io3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G7_IO3` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G7Io3R = crate::BitReader<G7Io3>;
impl G7Io3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G7Io3 {
        match self.bits {
            false => G7Io3::B0x0,
            true => G7Io3::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G7Io3::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G7Io3::B0x1
    }
}
#[doc = "Field `G7_IO3` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G7Io3W<'a, REG> = crate::BitWriter<'a, REG, G7Io3>;
impl<'a, REG> G7Io3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io3::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io3::B0x1)
    }
}
#[doc = "Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G7Io4 {
    #[doc = "0: Gx_IOy unused"]
    B0x0 = 0,
    #[doc = "1: Gx_IOy used as sampling capacitor"]
    B0x1 = 1,
}
impl From<G7Io4> for bool {
    #[inline(always)]
    fn from(variant: G7Io4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `G7_IO4` reader - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G7Io4R = crate::BitReader<G7Io4>;
impl G7Io4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G7Io4 {
        match self.bits {
            false => G7Io4::B0x0,
            true => G7Io4::B0x1,
        }
    }
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == G7Io4::B0x0
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == G7Io4::B0x1
    }
}
#[doc = "Field `G7_IO4` writer - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
pub type G7Io4W<'a, REG> = crate::BitWriter<'a, REG, G7Io4>;
impl<'a, REG> G7Io4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gx_IOy unused"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io4::B0x0)
    }
    #[doc = "Gx_IOy used as sampling capacitor"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(G7Io4::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g1_io1(&self) -> G1Io1R {
        G1Io1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g1_io2(&self) -> G1Io2R {
        G1Io2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g1_io3(&self) -> G1Io3R {
        G1Io3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g1_io4(&self) -> G1Io4R {
        G1Io4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g2_io1(&self) -> G2Io1R {
        G2Io1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g2_io2(&self) -> G2Io2R {
        G2Io2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g2_io3(&self) -> G2Io3R {
        G2Io3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g2_io4(&self) -> G2Io4R {
        G2Io4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g3_io1(&self) -> G3Io1R {
        G3Io1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g3_io2(&self) -> G3Io2R {
        G3Io2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g3_io3(&self) -> G3Io3R {
        G3Io3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g3_io4(&self) -> G3Io4R {
        G3Io4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g4_io1(&self) -> G4Io1R {
        G4Io1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g4_io2(&self) -> G4Io2R {
        G4Io2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g4_io3(&self) -> G4Io3R {
        G4Io3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g4_io4(&self) -> G4Io4R {
        G4Io4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g5_io1(&self) -> G5Io1R {
        G5Io1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g5_io2(&self) -> G5Io2R {
        G5Io2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g5_io3(&self) -> G5Io3R {
        G5Io3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g5_io4(&self) -> G5Io4R {
        G5Io4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g6_io1(&self) -> G6Io1R {
        G6Io1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g6_io2(&self) -> G6Io2R {
        G6Io2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g6_io3(&self) -> G6Io3R {
        G6Io3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g6_io4(&self) -> G6Io4R {
        G6Io4R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g7_io1(&self) -> G7Io1R {
        G7Io1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g7_io2(&self) -> G7Io2R {
        G7Io2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g7_io3(&self) -> G7Io3R {
        G7Io3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    pub fn g7_io4(&self) -> G7Io4R {
        G7Io4R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g1_io1(&mut self) -> G1Io1W<TscIoscrSpec> {
        G1Io1W::new(self, 0)
    }
    #[doc = "Bit 1 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g1_io2(&mut self) -> G1Io2W<TscIoscrSpec> {
        G1Io2W::new(self, 1)
    }
    #[doc = "Bit 2 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g1_io3(&mut self) -> G1Io3W<TscIoscrSpec> {
        G1Io3W::new(self, 2)
    }
    #[doc = "Bit 3 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g1_io4(&mut self) -> G1Io4W<TscIoscrSpec> {
        G1Io4W::new(self, 3)
    }
    #[doc = "Bit 4 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g2_io1(&mut self) -> G2Io1W<TscIoscrSpec> {
        G2Io1W::new(self, 4)
    }
    #[doc = "Bit 5 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g2_io2(&mut self) -> G2Io2W<TscIoscrSpec> {
        G2Io2W::new(self, 5)
    }
    #[doc = "Bit 6 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g2_io3(&mut self) -> G2Io3W<TscIoscrSpec> {
        G2Io3W::new(self, 6)
    }
    #[doc = "Bit 7 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g2_io4(&mut self) -> G2Io4W<TscIoscrSpec> {
        G2Io4W::new(self, 7)
    }
    #[doc = "Bit 8 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g3_io1(&mut self) -> G3Io1W<TscIoscrSpec> {
        G3Io1W::new(self, 8)
    }
    #[doc = "Bit 9 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g3_io2(&mut self) -> G3Io2W<TscIoscrSpec> {
        G3Io2W::new(self, 9)
    }
    #[doc = "Bit 10 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g3_io3(&mut self) -> G3Io3W<TscIoscrSpec> {
        G3Io3W::new(self, 10)
    }
    #[doc = "Bit 11 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g3_io4(&mut self) -> G3Io4W<TscIoscrSpec> {
        G3Io4W::new(self, 11)
    }
    #[doc = "Bit 12 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g4_io1(&mut self) -> G4Io1W<TscIoscrSpec> {
        G4Io1W::new(self, 12)
    }
    #[doc = "Bit 13 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g4_io2(&mut self) -> G4Io2W<TscIoscrSpec> {
        G4Io2W::new(self, 13)
    }
    #[doc = "Bit 14 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g4_io3(&mut self) -> G4Io3W<TscIoscrSpec> {
        G4Io3W::new(self, 14)
    }
    #[doc = "Bit 15 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g4_io4(&mut self) -> G4Io4W<TscIoscrSpec> {
        G4Io4W::new(self, 15)
    }
    #[doc = "Bit 16 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g5_io1(&mut self) -> G5Io1W<TscIoscrSpec> {
        G5Io1W::new(self, 16)
    }
    #[doc = "Bit 17 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g5_io2(&mut self) -> G5Io2W<TscIoscrSpec> {
        G5Io2W::new(self, 17)
    }
    #[doc = "Bit 18 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g5_io3(&mut self) -> G5Io3W<TscIoscrSpec> {
        G5Io3W::new(self, 18)
    }
    #[doc = "Bit 19 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g5_io4(&mut self) -> G5Io4W<TscIoscrSpec> {
        G5Io4W::new(self, 19)
    }
    #[doc = "Bit 20 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g6_io1(&mut self) -> G6Io1W<TscIoscrSpec> {
        G6Io1W::new(self, 20)
    }
    #[doc = "Bit 21 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g6_io2(&mut self) -> G6Io2W<TscIoscrSpec> {
        G6Io2W::new(self, 21)
    }
    #[doc = "Bit 22 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g6_io3(&mut self) -> G6Io3W<TscIoscrSpec> {
        G6Io3W::new(self, 22)
    }
    #[doc = "Bit 23 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g6_io4(&mut self) -> G6Io4W<TscIoscrSpec> {
        G6Io4W::new(self, 23)
    }
    #[doc = "Bit 24 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g7_io1(&mut self) -> G7Io1W<TscIoscrSpec> {
        G7Io1W::new(self, 24)
    }
    #[doc = "Bit 25 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g7_io2(&mut self) -> G7Io2W<TscIoscrSpec> {
        G7Io2W::new(self, 25)
    }
    #[doc = "Bit 26 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g7_io3(&mut self) -> G7Io3W<TscIoscrSpec> {
        G7Io3W::new(self, 26)
    }
    #[doc = "Bit 27 - Gx_IOy sampling mode These bits are set and cleared by software to configure the Gx_IOy as a sampling capacitor I/O. Only one I/O per analog I/O group must be defined as sampling capacitor. Note: These bits must not be modified when an acquisition is ongoing. Note: During the acquisition phase and even if the TSC peripheral alternate function is not enabled, as soon as the TSC_IOSCR bit is set, the corresponding GPIO analog switch is automatically controlled by the touch sensing controller."]
    #[inline(always)]
    #[must_use]
    pub fn g7_io4(&mut self) -> G7Io4W<TscIoscrSpec> {
        G7Io4W::new(self, 27)
    }
}
#[doc = "TSC I/O sampling control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsc_ioscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsc_ioscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscIoscrSpec;
impl crate::RegisterSpec for TscIoscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsc_ioscr::R`](R) reader structure"]
impl crate::Readable for TscIoscrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsc_ioscr::W`](W) writer structure"]
impl crate::Writable for TscIoscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSC_IOSCR to value 0"]
impl crate::Resettable for TscIoscrSpec {
    const RESET_VALUE: u32 = 0;
}
