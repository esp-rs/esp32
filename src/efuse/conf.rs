#[doc = "Reader of register CONF"]
pub type R = crate::R<u32, super::CONF>;
#[doc = "Writer for register CONF"]
pub type W = crate::W<u32, super::CONF>;
#[doc = "Register CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FORCE_NO_WR_RD_DIS`"]
pub type FORCE_NO_WR_RD_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_NO_WR_RD_DIS`"]
pub struct FORCE_NO_WR_RD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_NO_WR_RD_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `OP_CODE`"]
pub type OP_CODE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OP_CODE`"]
pub struct OP_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OP_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn force_no_wr_rd_dis(&self) -> FORCE_NO_WR_RD_DIS_R {
        FORCE_NO_WR_RD_DIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - efuse operation code"]
    #[inline(always)]
    pub fn op_code(&self) -> OP_CODE_R {
        OP_CODE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn force_no_wr_rd_dis(&mut self) -> FORCE_NO_WR_RD_DIS_W {
        FORCE_NO_WR_RD_DIS_W { w: self }
    }
    #[doc = "Bits 0:15 - efuse operation code"]
    #[inline(always)]
    pub fn op_code(&mut self) -> OP_CODE_W {
        OP_CODE_W { w: self }
    }
}
