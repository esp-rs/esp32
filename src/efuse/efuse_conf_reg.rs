#[doc = "Reader of register EFUSE_CONF_REG"]
pub type R = crate::R<u32, super::EFUSE_CONF_REG>;
#[doc = "Writer for register EFUSE_CONF_REG"]
pub type W = crate::W<u32, super::EFUSE_CONF_REG>;
#[doc = "Register EFUSE_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_FORCE_NO_WR_RD_DIS`"]
pub type EFUSE_FORCE_NO_WR_RD_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_FORCE_NO_WR_RD_DIS`"]
pub struct EFUSE_FORCE_NO_WR_RD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_FORCE_NO_WR_RD_DIS_W<'a> {
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
#[doc = "Reader of field `EFUSE_OP_CODE`"]
pub type EFUSE_OP_CODE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFUSE_OP_CODE`"]
pub struct EFUSE_OP_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_OP_CODE_W<'a> {
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
    pub fn efuse_force_no_wr_rd_dis(&self) -> EFUSE_FORCE_NO_WR_RD_DIS_R {
        EFUSE_FORCE_NO_WR_RD_DIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - efuse operation code"]
    #[inline(always)]
    pub fn efuse_op_code(&self) -> EFUSE_OP_CODE_R {
        EFUSE_OP_CODE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn efuse_force_no_wr_rd_dis(&mut self) -> EFUSE_FORCE_NO_WR_RD_DIS_W {
        EFUSE_FORCE_NO_WR_RD_DIS_W { w: self }
    }
    #[doc = "Bits 0:15 - efuse operation code"]
    #[inline(always)]
    pub fn efuse_op_code(&mut self) -> EFUSE_OP_CODE_W {
        EFUSE_OP_CODE_W { w: self }
    }
}
