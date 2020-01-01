#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_SIGMADELTA0_REG"]
    pub sigmadelta0: SIGMADELTA0,
    #[doc = "0x04 - GPIO_SIGMADELTA1_REG"]
    pub sigmadelta1: SIGMADELTA1,
    #[doc = "0x08 - GPIO_SIGMADELTA2_REG"]
    pub sigmadelta2: SIGMADELTA2,
    #[doc = "0x0c - GPIO_SIGMADELTA3_REG"]
    pub sigmadelta3: SIGMADELTA3,
    #[doc = "0x10 - GPIO_SIGMADELTA4_REG"]
    pub sigmadelta4: SIGMADELTA4,
    #[doc = "0x14 - GPIO_SIGMADELTA5_REG"]
    pub sigmadelta5: SIGMADELTA5,
    #[doc = "0x18 - GPIO_SIGMADELTA6_REG"]
    pub sigmadelta6: SIGMADELTA6,
    #[doc = "0x1c - GPIO_SIGMADELTA7_REG"]
    pub sigmadelta7: SIGMADELTA7,
    #[doc = "0x20 - GPIO_SIGMADELTA_CG_REG"]
    pub sigmadelta_cg: SIGMADELTA_CG,
    #[doc = "0x24 - GPIO_SIGMADELTA_MISC_REG"]
    pub sigmadelta_misc: SIGMADELTA_MISC,
    #[doc = "0x28 - GPIO_SIGMADELTA_VERSION_REG"]
    pub sigmadelta_version: SIGMADELTA_VERSION,
}
#[doc = "GPIO_SIGMADELTA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta0](sigmadelta0) module"]
pub type SIGMADELTA0 = crate::Reg<u32, _SIGMADELTA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA0;
#[doc = "`read()` method returns [sigmadelta0::R](sigmadelta0::R) reader structure"]
impl crate::Readable for SIGMADELTA0 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta0::W](sigmadelta0::W) writer structure"]
impl crate::Writable for SIGMADELTA0 {}
#[doc = "GPIO_SIGMADELTA0_REG"]
pub mod sigmadelta0;
#[doc = "GPIO_SIGMADELTA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta1](sigmadelta1) module"]
pub type SIGMADELTA1 = crate::Reg<u32, _SIGMADELTA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA1;
#[doc = "`read()` method returns [sigmadelta1::R](sigmadelta1::R) reader structure"]
impl crate::Readable for SIGMADELTA1 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta1::W](sigmadelta1::W) writer structure"]
impl crate::Writable for SIGMADELTA1 {}
#[doc = "GPIO_SIGMADELTA1_REG"]
pub mod sigmadelta1;
#[doc = "GPIO_SIGMADELTA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta2](sigmadelta2) module"]
pub type SIGMADELTA2 = crate::Reg<u32, _SIGMADELTA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA2;
#[doc = "`read()` method returns [sigmadelta2::R](sigmadelta2::R) reader structure"]
impl crate::Readable for SIGMADELTA2 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta2::W](sigmadelta2::W) writer structure"]
impl crate::Writable for SIGMADELTA2 {}
#[doc = "GPIO_SIGMADELTA2_REG"]
pub mod sigmadelta2;
#[doc = "GPIO_SIGMADELTA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta3](sigmadelta3) module"]
pub type SIGMADELTA3 = crate::Reg<u32, _SIGMADELTA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA3;
#[doc = "`read()` method returns [sigmadelta3::R](sigmadelta3::R) reader structure"]
impl crate::Readable for SIGMADELTA3 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta3::W](sigmadelta3::W) writer structure"]
impl crate::Writable for SIGMADELTA3 {}
#[doc = "GPIO_SIGMADELTA3_REG"]
pub mod sigmadelta3;
#[doc = "GPIO_SIGMADELTA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta4](sigmadelta4) module"]
pub type SIGMADELTA4 = crate::Reg<u32, _SIGMADELTA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA4;
#[doc = "`read()` method returns [sigmadelta4::R](sigmadelta4::R) reader structure"]
impl crate::Readable for SIGMADELTA4 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta4::W](sigmadelta4::W) writer structure"]
impl crate::Writable for SIGMADELTA4 {}
#[doc = "GPIO_SIGMADELTA4_REG"]
pub mod sigmadelta4;
#[doc = "GPIO_SIGMADELTA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta5](sigmadelta5) module"]
pub type SIGMADELTA5 = crate::Reg<u32, _SIGMADELTA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA5;
#[doc = "`read()` method returns [sigmadelta5::R](sigmadelta5::R) reader structure"]
impl crate::Readable for SIGMADELTA5 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta5::W](sigmadelta5::W) writer structure"]
impl crate::Writable for SIGMADELTA5 {}
#[doc = "GPIO_SIGMADELTA5_REG"]
pub mod sigmadelta5;
#[doc = "GPIO_SIGMADELTA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta6](sigmadelta6) module"]
pub type SIGMADELTA6 = crate::Reg<u32, _SIGMADELTA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA6;
#[doc = "`read()` method returns [sigmadelta6::R](sigmadelta6::R) reader structure"]
impl crate::Readable for SIGMADELTA6 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta6::W](sigmadelta6::W) writer structure"]
impl crate::Writable for SIGMADELTA6 {}
#[doc = "GPIO_SIGMADELTA6_REG"]
pub mod sigmadelta6;
#[doc = "GPIO_SIGMADELTA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta7](sigmadelta7) module"]
pub type SIGMADELTA7 = crate::Reg<u32, _SIGMADELTA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA7;
#[doc = "`read()` method returns [sigmadelta7::R](sigmadelta7::R) reader structure"]
impl crate::Readable for SIGMADELTA7 {}
#[doc = "`write(|w| ..)` method takes [sigmadelta7::W](sigmadelta7::W) writer structure"]
impl crate::Writable for SIGMADELTA7 {}
#[doc = "GPIO_SIGMADELTA7_REG"]
pub mod sigmadelta7;
#[doc = "GPIO_SIGMADELTA_CG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta_cg](sigmadelta_cg) module"]
pub type SIGMADELTA_CG = crate::Reg<u32, _SIGMADELTA_CG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA_CG;
#[doc = "`read()` method returns [sigmadelta_cg::R](sigmadelta_cg::R) reader structure"]
impl crate::Readable for SIGMADELTA_CG {}
#[doc = "`write(|w| ..)` method takes [sigmadelta_cg::W](sigmadelta_cg::W) writer structure"]
impl crate::Writable for SIGMADELTA_CG {}
#[doc = "GPIO_SIGMADELTA_CG_REG"]
pub mod sigmadelta_cg;
#[doc = "GPIO_SIGMADELTA_MISC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta_misc](sigmadelta_misc) module"]
pub type SIGMADELTA_MISC = crate::Reg<u32, _SIGMADELTA_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA_MISC;
#[doc = "`read()` method returns [sigmadelta_misc::R](sigmadelta_misc::R) reader structure"]
impl crate::Readable for SIGMADELTA_MISC {}
#[doc = "`write(|w| ..)` method takes [sigmadelta_misc::W](sigmadelta_misc::W) writer structure"]
impl crate::Writable for SIGMADELTA_MISC {}
#[doc = "GPIO_SIGMADELTA_MISC_REG"]
pub mod sigmadelta_misc;
#[doc = "GPIO_SIGMADELTA_VERSION_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sigmadelta_version](sigmadelta_version) module"]
pub type SIGMADELTA_VERSION = crate::Reg<u32, _SIGMADELTA_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIGMADELTA_VERSION;
#[doc = "`read()` method returns [sigmadelta_version::R](sigmadelta_version::R) reader structure"]
impl crate::Readable for SIGMADELTA_VERSION {}
#[doc = "`write(|w| ..)` method takes [sigmadelta_version::W](sigmadelta_version::W) writer structure"]
impl crate::Writable for SIGMADELTA_VERSION {}
#[doc = "GPIO_SIGMADELTA_VERSION_REG"]
pub mod sigmadelta_version;
