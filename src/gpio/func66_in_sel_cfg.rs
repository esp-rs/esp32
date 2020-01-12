#[doc = "Reader of register FUNC66_IN_SEL_CFG"]
pub type R = crate::R<u32, super::FUNC66_IN_SEL_CFG>;
#[doc = "Writer for register FUNC66_IN_SEL_CFG"]
pub type W = crate::W<u32, super::FUNC66_IN_SEL_CFG>;
#[doc = "Register FUNC66_IN_SEL_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FUNC66_IN_SEL_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIG66_IN_SEL`"]
pub type SIG66_IN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIG66_IN_SEL`"]
pub struct SIG66_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG66_IN_SEL_W<'a> {
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
#[doc = "Reader of field `FUNC66_IN_INV_SEL`"]
pub type FUNC66_IN_INV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUNC66_IN_INV_SEL`"]
pub struct FUNC66_IN_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC66_IN_INV_SEL_W<'a> {
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
#[doc = "Reader of field `FUNC66_IN_SEL`"]
pub type FUNC66_IN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUNC66_IN_SEL`"]
pub struct FUNC66_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC66_IN_SEL_W<'a> {
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
    pub fn sig66_in_sel(&self) -> SIG66_IN_SEL_R {
        SIG66_IN_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - revert the value of the input if you want to revert please set the value to 1"]
    #[inline(always)]
    pub fn func66_in_inv_sel(&self) -> FUNC66_IN_INV_SEL_R {
        FUNC66_IN_INV_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - select one of the 256 inputs"]
    #[inline(always)]
    pub fn func66_in_sel(&self) -> FUNC66_IN_SEL_R {
        FUNC66_IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - if the slow signal bypass the io matrix or not if you want setting the value to 1"]
    #[inline(always)]
    pub fn sig66_in_sel(&mut self) -> SIG66_IN_SEL_W {
        SIG66_IN_SEL_W { w: self }
    }
    #[doc = "Bit 6 - revert the value of the input if you want to revert please set the value to 1"]
    #[inline(always)]
    pub fn func66_in_inv_sel(&mut self) -> FUNC66_IN_INV_SEL_W {
        FUNC66_IN_INV_SEL_W { w: self }
    }
    #[doc = "Bits 0:5 - select one of the 256 inputs"]
    #[inline(always)]
    pub fn func66_in_sel(&mut self) -> FUNC66_IN_SEL_W {
        FUNC66_IN_SEL_W { w: self }
    }
}
