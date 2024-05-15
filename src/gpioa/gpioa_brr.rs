#[doc = "Register `GPIOA_BRR` writer"]
pub type W = crate::W<GpioaBrrSpec>;
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br0 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br0> for bool {
    #[inline(always)]
    fn from(variant: Br0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR0` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br0::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br1 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br1> for bool {
    #[inline(always)]
    fn from(variant: Br1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR1` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br1::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br2 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br2> for bool {
    #[inline(always)]
    fn from(variant: Br2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR2` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br2::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br3 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br3> for bool {
    #[inline(always)]
    fn from(variant: Br3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR3` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br3::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br4 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br4> for bool {
    #[inline(always)]
    fn from(variant: Br4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR4` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br4::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br5 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br5> for bool {
    #[inline(always)]
    fn from(variant: Br5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR5` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br5::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br6 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br6> for bool {
    #[inline(always)]
    fn from(variant: Br6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR6` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br6::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br7 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br7> for bool {
    #[inline(always)]
    fn from(variant: Br7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR7` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br7::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br8 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br8> for bool {
    #[inline(always)]
    fn from(variant: Br8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR8` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br8::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br9 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br9> for bool {
    #[inline(always)]
    fn from(variant: Br9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR9` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br9::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br10 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br10> for bool {
    #[inline(always)]
    fn from(variant: Br10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR10` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br10::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br11 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br11> for bool {
    #[inline(always)]
    fn from(variant: Br11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR11` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br11::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br12 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br12> for bool {
    #[inline(always)]
    fn from(variant: Br12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR12` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br12::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br13 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br13> for bool {
    #[inline(always)]
    fn from(variant: Br13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR13` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br13::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br14 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br14> for bool {
    #[inline(always)]
    fn from(variant: Br14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR14` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br14::B0x1)
    }
}
#[doc = "Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Br15 {
    #[doc = "0: No action on the corresponding ODx bit"]
    B0x0 = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    B0x1 = 1,
}
impl From<Br15> for bool {
    #[inline(always)]
    fn from(variant: Br15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR15` writer - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
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
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Br15::B0x1)
    }
}
impl W {
    #[doc = "Bit 0 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> Br0W<GpioaBrrSpec> {
        Br0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> Br1W<GpioaBrrSpec> {
        Br1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> Br2W<GpioaBrrSpec> {
        Br2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> Br3W<GpioaBrrSpec> {
        Br3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> Br4W<GpioaBrrSpec> {
        Br4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> Br5W<GpioaBrrSpec> {
        Br5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> Br6W<GpioaBrrSpec> {
        Br6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> Br7W<GpioaBrrSpec> {
        Br7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> Br8W<GpioaBrrSpec> {
        Br8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> Br9W<GpioaBrrSpec> {
        Br9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> Br10W<GpioaBrrSpec> {
        Br10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> Br11W<GpioaBrrSpec> {
        Br11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> Br12W<GpioaBrrSpec> {
        Br12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> Br13W<GpioaBrrSpec> {
        Br13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> Br14W<GpioaBrrSpec> {
        Br14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x reset IO pin y These bits are write-only. A read to these bits returns the value 0x0000."]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> Br15W<GpioaBrrSpec> {
        Br15W::new(self, 15)
    }
}
#[doc = "GPIO port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioa_brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioaBrrSpec;
impl crate::RegisterSpec for GpioaBrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpioa_brr::W`](W) writer structure"]
impl crate::Writable for GpioaBrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOA_BRR to value 0"]
impl crate::Resettable for GpioaBrrSpec {
    const RESET_VALUE: u32 = 0;
}
