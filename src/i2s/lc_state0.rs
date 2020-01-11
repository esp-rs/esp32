#[doc = "Reader of register LC_STATE0"]
pub type R = crate::R<u32, super::LC_STATE0>;
#[doc = "Writer for register LC_STATE0"]
pub type W = crate::W<u32, super::LC_STATE0>;
#[doc = "Register LC_STATE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LC_STATE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_LC_STATE0`"]
pub type I2S_LC_STATE0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `I2S_LC_STATE0`"]
pub struct I2S_LC_STATE0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LC_STATE0_W<'a> {
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
    pub fn i2s_lc_state0(&self) -> I2S_LC_STATE0_R {
        I2S_LC_STATE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_lc_state0(&mut self) -> I2S_LC_STATE0_W {
        I2S_LC_STATE0_W { w: self }
    }
}
