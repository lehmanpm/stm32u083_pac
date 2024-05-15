#[doc = "Register `RNG_DR` reader"]
pub type R = crate::R<RngDrSpec>;
#[doc = "Field `RNDATA` reader - Random data 32-bit random data, which are valid when DRDY1=11. When DRDY1=10, the RNDATA value is1zero. When DRDY is set, it is recommended to always verify that RNG_DR is different from zero. Because when it is the case a seed error occurred between RNG_SR polling and RND_DR output reading (rare event)."]
pub type RndataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Random data 32-bit random data, which are valid when DRDY1=11. When DRDY1=10, the RNDATA value is1zero. When DRDY is set, it is recommended to always verify that RNG_DR is different from zero. Because when it is the case a seed error occurred between RNG_SR polling and RND_DR output reading (rare event)."]
    #[inline(always)]
    pub fn rndata(&self) -> RndataR {
        RndataR::new(self.bits)
    }
}
#[doc = "RNG data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngDrSpec;
impl crate::RegisterSpec for RngDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_dr::R`](R) reader structure"]
impl crate::Readable for RngDrSpec {}
#[doc = "`reset()` method sets RNG_DR to value 0"]
impl crate::Resettable for RngDrSpec {
    const RESET_VALUE: u32 = 0;
}
