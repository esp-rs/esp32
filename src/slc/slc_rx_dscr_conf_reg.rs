#[doc = "Reader of register SLC_RX_DSCR_CONF_REG"]
pub type R = crate::R<u32, super::SLC_RX_DSCR_CONF_REG>;
#[doc = "Writer for register SLC_RX_DSCR_CONF_REG"]
pub type W = crate::W<u32, super::SLC_RX_DSCR_CONF_REG>;
#[doc = "Register SLC_RX_DSCR_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_RX_DSCR_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC1_RD_RETRY_THRESHOLD`"]
pub type SLC_SLC1_RD_RETRY_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_SLC1_RD_RETRY_THRESHOLD`"]
pub struct SLC_SLC1_RD_RETRY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RD_RETRY_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 21)) | (((value as u32) & 0x07ff) << 21);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RX_FILL_EN`"]
pub type SLC_SLC1_RX_FILL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_FILL_EN`"]
pub struct SLC_SLC1_RX_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_FILL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RX_EOF_MODE`"]
pub type SLC_SLC1_RX_EOF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_EOF_MODE`"]
pub struct SLC_SLC1_RX_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_EOF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RX_FILL_MODE`"]
pub type SLC_SLC1_RX_FILL_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RX_FILL_MODE`"]
pub struct SLC_SLC1_RX_FILL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RX_FILL_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_INFOR_NO_REPLACE`"]
pub type SLC_SLC1_INFOR_NO_REPLACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_INFOR_NO_REPLACE`"]
pub struct SLC_SLC1_INFOR_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_INFOR_NO_REPLACE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_TOKEN_NO_REPLACE`"]
pub type SLC_SLC1_TOKEN_NO_REPLACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TOKEN_NO_REPLACE`"]
pub struct SLC_SLC1_TOKEN_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TOKEN_NO_REPLACE_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RD_RETRY_THRESHOLD`"]
pub type SLC_SLC0_RD_RETRY_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_SLC0_RD_RETRY_THRESHOLD`"]
pub struct SLC_SLC0_RD_RETRY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RD_RETRY_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | (((value as u32) & 0x07ff) << 5);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC0_RX_FILL_EN`"]
pub type SLC_SLC0_RX_FILL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_FILL_EN`"]
pub struct SLC_SLC0_RX_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_FILL_EN_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_EOF_MODE`"]
pub type SLC_SLC0_RX_EOF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_EOF_MODE`"]
pub struct SLC_SLC0_RX_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_EOF_MODE_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_RX_FILL_MODE`"]
pub type SLC_SLC0_RX_FILL_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_RX_FILL_MODE`"]
pub struct SLC_SLC0_RX_FILL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_FILL_MODE_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_INFOR_NO_REPLACE`"]
pub type SLC_SLC0_INFOR_NO_REPLACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_INFOR_NO_REPLACE`"]
pub struct SLC_SLC0_INFOR_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_INFOR_NO_REPLACE_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TOKEN_NO_REPLACE`"]
pub type SLC_SLC0_TOKEN_NO_REPLACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TOKEN_NO_REPLACE`"]
pub struct SLC_SLC0_TOKEN_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TOKEN_NO_REPLACE_W<'a> {
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
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn slc_slc1_rd_retry_threshold(&self) -> SLC_SLC1_RD_RETRY_THRESHOLD_R {
        SLC_SLC1_RD_RETRY_THRESHOLD_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc1_rx_fill_en(&self) -> SLC_SLC1_RX_FILL_EN_R {
        SLC_SLC1_RX_FILL_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_slc1_rx_eof_mode(&self) -> SLC_SLC1_RX_EOF_MODE_R {
        SLC_SLC1_RX_EOF_MODE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_slc1_rx_fill_mode(&self) -> SLC_SLC1_RX_FILL_MODE_R {
        SLC_SLC1_RX_FILL_MODE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_slc1_infor_no_replace(&self) -> SLC_SLC1_INFOR_NO_REPLACE_R {
        SLC_SLC1_INFOR_NO_REPLACE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc1_token_no_replace(&self) -> SLC_SLC1_TOKEN_NO_REPLACE_R {
        SLC_SLC1_TOKEN_NO_REPLACE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc_slc0_rd_retry_threshold(&self) -> SLC_SLC0_RD_RETRY_THRESHOLD_R {
        SLC_SLC0_RD_RETRY_THRESHOLD_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_slc0_rx_fill_en(&self) -> SLC_SLC0_RX_FILL_EN_R {
        SLC_SLC0_RX_FILL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_slc0_rx_eof_mode(&self) -> SLC_SLC0_RX_EOF_MODE_R {
        SLC_SLC0_RX_EOF_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_slc0_rx_fill_mode(&self) -> SLC_SLC0_RX_FILL_MODE_R {
        SLC_SLC0_RX_FILL_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_slc0_infor_no_replace(&self) -> SLC_SLC0_INFOR_NO_REPLACE_R {
        SLC_SLC0_INFOR_NO_REPLACE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_slc0_token_no_replace(&self) -> SLC_SLC0_TOKEN_NO_REPLACE_R {
        SLC_SLC0_TOKEN_NO_REPLACE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn slc_slc1_rd_retry_threshold(&mut self) -> SLC_SLC1_RD_RETRY_THRESHOLD_W {
        SLC_SLC1_RD_RETRY_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc1_rx_fill_en(&mut self) -> SLC_SLC1_RX_FILL_EN_W {
        SLC_SLC1_RX_FILL_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc_slc1_rx_eof_mode(&mut self) -> SLC_SLC1_RX_EOF_MODE_W {
        SLC_SLC1_RX_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc_slc1_rx_fill_mode(&mut self) -> SLC_SLC1_RX_FILL_MODE_W {
        SLC_SLC1_RX_FILL_MODE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc_slc1_infor_no_replace(&mut self) -> SLC_SLC1_INFOR_NO_REPLACE_W {
        SLC_SLC1_INFOR_NO_REPLACE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_slc1_token_no_replace(&mut self) -> SLC_SLC1_TOKEN_NO_REPLACE_W {
        SLC_SLC1_TOKEN_NO_REPLACE_W { w: self }
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc_slc0_rd_retry_threshold(&mut self) -> SLC_SLC0_RD_RETRY_THRESHOLD_W {
        SLC_SLC0_RD_RETRY_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_slc0_rx_fill_en(&mut self) -> SLC_SLC0_RX_FILL_EN_W {
        SLC_SLC0_RX_FILL_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_slc0_rx_eof_mode(&mut self) -> SLC_SLC0_RX_EOF_MODE_W {
        SLC_SLC0_RX_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_slc0_rx_fill_mode(&mut self) -> SLC_SLC0_RX_FILL_MODE_W {
        SLC_SLC0_RX_FILL_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_slc0_infor_no_replace(&mut self) -> SLC_SLC0_INFOR_NO_REPLACE_W {
        SLC_SLC0_INFOR_NO_REPLACE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_slc0_token_no_replace(&mut self) -> SLC_SLC0_TOKEN_NO_REPLACE_W {
        SLC_SLC0_TOKEN_NO_REPLACE_W { w: self }
    }
}
