#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_SIGMADELTA0_REG"]
    pub gpio_sigmadelta0_reg: GPIO_SIGMADELTA0_REG,
    #[doc = "0x04 - GPIO_SIGMADELTA1_REG"]
    pub gpio_sigmadelta1_reg: GPIO_SIGMADELTA1_REG,
    #[doc = "0x08 - GPIO_SIGMADELTA2_REG"]
    pub gpio_sigmadelta2_reg: GPIO_SIGMADELTA2_REG,
    #[doc = "0x0c - GPIO_SIGMADELTA3_REG"]
    pub gpio_sigmadelta3_reg: GPIO_SIGMADELTA3_REG,
    #[doc = "0x10 - GPIO_SIGMADELTA4_REG"]
    pub gpio_sigmadelta4_reg: GPIO_SIGMADELTA4_REG,
    #[doc = "0x14 - GPIO_SIGMADELTA5_REG"]
    pub gpio_sigmadelta5_reg: GPIO_SIGMADELTA5_REG,
    #[doc = "0x18 - GPIO_SIGMADELTA6_REG"]
    pub gpio_sigmadelta6_reg: GPIO_SIGMADELTA6_REG,
    #[doc = "0x1c - GPIO_SIGMADELTA7_REG"]
    pub gpio_sigmadelta7_reg: GPIO_SIGMADELTA7_REG,
    #[doc = "0x20 - GPIO_SIGMADELTA_CG_REG"]
    pub gpio_sigmadelta_cg_reg: GPIO_SIGMADELTA_CG_REG,
    #[doc = "0x24 - GPIO_SIGMADELTA_MISC_REG"]
    pub gpio_sigmadelta_misc_reg: GPIO_SIGMADELTA_MISC_REG,
    #[doc = "0x28 - GPIO_SIGMADELTA_VERSION_REG"]
    pub gpio_sigmadelta_version_reg: GPIO_SIGMADELTA_VERSION_REG,
}
#[doc = "GPIO_SIGMADELTA0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta0_reg](gpio_sigmadelta0_reg) module"]
pub type GPIO_SIGMADELTA0_REG = crate::Reg<u32, _GPIO_SIGMADELTA0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA0_REG;
#[doc = "`read()` method returns [gpio_sigmadelta0_reg::R](gpio_sigmadelta0_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA0_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta0_reg::W](gpio_sigmadelta0_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA0_REG {}
#[doc = "GPIO_SIGMADELTA0_REG"]
pub mod gpio_sigmadelta0_reg;
#[doc = "GPIO_SIGMADELTA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta1_reg](gpio_sigmadelta1_reg) module"]
pub type GPIO_SIGMADELTA1_REG = crate::Reg<u32, _GPIO_SIGMADELTA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA1_REG;
#[doc = "`read()` method returns [gpio_sigmadelta1_reg::R](gpio_sigmadelta1_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA1_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta1_reg::W](gpio_sigmadelta1_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA1_REG {}
#[doc = "GPIO_SIGMADELTA1_REG"]
pub mod gpio_sigmadelta1_reg;
#[doc = "GPIO_SIGMADELTA2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta2_reg](gpio_sigmadelta2_reg) module"]
pub type GPIO_SIGMADELTA2_REG = crate::Reg<u32, _GPIO_SIGMADELTA2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA2_REG;
#[doc = "`read()` method returns [gpio_sigmadelta2_reg::R](gpio_sigmadelta2_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA2_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta2_reg::W](gpio_sigmadelta2_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA2_REG {}
#[doc = "GPIO_SIGMADELTA2_REG"]
pub mod gpio_sigmadelta2_reg;
#[doc = "GPIO_SIGMADELTA3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta3_reg](gpio_sigmadelta3_reg) module"]
pub type GPIO_SIGMADELTA3_REG = crate::Reg<u32, _GPIO_SIGMADELTA3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA3_REG;
#[doc = "`read()` method returns [gpio_sigmadelta3_reg::R](gpio_sigmadelta3_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA3_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta3_reg::W](gpio_sigmadelta3_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA3_REG {}
#[doc = "GPIO_SIGMADELTA3_REG"]
pub mod gpio_sigmadelta3_reg;
#[doc = "GPIO_SIGMADELTA4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta4_reg](gpio_sigmadelta4_reg) module"]
pub type GPIO_SIGMADELTA4_REG = crate::Reg<u32, _GPIO_SIGMADELTA4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA4_REG;
#[doc = "`read()` method returns [gpio_sigmadelta4_reg::R](gpio_sigmadelta4_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA4_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta4_reg::W](gpio_sigmadelta4_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA4_REG {}
#[doc = "GPIO_SIGMADELTA4_REG"]
pub mod gpio_sigmadelta4_reg;
#[doc = "GPIO_SIGMADELTA5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta5_reg](gpio_sigmadelta5_reg) module"]
pub type GPIO_SIGMADELTA5_REG = crate::Reg<u32, _GPIO_SIGMADELTA5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA5_REG;
#[doc = "`read()` method returns [gpio_sigmadelta5_reg::R](gpio_sigmadelta5_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA5_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta5_reg::W](gpio_sigmadelta5_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA5_REG {}
#[doc = "GPIO_SIGMADELTA5_REG"]
pub mod gpio_sigmadelta5_reg;
#[doc = "GPIO_SIGMADELTA6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta6_reg](gpio_sigmadelta6_reg) module"]
pub type GPIO_SIGMADELTA6_REG = crate::Reg<u32, _GPIO_SIGMADELTA6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA6_REG;
#[doc = "`read()` method returns [gpio_sigmadelta6_reg::R](gpio_sigmadelta6_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA6_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta6_reg::W](gpio_sigmadelta6_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA6_REG {}
#[doc = "GPIO_SIGMADELTA6_REG"]
pub mod gpio_sigmadelta6_reg;
#[doc = "GPIO_SIGMADELTA7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta7_reg](gpio_sigmadelta7_reg) module"]
pub type GPIO_SIGMADELTA7_REG = crate::Reg<u32, _GPIO_SIGMADELTA7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA7_REG;
#[doc = "`read()` method returns [gpio_sigmadelta7_reg::R](gpio_sigmadelta7_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA7_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta7_reg::W](gpio_sigmadelta7_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA7_REG {}
#[doc = "GPIO_SIGMADELTA7_REG"]
pub mod gpio_sigmadelta7_reg;
#[doc = "GPIO_SIGMADELTA_CG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta_cg_reg](gpio_sigmadelta_cg_reg) module"]
pub type GPIO_SIGMADELTA_CG_REG = crate::Reg<u32, _GPIO_SIGMADELTA_CG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA_CG_REG;
#[doc = "`read()` method returns [gpio_sigmadelta_cg_reg::R](gpio_sigmadelta_cg_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA_CG_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta_cg_reg::W](gpio_sigmadelta_cg_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA_CG_REG {}
#[doc = "GPIO_SIGMADELTA_CG_REG"]
pub mod gpio_sigmadelta_cg_reg;
#[doc = "GPIO_SIGMADELTA_MISC_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta_misc_reg](gpio_sigmadelta_misc_reg) module"]
pub type GPIO_SIGMADELTA_MISC_REG = crate::Reg<u32, _GPIO_SIGMADELTA_MISC_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA_MISC_REG;
#[doc = "`read()` method returns [gpio_sigmadelta_misc_reg::R](gpio_sigmadelta_misc_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA_MISC_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta_misc_reg::W](gpio_sigmadelta_misc_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA_MISC_REG {}
#[doc = "GPIO_SIGMADELTA_MISC_REG"]
pub mod gpio_sigmadelta_misc_reg;
#[doc = "GPIO_SIGMADELTA_VERSION_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpio_sigmadelta_version_reg](gpio_sigmadelta_version_reg) module"]
pub type GPIO_SIGMADELTA_VERSION_REG = crate::Reg<u32, _GPIO_SIGMADELTA_VERSION_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA_VERSION_REG;
#[doc = "`read()` method returns [gpio_sigmadelta_version_reg::R](gpio_sigmadelta_version_reg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA_VERSION_REG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta_version_reg::W](gpio_sigmadelta_version_reg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA_VERSION_REG {}
#[doc = "GPIO_SIGMADELTA_VERSION_REG"]
pub mod gpio_sigmadelta_version_reg;
