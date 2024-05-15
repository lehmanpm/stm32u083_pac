#[doc = "Register `GPIOD_BSRR` writer"]
pub type W = crate::W<GpiodBsrrSpec>;
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs0 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs0> for bool {
    #[inline(always)]
    fn from(variant: Bs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS0` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs0W<'a, REG> = crate::BitWriter<'a, REG, Bs0>;
impl<'a, REG> Bs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs0::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs0::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs1 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs1> for bool {
    #[inline(always)]
    fn from(variant: Bs1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS1` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs1W<'a, REG> = crate::BitWriter<'a, REG, Bs1>;
impl<'a, REG> Bs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs1::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs1::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs2 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs2> for bool {
    #[inline(always)]
    fn from(variant: Bs2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS2` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs2W<'a, REG> = crate::BitWriter<'a, REG, Bs2>;
impl<'a, REG> Bs2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs2::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs2::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs3 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs3> for bool {
    #[inline(always)]
    fn from(variant: Bs3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS3` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs3W<'a, REG> = crate::BitWriter<'a, REG, Bs3>;
impl<'a, REG> Bs3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs3::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs3::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs4 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs4> for bool {
    #[inline(always)]
    fn from(variant: Bs4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS4` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs4W<'a, REG> = crate::BitWriter<'a, REG, Bs4>;
impl<'a, REG> Bs4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs4::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs4::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs5 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs5> for bool {
    #[inline(always)]
    fn from(variant: Bs5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS5` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs5W<'a, REG> = crate::BitWriter<'a, REG, Bs5>;
impl<'a, REG> Bs5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs5::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs5::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs6 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs6> for bool {
    #[inline(always)]
    fn from(variant: Bs6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS6` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs6W<'a, REG> = crate::BitWriter<'a, REG, Bs6>;
impl<'a, REG> Bs6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs6::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs6::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs7 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs7> for bool {
    #[inline(always)]
    fn from(variant: Bs7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS7` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs7W<'a, REG> = crate::BitWriter<'a, REG, Bs7>;
impl<'a, REG> Bs7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs7::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs7::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs8 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs8> for bool {
    #[inline(always)]
    fn from(variant: Bs8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS8` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs8W<'a, REG> = crate::BitWriter<'a, REG, Bs8>;
impl<'a, REG> Bs8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs8::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs8::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs9 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs9> for bool {
    #[inline(always)]
    fn from(variant: Bs9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS9` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs9W<'a, REG> = crate::BitWriter<'a, REG, Bs9>;
impl<'a, REG> Bs9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs9::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs9::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs10 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs10> for bool {
    #[inline(always)]
    fn from(variant: Bs10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS10` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs10W<'a, REG> = crate::BitWriter<'a, REG, Bs10>;
impl<'a, REG> Bs10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs10::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs10::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs11 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs11> for bool {
    #[inline(always)]
    fn from(variant: Bs11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS11` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs11W<'a, REG> = crate::BitWriter<'a, REG, Bs11>;
impl<'a, REG> Bs11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs11::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs11::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs12 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs12> for bool {
    #[inline(always)]
    fn from(variant: Bs12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS12` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs12W<'a, REG> = crate::BitWriter<'a, REG, Bs12>;
impl<'a, REG> Bs12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs12::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs12::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs13 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs13> for bool {
    #[inline(always)]
    fn from(variant: Bs13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS13` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs13W<'a, REG> = crate::BitWriter<'a, REG, Bs13>;
impl<'a, REG> Bs13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs13::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs13::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs14 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs14> for bool {
    #[inline(always)]
    fn from(variant: Bs14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS14` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs14W<'a, REG> = crate::BitWriter<'a, REG, Bs14>;
impl<'a, REG> Bs14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs14::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs14::B0x1)
    }
}
#[doc = "Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs15 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Sets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Bs15> for bool {
    #[inline(always)]
    fn from(variant: Bs15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS15` writer - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
pub type Bs15W<'a, REG> = crate::BitWriter<'a, REG, Bs15>;
impl<'a, REG> Bs15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bs15::B0x0)
    }
    #[doc = "Sets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Bs15::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br0 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br0> for bool {
    #[inline(always)]
    fn from(variant: Br0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR0` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG, Br0>;
impl<'a, REG> Br0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br0::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br0::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br1 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br1> for bool {
    #[inline(always)]
    fn from(variant: Br1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR1` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG, Br1>;
impl<'a, REG> Br1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br1::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br1::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br2 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br2> for bool {
    #[inline(always)]
    fn from(variant: Br2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR2` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br2W<'a, REG> = crate::BitWriter<'a, REG, Br2>;
impl<'a, REG> Br2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br2::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br2::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br3 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br3> for bool {
    #[inline(always)]
    fn from(variant: Br3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR3` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br3W<'a, REG> = crate::BitWriter<'a, REG, Br3>;
impl<'a, REG> Br3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br3::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br3::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br4 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br4> for bool {
    #[inline(always)]
    fn from(variant: Br4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR4` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br4W<'a, REG> = crate::BitWriter<'a, REG, Br4>;
impl<'a, REG> Br4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br4::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br4::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br5 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br5> for bool {
    #[inline(always)]
    fn from(variant: Br5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR5` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br5W<'a, REG> = crate::BitWriter<'a, REG, Br5>;
impl<'a, REG> Br5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br5::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br5::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br6 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br6> for bool {
    #[inline(always)]
    fn from(variant: Br6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR6` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br6W<'a, REG> = crate::BitWriter<'a, REG, Br6>;
impl<'a, REG> Br6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br6::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br6::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br7 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br7> for bool {
    #[inline(always)]
    fn from(variant: Br7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR7` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br7W<'a, REG> = crate::BitWriter<'a, REG, Br7>;
impl<'a, REG> Br7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br7::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br7::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br8 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br8> for bool {
    #[inline(always)]
    fn from(variant: Br8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR8` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br8W<'a, REG> = crate::BitWriter<'a, REG, Br8>;
impl<'a, REG> Br8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br8::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br8::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br9 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br9> for bool {
    #[inline(always)]
    fn from(variant: Br9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR9` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br9W<'a, REG> = crate::BitWriter<'a, REG, Br9>;
impl<'a, REG> Br9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br9::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br9::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br10 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br10> for bool {
    #[inline(always)]
    fn from(variant: Br10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR10` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br10W<'a, REG> = crate::BitWriter<'a, REG, Br10>;
impl<'a, REG> Br10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br10::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br10::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br11 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br11> for bool {
    #[inline(always)]
    fn from(variant: Br11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR11` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br11W<'a, REG> = crate::BitWriter<'a, REG, Br11>;
impl<'a, REG> Br11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br11::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br11::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br12 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br12> for bool {
    #[inline(always)]
    fn from(variant: Br12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR12` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br12W<'a, REG> = crate::BitWriter<'a, REG, Br12>;
impl<'a, REG> Br12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br12::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br12::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br13 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br13> for bool {
    #[inline(always)]
    fn from(variant: Br13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR13` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br13W<'a, REG> = crate::BitWriter<'a, REG, Br13>;
impl<'a, REG> Br13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br13::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br13::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br14 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br14> for bool {
    #[inline(always)]
    fn from(variant: Br14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR14` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br14W<'a, REG> = crate::BitWriter<'a, REG, Br14>;
impl<'a, REG> Br14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br14::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br14::B0x1)
    }
}
#[doc = "Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br15 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Resets the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br15> for bool {
    #[inline(always)]
    fn from(variant: Br15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR15` writer - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
pub type Br15W<'a, REG> = crate::BitWriter<'a, REG, Br15>;
impl<'a, REG> Br15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Br15::B0x0)
    }
    #[doc = "Resets the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br15::B0x1)
    }
}
impl W {
    #[doc = "Bit 0 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> Bs0W<GpiodBsrrSpec> {
        Bs0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> Bs1W<GpiodBsrrSpec> {
        Bs1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> Bs2W<GpiodBsrrSpec> {
        Bs2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> Bs3W<GpiodBsrrSpec> {
        Bs3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> Bs4W<GpiodBsrrSpec> {
        Bs4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> Bs5W<GpiodBsrrSpec> {
        Bs5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> Bs6W<GpiodBsrrSpec> {
        Bs6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> Bs7W<GpiodBsrrSpec> {
        Bs7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs8(&mut self) -> Bs8W<GpiodBsrrSpec> {
        Bs8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs9(&mut self) -> Bs9W<GpiodBsrrSpec> {
        Bs9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs10(&mut self) -> Bs10W<GpiodBsrrSpec> {
        Bs10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs11(&mut self) -> Bs11W<GpiodBsrrSpec> {
        Bs11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs12(&mut self) -> Bs12W<GpiodBsrrSpec> {
        Bs12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs13(&mut self) -> Bs13W<GpiodBsrrSpec> {
        Bs13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs14(&mut self) -> Bs14W<GpiodBsrrSpec> {
        Bs14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x set I/O pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn bs15(&mut self) -> Bs15W<GpiodBsrrSpec> {
        Bs15W::new(self, 15)
    }
    #[doc = "Bit 16 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> Br0W<GpiodBsrrSpec> {
        Br0W::new(self, 16)
    }
    #[doc = "Bit 17 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> Br1W<GpiodBsrrSpec> {
        Br1W::new(self, 17)
    }
    #[doc = "Bit 18 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> Br2W<GpiodBsrrSpec> {
        Br2W::new(self, 18)
    }
    #[doc = "Bit 19 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> Br3W<GpiodBsrrSpec> {
        Br3W::new(self, 19)
    }
    #[doc = "Bit 20 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> Br4W<GpiodBsrrSpec> {
        Br4W::new(self, 20)
    }
    #[doc = "Bit 21 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> Br5W<GpiodBsrrSpec> {
        Br5W::new(self, 21)
    }
    #[doc = "Bit 22 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> Br6W<GpiodBsrrSpec> {
        Br6W::new(self, 22)
    }
    #[doc = "Bit 23 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> Br7W<GpiodBsrrSpec> {
        Br7W::new(self, 23)
    }
    #[doc = "Bit 24 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> Br8W<GpiodBsrrSpec> {
        Br8W::new(self, 24)
    }
    #[doc = "Bit 25 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> Br9W<GpiodBsrrSpec> {
        Br9W::new(self, 25)
    }
    #[doc = "Bit 26 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> Br10W<GpiodBsrrSpec> {
        Br10W::new(self, 26)
    }
    #[doc = "Bit 27 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> Br11W<GpiodBsrrSpec> {
        Br11W::new(self, 27)
    }
    #[doc = "Bit 28 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> Br12W<GpiodBsrrSpec> {
        Br12W::new(self, 28)
    }
    #[doc = "Bit 29 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> Br13W<GpiodBsrrSpec> {
        Br13W::new(self, 29)
    }
    #[doc = "Bit 30 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> Br14W<GpiodBsrrSpec> {
        Br14W::new(self, 30)
    }
    #[doc = "Bit 31 - Port x reset I/O pin y These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority."]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> Br15W<GpiodBsrrSpec> {
        Br15W::new(self, 31)
    }
}
#[doc = "GPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiodBsrrSpec;
impl crate::RegisterSpec for GpiodBsrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpiod_bsrr::W`](W) writer structure"]
impl crate::Writable for GpiodBsrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOD_BSRR to value 0"]
impl crate::Resettable for GpiodBsrrSpec {
    const RESET_VALUE: u32 = 0;
}
