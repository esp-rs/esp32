#[doc = "Reader of register HOST_SLCHOST_TOKEN_CON_REG"]
pub type R = crate::R<u32, super::HOST_SLCHOST_TOKEN_CON_REG>;
#[doc = "Writer for register HOST_SLCHOST_TOKEN_CON_REG"]
pub type W = crate::W<u32, super::HOST_SLCHOST_TOKEN_CON_REG>;
#[doc = "Register HOST_SLCHOST_TOKEN_CON_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_TOKEN_CON_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_SLC0HOST_LEN_WR`"]
pub type HOST_SLC0HOST_LEN_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC0HOST_LEN_WR`"]
pub struct HOST_SLC0HOST_LEN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_LEN_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC1HOST_TOKEN1_WR`"]
pub type HOST_SLC1HOST_TOKEN1_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC1HOST_TOKEN1_WR`"]
pub struct HOST_SLC1HOST_TOKEN1_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN1_WR_W<'a> {
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
#[doc = "Reader of field `HOST_SLC1HOST_TOKEN0_WR`"]
pub type HOST_SLC1HOST_TOKEN0_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC1HOST_TOKEN0_WR`"]
pub struct HOST_SLC1HOST_TOKEN0_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN0_WR_W<'a> {
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
#[doc = "Reader of field `HOST_SLC1HOST_TOKEN1_DEC`"]
pub type HOST_SLC1HOST_TOKEN1_DEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC1HOST_TOKEN1_DEC`"]
pub struct HOST_SLC1HOST_TOKEN1_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN1_DEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC1HOST_TOKEN0_DEC`"]
pub type HOST_SLC1HOST_TOKEN0_DEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC1HOST_TOKEN0_DEC`"]
pub struct HOST_SLC1HOST_TOKEN0_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN0_DEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC0HOST_TOKEN1_WR`"]
pub type HOST_SLC0HOST_TOKEN1_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC0HOST_TOKEN1_WR`"]
pub struct HOST_SLC0HOST_TOKEN1_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN1_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC0HOST_TOKEN0_WR`"]
pub type HOST_SLC0HOST_TOKEN0_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC0HOST_TOKEN0_WR`"]
pub struct HOST_SLC0HOST_TOKEN0_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN0_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC0HOST_TOKEN1_DEC`"]
pub type HOST_SLC0HOST_TOKEN1_DEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC0HOST_TOKEN1_DEC`"]
pub struct HOST_SLC0HOST_TOKEN1_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN1_DEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `HOST_SLC0HOST_TOKEN0_DEC`"]
pub type HOST_SLC0HOST_TOKEN0_DEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_SLC0HOST_TOKEN0_DEC`"]
pub struct HOST_SLC0HOST_TOKEN0_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN0_DEC_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc0host_len_wr(&self) -> HOST_SLC0HOST_LEN_WR_R {
        HOST_SLC0HOST_LEN_WR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc1host_token1_wr(&self) -> HOST_SLC1HOST_TOKEN1_WR_R {
        HOST_SLC1HOST_TOKEN1_WR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc1host_token0_wr(&self) -> HOST_SLC1HOST_TOKEN0_WR_R {
        HOST_SLC1HOST_TOKEN0_WR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc1host_token1_dec(&self) -> HOST_SLC1HOST_TOKEN1_DEC_R {
        HOST_SLC1HOST_TOKEN1_DEC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc1host_token0_dec(&self) -> HOST_SLC1HOST_TOKEN0_DEC_R {
        HOST_SLC1HOST_TOKEN0_DEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc0host_token1_wr(&self) -> HOST_SLC0HOST_TOKEN1_WR_R {
        HOST_SLC0HOST_TOKEN1_WR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc0host_token0_wr(&self) -> HOST_SLC0HOST_TOKEN0_WR_R {
        HOST_SLC0HOST_TOKEN0_WR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc0host_token1_dec(&self) -> HOST_SLC0HOST_TOKEN1_DEC_R {
        HOST_SLC0HOST_TOKEN1_DEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc0host_token0_dec(&self) -> HOST_SLC0HOST_TOKEN0_DEC_R {
        HOST_SLC0HOST_TOKEN0_DEC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc0host_len_wr(&mut self) -> HOST_SLC0HOST_LEN_WR_W {
        HOST_SLC0HOST_LEN_WR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc1host_token1_wr(&mut self) -> HOST_SLC1HOST_TOKEN1_WR_W {
        HOST_SLC1HOST_TOKEN1_WR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc1host_token0_wr(&mut self) -> HOST_SLC1HOST_TOKEN0_WR_W {
        HOST_SLC1HOST_TOKEN0_WR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc1host_token1_dec(&mut self) -> HOST_SLC1HOST_TOKEN1_DEC_W {
        HOST_SLC1HOST_TOKEN1_DEC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc1host_token0_dec(&mut self) -> HOST_SLC1HOST_TOKEN0_DEC_W {
        HOST_SLC1HOST_TOKEN0_DEC_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc0host_token1_wr(&mut self) -> HOST_SLC0HOST_TOKEN1_WR_W {
        HOST_SLC0HOST_TOKEN1_WR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc0host_token0_wr(&mut self) -> HOST_SLC0HOST_TOKEN0_WR_W {
        HOST_SLC0HOST_TOKEN0_WR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc0host_token1_dec(&mut self) -> HOST_SLC0HOST_TOKEN1_DEC_W {
        HOST_SLC0HOST_TOKEN1_DEC_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc0host_token0_dec(&mut self) -> HOST_SLC0HOST_TOKEN0_DEC_W {
        HOST_SLC0HOST_TOKEN0_DEC_W { w: self }
    }
}
