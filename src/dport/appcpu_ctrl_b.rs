#[doc = "Reader of register APPCPU_CTRL_B"]
pub type R = crate::R<u32, super::APPCPU_CTRL_B>;
#[doc = "Writer for register APPCPU_CTRL_B"]
pub type W = crate::W<u32, super::APPCPU_CTRL_B>;
#[doc = "Register APPCPU_CTRL_B `reset()`'s with value 0"]
impl crate::ResetValue for super::APPCPU_CTRL_B {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPCPU_CLKGATE_EN`"]
pub type APPCPU_CLKGATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APPCPU_CLKGATE_EN`"]
pub struct APPCPU_CLKGATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCPU_CLKGATE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn appcpu_clkgate_en(&self) -> APPCPU_CLKGATE_EN_R {
        APPCPU_CLKGATE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn appcpu_clkgate_en(&mut self) -> APPCPU_CLKGATE_EN_W {
        APPCPU_CLKGATE_EN_W { w: self }
    }
}
