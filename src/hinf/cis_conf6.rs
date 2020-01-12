#[doc = "Reader of register CIS_CONF6"]
pub type R = crate::R<u32, super::CIS_CONF6>;
#[doc = "Writer for register CIS_CONF6"]
pub type W = crate::W<u32, super::CIS_CONF6>;
#[doc = "Register CIS_CONF6 `reset()`'s with value 0"]
impl crate::ResetValue for super::CIS_CONF6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CIS_CONF_W6`"]
pub type CIS_CONF_W6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CIS_CONF_W6`"]
pub struct CIS_CONF_W6_W<'a> {
    w: &'a mut W,
}
impl<'a> CIS_CONF_W6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cis_conf_w6(&self) -> CIS_CONF_W6_R {
        CIS_CONF_W6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cis_conf_w6(&mut self) -> CIS_CONF_W6_W {
        CIS_CONF_W6_W { w: self }
    }
}
