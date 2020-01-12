#[doc = "Reader of register TIMING"]
pub type R = crate::R<u32, super::TIMING>;
#[doc = "Writer for register TIMING"]
pub type W = crate::W<u32, super::TIMING>;
#[doc = "Register TIMING `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_BCK_IN_INV`"]
pub type TX_BCK_IN_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BCK_IN_INV`"]
pub struct TX_BCK_IN_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_IN_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DATA_ENABLE_DELAY`"]
pub type DATA_ENABLE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_ENABLE_DELAY`"]
pub struct DATA_ENABLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ENABLE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `RX_DSYNC_SW`"]
pub type RX_DSYNC_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DSYNC_SW`"]
pub struct RX_DSYNC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DSYNC_SW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TX_DSYNC_SW`"]
pub type TX_DSYNC_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DSYNC_SW`"]
pub struct TX_DSYNC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DSYNC_SW_W<'a> {
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
#[doc = "Reader of field `RX_BCK_OUT_DELAY`"]
pub type RX_BCK_OUT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_BCK_OUT_DELAY`"]
pub struct RX_BCK_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `RX_WS_OUT_DELAY`"]
pub type RX_WS_OUT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_WS_OUT_DELAY`"]
pub struct RX_WS_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `TX_SD_OUT_DELAY`"]
pub type TX_SD_OUT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_SD_OUT_DELAY`"]
pub struct TX_SD_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SD_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `TX_WS_OUT_DELAY`"]
pub type TX_WS_OUT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_WS_OUT_DELAY`"]
pub struct TX_WS_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WS_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `TX_BCK_OUT_DELAY`"]
pub type TX_BCK_OUT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_BCK_OUT_DELAY`"]
pub struct TX_BCK_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `RX_SD_IN_DELAY`"]
pub type RX_SD_IN_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_SD_IN_DELAY`"]
pub struct RX_SD_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SD_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RX_WS_IN_DELAY`"]
pub type RX_WS_IN_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_WS_IN_DELAY`"]
pub struct RX_WS_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `RX_BCK_IN_DELAY`"]
pub type RX_BCK_IN_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_BCK_IN_DELAY`"]
pub struct RX_BCK_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `TX_WS_IN_DELAY`"]
pub type TX_WS_IN_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_WS_IN_DELAY`"]
pub struct TX_WS_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WS_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TX_BCK_IN_DELAY`"]
pub type TX_BCK_IN_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_BCK_IN_DELAY`"]
pub struct TX_BCK_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_bck_in_inv(&self) -> TX_BCK_IN_INV_R {
        TX_BCK_IN_INV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn data_enable_delay(&self) -> DATA_ENABLE_DELAY_R {
        DATA_ENABLE_DELAY_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rx_dsync_sw(&self) -> RX_DSYNC_SW_R {
        RX_DSYNC_SW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_dsync_sw(&self) -> TX_DSYNC_SW_R {
        TX_DSYNC_SW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rx_bck_out_delay(&self) -> RX_BCK_OUT_DELAY_R {
        RX_BCK_OUT_DELAY_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rx_ws_out_delay(&self) -> RX_WS_OUT_DELAY_R {
        RX_WS_OUT_DELAY_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tx_sd_out_delay(&self) -> TX_SD_OUT_DELAY_R {
        TX_SD_OUT_DELAY_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tx_ws_out_delay(&self) -> TX_WS_OUT_DELAY_R {
        TX_WS_OUT_DELAY_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tx_bck_out_delay(&self) -> TX_BCK_OUT_DELAY_R {
        TX_BCK_OUT_DELAY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rx_sd_in_delay(&self) -> RX_SD_IN_DELAY_R {
        RX_SD_IN_DELAY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rx_ws_in_delay(&self) -> RX_WS_IN_DELAY_R {
        RX_WS_IN_DELAY_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn rx_bck_in_delay(&self) -> RX_BCK_IN_DELAY_R {
        RX_BCK_IN_DELAY_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_ws_in_delay(&self) -> TX_WS_IN_DELAY_R {
        TX_WS_IN_DELAY_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_bck_in_delay(&self) -> TX_BCK_IN_DELAY_R {
        TX_BCK_IN_DELAY_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_bck_in_inv(&mut self) -> TX_BCK_IN_INV_W {
        TX_BCK_IN_INV_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn data_enable_delay(&mut self) -> DATA_ENABLE_DELAY_W {
        DATA_ENABLE_DELAY_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rx_dsync_sw(&mut self) -> RX_DSYNC_SW_W {
        RX_DSYNC_SW_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_dsync_sw(&mut self) -> TX_DSYNC_SW_W {
        TX_DSYNC_SW_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rx_bck_out_delay(&mut self) -> RX_BCK_OUT_DELAY_W {
        RX_BCK_OUT_DELAY_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rx_ws_out_delay(&mut self) -> RX_WS_OUT_DELAY_W {
        RX_WS_OUT_DELAY_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tx_sd_out_delay(&mut self) -> TX_SD_OUT_DELAY_W {
        TX_SD_OUT_DELAY_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tx_ws_out_delay(&mut self) -> TX_WS_OUT_DELAY_W {
        TX_WS_OUT_DELAY_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tx_bck_out_delay(&mut self) -> TX_BCK_OUT_DELAY_W {
        TX_BCK_OUT_DELAY_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rx_sd_in_delay(&mut self) -> RX_SD_IN_DELAY_W {
        RX_SD_IN_DELAY_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rx_ws_in_delay(&mut self) -> RX_WS_IN_DELAY_W {
        RX_WS_IN_DELAY_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn rx_bck_in_delay(&mut self) -> RX_BCK_IN_DELAY_W {
        RX_BCK_IN_DELAY_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_ws_in_delay(&mut self) -> TX_WS_IN_DELAY_W {
        TX_WS_IN_DELAY_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_bck_in_delay(&mut self) -> TX_BCK_IN_DELAY_W {
        TX_BCK_IN_DELAY_W { w: self }
    }
}
