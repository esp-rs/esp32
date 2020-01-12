#[doc = "Reader of register 0_LEN_CONF"]
pub type R = crate::R<u32, super::_0_LEN_CONF>;
#[doc = "Writer for register 0_LEN_CONF"]
pub type W = crate::W<u32, super::_0_LEN_CONF>;
#[doc = "Register 0_LEN_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::_0_LEN_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC0_TX_NEW_PKT_IND`"]
pub type SLC0_TX_NEW_PKT_IND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_TX_NEW_PKT_IND`"]
pub struct SLC0_TX_NEW_PKT_IND_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_NEW_PKT_IND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SLC0_RX_NEW_PKT_IND`"]
pub type SLC0_RX_NEW_PKT_IND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_RX_NEW_PKT_IND`"]
pub struct SLC0_RX_NEW_PKT_IND_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_NEW_PKT_IND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SLC0_TX_GET_USED_DSCR`"]
pub type SLC0_TX_GET_USED_DSCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_TX_GET_USED_DSCR`"]
pub struct SLC0_TX_GET_USED_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_GET_USED_DSCR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SLC0_RX_GET_USED_DSCR`"]
pub type SLC0_RX_GET_USED_DSCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_RX_GET_USED_DSCR`"]
pub struct SLC0_RX_GET_USED_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_GET_USED_DSCR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SLC0_TX_PACKET_LOAD_EN`"]
pub type SLC0_TX_PACKET_LOAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_TX_PACKET_LOAD_EN`"]
pub struct SLC0_TX_PACKET_LOAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_PACKET_LOAD_EN_W<'a> {
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
#[doc = "Reader of field `SLC0_RX_PACKET_LOAD_EN`"]
pub type SLC0_RX_PACKET_LOAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_RX_PACKET_LOAD_EN`"]
pub struct SLC0_RX_PACKET_LOAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_PACKET_LOAD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SLC0_LEN_INC_MORE`"]
pub type SLC0_LEN_INC_MORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_LEN_INC_MORE`"]
pub struct SLC0_LEN_INC_MORE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_INC_MORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SLC0_LEN_INC`"]
pub type SLC0_LEN_INC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_LEN_INC`"]
pub struct SLC0_LEN_INC_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_INC_W<'a> {
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
#[doc = "Reader of field `SLC0_LEN_WR`"]
pub type SLC0_LEN_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_LEN_WR`"]
pub struct SLC0_LEN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_WR_W<'a> {
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
#[doc = "Reader of field `SLC0_LEN_WDATA`"]
pub type SLC0_LEN_WDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC0_LEN_WDATA`"]
pub struct SLC0_LEN_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc0_tx_new_pkt_ind(&self) -> SLC0_TX_NEW_PKT_IND_R {
        SLC0_TX_NEW_PKT_IND_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn slc0_rx_new_pkt_ind(&self) -> SLC0_RX_NEW_PKT_IND_R {
        SLC0_RX_NEW_PKT_IND_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc0_tx_get_used_dscr(&self) -> SLC0_TX_GET_USED_DSCR_R {
        SLC0_TX_GET_USED_DSCR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc0_rx_get_used_dscr(&self) -> SLC0_RX_GET_USED_DSCR_R {
        SLC0_RX_GET_USED_DSCR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc0_tx_packet_load_en(&self) -> SLC0_TX_PACKET_LOAD_EN_R {
        SLC0_TX_PACKET_LOAD_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc0_rx_packet_load_en(&self) -> SLC0_RX_PACKET_LOAD_EN_R {
        SLC0_RX_PACKET_LOAD_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc0_len_inc_more(&self) -> SLC0_LEN_INC_MORE_R {
        SLC0_LEN_INC_MORE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc0_len_inc(&self) -> SLC0_LEN_INC_R {
        SLC0_LEN_INC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc0_len_wr(&self) -> SLC0_LEN_WR_R {
        SLC0_LEN_WR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_len_wdata(&self) -> SLC0_LEN_WDATA_R {
        SLC0_LEN_WDATA_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc0_tx_new_pkt_ind(&mut self) -> SLC0_TX_NEW_PKT_IND_W {
        SLC0_TX_NEW_PKT_IND_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn slc0_rx_new_pkt_ind(&mut self) -> SLC0_RX_NEW_PKT_IND_W {
        SLC0_RX_NEW_PKT_IND_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc0_tx_get_used_dscr(&mut self) -> SLC0_TX_GET_USED_DSCR_W {
        SLC0_TX_GET_USED_DSCR_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc0_rx_get_used_dscr(&mut self) -> SLC0_RX_GET_USED_DSCR_W {
        SLC0_RX_GET_USED_DSCR_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc0_tx_packet_load_en(&mut self) -> SLC0_TX_PACKET_LOAD_EN_W {
        SLC0_TX_PACKET_LOAD_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc0_rx_packet_load_en(&mut self) -> SLC0_RX_PACKET_LOAD_EN_W {
        SLC0_RX_PACKET_LOAD_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc0_len_inc_more(&mut self) -> SLC0_LEN_INC_MORE_W {
        SLC0_LEN_INC_MORE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc0_len_inc(&mut self) -> SLC0_LEN_INC_W {
        SLC0_LEN_INC_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc0_len_wr(&mut self) -> SLC0_LEN_WR_W {
        SLC0_LEN_WR_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_len_wdata(&mut self) -> SLC0_LEN_WDATA_W {
        SLC0_LEN_WDATA_W { w: self }
    }
}
