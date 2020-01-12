#[doc = "Reader of register FUNC4_IN_SEL_CFG"]
pub type R = crate::R<u32, super::FUNC4_IN_SEL_CFG>;
#[doc = "Writer for register FUNC4_IN_SEL_CFG"]
pub type W = crate::W<u32, super::FUNC4_IN_SEL_CFG>;
#[doc = "Register FUNC4_IN_SEL_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FUNC4_IN_SEL_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIG4_IN_SEL`"]
pub type SIG4_IN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIG4_IN_SEL`"]
pub struct SIG4_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG4_IN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FUNC4_IN_INV_SEL`"]
pub type FUNC4_IN_INV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUNC4_IN_INV_SEL`"]
pub struct FUNC4_IN_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC4_IN_INV_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FUNC4_IN_SEL`"]
pub type FUNC4_IN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUNC4_IN_SEL`"]
pub struct FUNC4_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC4_IN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - if the slow signal bypass the io matrix or not if you want setting the value to 1"]
    #[inline(always)]
    pub fn sig4_in_sel(&self) -> SIG4_IN_SEL_R {
        SIG4_IN_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - revert the value of the input if you want to revert please set the value to 1"]
    #[inline(always)]
    pub fn func4_in_inv_sel(&self) -> FUNC4_IN_INV_SEL_R {
        FUNC4_IN_INV_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - select one of the 256 inputs"]
    #[inline(always)]
    pub fn func4_in_sel(&self) -> FUNC4_IN_SEL_R {
        FUNC4_IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - if the slow signal bypass the io matrix or not if you want setting the value to 1"]
    #[inline(always)]
    pub fn sig4_in_sel(&mut self) -> SIG4_IN_SEL_W {
        SIG4_IN_SEL_W { w: self }
    }
    #[doc = "Bit 6 - revert the value of the input if you want to revert please set the value to 1"]
    #[inline(always)]
    pub fn func4_in_inv_sel(&mut self) -> FUNC4_IN_INV_SEL_W {
        FUNC4_IN_INV_SEL_W { w: self }
    }
    #[doc = "Bits 0:5 - select one of the 256 inputs"]
    #[inline(always)]
    pub fn func4_in_sel(&mut self) -> FUNC4_IN_SEL_W {
        FUNC4_IN_SEL_W { w: self }
    }
}
