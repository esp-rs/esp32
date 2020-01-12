#[doc = "Reader of register APPCPU_CTRL_A"]
pub type R = crate::R<u32, super::APPCPU_CTRL_A>;
#[doc = "Writer for register APPCPU_CTRL_A"]
pub type W = crate::W<u32, super::APPCPU_CTRL_A>;
#[doc = "Register APPCPU_CTRL_A `reset()`'s with value 0"]
impl crate::ResetValue for super::APPCPU_CTRL_A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPCPU_RESETTING`"]
pub type APPCPU_RESETTING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APPCPU_RESETTING`"]
pub struct APPCPU_RESETTING_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCPU_RESETTING_W<'a> {
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
    pub fn appcpu_resetting(&self) -> APPCPU_RESETTING_R {
        APPCPU_RESETTING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn appcpu_resetting(&mut self) -> APPCPU_RESETTING_W {
        APPCPU_RESETTING_W { w: self }
    }
}
