#[doc = "Reader of register RX_DSCR_CONF"]
pub type R = crate::R<u32, super::RX_DSCR_CONF>;
#[doc = "Writer for register RX_DSCR_CONF"]
pub type W = crate::W<u32, super::RX_DSCR_CONF>;
#[doc = "Register RX_DSCR_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_DSCR_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC1_RD_RETRY_THRESHOLD`"]
pub type SLC1_RD_RETRY_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC1_RD_RETRY_THRESHOLD`"]
pub struct SLC1_RD_RETRY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RD_RETRY_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 21)) | (((value as u32) & 0x07ff) << 21);
        self.w
    }
}
#[doc = "Reader of field `SLC1_RX_FILL_EN`"]
pub type SLC1_RX_FILL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_RX_FILL_EN`"]
pub struct SLC1_RX_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_FILL_EN_W<'a> {
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
#[doc = "Reader of field `SLC1_RX_EOF_MODE`"]
pub type SLC1_RX_EOF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_RX_EOF_MODE`"]
pub struct SLC1_RX_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_EOF_MODE_W<'a> {
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
#[doc = "Reader of field `SLC1_RX_FILL_MODE`"]
pub type SLC1_RX_FILL_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_RX_FILL_MODE`"]
pub struct SLC1_RX_FILL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_FILL_MODE_W<'a> {
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
#[doc = "Reader of field `SLC1_INFOR_NO_REPLACE`"]
pub type SLC1_INFOR_NO_REPLACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_INFOR_NO_REPLACE`"]
pub struct SLC1_INFOR_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_INFOR_NO_REPLACE_W<'a> {
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
#[doc = "Reader of field `SLC1_TOKEN_NO_REPLACE`"]
pub type SLC1_TOKEN_NO_REPLACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_TOKEN_NO_REPLACE`"]
pub struct SLC1_TOKEN_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TOKEN_NO_REPLACE_W<'a> {
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
#[doc = "Reader of field `SLC0_RD_RETRY_THRESHOLD`"]
pub type SLC0_RD_RETRY_THRESHOLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC0_RD_RETRY_THRESHOLD`"]
pub struct SLC0_RD_RETRY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RD_RETRY_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | (((value as u32) & 0x07ff) << 5);
        self.w
    }
}
#[doc = "Reader of field `SLC0_RX_FILL_EN`"]
pub type SLC0_RX_FILL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_RX_FILL_EN`"]
pub struct SLC0_RX_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_FILL_EN_W<'a> {
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
#[doc = "Reader of field `SLC0_RX_EOF_MODE`"]
pub type SLC0_RX_EOF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_RX_EOF_MODE`"]
pub struct SLC0_RX_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_EOF_MODE_W<'a> {
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
#[doc = "Reader of field `SLC0_RX_FILL_MODE`"]
pub type SLC0_RX_FILL_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_RX_FILL_MODE`"]
pub struct SLC0_RX_FILL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_FILL_MODE_W<'a> {
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
#[doc = "Reader of field `SLC0_INFOR_NO_REPLACE`"]
pub type SLC0_INFOR_NO_REPLACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_INFOR_NO_REPLACE`"]
pub struct SLC0_INFOR_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_INFOR_NO_REPLACE_W<'a> {
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
#[doc = "Reader of field `SLC0_TOKEN_NO_REPLACE`"]
pub type SLC0_TOKEN_NO_REPLACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_TOKEN_NO_REPLACE`"]
pub struct SLC0_TOKEN_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TOKEN_NO_REPLACE_W<'a> {
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
    pub fn slc1_rd_retry_threshold(&self) -> SLC1_RD_RETRY_THRESHOLD_R {
        SLC1_RD_RETRY_THRESHOLD_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_fill_en(&self) -> SLC1_RX_FILL_EN_R {
        SLC1_RX_FILL_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_rx_eof_mode(&self) -> SLC1_RX_EOF_MODE_R {
        SLC1_RX_EOF_MODE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_fill_mode(&self) -> SLC1_RX_FILL_MODE_R {
        SLC1_RX_FILL_MODE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_infor_no_replace(&self) -> SLC1_INFOR_NO_REPLACE_R {
        SLC1_INFOR_NO_REPLACE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_token_no_replace(&self) -> SLC1_TOKEN_NO_REPLACE_R {
        SLC1_TOKEN_NO_REPLACE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc0_rd_retry_threshold(&self) -> SLC0_RD_RETRY_THRESHOLD_R {
        SLC0_RD_RETRY_THRESHOLD_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_rx_fill_en(&self) -> SLC0_RX_FILL_EN_R {
        SLC0_RX_FILL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc0_rx_eof_mode(&self) -> SLC0_RX_EOF_MODE_R {
        SLC0_RX_EOF_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_fill_mode(&self) -> SLC0_RX_FILL_MODE_R {
        SLC0_RX_FILL_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_infor_no_replace(&self) -> SLC0_INFOR_NO_REPLACE_R {
        SLC0_INFOR_NO_REPLACE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_token_no_replace(&self) -> SLC0_TOKEN_NO_REPLACE_R {
        SLC0_TOKEN_NO_REPLACE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn slc1_rd_retry_threshold(&mut self) -> SLC1_RD_RETRY_THRESHOLD_W {
        SLC1_RD_RETRY_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_fill_en(&mut self) -> SLC1_RX_FILL_EN_W {
        SLC1_RX_FILL_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_rx_eof_mode(&mut self) -> SLC1_RX_EOF_MODE_W {
        SLC1_RX_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_fill_mode(&mut self) -> SLC1_RX_FILL_MODE_W {
        SLC1_RX_FILL_MODE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_infor_no_replace(&mut self) -> SLC1_INFOR_NO_REPLACE_W {
        SLC1_INFOR_NO_REPLACE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_token_no_replace(&mut self) -> SLC1_TOKEN_NO_REPLACE_W {
        SLC1_TOKEN_NO_REPLACE_W { w: self }
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc0_rd_retry_threshold(&mut self) -> SLC0_RD_RETRY_THRESHOLD_W {
        SLC0_RD_RETRY_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_rx_fill_en(&mut self) -> SLC0_RX_FILL_EN_W {
        SLC0_RX_FILL_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc0_rx_eof_mode(&mut self) -> SLC0_RX_EOF_MODE_W {
        SLC0_RX_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_fill_mode(&mut self) -> SLC0_RX_FILL_MODE_W {
        SLC0_RX_FILL_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_infor_no_replace(&mut self) -> SLC0_INFOR_NO_REPLACE_W {
        SLC0_INFOR_NO_REPLACE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_token_no_replace(&mut self) -> SLC0_TOKEN_NO_REPLACE_W {
        SLC0_TOKEN_NO_REPLACE_W { w: self }
    }
}
