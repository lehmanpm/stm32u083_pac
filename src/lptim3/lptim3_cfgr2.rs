#[doc = "Register `LPTIM3_CFGR2` reader"]
pub type R = crate::R<Lptim3Cfgr2Spec>;
#[doc = "Register `LPTIM3_CFGR2` writer"]
pub type W = crate::W<Lptim3Cfgr2Spec>;
#[doc = "LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum In1sel {
    #[doc = "0: LPTIM3_in1_mux0"]
    B0x0 = 0,
    #[doc = "1: LPTIM3_in1_mux1"]
    B0x1 = 1,
    #[doc = "2: LPTIM3_in1_mux2"]
    B0x2 = 2,
    #[doc = "3: LPTIM3_in1_mux3"]
    B0x3 = 3,
}
impl From<In1sel> for u8 {
    #[inline(always)]
    fn from(variant: In1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for In1sel {
    type Ux = u8;
}
impl crate::IsEnum for In1sel {}
#[doc = "Field `IN1SEL` reader - LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
pub type In1selR = crate::FieldReader<In1sel>;
impl In1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In1sel {
        match self.bits {
            0 => In1sel::B0x0,
            1 => In1sel::B0x1,
            2 => In1sel::B0x2,
            3 => In1sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "LPTIM3_in1_mux0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == In1sel::B0x0
    }
    #[doc = "LPTIM3_in1_mux1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == In1sel::B0x1
    }
    #[doc = "LPTIM3_in1_mux2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == In1sel::B0x2
    }
    #[doc = "LPTIM3_in1_mux3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == In1sel::B0x3
    }
}
#[doc = "Field `IN1SEL` writer - LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
pub type In1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, In1sel, crate::Safe>;
impl<'a, REG> In1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPTIM3_in1_mux0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(In1sel::B0x0)
    }
    #[doc = "LPTIM3_in1_mux1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(In1sel::B0x1)
    }
    #[doc = "LPTIM3_in1_mux2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(In1sel::B0x2)
    }
    #[doc = "LPTIM3_in1_mux3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(In1sel::B0x3)
    }
}
#[doc = "LPTIM input 2 selection The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum In2sel {
    #[doc = "0: LPTIM3_in2_mux0"]
    B0x0 = 0,
    #[doc = "1: LPTIM3_in2_mux1"]
    B0x1 = 1,
    #[doc = "2: LPTIM3_in2_mux2"]
    B0x2 = 2,
    #[doc = "3: LPTIM3_in2_mux3"]
    B0x3 = 3,
}
impl From<In2sel> for u8 {
    #[inline(always)]
    fn from(variant: In2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for In2sel {
    type Ux = u8;
}
impl crate::IsEnum for In2sel {}
#[doc = "Field `IN2SEL` reader - LPTIM input 2 selection The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
pub type In2selR = crate::FieldReader<In2sel>;
impl In2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In2sel {
        match self.bits {
            0 => In2sel::B0x0,
            1 => In2sel::B0x1,
            2 => In2sel::B0x2,
            3 => In2sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "LPTIM3_in2_mux0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == In2sel::B0x0
    }
    #[doc = "LPTIM3_in2_mux1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == In2sel::B0x1
    }
    #[doc = "LPTIM3_in2_mux2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == In2sel::B0x2
    }
    #[doc = "LPTIM3_in2_mux3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == In2sel::B0x3
    }
}
#[doc = "Field `IN2SEL` writer - LPTIM input 2 selection The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
pub type In2selW<'a, REG> = crate::FieldWriter<'a, REG, 2, In2sel, crate::Safe>;
impl<'a, REG> In2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPTIM3_in2_mux0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(In2sel::B0x0)
    }
    #[doc = "LPTIM3_in2_mux1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(In2sel::B0x1)
    }
    #[doc = "LPTIM3_in2_mux2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(In2sel::B0x2)
    }
    #[doc = "LPTIM3_in2_mux3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(In2sel::B0x3)
    }
}
#[doc = "LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic1sel {
    #[doc = "0: LPTIM3_ic1_mux0"]
    B0x0 = 0,
    #[doc = "1: LPTIM3_ic1_mux1"]
    B0x1 = 1,
    #[doc = "2: LPTIM3_ic1_mux2"]
    B0x2 = 2,
    #[doc = "3: LPTIM3_ic1_mux3"]
    B0x3 = 3,
}
impl From<Ic1sel> for u8 {
    #[inline(always)]
    fn from(variant: Ic1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic1sel {
    type Ux = u8;
}
impl crate::IsEnum for Ic1sel {}
#[doc = "Field `IC1SEL` reader - LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
pub type Ic1selR = crate::FieldReader<Ic1sel>;
impl Ic1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic1sel {
        match self.bits {
            0 => Ic1sel::B0x0,
            1 => Ic1sel::B0x1,
            2 => Ic1sel::B0x2,
            3 => Ic1sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "LPTIM3_ic1_mux0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic1sel::B0x0
    }
    #[doc = "LPTIM3_ic1_mux1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic1sel::B0x1
    }
    #[doc = "LPTIM3_ic1_mux2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic1sel::B0x2
    }
    #[doc = "LPTIM3_ic1_mux3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic1sel::B0x3
    }
}
#[doc = "Field `IC1SEL` writer - LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
pub type Ic1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic1sel, crate::Safe>;
impl<'a, REG> Ic1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPTIM3_ic1_mux0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1sel::B0x0)
    }
    #[doc = "LPTIM3_ic1_mux1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1sel::B0x1)
    }
    #[doc = "LPTIM3_ic1_mux2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1sel::B0x2)
    }
    #[doc = "LPTIM3_ic1_mux3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic1sel::B0x3)
    }
}
#[doc = "LPTIM input capture 2 selection The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ic2sel {
    #[doc = "0: LPTIM3_ic2_mux0"]
    B0x0 = 0,
    #[doc = "1: LPTIM3_ic2_mux1"]
    B0x1 = 1,
    #[doc = "2: LPTIM3_ic2_mux2"]
    B0x2 = 2,
    #[doc = "3: LPTIM3_ic2_mux3"]
    B0x3 = 3,
}
impl From<Ic2sel> for u8 {
    #[inline(always)]
    fn from(variant: Ic2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ic2sel {
    type Ux = u8;
}
impl crate::IsEnum for Ic2sel {}
#[doc = "Field `IC2SEL` reader - LPTIM input capture 2 selection The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
pub type Ic2selR = crate::FieldReader<Ic2sel>;
impl Ic2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ic2sel {
        match self.bits {
            0 => Ic2sel::B0x0,
            1 => Ic2sel::B0x1,
            2 => Ic2sel::B0x2,
            3 => Ic2sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "LPTIM3_ic2_mux0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ic2sel::B0x0
    }
    #[doc = "LPTIM3_ic2_mux1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ic2sel::B0x1
    }
    #[doc = "LPTIM3_ic2_mux2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Ic2sel::B0x2
    }
    #[doc = "LPTIM3_ic2_mux3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Ic2sel::B0x3
    }
}
#[doc = "Field `IC2SEL` writer - LPTIM input capture 2 selection The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
pub type Ic2selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ic2sel, crate::Safe>;
impl<'a, REG> Ic2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPTIM3_ic2_mux0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2sel::B0x0)
    }
    #[doc = "LPTIM3_ic2_mux1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2sel::B0x1)
    }
    #[doc = "LPTIM3_ic2_mux2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2sel::B0x2)
    }
    #[doc = "LPTIM3_ic2_mux3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ic2sel::B0x3)
    }
}
impl R {
    #[doc = "Bits 0:1 - LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
    #[inline(always)]
    pub fn in1sel(&self) -> In1selR {
        In1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - LPTIM input 2 selection The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
    #[inline(always)]
    pub fn in2sel(&self) -> In2selR {
        In2selR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
    #[inline(always)]
    pub fn ic1sel(&self) -> Ic1selR {
        Ic1selR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - LPTIM input capture 2 selection The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
    #[inline(always)]
    pub fn ic2sel(&self) -> Ic2selR {
        Ic2selR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
    #[inline(always)]
    #[must_use]
    pub fn in1sel(&mut self) -> In1selW<Lptim3Cfgr2Spec> {
        In1selW::new(self, 0)
    }
    #[doc = "Bits 4:5 - LPTIM input 2 selection The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
    #[inline(always)]
    #[must_use]
    pub fn in2sel(&mut self) -> In2selW<Lptim3Cfgr2Spec> {
        In2selW::new(self, 4)
    }
    #[doc = "Bits 16:17 - LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
    #[inline(always)]
    #[must_use]
    pub fn ic1sel(&mut self) -> Ic1selW<Lptim3Cfgr2Spec> {
        Ic1selW::new(self, 16)
    }
    #[doc = "Bits 20:21 - LPTIM input capture 2 selection The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture 2 to one of the available inputs. For connection details refer to Section125.4.3: LPTIM input and trigger mapping."]
    #[inline(always)]
    #[must_use]
    pub fn ic2sel(&mut self) -> Ic2selW<Lptim3Cfgr2Spec> {
        Ic2selW::new(self, 20)
    }
}
#[doc = "LPTIM configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim3_cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim3_cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lptim3Cfgr2Spec;
impl crate::RegisterSpec for Lptim3Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim3_cfgr2::R`](R) reader structure"]
impl crate::Readable for Lptim3Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`lptim3_cfgr2::W`](W) writer structure"]
impl crate::Writable for Lptim3Cfgr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM3_CFGR2 to value 0"]
impl crate::Resettable for Lptim3Cfgr2Spec {
    const RESET_VALUE: u32 = 0;
}
